//! Product Quantization (PQ) and IVF-PQ indexes for compressed approximate search.

#![allow(dead_code)]

use crate::core::ann_index;
use crate::core::kmeans;
use crate::core::metrics;
use crate::core::neighbor::Neighbor;
use crate::core::node;
use crate::index::pq_params::IVFPQParams;
use crate::index::pq_params::PQParams;
use crate::vec_iter_mut;
#[cfg(not(feature = "no_thread"))]
use rayon::prelude::*;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::collections::BinaryHeap;
use std::fs::File;
use std::io::Write;

/// PQ index: vectors split into subvectors, each quantized to a codebook; search via lookup tables.
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct PQIndex<E: node::FloatElement, T: node::IdxType> {
    dimension: usize,                 //dimension of data
    n_sub: usize,                     //num of subdata
    subdimension: usize,              //dimension of subdata
    dimension_range: Vec<Vec<usize>>, //dimension preset
    sub_bits: usize,                  // size of subdata code
    sub_bytes: usize,                 //code save as byte: (_sub_bit + 7)//8
    n_sub_center: usize,              //num of centers per subdata code
    //n_center_per_sub = 1 << sub_bits
    code_bytes: usize,         // byte of code
    train_epoch: usize,        // training epoch
    centers: Vec<Vec<Vec<E>>>, // size to be n_sub * n_sub_center * subdimension
    is_trained: bool,
    has_residual: bool,
    residual: Vec<E>,

    n_items: usize,
    max_item: usize,
    nodes: Vec<Box<node::Node<E, T>>>,
    assigned_center: Vec<Vec<usize>>,
    mt: metrics::Metric, //compute metrics
    // _item2id: HashMap<i32, usize>,
    nodes_tmp: Vec<node::Node<E, T>>,
}

impl<E: node::FloatElement, T: node::IdxType> PQIndex<E, T> {
    pub fn new(dimension: usize, params: &PQParams<E>) -> PQIndex<E, T> {
        let n_sub = params.n_sub;
        let sub_bits = params.sub_bits;
        let train_epoch = params.train_epoch;
        let subdimension = dimension / n_sub;

        let sub_bytes = (sub_bits + 7) / 8;
        assert!(sub_bits <= 32);
        let n_center_per_sub = (1 << sub_bits) as usize;
        let code_bytes = sub_bytes * n_sub;
        let mut new_pq = PQIndex::<E, T> {
            dimension: dimension,
            n_sub: n_sub,
            subdimension: subdimension,
            sub_bits: sub_bits,
            sub_bytes: sub_bytes,
            n_sub_center: n_center_per_sub,
            code_bytes: code_bytes,
            train_epoch: train_epoch,
            is_trained: false,
            n_items: 0,
            max_item: 100000,
            has_residual: false,
            mt: metrics::Metric::Euclidean,
            ..Default::default()
        };

        for i in 0..n_sub {
            let begin;
            let end;
            if i < dimension % subdimension {
                begin = i * (subdimension + 1);
                end = (i + 1) * (subdimension + 1);
            } else {
                begin = (dimension % subdimension) * (subdimension + 1)
                    + (i - dimension % subdimension) * subdimension;
                end = (dimension % subdimension) * (subdimension + 1)
                    + (i + 1 - dimension % subdimension) * subdimension;
            };
            new_pq.dimension_range.push(vec![begin, end]);
        }
        new_pq
    }

    fn init_item(&mut self, data: &node::Node<E, T>) -> usize {
        let cur_id = self.n_items;
        // self._item2id.insert(item, cur_id);
        self.nodes.push(Box::new(data.clone()));
        self.n_items += 1;
        cur_id
    }

    fn add_item(&mut self, data: &node::Node<E, T>) -> Result<usize, &'static str> {
        if data.len() != self.dimension {
            return Err("dimension is different");
        }
        // if self._item2id.contains_key(&item) {
        //     //to_do update point
        //     return Ok(self._item2id[&item]);
        // }

        if self.n_items > self.max_item {
            return Err("The number of elements exceeds the specified limit");
        }

