#![allow(dead_code)]
use crate::core::ann_index;
use crate::core::metrics;
use crate::core::neighbor::Neighbor;
use crate::core::node;
use crate::index::hnsw_params::HNSWParams;
use crate::into_iter;
use fixedbitset::FixedBitSet;
use rand::prelude::*;
#[cfg(not(feature = "no_thread"))]
use rayon::prelude::*;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::collections::BinaryHeap;

use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::Write;

use std::sync::RwLock;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct HNSWIndex<T: node::IdxType, N: node::Node<T = T>> {
    _dimension: usize, // dimension
    _n_items: usize,   // next item count
    _n_constructed_items: usize,
    _max_item: usize,
    _n_neighbor: usize,  // neighbor num except level 0
    _n_neighbor0: usize, // neight num of level 0
    _max_level: usize,   //max level
    _cur_level: usize,   //current level
    #[serde(skip_serializing, skip_deserializing)]
    _id2neighbor: Vec<Vec<RwLock<Vec<usize>>>>, //neight_id from level 1 to level _max_level
    #[serde(skip_serializing, skip_deserializing)]
    _id2neighbor0: Vec<RwLock<Vec<usize>>>, //neigh_id at level 0
    #[serde(skip_serializing, skip_deserializing)]
    _nodes: Vec<Box<N>>, // data saver
    #[serde(skip_serializing, skip_deserializing)]
    _item2id: HashMap<T, usize>, //item_id to id in Hnsw
    _root_id: usize,     //root of hnsw
    _id2level: Vec<usize>,
    _has_removed: bool,
    _ef_build: usize,  // num of max candidates when building
    _ef_search: usize, // num of max candidates when searching
    #[serde(skip_serializing, skip_deserializing)]
    _delete_ids: HashSet<usize>, //save deleted ids
    mt: metrics::Metric, //compute metrics

    // use for serde
    _id2neighbor_tmp: Vec<Vec<Vec<usize>>>,
    _id2neighbor0_tmp: Vec<Vec<usize>>,
    _nodes_tmp: Vec<N>,
    _item2id_tmp: Vec<(T, usize)>,
    _delete_ids_tmp: Vec<usize>,
}

impl<E: node::FloatElement, T: node::IdxType, N: node::Node<E = E, T = T>> HNSWIndex<T, N> {
    pub fn new(dimension: usize, params: &HNSWParams<E>) -> HNSWIndex<T, N> {
        HNSWIndex {
            _dimension: dimension,
            _n_items: 0,
            _n_constructed_items: 0,
            _max_item: params.max_item,
            _n_neighbor: params.n_neighbor,
            _n_neighbor0: params.n_neighbor0,
            _max_level: params.max_level,
            _cur_level: 0,
            _root_id: 0,
            _has_removed: params.has_deletion,
            _ef_build: params.ef_build,
            _ef_search: params.ef_search,
            mt: metrics::Metric::Unknown,
            ..Default::default()
        }
    }

    fn get_random_level(&self) -> usize {
        let mut rng = rand::thread_rng();
        let mut ret = 0;
        while ret < self._max_level {
            if rng.gen_range(0.0..1.0) > 0.5 {
                ret += 1;
            } else {
                break;
            }
        }
        ret
    }
    //input top_candidate as max top heap
    //return min top heap in top_candidates, delete part candidate
    fn get_neighbors_by_heuristic2(
        &self,
        sorted_list: &[Neighbor<E, usize>],
        ret_size: usize,
    ) -> Vec<Neighbor<E, usize>> {
        let sorted_list_len = sorted_list.len();
        let mut return_list: Vec<Neighbor<E, usize>> = Vec::with_capacity(sorted_list_len);

        for iter in sorted_list.iter() {
            if return_list.len() >= ret_size {
                break;
            }

            let idx = iter.idx();
            let distance = iter._distance;
            if sorted_list_len < ret_size {
                return_list.push(Neighbor::new(idx, distance));
                continue;
            }

            let mut good = true;

            for ret_neighbor in return_list.iter() {
                let cur2ret_dis = self.get_distance_from_id(idx, ret_neighbor.idx());
                if cur2ret_dis < distance {
                    good = false;
                    break;
                }
            }

            if good {
                return_list.push(Neighbor::new(idx, distance));
            }
        }

        return_list // from small to large
    }

