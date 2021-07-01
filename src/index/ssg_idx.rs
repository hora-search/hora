#![allow(dead_code)]
use crate::core::ann_index;
use crate::core::kmeans;
use crate::core::metrics;
use crate::core::neighbor;
use crate::core::node;
use crate::index::ssg_params::SSGParams;
use fixedbitset::FixedBitSet;
#[cfg(feature = "without_std")]
use hashbrown::HashSet;
use rand::prelude::*;
use rayon::prelude::*;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
#[cfg(not(feature = "without_std"))]
use std::collections::HashSet;
use std::collections::LinkedList;
use std::collections::VecDeque;

use std::fs::File;
use std::io::Write;
use std::sync::{Arc, Mutex};

#[derive(Debug, Serialize, Deserialize)]
pub struct SSGIndex<E: node::FloatElement, T: node::IdxType> {
    #[serde(skip_serializing, skip_deserializing)]
    nodes: Vec<Box<node::Node<E, T>>>,
    tmp_nodes: Vec<node::Node<E, T>>, // only use for serialization scene
    mt: metrics::Metric,
    dimension: usize,
    neighbor_neighbor_size: usize,
    index_size: usize,
    graph: Vec<Vec<usize>>,
    knn_graph: Vec<Vec<usize>>,
    init_k: usize, // as knn's k
    root_nodes: Vec<usize>,
    width: usize,
    angle: E,
    threshold: E,
    root_size: usize,

    // stat
    search_times: usize,
}

impl<E: node::FloatElement, T: node::IdxType> SSGIndex<E, T> {
    pub fn new(dimension: usize, params: &SSGParams<E>) -> SSGIndex<E, T> {
        SSGIndex::<E, T> {
            nodes: Vec::new(),
            tmp_nodes: Vec::new(),
            mt: metrics::Metric::Unknown,
            dimension,
            neighbor_neighbor_size: params.neighbor_neighbor_size,
            init_k: params.init_k,
            graph: Vec::new(),
            knn_graph: Vec::new(),
            root_nodes: Vec::new(),
            width: 0,
            index_size: params.index_size,
            angle: params.angle,
            threshold: (params.angle / E::from_f32(180.0).unwrap() * E::PI()).cos(),
            root_size: params.root_size,

            search_times: 0,
        }
    }

    fn build_knn_graph(&mut self) {
        let tmp_graph = Arc::new(Mutex::new(vec![vec![0]; self.nodes.len()]));
        (0..self.nodes.len()).into_par_iter().for_each(|n| {
            let item = &self.nodes[n];
            let mut heap = BinaryHeap::with_capacity(self.init_k);
            self.nodes
                .iter()
                .zip(0..self.nodes.len())
                .for_each(|(node, i)| {
                    if i == n {
                        return;
                    }
                    heap.push(neighbor::Neighbor::new(
                        i,
                        item.metric(node, self.mt).unwrap(),
                    ));
                    if heap.len() > self.init_k {
                        heap.pop();
                    }
                });
            let mut tmp = Vec::with_capacity(heap.len());
            while !heap.is_empty() {
                tmp.push(heap.pop().unwrap().idx());
            }

            tmp_graph.lock().unwrap()[n] = tmp;
        });
        self.graph = tmp_graph.lock().unwrap().to_vec();
        self.knn_graph = tmp_graph.lock().unwrap().to_vec();
    }

    fn get_random_nodes_idx_lite(&self, indices: &mut [usize]) {
        let mut rng = rand::thread_rng();
        (0..indices.len()).for_each(|i| {
            indices[i] = rng.gen_range(0..self.nodes.len() - indices.len());
        });
    }

    fn get_point_neighbor_size_neighbors(
        &self,
        q: usize,
        expand_neighbors_tmp: &mut Vec<neighbor::Neighbor<E, usize>>,
    ) {
        let mut flags = HashSet::with_capacity(self.neighbor_neighbor_size);

        flags.insert(q);
        for neighbor_id in self.graph[q].iter() {
            for nn_id in self.graph[*neighbor_id].iter() {
                if *neighbor_id == *nn_id {
                    continue;
                }
                if flags.contains(nn_id) {
                    continue;
                }
                flags.insert(*nn_id);
                let dist = self.nodes[q].metric(&self.nodes[*nn_id], self.mt).unwrap();
                expand_neighbors_tmp.push(neighbor::Neighbor::new(*nn_id, dist));
                if expand_neighbors_tmp.len() >= self.neighbor_neighbor_size {
                    return;
                }
            }
        }
    }

