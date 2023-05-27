#![allow(dead_code)]
use crate::core::ann_index;
use crate::core::metrics;
use crate::core::neighbor;
use crate::core::node;
use crate::index::bruteforce_params::BruteForceParams;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::collections::BinaryHeap;

use std::fs::File;

use std::io::Write;

#[derive(Debug, Serialize, Deserialize)]
pub struct BruteForceIndex<N: node::Node> {
    #[serde(skip_serializing, skip_deserializing)]
    nodes: Vec<Box<N>>,
    tmp_nodes: Vec<N>, // only use for serialization scene
    mt: metrics::Metric,
    dimension: usize,
}

impl<E: node::FloatElement, T: node::IdxType, N: node::Node<E = E, T = T>> BruteForceIndex<N> {
    pub fn new(dimension: usize, _params: &BruteForceParams) -> BruteForceIndex<N> {
        BruteForceIndex::<N> {
            nodes: Vec::new(),
            mt: metrics::Metric::Unknown,
            tmp_nodes: Vec::new(),
            dimension,
        }
    }
}

impl<E: node::FloatElement, T: node::IdxType, N: node::Node<E = E, T = T>>
    ann_index::ANNIndex<E, T, N> for BruteForceIndex<N>
{
    fn build(&mut self, mt: metrics::Metric) -> Result<(), &'static str> {
        self.mt = mt;
        Result::Ok(())
    }
    fn add_node(&mut self, item: &N) -> Result<(), &'static str> {
        self.nodes.push(Box::new(item.clone()));
        Result::Ok(())
    }
    fn built(&self) -> bool {
        true
    }
    fn node_search_k(&self, item: &N, k: usize) -> Vec<(N, E)> {
        let mut heap = BinaryHeap::with_capacity(k + 1);
        self.nodes
            .iter()
            .zip(0..self.nodes.len())
            .for_each(|(node, i)| {
                heap.push(neighbor::Neighbor::new(
                    // use max heap, and every time pop out the greatest one in the heap
                    i,
                    item.metric(&**node, self.mt).unwrap(),
                ));
                if heap.len() > k {
                    let _xp = heap.pop().unwrap();
                }
            });

        let mut result = Vec::with_capacity(heap.len());
        while !heap.is_empty() {
            let neighbor_rev = heap.pop().unwrap();
            result.push((
                *self.nodes[neighbor_rev.idx()].clone(),
                neighbor_rev.distance(),
            ))
        }
        result.reverse();
        result
    }

    fn name(&self) -> &'static str {
        "BruteForceIndex"
    }

    fn dimension(&self) -> usize {
        self.dimension
    }
}

impl<
        E: node::FloatElement + DeserializeOwned,
        T: node::IdxType + DeserializeOwned,
        N: node::Node<E = E, T = T> + DeserializeOwned,
    > ann_index::SerializableIndex<E, T, N> for BruteForceIndex<N>
{
    fn load(path: &str) -> Result<Self, &'static str> {
        let file = File::open(path).unwrap_or_else(|_| panic!("unable to open file {:?}", path));
        let mut instance: BruteForceIndex<N> = bincode::deserialize_from(file).unwrap();
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
