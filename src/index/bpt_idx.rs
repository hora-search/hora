#![allow(dead_code)]
use crate::core::ann_index;
use crate::core::calc;
use crate::core::metrics;
use crate::core::neighbor;
use crate::core::node;
use crate::core::random;
use crate::index::bpt_params::BPTParams;
use bincode::{config, Decode, Encode};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fs::File;

use std::io::Write;

// TODO: leaf as a trait with getter setter function
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
struct Leaf<E: node::FloatElement, T: node::IdxType> {
    n_descendants: i32, // tot n_descendants
    children: Vec<i32>, // left and right and if it's a leaf leaf, children would be very large (depend on _K)
    #[serde(skip_serializing, skip_deserializing)]
    node: Box<node::Node<E, T>>,
    tmp_node: Option<node::Node<E, T>>,

    // biz field
    norm: E,
    has_init: bool,
}

impl<E: node::FloatElement, T: node::IdxType> Leaf<E, T> {
    fn new() -> Leaf<E, T> {
        Leaf {
            children: vec![0, 0],
            ..Default::default()
        }
    }

    fn new_with_vectors(_v: &[E]) -> Leaf<E, T> {
        Leaf {
            children: vec![0, 0],
            node: Box::new(node::Node::new(_v)),
            ..Default::default()
        }
    }

    fn new_with_item(_v: &node::Node<E, T>) -> Leaf<E, T> {
        Leaf {
            children: vec![0, 0],
            node: Box::new(_v.clone()),
            ..Default::default()
        }
    }

    fn is_empty(&self) -> bool {
        self.has_init
    }

    fn init(&mut self) {
        self.children = vec![0, 0];
    }

    fn clone_node(&self) -> node::Node<E, T> {
        *self.node.clone()
    }

    fn normalize(&mut self) {
        let norm = calc::get_norm(self.node.vectors()).unwrap();
        if norm > E::float_zero() {
            for i in 0..self.node.len() {
                self.node.mut_vectors()[i] /= norm;
            }
        }
    }

    fn copy(dst: &mut Leaf<E, T>, src: &Leaf<E, T>) {
        dst.n_descendants = src.n_descendants;
        dst.children = src.children.clone();
        dst.node = src.node.clone();
        dst.norm = src.norm;
    }

    pub fn get_literal(&self) -> String {
        format!(
            "{{ \"n_descendants\": {:?}, \"children\": {:?}, \"has_init\": {:?} }}, \"node\": {:?},",
            self.n_descendants, self.children, self.has_init, *self.node
        )
    }

    // replace distance copy_leaf
    fn copy_leaf(src: &Leaf<E, T>) -> Leaf<E, T> {
        Leaf {
            n_descendants: src.n_descendants,
            node: src.node.clone(),
            children: src.children.clone(),
            ..Default::default()
        }
    }
}