    fn expand_connectivity(&mut self) {
        let range = self.index_size;

        let mut ids: Vec<usize> = (0..self.nodes.len()).collect();
        ids.shuffle(&mut thread_rng());
        for id in ids.iter().take(self.root_size) {
            self.root_nodes.push(*id);
        }

        (0..self.root_size).for_each(|i| {
            let root_id = self.root_nodes[i];
            let mut flags = HashSet::new();
            let mut my_queue = VecDeque::new();
            my_queue.push_back(root_id);
            flags.insert(root_id);

            let mut unknown_set: Vec<usize> = Vec::with_capacity(1);
            while !unknown_set.is_empty() {
                while !my_queue.is_empty() {
                    let q_front = my_queue.pop_front().unwrap();

                    for j in 0..self.graph[q_front].len() {
                        let child = self.graph[q_front][j];
                        if flags.contains(&child) {
                            continue;
                        }
                        flags.insert(child);
                        my_queue.push_back(child);
                    }
                }
                unknown_set.clear();
                for j in 0..self.nodes.len() {
                    if flags.contains(&j) {
                        continue;
                    }
                    unknown_set.push(j);
                }
                if !unknown_set.is_empty() {
                    for j in 0..self.nodes.len() {
                        if flags.contains(&j) && self.graph[j].len() < range {
                            self.graph[j].push(unknown_set[0]);
                            break;
                        }
                    }
                    my_queue.push_back(unknown_set[0]);
                    flags.insert(unknown_set[0]);
                }
            }
        });
    }

    fn link_each_nodes(&mut self, pruned_graph_tmp: &mut Vec<neighbor::Neighbor<E, usize>>) {
        let mut expand_neighbors_tmp = Vec::new();
        (0..self.nodes.len()).for_each(|i| {
            expand_neighbors_tmp.clear();
            self.get_point_neighbor_size_neighbors(i, &mut expand_neighbors_tmp); // get related one
            self.prune_graph(
                i,
                &mut expand_neighbors_tmp,
                self.threshold,
                pruned_graph_tmp,
            );
        });
        (0..self.nodes.len()).for_each(|i| {
            self.inter_insert(i, self.index_size, pruned_graph_tmp);
        });
    }

    fn prune_graph(
        &mut self,
        query_id: usize,
        expand_neighbors_tmp: &mut Vec<neighbor::Neighbor<E, usize>>,
        threshold: E,
        pruned_graph_tmp: &mut Vec<neighbor::Neighbor<E, usize>>,
    ) {
        let mut start = 0;
        let mut flags = HashSet::with_capacity(expand_neighbors_tmp.len());
        for iter in expand_neighbors_tmp.iter() {
            flags.insert(iter.idx());
        }
        self.graph[query_id].iter().for_each(|linked_id| {
            if flags.contains(linked_id) {
                return;
            }
            expand_neighbors_tmp.push(neighbor::Neighbor::new(
                *linked_id,
                self.nodes[query_id]
                    .metric(&self.nodes[*linked_id], self.mt)
                    .unwrap(),
            ));
        });

        expand_neighbors_tmp.sort_unstable();
        let mut result = Vec::new();
        if expand_neighbors_tmp[start].idx() == query_id {
            start += 1;
        }
        result.push(expand_neighbors_tmp[start].clone());

        start += 1;
        while result.len() < self.index_size && start < expand_neighbors_tmp.len() {
            let p = &expand_neighbors_tmp[start];
            let mut occlude = false;
            // TODO: check every metrics, and decide use euclidean forcibly.
            for iter in result.iter() {
                if p.idx() == iter.idx() {
                    // stop early
                    occlude = true;
                    break;
                }
                let djk = self.nodes[iter.idx()]
                    .metric(&self.nodes[p.idx()], self.mt)
                    .unwrap();
                let cos_ij = (p.distance().powi(2) + iter.distance().powi(2) - djk.powi(2))
                    / (E::from_usize(2).unwrap() * (p.distance() * iter.distance()));

                if cos_ij > threshold {
                    occlude = true;
                    break;
                }
            }
            if !occlude {
                result.push(p.clone());
            }
            start += 1;
        }

        (0..result.len()).for_each(|t| {
            pruned_graph_tmp[t + query_id * self.index_size]._idx = result[t].idx();
            pruned_graph_tmp[t + query_id * self.index_size]._distance = result[t].distance();
        });
        if result.len() < self.index_size {
            (result.len()..self.index_size).for_each(|i| {
                pruned_graph_tmp[query_id * self.index_size + i]._distance = E::max_value();
                pruned_graph_tmp[query_id * self.index_size + i]._idx = self.nodes.len();
                // means not exist
            });
        }
    }