        let insert_id = self.init_item(data);
        Ok(insert_id)
    }

    fn set_residual(&mut self, residual: Vec<E>) {
        self.has_residual = true;
        self.residual = residual;
    }

    fn train_center(&mut self) {
        let n_item = self.n_items;
        let n_sub = self.n_sub;
        (0..n_sub).for_each(|i| {
            let n_center = self.n_sub_center;
            let n_epoch = self.train_epoch;
            let begin = self.dimension_range[i][0];
            let end = self.dimension_range[i][1];
            let mut data_vec: Vec<Vec<E>> = Vec::new();
            for node in self.nodes.iter() {
                data_vec.push(node.vectors().to_vec());
            }

            let mut cluster = kmeans::Kmeans::<E>::new(end - begin, n_center, self.mt);
            cluster.set_range(begin, end);
            if self.has_residual {
                cluster.set_residual(self.residual.to_vec());
            }

            cluster.train(n_item, &data_vec, n_epoch);
            let mut assigned_center: Vec<usize> = Vec::new();
            cluster.search_data(n_item, &data_vec, &mut assigned_center);
            self.centers.push(cluster.centers().to_vec());
            self.assigned_center.push(assigned_center);
        });
        self.is_trained = true;
    }

    fn get_distance_from_vec_range(
        &self,
        x: &node::Node<E, T>,
        y: &[E],
        begin: usize,
        end: usize,
    ) -> E {
        let mut z = x.vectors()[begin..end].to_vec();
        if self.has_residual {
            (0..end - begin).for_each(|i| z[i] -= self.residual[i + begin]);
        }
        return metrics::metric(&z, y, self.mt).unwrap();
    }

    fn search_knn_adc(
        &self,
        search_data: &node::Node<E, T>,
        k: usize,
    ) -> Result<BinaryHeap<Neighbor<E, usize>>, &'static str> {
        let mut dis2centers: Vec<E> = Vec::new();
        dis2centers.resize(self.n_sub * self.n_sub_center, E::from_f32(0.0).unwrap());
        vec_iter_mut!(dis2centers, ctr);
        ctr.enumerate().for_each(|(idx, x)| {
            let i = idx / self.n_sub_center;
            let j = idx % self.n_sub_center;
            let begin = self.dimension_range[i][0];
            let end = self.dimension_range[i][1];
            *x = self.get_distance_from_vec_range(search_data, &self.centers[i][j], begin, end);
        });

        let mut top_candidate: BinaryHeap<Neighbor<E, usize>> = BinaryHeap::new();
        (0..self.n_items).for_each(|i| {
            let mut distance = E::from_f32(0.0).unwrap();
            (0..self.n_sub).for_each(|j| {
                distance += dis2centers[j * self.n_sub_center + self.assigned_center[j][i]];
            });
            top_candidate.push(Neighbor::new(i, distance));
        });
        while top_candidate.len() > k {
            top_candidate.pop();
        }

        Ok(top_candidate)
    }
}

impl<E: node::FloatElement, T: node::IdxType> ann_index::ANNIndex<E, T> for PQIndex<E, T> {
    fn build(&mut self, _mt: metrics::Metric) -> Result<(), &'static str> {
        self.mt = _mt;
        self.train_center();
        Result::Ok(())
    }
    fn add_node(&mut self, item: &node::Node<E, T>) -> Result<(), &'static str> {
        match self.add_item(item) {
            Err(err) => Err(err),
            _ => Ok(()),
        }
    }
    fn built(&self) -> bool {
        true
    }

    fn node_search_k(&self, item: &node::Node<E, T>, k: usize) -> Vec<(node::Node<E, T>, E)> {
        let mut ret: BinaryHeap<Neighbor<E, usize>> = self.search_knn_adc(item, k).unwrap();
        let mut result: Vec<(node::Node<E, T>, E)> = Vec::new();
        let mut result_idx: Vec<(usize, E)> = Vec::new();
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
                *self.nodes[result_idx[cur_id].0].clone(),
                result_idx[cur_id].1,
            ));
        }
        result
    }

    fn name(&self) -> &'static str {
        "PQIndex"
    }

    fn dimension(&self) -> usize {
        self.dimension
    }
}