    fn get_neighbor(&self, id: usize, level: usize) -> &RwLock<Vec<usize>> {
        if level == 0 {
            return &self._id2neighbor0[id];
        }
        &self._id2neighbor[id][level - 1]
    }

    #[allow(dead_code)]
    fn get_level(&self, id: usize) -> usize {
        self._id2level[id]
    }

    fn connect_neighbor(
        &self,
        cur_id: usize,
        sorted_candidates: &[Neighbor<E, usize>],
        level: usize,
        is_update: bool,
    ) -> Result<usize, &'static str> {
        let n_neigh = if level == 0 {
            self._n_neighbor0
        } else {
            self._n_neighbor
        };
        let selected_neighbors = self.get_neighbors_by_heuristic2(sorted_candidates, n_neigh);
        if selected_neighbors.len() > n_neigh {
            return Err("Should be not be more than M_ candidates returned by the heuristic");
        }
        if selected_neighbors.is_empty() {
            return Err("top candidate is empty, impossible!");
        }

        let next_closest_entry_point = selected_neighbors[0].idx();

        {
            let mut cur_neigh = self.get_neighbor(cur_id, level).write().unwrap();
            cur_neigh.clear();
            selected_neighbors.iter().for_each(|selected_neighbor| {
                cur_neigh.push(selected_neighbor.idx());
            });
        }

        for selected_neighbor in selected_neighbors.iter() {
            let mut neighbor_of_selected_neighbors = self
                .get_neighbor(selected_neighbor.idx(), level)
                .write()
                .unwrap();
            if neighbor_of_selected_neighbors.len() > n_neigh {
                return Err("Bad Value of neighbor_of_selected_neighbors");
            }
            if selected_neighbor.idx() == cur_id {
                return Err("Trying to connect an element to itself");
            }

            let mut is_cur_id_present = false;

            if is_update {
                for iter in neighbor_of_selected_neighbors.iter() {
                    if *iter == cur_id {
                        is_cur_id_present = true;
                        break;
                    }
                }
            }

            if !is_cur_id_present {
                if neighbor_of_selected_neighbors.len() < n_neigh {
                    neighbor_of_selected_neighbors.push(cur_id);
                } else {
                    let d_max = self.get_distance_from_id(cur_id, selected_neighbor.idx());

                    let mut candidates: BinaryHeap<Neighbor<E, usize>> = BinaryHeap::new();
                    candidates.push(Neighbor::new(cur_id, d_max));
                    for iter in neighbor_of_selected_neighbors.iter() {
                        let neighbor_id = *iter;
                        let d_neigh =
                            self.get_distance_from_id(neighbor_id, selected_neighbor.idx());
                        candidates.push(Neighbor::new(neighbor_id, d_neigh));
                    }
                    let return_list =
                        self.get_neighbors_by_heuristic2(&candidates.into_sorted_vec(), n_neigh);

                    neighbor_of_selected_neighbors.clear();
                    for neighbor_in_list in return_list {
                        neighbor_of_selected_neighbors.push(neighbor_in_list.idx());
                    }
                }
            }
        }