    // to handle neighbor's graph
    fn inter_insert(
        &self,
        n: usize,
        range: usize,
        pruned_graph_tmp: &mut Vec<neighbor::Neighbor<E, usize>>,
    ) {
        (0..range).for_each(|i| {
            if pruned_graph_tmp[i + n].distance() == E::max_value() {
                return;
            }

            let sn = neighbor::Neighbor::new(n, pruned_graph_tmp[i + n].distance()); // distance of n to i
            let des = pruned_graph_tmp[i + n].idx();
            let mut temp_pool = Vec::new();
            let mut dup = false;

            for j in 0..range {
                if pruned_graph_tmp[j + des * self.index_size].distance() == E::max_value() {
                    break;
                }
                // each other has neighbor relationship
                if n == pruned_graph_tmp[j + des * self.index_size].idx() {
                    // neighbor and point meet
                    dup = true;
                    break;
                }
                temp_pool.push(pruned_graph_tmp[j + des * self.index_size].clone());
                // neighbor's neighbor
            }

            if dup {
                return;
            }

            temp_pool.push(sn.clone());
            if temp_pool.len() > range {
                let mut result = Vec::new();
                let mut start = 0;
                temp_pool.sort_unstable();
                result.push(temp_pool[start].clone());
                start += 1;
                while result.len() < range && start < temp_pool.len() {
                    let p = &temp_pool[start];
                    let mut occlude = false;
                    for rt in result.iter() {
                        if p.idx() == rt.idx() {
                            occlude = true;
                            break;
                        }
                        let djk = self.nodes[rt.idx()]
                            .metric(&self.nodes[p.idx()], self.mt)
                            .unwrap();
                        let cos_ij = (p.distance().powi(2) + rt.distance().powi(2) - djk.powi(2))
                            / (E::from_usize(2).unwrap() * (p.distance() * rt.distance()));

                        if cos_ij > self.threshold {
                            occlude = true;
                            break;
                        }
                    }
                    if !occlude {
                        result.push(p.clone());
                    }
                    start += 1;
                }
                (0..result.len()).for_each(|t| {
                    pruned_graph_tmp[t + des * self.index_size] = result[t].clone();
                });

                if result.len() < range {
                    pruned_graph_tmp[result.len() + des * self.index_size]._distance =
                        E::max_value();
                }
            } else {
                for t in 0..range {
                    if pruned_graph_tmp[t + des * self.index_size].distance() == E::max_value() {
                        pruned_graph_tmp[t + des * self.index_size] = sn.clone();
                        if (t + 1) < range {
                            pruned_graph_tmp[t + des * self.index_size]._distance = E::max_value();
                            break;
                        }
                    }
                }
            }
        });
    }

    fn _build(&mut self) {
        self.build_knn_graph();

        let mut pruned_graph_tmp: Vec<neighbor::Neighbor<E, usize>> =
            Vec::with_capacity(self.nodes.len() * self.index_size);
        (0..self.nodes.len() * self.index_size).for_each(|i| {
            pruned_graph_tmp.push(neighbor::Neighbor::<E, usize>::new(i, E::float_zero()));
        });
        self.link_each_nodes(&mut pruned_graph_tmp);

        for i in 0..self.nodes.len() {
            let mut pool_size = 0;
            for j in 0..self.index_size {
                if pruned_graph_tmp[i * self.index_size + j].distance() == E::max_value() {
                    break;
                }
                pool_size = j;
            }
            pool_size += 1;
            self.graph[i] = Vec::with_capacity(pool_size);
            for j in 0..pool_size {
                self.graph[i].push(pruned_graph_tmp[i * self.index_size + j].idx());
            }
        }

        self.expand_connectivity();

        self.root_nodes = kmeans::general_kmeans(self.root_size, 256, &self.nodes, self.mt);

        let mut max = 0;
        let mut min = self.nodes.len();
        let mut avg: f32 = 0.;
        for t in 0..self.nodes.len() {
            let size = self.graph[t].len();
            max = if max < size { size } else { max };
            min = if min > size { size } else { min };
            avg += size as f32;
        }
        avg /= 1.0 * self.nodes.len() as f32;
        println!(
            "stat: k: {:?}, max {:?}, min {:?}, avg {:?}",
            self.init_k, max, min, avg
        );
    }