impl<E: node::FloatElement + DeserializeOwned, T: node::IdxType + DeserializeOwned>
    ann_index::SerializableIndex<E, T> for PQIndex<E, T>
{
    fn load(path: &str) -> Result<Self, &'static str> {
        let file = File::open(path).unwrap_or_else(|_| panic!("unable to open file {:?}", path));
        let mut instance: PQIndex<E, T> = bincode::deserialize_from(&file).unwrap();
        instance.nodes = instance
            .nodes_tmp
            .iter()
            .map(|x| Box::new(x.clone()))
            .collect();
        Ok(instance)
    }

    fn dump(&mut self, path: &str) -> Result<(), &'static str> {
        self.nodes_tmp = self.nodes.iter().map(|x| *x.clone()).collect();
        let encoded_bytes = bincode::serialize(&self).unwrap();
        let mut file = File::create(path).unwrap();
        file.write_all(&encoded_bytes)
            .unwrap_or_else(|_| panic!("unable to write file {:?}", path));
        Result::Ok(())
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct IVFPQIndex<E: node::FloatElement, T: node::IdxType> {
    dimension: usize,    //dimension of data
    n_sub: usize,        //num of subdata
    subdimension: usize, //dimension of subdata
    sub_bits: usize,     // size of subdata code
    sub_bytes: usize,    //code save as byte: (_sub_bit + 7)//8
    n_sub_center: usize, //num of centers per subdata code
    //n_center_per_sub = 1 << sub_bits
    code_bytes: usize,  // byte of code
    train_epoch: usize, // training epoch
    search_n_center: usize,
    n_kmeans_center: usize,
    centers: Vec<Vec<E>>,
    ivf_list: Vec<Vec<usize>>, //ivf center id
    pq_list: Vec<PQIndex<E, T>>,
    is_trained: bool,

    n_items: usize,
    max_item: usize,
    nodes: Vec<Box<node::Node<E, T>>>,
    assigned_center: Vec<Vec<usize>>,
    mt: metrics::Metric, //compute metrics
    // _item2id: HashMap<i32, usize>,
    nodes_tmp: Vec<node::Node<E, T>>,
}

impl<E: node::FloatElement, T: node::IdxType> IVFPQIndex<E, T> {
    pub fn new(dimension: usize, params: &IVFPQParams<E>) -> IVFPQIndex<E, T> {
        let n_sub = params.n_sub;
        let sub_bits = params.sub_bits;
        let n_kmeans_center = params.n_kmeans_center;
        let search_n_center = params.search_n_center;
        let train_epoch = params.train_epoch;

        let subdimension = dimension / n_sub;
        let sub_bytes = (sub_bits + 7) / 8;
        assert!(sub_bits <= 32);
        let n_center_per_sub = (1 << sub_bits) as usize;
        let code_bytes = sub_bytes * n_sub;
        let mut ivflist: Vec<Vec<usize>> = Vec::new();
        for _i in 0..n_kmeans_center {
            let ivf: Vec<usize> = Vec::new();
            ivflist.push(ivf);
        }
        IVFPQIndex {
            dimension: dimension,
            n_sub: n_sub,
            subdimension: subdimension,
            sub_bits: sub_bits,
            sub_bytes: sub_bytes,
            n_sub_center: n_center_per_sub,
            code_bytes: code_bytes,
            n_kmeans_center: n_kmeans_center,
            search_n_center: search_n_center,
            ivf_list: ivflist,
            train_epoch: train_epoch,
            is_trained: false,
            n_items: 0,
            max_item: 100000,
            mt: metrics::Metric::Unknown,
            ..Default::default()
        }
    }

    fn init_item(&mut self, data: &node::Node<E, T>) -> usize {
        let cur_id = self.n_items;
        // self._item2id.insert(item, cur_id);
        self.nodes.push(Box::new(data.clone()));
        self.n_items += 1;
        cur_id
    }

    fn add_item(&mut self, data: &node::Node<E, T>) -> Result<usize, &'static str> {
        if data.len() != self.dimension {
            return Err("dimension is different");
        }
        // if self._item2id.contains_key(&item) {
        //     //to_do update point
        //     return Ok(self._item2id[&item]);
        // }

        if self.n_items > self.max_item {
            return Err("The number of elements exceeds the specified limit");
        }