        Ok(next_closest_entry_point)
    }

    #[allow(dead_code)]
    fn delete_id(&mut self, id: usize) -> Result<(), &'static str> {
        if id > self._n_constructed_items {
            return Err("Invalid delete id");
        }
        if self.is_deleted(id) {
            return Err("id has deleted");
        }
        self._delete_ids.insert(id);
        Ok(())
    }

    fn is_deleted(&self, id: usize) -> bool {
        self._has_removed && self._delete_ids.contains(&id)
    }

    fn get_data(&self, id: usize) -> &N {
        &self._nodes[id]
    }

    fn get_distance_from_vec(&self, x: &N, y: &N) -> E {
        return metrics::metric(x.vectors(), y.vectors(), self.mt).unwrap();
    }

    fn get_distance_from_id(&self, x: usize, y: usize) -> E {
        return metrics::metric(
            self.get_data(x).vectors(),
            self.get_data(y).vectors(),
            self.mt,
        )
        .unwrap();
    }

    fn search_layer_with_candidate(
        &self,
        search_data: &N,
        sorted_candidates: &[Neighbor<E, usize>],
        visited_id: &mut FixedBitSet,
        level: usize,
        ef: usize,
        has_deletion: bool,
    ) -> BinaryHeap<Neighbor<E, usize>> {
        let mut candidates: BinaryHeap<Neighbor<E, usize>> = BinaryHeap::new();
        let mut top_candidates: BinaryHeap<Neighbor<E, usize>> = BinaryHeap::new();
        for neighbor in sorted_candidates.iter() {
            let root = neighbor.idx();
            if !has_deletion || !self.is_deleted(root) {
                let dist = self.get_distance_from_vec(self.get_data(root), search_data);
                top_candidates.push(Neighbor::new(root, dist));
                candidates.push(Neighbor::new(root, -dist));
            } else {
                candidates.push(Neighbor::new(root, -E::max_value()))
            }
            visited_id.insert(root);
        }
        let mut lower_bound = if top_candidates.is_empty() {
            E::max_value() //max dist in top_candidates
        } else {
            top_candidates.peek().unwrap()._distance
        };

        while !candidates.is_empty() {
            let cur_neigh = candidates.peek().unwrap();
            let cur_dist = -cur_neigh._distance;
            let cur_id = cur_neigh.idx();
            candidates.pop();
            if cur_dist > lower_bound {
                break;
            }
            let cur_neighbors = self.get_neighbor(cur_id, level).read().unwrap();
            cur_neighbors.iter().for_each(|neigh| {
                if visited_id.contains(*neigh) {
                    return;
                }
                visited_id.insert(*neigh);
                let dist = self.get_distance_from_vec(self.get_data(*neigh), search_data);
                if top_candidates.len() < ef || dist < lower_bound {
                    candidates.push(Neighbor::new(*neigh, -dist));

                    if !self.is_deleted(*neigh) {
                        top_candidates.push(Neighbor::new(*neigh, dist))
                    }

                    if top_candidates.len() > ef {
                        top_candidates.pop();
                    }

                    if !top_candidates.is_empty() {
                        lower_bound = top_candidates.peek().unwrap()._distance;
                    }
                }
            });
        }

        top_candidates
    }
    //find ef nearist nodes to search data from root at level
    fn search_layer(
        &self,
        root: usize,
        search_data: &N,
        level: usize,
        ef: usize,
        has_deletion: bool,
    ) -> BinaryHeap<Neighbor<E, usize>> {
        let mut visited_id = FixedBitSet::with_capacity(self._nodes.len());
        let mut top_candidates: BinaryHeap<Neighbor<E, usize>> = BinaryHeap::new();
        let mut candidates: BinaryHeap<Neighbor<E, usize>> = BinaryHeap::new();
        let mut lower_bound: E;

        if !has_deletion || !self.is_deleted(root) {
            let dist = self.get_distance_from_vec(self.get_data(root), search_data);
            top_candidates.push(Neighbor::new(root, dist));
            candidates.push(Neighbor::new(root, -dist));
            lower_bound = dist;
        } else {
            lower_bound = E::max_value(); //max dist in top_candidates
            candidates.push(Neighbor::new(root, -lower_bound))
        }
        visited_id.insert(root);

        while !candidates.is_empty() {
            let cur_neigh = candidates.peek().unwrap();
            let cur_dist = -cur_neigh._distance;
            let cur_id = cur_neigh.idx();
            candidates.pop();
            if cur_dist > lower_bound {
                break;
            }
            let cur_neighbors = self.get_neighbor(cur_id, level).read().unwrap();
            cur_neighbors.iter().for_each(|neigh| {
                if visited_id.contains(*neigh) {
                    return;
                }
                visited_id.insert(*neigh);
                let dist = self.get_distance_from_vec(self.get_data(*neigh), search_data);
                if top_candidates.len() < ef || dist < lower_bound {
                    candidates.push(Neighbor::new(*neigh, -dist));

                    if !self.is_deleted(*neigh) {
                        top_candidates.push(Neighbor::new(*neigh, dist))
                    }

                    if top_candidates.len() > ef {
                        top_candidates.pop();
                    }

                    if !top_candidates.is_empty() {
                        lower_bound = top_candidates.peek().unwrap()._distance;
                    }
                }
            });
        }

        top_candidates
    }

    // fn search_layer_default(
    //     &self,
    //     root: usize,
    //     search_data: &N,
    //     level: usize,
    // ) -> BinaryHeap<Neighbor<E, usize>> {
    //     return self.search_layer(root, search_data, level, self._ef_build, false);
    // }

    fn search_knn(
        &self,
        search_data: &N,
        k: usize,
    ) -> Result<BinaryHeap<Neighbor<E, usize>>, &'static str> {
        let mut top_candidate: BinaryHeap<Neighbor<E, usize>> = BinaryHeap::new();
        if self._n_constructed_items == 0 {
            return Ok(top_candidate);
        }
        let mut cur_id = self._root_id;
        let mut cur_dist = self.get_distance_from_vec(self.get_data(cur_id), search_data);
        let mut cur_level = self._cur_level;
        loop {
            let mut changed = true;
            while changed {
                changed = false;
                let cur_neighs = self
                    .get_neighbor(cur_id, cur_level as usize)
                    .read()
                    .unwrap();
                for neigh in cur_neighs.iter() {
                    if *neigh > self._max_item {
                        return Err("cand error");
                    }
                    let dist = self.get_distance_from_vec(self.get_data(cur_id), search_data);
                    if dist < cur_dist {
                        cur_dist = dist;
                        cur_id = *neigh;
                        changed = true;
                    }
                }
            }
            if cur_level == 0 {
                break;
            }
            cur_level -= 1;
        }

        let search_range = if self._ef_search > k {
            self._ef_search
        } else {
            k
        };

        top_candidate = self.search_layer(cur_id, search_data, 0, search_range, self._has_removed);
        while top_candidate.len() > k {
            top_candidate.pop();
        }

        Ok(top_candidate)
    }

    fn init_item(&mut self, data: &N) -> usize {
        let cur_id = self._n_items;
        let mut cur_level = self.get_random_level();
        if cur_id == 0 {
            cur_level = self._max_level;
            self._cur_level = cur_level;
            self._root_id = cur_id;
        }
        let neigh0: RwLock<Vec<usize>> = RwLock::new(Vec::with_capacity(self._n_neighbor0));
        let mut neigh: Vec<RwLock<Vec<usize>>> = Vec::with_capacity(cur_level);
        for _i in 0..cur_level {
            let level_neigh: RwLock<Vec<usize>> = RwLock::new(Vec::with_capacity(self._n_neighbor));
            neigh.push(level_neigh);
        }
        self._nodes.push(Box::new(data.clone()));
        self._id2neighbor0.push(neigh0);
        self._id2neighbor.push(neigh);
        self._id2level.push(cur_level);
        // self._item2id.insert(data.idx().unwrap(), cur_id);
        self._n_items += 1;
        cur_id
    }

    fn batch_construct(&mut self, _mt: metrics::Metric) -> Result<(), &'static str> {
        if self._n_items < self._n_constructed_items {
            return Err("contruct error");
        }

        into_iter!((self._n_constructed_items..self._n_items), ctr);
        ctr.for_each(|insert_id: usize| {
            self.construct_single_item(insert_id).unwrap();
        });

        self._n_constructed_items = self._n_items;
        Ok(())
    }

    fn add_item_not_constructed(&mut self, data: &N) -> Result<(), &'static str> {
        if data.len() != self._dimension {
            return Err("dimension is different");
        }
        {
            // if self._item2id.contains_key(data.idx().unwrap()) {
            //     //to_do update point
            //     return Ok(self._item2id[data.idx().unwrap()]);
            // }

            if self._n_items >= self._max_item {
                return Err("The number of elements exceeds the specified limit");
            }
        }

        let insert_id = self.init_item(data);
        let _insert_level = self.get_level(insert_id);
        Ok(())
    }

    fn add_single_item(&mut self, data: &N) -> Result<(), &'static str> {
        //not support asysn
        if data.len() != self._dimension {
            return Err("dimension is different");
        }
        {
            // if self._item2id.contains_key(data.idx().unwrap()) {
            //     //to_do update point
            //     return Ok(self._item2id[data.idx().unwrap()]);
            // }

            if self._n_items >= self._max_item {
                return Err("The number of elements exceeds the specified limit");
            }
        }

        let insert_id = self.init_item(data);
        let _insert_level = self.get_level(insert_id);
        self.construct_single_item(insert_id).unwrap();

        self._n_constructed_items += 1;

        Ok(())
    }

    fn construct_single_item(&self, insert_id: usize) -> Result<(), &'static str> {
        let insert_level = self._id2level[insert_id];
        let mut cur_id = self._root_id;

        if insert_id == 0 {
            return Ok(());
        }

        if insert_level < self._cur_level {
            let mut cur_dist = self.get_distance_from_id(cur_id, insert_id);
            let mut cur_level = self._cur_level;
            while cur_level > insert_level {
                let mut changed = true;
                while changed {
                    changed = false;
                    let cur_neighs = self.get_neighbor(cur_id, cur_level).read().unwrap();
                    for cur_neigh in cur_neighs.iter() {
                        if *cur_neigh > self._n_items {
                            return Err("cand error");
                        }
                        let neigh_dist = self.get_distance_from_id(*cur_neigh, insert_id);
                        if neigh_dist < cur_dist {
                            cur_dist = neigh_dist;
                            cur_id = *cur_neigh;
                            changed = true;
                        }
                    }
                }
                cur_level -= 1;
            }
        }

        let mut level = if insert_level < self._cur_level {
            insert_level
        } else {
            self._cur_level
        };
        let mut visited_id = FixedBitSet::with_capacity(self._nodes.len());
        let mut sorted_candidates: Vec<Neighbor<E, usize>> = Vec::new();
        let insert_data = self.get_data(insert_id);
        visited_id.insert(insert_id);
        sorted_candidates.push(Neighbor::new(
            cur_id,
            self.get_distance_from_id(cur_id, insert_id),
        ));
        loop {
            // let mut visited_id: HashSet<usize> = HashSet::new();
            let mut top_candidates = self.search_layer_with_candidate(
                insert_data,
                &sorted_candidates,
                &mut visited_id,
                level,
                self._ef_build,
                false,
            );
            // let mut top_candidates = self.search_layer_default(cur_id, insert_data, level);
            if self.is_deleted(cur_id) {
                let cur_dist = self.get_distance_from_id(cur_id, insert_id);
                top_candidates.push(Neighbor::new(cur_id, cur_dist));
                if top_candidates.len() > self._ef_build {
                    top_candidates.pop();
                }
            }
            sorted_candidates = top_candidates.into_sorted_vec();
            if sorted_candidates.is_empty() {
                return Err("sorted sorted_candidate is empty");
            }
            cur_id = self
                .connect_neighbor(insert_id, &sorted_candidates, level, false)
                .unwrap();
            if level == 0 {
                break;
            }
            level -= 1;
        }
        Ok(())
    }
}