    fn search(&self, query: &node::Node<E, T>, k: usize) -> Vec<(node::Node<E, T>, E)> {
        // let mut search_flags = HashSet::with_capacity(self.nodes.len());
        let mut search_flags = FixedBitSet::with_capacity(self.nodes.len());
        let mut heap: BinaryHeap<neighbor::Neighbor<E, usize>> = BinaryHeap::new(); // max-heap
        let mut search_queue: LinkedList<usize> = LinkedList::new();

        let mut vec_tmp = Vec::with_capacity(self.root_nodes.len());
        self.root_nodes.iter().for_each(|n| {
            let dist = self.nodes[*n].metric(query, self.mt).unwrap();
            vec_tmp.push(neighbor::Neighbor::new(*n, dist));
        });
        vec_tmp.sort();
        for iter in vec_tmp.iter() {
            if heap.len() < k {
                heap.push(iter.clone());
                search_queue.push_back(iter.idx());
            }
            search_flags.insert(iter.idx());
        }

        // greedy BFS search
        while !search_queue.is_empty() {
            let id = search_queue.pop_front().unwrap();

            let mut tmp = BinaryHeap::with_capacity(self.graph[id].len());
            for iter in self.graph[id].iter() {
                if search_flags.contains(*iter) {
                    continue;
                }

                let dist = self.nodes[*iter].metric(query, self.mt).unwrap();
                tmp.push(Reverse(neighbor::Neighbor::new(*iter, dist)));
                search_flags.insert(*iter);
            }
            while !tmp.is_empty() {
                let Reverse(item) = tmp.pop().unwrap();
                // let item = tmp.pop().unwrap();
                if item.distance() > heap.peek().unwrap().distance() {
                    break;
                }
                heap.pop();
                search_queue.push_back(item.idx());
                heap.push(item);
            }
        }

        let mut result = Vec::with_capacity(heap.len());

        while !heap.is_empty() {
            let tmp = heap.pop().unwrap();
            result.push((*self.nodes[tmp.idx()].clone(), tmp.distance()));
        }
        result.reverse();
        result
    }

    fn check_edge(&self, h: usize, t: usize) -> bool {
        let mut flag = true;
        for i in 0..self.graph[h].len() {
            if t == self.graph[h][i] {
                flag = false;
            }
        }
        flag
    }

    fn connectivity_profile(&self) {
        let mut visited = HashSet::with_capacity(self.nodes.len());
        let mut queue = VecDeque::new();

        queue.push_back(0);
        while !queue.is_empty() {
            let id = queue.pop_front().unwrap();
            if visited.contains(&id) {
                continue;
            }

            for x in 0..self.graph[id].len() {
                queue.push_back(self.graph[id][x]);
                if self.graph[id][x] > self.nodes.len() {
                    // println!("{:?} {:?} {:?}", self.graph[id][x], self.graph[id], id);
                }
            }
            visited.insert(id);
        }

        println!("connectivity: {:?} {:?}", self.nodes.len(), visited.len());
    }
}

impl<E: node::FloatElement + DeserializeOwned, T: node::IdxType + DeserializeOwned>
    ann_index::SerializableIndex<E, T> for SSGIndex<E, T>
{
    fn load(path: &str) -> Result<Self, &'static str> {
        let file = File::open(path).unwrap_or_else(|_| panic!("unable to open file {:?}", path));
        let mut instance: SSGIndex<E, T> = bincode::deserialize_from(&file).unwrap();
        instance.nodes = instance
            .tmp_nodes
            .iter()
            .map(|x| Box::new(x.clone()))
            .collect();
        Ok(instance)
    }

    fn dump(&mut self, path: &str) -> Result<(), &'static str> {
        self.tmp_nodes = self.nodes.iter().map(|x| *x.clone()).collect();
        let encoded_bytes = bincode::serialize(&self).unwrap();
        let mut file = File::create(path).unwrap();
        file.write_all(&encoded_bytes)
            .unwrap_or_else(|_| panic!("unable to write file {:?}", path));
        Result::Ok(())
    }
}

impl<E: node::FloatElement, T: node::IdxType> ann_index::ANNIndex<E, T> for SSGIndex<E, T> {
    fn build(&mut self, mt: metrics::Metric) -> Result<(), &'static str> {
        self.mt = mt;
        self._build();

        Result::Ok(())
    }
    fn add_node(&mut self, item: &node::Node<E, T>) -> Result<(), &'static str> {
        self.nodes.push(Box::new(item.clone()));
        Result::Ok(())
    }
    fn built(&self) -> bool {
        true
    }
    fn node_search_k(&self, item: &node::Node<E, T>, k: usize) -> Vec<(node::Node<E, T>, E)> {
        self.search(item, k)
    }

    fn name(&self) -> &'static str {
        "SSGIndex"
    }

    fn nodes_size(&self) -> usize {
        self.nodes.len()
    }

    fn dimension(&self) -> usize {
        self.dimension
    }
}