fn two_means<E: node::FloatElement, T: node::IdxType>(
    leaves: &[Leaf<E, T>],
    mt: metrics::Metric,
) -> Result<(Leaf<E, T>, Leaf<E, T>), &'static str> {
    const ITERATION_STEPS: usize = 200;
    if leaves.len() < 2 {
        return Err("empty leaves");
    }

    let count = leaves.len();

    let i = random::index(count);
    let mut j = random::index(count - 1);
    // make sure j not equal to i;
    if j >= i {
        j += 1;
    }

    let mut first = Leaf::copy_leaf(&leaves[i]);
    let mut second = Leaf::copy_leaf(&leaves[j]);

    if mt == metrics::Metric::CosineSimilarity {
        first.normalize();
        second.normalize();
    }
    // TODO: dot normalize

    let one = E::float_one();
    let zero = E::float_zero();

    let mut ic: E = one;
    let mut jc: E = one;

    // produce two mean point.
    for _z in 0..ITERATION_STEPS {
        let rand_k = random::index(count);
        let di =
            ic * metrics::metric(first.node.vectors(), leaves[rand_k].node.vectors(), mt).unwrap();
        let dj =
            jc * metrics::metric(second.node.vectors(), leaves[rand_k].node.vectors(), mt).unwrap();

        //
        let mut norm = one;
        if mt == metrics::Metric::CosineSimilarity {
            norm = calc::get_norm(leaves[rand_k].node.vectors()).unwrap();
            match norm.partial_cmp(&zero) {
                Some(Ordering::Equal) | Some(Ordering::Less) => continue,
                _ => {}
            };
        }

        // make p more closer to k in space.
        if di < dj {
            for l in 0..first.node.len() {
                first.node.mut_vectors()[l] = (first.node.vectors()[l] * ic
                    + leaves[rand_k].node.vectors()[l] / norm)
                    / (ic + one);
            }
            ic += one;
        } else if dj < di {
            for l in 0..second.node.len() {
                second.node.mut_vectors()[l] = (second.node.vectors()[l] * jc
                    + leaves[rand_k].node.vectors()[l] / norm)
                    / (jc + one);
            }
            jc += one;
        }
    }
    Ok((first, second))
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct BPTIndex<E: node::FloatElement, T: node::IdxType> {
    _dimension: usize,    // dimension
    _tot_items_cnt: i32, // add items count, means the physically the item count, _tot_items_cnt == leaves.size()
    _tot_leaves_cnt: i32, // leaves count, whole tree leaves count
    // _leaves_size: i32, // in source code, this means the memory which has been allocated, and we can use leaf's size to get data
    _roots: Vec<i32>,     // dummy root's children
    _leaf_max_items: i32, // max number of n_descendants to fit into leaf
    _built: bool,
    leaves: Vec<Leaf<E, T>>,
    mt: metrics::Metric,
    _tree_num: i32,
    _candidate_size: i32,
}

impl<E: node::FloatElement, T: node::IdxType> BPTIndex<E, T> {
    pub fn new(dimension: usize, params: &BPTParams) -> BPTIndex<E, T> {
        BPTIndex {
            _built: false,
            _dimension: dimension,
            _leaf_max_items: ((dimension / 2) as i32) + 2,
            _tree_num: params.tree_num,
            _candidate_size: params.candidate_size,
            leaves: vec![Leaf::new()], // the id count should start from 1, use a node as placeholder
            ..Default::default()
        }
    }

    fn _add_item(&mut self, w: &node::Node<E, T>) -> Result<(), &'static str> {
        // TODO: remove
        if w.len() != self._dimension {
            return Err("dimension is different");
        }

        let mut nn = Leaf::new_with_item(w);

        nn.children[0] = 0; // TODO: as const value
        nn.children[1] = 0;
        nn.n_descendants = 1; // only the leaf itself, so the n_descendants include it self

        // no update method
        self._tot_items_cnt += 1;

        self.leaves.push(nn);