impl<E: node::FloatElement, T: node::IdxType, N: node::Node<E = E, T = T>>
    ann_index::ANNIndex<E, T, N> for HNSWIndex<T, N>
{
    fn build(&mut self, mt: metrics::Metric) -> Result<(), &'static str> {
        self.mt = mt;
        self.batch_construct(mt)
    }
    fn add_node(&mut self, item: &N) -> Result<(), &'static str> {
        self.add_item_not_constructed(item)
    }
    fn built(&self) -> bool {
        true
    }

    fn node_search_k(&self, item: &N, k: usize) -> Vec<(N, E)> {
        let mut ret: BinaryHeap<Neighbor<E, usize>> = self.search_knn(item, k).unwrap();
        let mut result: Vec<(N, E)> = Vec::with_capacity(k);
        let mut result_idx: Vec<(usize, E)> = Vec::with_capacity(k);
        while !ret.is_empty() {
            let top = ret.peek().unwrap();
            let top_idx = top.idx();
            let top_distance = top.distance();
            ret.pop();
            result_idx.push((top_idx, top_distance))
        }
        for i in 0..result_idx.len() {
            let cur_id = result_idx.len() - i - 1;
            result.push((
                *self._nodes[result_idx[cur_id].0].clone(),
                result_idx[cur_id].1,
            ));
        }
        result
    }

    fn name(&self) -> &'static str {
        "HNSWIndex"
    }

    fn dimension(&self) -> usize {
        self._dimension
    }
}