        let insert_id = self.init_item(data);
        Ok(insert_id)
    }

    fn train(&mut self) {
        let n_item = self.n_items;
        let dimension = self.dimension;
        let n_center = self.n_kmeans_center;
        let n_epoch = self.train_epoch;
        let mut cluster = kmeans::Kmeans::<E>::new(dimension, n_center, self.mt);
        let mut data_vec: Vec<Vec<E>> = Vec::new();
        for node in self.nodes.iter() {
            data_vec.push(node.vectors().to_vec());
        }
        cluster.set_range(0, dimension);
        cluster.train(n_item, &data_vec, n_epoch);
        let mut assigned_center: Vec<usize> = Vec::new();
        cluster.search_data(n_item, &data_vec, &mut assigned_center);
        self.centers = cluster.centers().to_vec();
        (0..n_item).for_each(|i| {
            let center_id = assigned_center[i];
            self.ivf_list[center_id].push(i);
        });
        for i in 0..n_center {
            let mut center_pq = PQIndex::<E, T>::new(
                self.dimension,
                &PQParams::default()
                    .n_sub(self.n_sub)
                    .sub_bits(self.sub_bits)
                    .train_epoch(self.train_epoch),
            );

            for j in 0..self.ivf_list[i].len() {
                center_pq
                    .add_item(&self.nodes[self.ivf_list[i][j]].clone())
                    .unwrap();
            }
            center_pq.set_residual(self.centers[i].to_vec());
            center_pq.train_center();
            self.pq_list.push(center_pq);
        }

        self.is_trained = true;
    }

    fn get_distance_from_vec_range(
        &self,
        x: &node::Node<E, T>,
        y: &[E],
        begin: usize,
        end: usize,
    ) -> E {
        return metrics::metric(&x.vectors()[begin..end], y, self.mt).unwrap();
    }

    fn search_knn_adc(
        &self,
        search_data: &node::Node<E, T>,
        k: usize,
    ) -> Result<BinaryHeap<Neighbor<E, usize>>, &'static str> {
        let mut topcenters: BinaryHeap<Neighbor<E, usize>> = BinaryHeap::new();
        let n_kmeans_center = self.n_kmeans_center;
        let dimension = self.dimension;
        for i in 0..n_kmeans_center {
            topcenters.push(Neighbor::new(
                i,
                -self.get_distance_from_vec_range(search_data, &self.centers[i], 0, dimension),
            ))
        }

        let mut top_candidate: BinaryHeap<Neighbor<E, usize>> = BinaryHeap::new();
        for _i in 0..self.search_n_center {
            let center = topcenters.pop().unwrap().idx();
            let mut ret = self.pq_list[center].search_knn_adc(search_data, k).unwrap();
            while !ret.is_empty() {
                let ret_peek = ret.pop().unwrap();
                let new_idx = self.ivf_list[center][ret_peek.idx()];
                top_candidate.push(Neighbor::new(new_idx, ret_peek.distance()));
                if top_candidate.len() > k {
                    top_candidate.pop();
                }
            }
        }
        Ok(top_candidate)
    }
}

impl<E: node::FloatElement, T: node::IdxType> ann_index::ANNIndex<E, T> for IVFPQIndex<E, T> {
    fn build(&mut self, _mt: metrics::Metric) -> Result<(), &'static str> {
        self.mt = _mt;
        self.train();
        Result::Ok(())
    }
    fn add_node(&mut self, item: &node::Node<E, T>) -> Result<(), &'static str> {
        match self.add_item(item) {
            Err(err) => Err(err),
            _ => Ok(()),
        }
    }
    fn built(&self) -> bool {
        true
    }

    fn node_search_k(&self, item: &node::Node<E, T>, k: usize) -> Vec<(node::Node<E, T>, E)> {
        let mut ret: BinaryHeap<Neighbor<E, usize>> = self.search_knn_adc(item, k).unwrap();
        let mut result: Vec<(node::Node<E, T>, E)> = Vec::new();
        let mut result_idx: Vec<(usize, E)> = Vec::new();
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
                *self.nodes[result_idx[cur_id].0].clone(),
                result_idx[cur_id].1,
            ));
        }
        result
    }

    fn name(&self) -> &'static str {
        "IVFPQIndex"
    }

    fn dimension(&self) -> usize {
        self.dimension
    }
}

impl<E: node::FloatElement + DeserializeOwned, T: node::IdxType + DeserializeOwned>
    ann_index::SerializableIndex<E, T> for IVFPQIndex<E, T>
{
    fn load(path: &str) -> Result<Self, &'static str> {
        let file = File::open(path).unwrap_or_else(|_| panic!("unable to open file {:?}", path));
        let mut instance: IVFPQIndex<E, T> = bincode::deserialize_from(&file).unwrap();
        instance.nodes = instance
            .nodes_tmp
            .iter()
            .map(|x| Box::new(x.clone()))
            .collect();
        instance.nodes_tmp.clear();
        for i in 0..instance.n_kmeans_center {
            instance.pq_list[i].nodes = instance.pq_list[i]
                .nodes_tmp
                .iter()
                .map(|x| Box::new(x.clone()))
                .collect();
            instance.pq_list[i].nodes_tmp.clear();
        }
        Ok(instance)
    }

    fn dump(&mut self, path: &str) -> Result<(), &'static str> {
        self.nodes_tmp = self.nodes.iter().map(|x| *x.clone()).collect();
        for i in 0..self.n_kmeans_center {
            self.pq_list[i].nodes_tmp = self.pq_list[i].nodes.iter().map(|x| *x.clone()).collect();
        }
        let encoded_bytes = bincode::serialize(&self).unwrap();
        let mut file = File::create(path).unwrap();
        file.write_all(&encoded_bytes)
            .unwrap_or_else(|_| panic!("unable to write file {:?}", path));
        Result::Ok(())
    }
}