        Ok(())
    }

    fn build(&mut self, mt: metrics::Metric) -> Result<(), &'static str> {
        if self._built {
            return Err("has built");
        }

        self.mt = mt;
        self._tot_leaves_cnt = self._tot_items_cnt; // init with build.
        self._build(self._tree_num, self.mt);
        self._built = true;
        Ok(())
    }

    fn clear(&mut self) {
        self._roots.clear();
        self._tot_leaves_cnt = self._tot_items_cnt;
        self._built = false;
    }
    fn get_distance(&self, i: i32, j: i32) -> E {
        let ni = self.get_leaf(i).unwrap();
        let nj = self.get_leaf(j).unwrap();
        return metrics::metric(ni.node.vectors(), nj.node.vectors(), self.mt).unwrap();
    }

    fn get_tot_items_cnt(&self) -> i32 {
        self._tot_items_cnt
    }
    fn get_n_tree(&self) -> i32 {
        self._roots.len() as i32
    }

    fn get_dimension(&self) -> usize {
        self._dimension
    }

    fn get_k(&self) -> i32 {
        self._leaf_max_items
    }

    fn get_leaf_mut(&mut self, i: i32) -> &mut Leaf<E, T> {
        if self.leaves.len() <= i as usize {
            self.extent_leaves(i as usize);
        }
        &mut self.leaves[i as usize]
    }

    fn extent_leaf(&mut self) -> &mut Leaf<E, T> {
        let i = self.leaves.len();
        self.extent_leaves(self.leaves.len());
        if self.leaves[i].is_empty() {
            self.leaves[i].init();
        }
        &mut self.leaves[i]
    }

    fn get_leaf(&self, i: i32) -> Option<&Leaf<E, T>> {
        if self.leaves.len() < i as usize {
            return None;
        }
        if self.leaves[i as usize].is_empty() {
            return None;
        }
        Some(&self.leaves[i as usize])
    }

    fn extent_leaves(&mut self, i: usize) {
        let diff = i - self.leaves.len() + 1;
        if diff > 0 {
            for _i in 0..diff {
                self.leaves.push(Leaf::new());
            }
        }
    }

    // q => tree count
    // TODO: build failed
    fn _build(&mut self, tree_num: i32, mt: metrics::Metric) {
        let mut this_root: Vec<i32> = Vec::new();

        loop {
            if tree_num == -1 {
                if self._tot_leaves_cnt >= 2 * self._tot_items_cnt {
                    break;
                }
            } else if this_root.len() >= (tree_num as usize) {
                break;
            }

            let mut indices: Vec<i32> = Vec::new();
            for i in 1..self._tot_items_cnt {
                let leaf = self.get_leaf(i).unwrap();
                if leaf.n_descendants >= 1 {
                    indices.push(i as i32);
                }
            }

            let tree = self.make_tree(&indices, true, mt).unwrap();
            this_root.push(tree);
        }

        // thread lock
        self._roots.extend_from_slice(&this_root);
    }

    fn make_tree(
        &mut self,
        indices: &[i32],
        is_root: bool,
        mt: metrics::Metric,
    ) -> Result<i32, &'static str> {
        if indices.is_empty() {
            return Err("empty indices");
        }
        if indices.len() == 1 && !is_root {
            return Ok(indices[0]);
        }

        // the batch is a leaf cluster, make a parent node
        if (indices.len() as i32) <= self._leaf_max_items
            && (!is_root || self._tot_items_cnt <= self._leaf_max_items || indices.len() == 1)
        {
            self._tot_leaves_cnt += 1;
            let item_cnt = self._tot_items_cnt;
            let mut n = self.extent_leaf();

            n.n_descendants = if is_root {
                item_cnt
            } else {
                indices.len() as i32
            };
            n.children = indices.to_vec();

            return Ok(self._tot_leaves_cnt);
        }

        let mut children: Vec<Leaf<E, T>> = Vec::new();
        for j in indices.iter().skip(1) {
            match self.get_leaf(*j) {
                None => continue,
                Some(leaf) => {
                    children.push(leaf.clone());
                }
            }
        }

        let mut new_parent_leaf = Leaf::new();
        let mut children_indices: [Vec<i32>; 2] = [Vec::new(), Vec::new()];

        const ATTEMPT: usize = 5;
        // find split hyperplane
        for _i in 0..ATTEMPT {
            children_indices[0].clear();
            children_indices[1].clear();
            self.create_split(children.as_slice(), &mut new_parent_leaf, mt)
                .unwrap();

            for leaf_idx in indices.iter().skip(1) {
                let leaf = self.get_leaf(*leaf_idx as i32).unwrap();
                let side = self.side(&new_parent_leaf, leaf.node.vectors());
                children_indices[(side as usize)].push(*leaf_idx);
            }

            if calc::split_imbalance(&children_indices[0], &children_indices[1]) < 0.85 {
                break;
            }
        }

        // don't get correct hyperplane situation
        // TODO: record
        while calc::split_imbalance(&children_indices[0], &children_indices[1]) > 0.98 {
            children_indices[0].clear();
            children_indices[1].clear();

            let is_initial = new_parent_leaf.node.len() == 0;
            for z in 0..self._dimension {
                if is_initial {
                    new_parent_leaf.node.push(&E::float_zero()); // TODO: make it const value
                } else {
                    new_parent_leaf.node.mut_vectors()[z] = E::float_zero();
                }
            }

            for j in indices.iter().skip(1) {
                children_indices[random::flip() as usize].push(*j);
            }
        }

        let flip = (children_indices[0].len() > children_indices[1].len()) as bool;

        new_parent_leaf.n_descendants = if is_root {
            self._tot_items_cnt
        } else {
            indices.len() as i32
        };

        for side in 0..2 {
            match self.make_tree(&children_indices[side ^ (flip as usize)], false, mt) {
                Ok(tree) => {
                    new_parent_leaf.children[side ^ (flip as usize)] = tree;
                }
                Err(_e) => {
                    // TODO: log
                    continue;
                }
            }
        }
        self._tot_leaves_cnt += 1;
        self.leaves.push(new_parent_leaf);

        Ok((self._tot_leaves_cnt) as i32)
    }

    fn _search_k(
        &self,
        vectors: &[E],
        n: usize,
    ) -> Result<Vec<(node::Node<E, T>, E)>, &'static str> {
        let mut v_leaf = Leaf::<E, T>::new();

        v_leaf.node.set_vectors(&vectors.to_vec());

        if self._roots.is_empty() || !self._built {
            return Err("empty tree");
        }

        let mut candidate_size = self._candidate_size;
        if candidate_size <= 0 {
            candidate_size = (n * self._roots.len() * 2) as i32;
        }

        let mut heap: BinaryHeap<neighbor::Neighbor<E, i32>> = BinaryHeap::new(); // max-heap
        self._roots.iter().for_each(|root| {
            heap.push(neighbor::Neighbor {
                _distance: self.pq_initial_value(), // float MAX
                _idx: *root,
            });
        });

        // it use a heap to ensure the minest distance node will pop up
        let mut nns: Vec<i32> = Vec::new();
        while nns.len() < (candidate_size as usize) && !(heap.is_empty()) {
            let top = heap.peek().unwrap();
            let top_idx = top._idx;
            let top_distance = top._distance;

            let nd = self.get_leaf(top_idx).unwrap();
            heap.pop();

            if nd.n_descendants == 1 && (top_idx) < self._tot_items_cnt {
                nns.push(top_idx);
            } else if nd.n_descendants <= self._leaf_max_items {
                nns.extend_from_slice(&nd.children); // push all of its children
            } else {
                let margin = self.margin(nd, vectors)?;
                // put two children into heap, and use distance to sort the order for poping up.
                heap.push(neighbor::Neighbor {
                    _distance: self.pq_distance(top_distance, margin, 1),
                    _idx: nd.children[1],
                });
                heap.push(neighbor::Neighbor {
                    _distance: self.pq_distance(top_distance, margin, 0),
                    _idx: nd.children[0],
                });
            }
        }

        nns.sort_unstable(); // sort id and filter dup to avoid same id;
        let mut nns_vec: Vec<neighbor::Neighbor<E, usize>> = Vec::new();
        let mut last = -1;
        for j in nns.iter() {
            if *j == last {
                continue;
            }
            last = *j;
            let leaf = self.get_leaf(*j).unwrap();
            if leaf.n_descendants == 1 {
                nns_vec.push(neighbor::Neighbor::new(
                    *j as usize,
                    metrics::metric(v_leaf.node.vectors(), leaf.node.vectors(), self.mt).unwrap(),
                ))
            }
        }

        nns_vec.sort_by(|a, b| a.distance().partial_cmp(&b.distance()).unwrap());
        let return_size = if n < nns_vec.len() { n } else { nns_vec.len() };
        let mut result: Vec<(node::Node<E, T>, E)> = Vec::new();

        for item in nns_vec.iter().take(return_size) {
            result.push((
                self.get_leaf(item._idx as i32).unwrap().clone_node(),
                item._distance,
            ));
        }

        Ok(result)
    }

    fn show_trees(&self) {
        let mut v = self._roots.clone();
        while !v.is_empty() {
            let i = v.pop().unwrap();
            let item = self.get_leaf(i).unwrap();
            if item.n_descendants == 1 {
                continue;
            }
            if !(item.children[0] == 0 && item.children[1] == 0) {
                v.extend(&item.children);
            }
        }
    }

    // means same side?
    fn margin(&self, src: &Leaf<E, T>, dst: &[E]) -> Result<E, &'static str> {
        calc::dot(src.node.vectors(), dst)
    }

    fn side(&self, src: &Leaf<E, T>, dst: &[E]) -> bool {
        match self.margin(src, dst) {
            Ok(x) => x > E::float_zero(),
            Err(_e) => random::flip(),
        }
    }

    fn create_split(
        &self,
        leaves: &[Leaf<E, T>],
        new_mean_leaf: &mut Leaf<E, T>,
        mt: metrics::Metric,
    ) -> Result<(), &'static str> {
        let (p, q) = two_means(leaves, mt)?;

        // TODO: remove
        if new_mean_leaf.node.len() != 0 && new_mean_leaf.node.len() != p.node.len() {
            return Err("empty leaf input");
        }

        // // get mean point between p and q.
        let mut v = Vec::with_capacity(p.node.len());
        for i in 0..p.node.len() {
            v.push(p.node.vectors()[i] - q.node.vectors()[i]);
        }
        new_mean_leaf.node.set_vectors(&v);
        new_mean_leaf.normalize();
        Ok(())
    }

    fn pq_distance(&self, distance: E, mut margin: E, child_nr: usize) -> E {
        if child_nr == 0 {
            margin = -margin;
        }
        if distance < margin {
            distance
        } else {
            margin
        }
    }

    fn pq_initial_value(&self) -> E {
        E::max_value()
    }
}