impl<
        E: node::FloatElement + DeserializeOwned,
        T: node::IdxType + DeserializeOwned,
        N: node::Node<E = E, T = T>,
    > ann_index::SerializableIndex<E, T, N> for HNSWIndex<T, N>
{
    fn load(path: &str) -> Result<Self, &'static str> {
        let file = File::open(path).unwrap_or_else(|_| panic!("unable to open file {:?}", path));
        let mut instance: HNSWIndex<E, T> = bincode::deserialize_from(&file).unwrap();
        instance._nodes = instance
            ._nodes_tmp
            .iter()
            .map(|x| Box::new(x.clone()))
            .collect();
        instance._id2neighbor = Vec::with_capacity(instance._id2neighbor_tmp.len());
        for i in 0..instance._id2neighbor_tmp.len() {
            let mut tmp = Vec::with_capacity(instance._id2neighbor_tmp[i].len());
            for j in 0..instance._id2neighbor_tmp[i].len() {
                tmp.push(RwLock::new(instance._id2neighbor_tmp[i][j].clone()));
            }
            instance._id2neighbor.push(tmp);
        }
        instance._id2neighbor0 = Vec::with_capacity(instance._id2neighbor0_tmp.len());
        for i in 0..instance._id2neighbor0_tmp.len() {
            instance
                ._id2neighbor0
                .push(RwLock::new(instance._id2neighbor0_tmp[i].clone()));
        }

        instance._item2id = HashMap::new();
        for iter in instance._item2id_tmp.iter() {
            let (k, v) = &*iter;
            instance._item2id.insert(k.clone(), *v);
        }

        instance._delete_ids = HashSet::new();
        for iter in instance._delete_ids_tmp.iter() {
            instance._delete_ids.insert(*iter);
        }
        instance._id2neighbor_tmp.clear();
        instance._id2neighbor0_tmp.clear();
        instance._nodes_tmp.clear();
        instance._item2id_tmp.clear();
        instance._delete_ids_tmp.clear();
        Ok(instance)
    }

    fn dump(&mut self, path: &str) -> Result<(), &'static str> {
        self._id2neighbor_tmp = Vec::with_capacity(self._id2neighbor.len());
        for i in 0..self._id2neighbor.len() {
            let mut tmp = Vec::with_capacity(self._id2neighbor[i].len());
            for j in 0..self._id2neighbor[i].len() {
                tmp.push(self._id2neighbor[i][j].read().unwrap().clone());
            }
            self._id2neighbor_tmp.push(tmp);
        }

        self._id2neighbor0_tmp = Vec::with_capacity(self._id2neighbor0.len());
        for i in 0..self._id2neighbor0.len() {
            self._id2neighbor0_tmp
                .push(self._id2neighbor0[i].read().unwrap().clone());
        }

        self._nodes_tmp = self._nodes.iter().map(|x| *x.clone()).collect();
        self._item2id_tmp = Vec::with_capacity(self._item2id.len());
        for (k, v) in &self._item2id {
            self._item2id_tmp.push((k.clone(), *v));
        }
        self._delete_ids_tmp = Vec::new();
        for iter in &self._delete_ids {
            self._delete_ids_tmp.push(*iter);
        }

        let encoded_bytes = bincode::serialize(&self).unwrap();
        let mut file = File::create(path).unwrap();
        file.write_all(&encoded_bytes)
            .unwrap_or_else(|_| panic!("unable to write file {:?}", path));
        Result::Ok(())
    }
}