impl<E: node::FloatElement, T: node::IdxType> ann_index::ANNIndex<E, T> for BPTIndex<E, T> {
    fn build(&mut self, mt: metrics::Metric) -> Result<(), &'static str> {
        self.build(mt)
    }
    fn add_node(&mut self, item: &node::Node<E, T>) -> Result<(), &'static str> {
        self._add_item(item)
    }
    fn built(&self) -> bool {
        self._built
    }

    fn node_search_k(&self, item: &node::Node<E, T>, k: usize) -> Vec<(node::Node<E, T>, E)> {
        self._search_k(item.vectors(), k).unwrap()
    }

    fn name(&self) -> &'static str {
        "BPForestIndex"
    }

    fn dimension(&self) -> usize {
        self._dimension
    }
}

impl<E: node::FloatElement + DeserializeOwned, T: node::IdxType + DeserializeOwned>
    ann_index::SerializableIndex<E, T> for BPTIndex<E, T>
{
    fn load_bytes(bytes: &[u8]) -> Result<Self, &'static str> {
        let mut instance: BPTIndex<E, T> = bincode::deserialize_from(bytes).unwrap();

        for i in 0..instance.leaves.len() {
            instance.leaves[i].node =
                Box::new(instance.leaves[i].tmp_node.as_ref().unwrap().clone());
            instance.leaves[i].tmp_node = None;
        }

        Ok(instance)
    }

    fn load(path: &str) -> Result<Self, &'static str> {
        let file = File::open(path).unwrap_or_else(|_| panic!("unable to open file {:?}", path));
        let mut instance: BPTIndex<E, T> = bincode::deserialize_from(&file).unwrap();

        for i in 0..instance.leaves.len() {
            instance.leaves[i].node =
                Box::new(instance.leaves[i].tmp_node.as_ref().unwrap().clone());
            instance.leaves[i].tmp_node = None;
        }

        Ok(instance)
    }

    fn dump(&mut self, path: &str) -> Result<(), &'static str> {
        self.leaves
            .iter_mut()
            .for_each(|x| x.tmp_node = Some(*x.node.clone()));
        let encoded_bytes = bincode::serialize(&self).unwrap();
        let mut file = File::create(path).unwrap();
        file.write_all(&encoded_bytes)
            .unwrap_or_else(|_| panic!("unable to write file {:?}", path));
        Result::Ok(())
    }
}
