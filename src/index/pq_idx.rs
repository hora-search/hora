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
use std::collections::BinaryHeap;

use serde::{Deserialize, Serialize};

use std::fs::File;

use std::io::Write;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct PQIndex<E: node::FloatElement, N: node::Node<E = E>> {
    _dimension: usize,                 //dimension of data
    _n_sub: usize,                     //num of subdata
    _sub_dimension: usize,             //dimension of subdata
    _dimension_range: Vec<Vec<usize>>, //dimension preset
    _sub_bits: usize,                  // size of subdata code
    _sub_bytes: usize,                 //code save as byte: (_sub_bit + 7)//8
    _n_sub_center: usize,              //num of centers per subdata code
    //n_center_per_sub = 1 << sub_bits
    _code_bytes: usize,         // byte of code
    _train_epoch: usize,        // training epoch
    _centers: Vec<Vec<Vec<E>>>, // size to be _n_sub * _n_sub_center * _sub_dimension
    _is_trained: bool,
    _has_residual: bool,
    _residual: Vec<E>,

    _n_items: usize,
    _max_item: usize,
    _nodes: Vec<Box<N>>,
    _assigned_center: Vec<Vec<usize>>,
    mt: metrics::Metric, //compute metrics
    // _item2id: HashMap<i32, usize>,
    _nodes_tmp: Vec<N>,
}

impl<E: node::FloatElement, T: node::IdxType, N: node::Node<E = E, T = T>> PQIndex<E, N> {
    pub fn new(dimension: usize, params: &PQParams<E>) -> PQIndex<E, N> {
        let n_sub = params.n_sub;
        let sub_bits = params.sub_bits;
        let train_epoch = params.train_epoch;
        let sub_dimension = dimension / n_sub;

        let sub_bytes = (sub_bits + 7) / 8;
        assert!(sub_bits <= 32);
        let n_center_per_sub = (1 << sub_bits) as usize;
        let code_bytes = sub_bytes * n_sub;
        let mut new_pq = PQIndex::<E, T> {
            _dimension: dimension,
            _n_sub: n_sub,
            _sub_dimension: sub_dimension,
            _sub_bits: sub_bits,
            _sub_bytes: sub_bytes,
            _n_sub_center: n_center_per_sub,
            _code_bytes: code_bytes,
            _train_epoch: train_epoch,
            _is_trained: false,
            _n_items: 0,
            _max_item: 100000,
            _has_residual: false,
            mt: metrics::Metric::Euclidean,
            ..Default::default()
        };

        for i in 0..n_sub {
            let begin;
            let end;
            if i < dimension % sub_dimension {
                begin = i * (sub_dimension + 1);
                end = (i + 1) * (sub_dimension + 1);
            } else {
                begin = (dimension % sub_dimension) * (sub_dimension + 1)
                    + (i - dimension % sub_dimension) * sub_dimension;
                end = (dimension % sub_dimension) * (sub_dimension + 1)
                    + (i + 1 - dimension % sub_dimension) * sub_dimension;
            };
            new_pq._dimension_range.push(vec![begin, end]);
        }
        new_pq
    }

    fn init_item(&mut self, data: &N) -> usize {
        let cur_id = self._n_items;
        // self._item2id.insert(item, cur_id);
        self._nodes.push(Box::new(data.clone()));
        self._n_items += 1;
        cur_id
    }

    fn add_item(&mut self, data: &N) -> Result<usize, &'static str> {
        if data.len() != self._dimension {
            return Err("dimension is different");
        }
        // if self._item2id.contains_key(&item) {
        //     //to_do update point
        //     return Ok(self._item2id[&item]);
        // }

        if self._n_items > self._max_item {
            return Err("The number of elements exceeds the specified limit");
        }

        let insert_id = self.init_item(data);
        Ok(insert_id)
    }

    fn set_residual(&mut self, residual: Vec<E>) {
        self._has_residual = true;
        self._residual = residual;
    }

    fn train_center(&mut self) {
        let n_item = self._n_items;
        let n_sub = self._n_sub;
        (0..n_sub).for_each(|i| {
            let _dimension = self._sub_dimension;
            let n_center = self._n_sub_center;
            let n_epoch = self._train_epoch;
            let begin = self._dimension_range[i][0];
            let end = self._dimension_range[i][1];
            let mut data_vec: Vec<Vec<E>> = Vec::new();
            for node in self._nodes.iter() {
                data_vec.push(node.vectors().to_vec());
            }

            let mut cluster = kmeans::Kmeans::<E>::new(end - begin, n_center, self.mt);
            cluster.set_range(begin, end);
            if self._has_residual {
                cluster.set_residual(self._residual.to_vec());
            }

            cluster.train(n_item, &data_vec, n_epoch);
            let mut assigned_center: Vec<usize> = Vec::new();
            cluster.search_data(n_item, &data_vec, &mut assigned_center);
            self._centers.push(cluster.centers().to_vec());
            self._assigned_center.push(assigned_center);
        });
        self._is_trained = true;
    }

    fn get_distance_from_vec_range(&self, x: &N, y: &[E], begin: usize, end: usize) -> E {
        let mut z = x.vectors()[begin..end].to_vec();
        if self._has_residual {
            (0..end - begin).for_each(|i| z[i] -= self._residual[i + begin]);
        }
        return metrics::metric(&z, y, self.mt).unwrap();
    }

    fn search_knn_adc(
        &self,
        search_data: &N,
        k: usize,
    ) -> Result<BinaryHeap<Neighbor<E, usize>>, &'static str> {
        let mut dis2centers: Vec<E> = Vec::new();
        dis2centers.resize(self._n_sub * self._n_sub_center, E::from_f32(0.0).unwrap());
        vec_iter_mut!(dis2centers, ctr);
        ctr.enumerate().for_each(|(idx, x)| {
            let i = idx / self._n_sub_center;
            let j = idx % self._n_sub_center;
            let begin = self._dimension_range[i][0];
            let end = self._dimension_range[i][1];
            *x = self.get_distance_from_vec_range(search_data, &self._centers[i][j], begin, end);
        });

        let mut top_candidate: BinaryHeap<Neighbor<E, usize>> = BinaryHeap::new();
        (0..self._n_items).for_each(|i| {
            let mut distance = E::from_f32(0.0).unwrap();
            (0..self._n_sub).for_each(|j| {
                distance += dis2centers[j * self._n_sub_center + self._assigned_center[j][i]];
            });
            top_candidate.push(Neighbor::new(i, distance));
        });
        while top_candidate.len() > k {
            top_candidate.pop();
        }

        Ok(top_candidate)
    }
}

impl<E: node::FloatElement, T: node::IdxType, N: node::Node<E = E, T = T>>
    ann_index::ANNIndex<E, T, N> for PQIndex<E, N>
{
    fn build(&mut self, _mt: metrics::Metric) -> Result<(), &'static str> {
        self.mt = _mt;
        self.train_center();
        Result::Ok(())
    }
    fn add_node(&mut self, item: &N) -> Result<(), &'static str> {
        match self.add_item(item) {
            Err(err) => Err(err),
            _ => Ok(()),
        }
    }
    fn built(&self) -> bool {
        true
    }

    fn node_search_k(&self, item: &N, k: usize) -> Vec<(N, E)> {
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
                *self._nodes[result_idx[cur_id].0].clone(),
                result_idx[cur_id].1,
            ));
        }
        result
    }

    fn name(&self) -> &'static str {
        "PQIndex"
    }

    fn dimension(&self) -> usize {
        self._dimension
    }
}

impl<
        E: node::FloatElement + DeserializeOwned,
        T: node::IdxType + DeserializeOwned,
        N: node::Node<E = E, T = T> + DeserializeOwned,
    > ann_index::SerializableIndex<E, T, N> for PQIndex<E, N>
{
    fn load(path: &str) -> Result<Self, &'static str> {
        let file = File::open(path).unwrap_or_else(|_| panic!("unable to open file {:?}", path));
        let mut instance: PQIndex<E, T> = bincode::deserialize_from(&file).unwrap();
        instance._nodes = instance
            ._nodes_tmp
            .iter()
            .map(|x| Box::new(x.clone()))
            .collect();
        Ok(instance)
    }

    fn dump(&mut self, path: &str) -> Result<(), &'static str> {
        self._nodes_tmp = self._nodes.iter().map(|x| *x.clone()).collect();
        let encoded_bytes = bincode::serialize(&self).unwrap();
        let mut file = File::create(path).unwrap();
        file.write_all(&encoded_bytes)
            .unwrap_or_else(|_| panic!("unable to write file {:?}", path));
        Result::Ok(())
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct IVFPQIndex<E: node::FloatElement, T: node::IdxType, N: node::Node<E = E, T = T>> {
    _dimension: usize,     //dimension of data
    _n_sub: usize,         //num of subdata
    _sub_dimension: usize, //dimension of subdata
    _sub_bits: usize,      // size of subdata code
    _sub_bytes: usize,     //code save as byte: (_sub_bit + 7)//8
    _n_sub_center: usize,  //num of centers per subdata code
    //n_center_per_sub = 1 << sub_bits
    _code_bytes: usize,  // byte of code
    _train_epoch: usize, // training epoch
    _search_n_center: usize,
    _n_kmeans_center: usize,
    _centers: Vec<Vec<E>>,
    _ivflist: Vec<Vec<usize>>, //ivf center id
    _pq_list: Vec<PQIndex<E, N>>,
    _is_trained: bool,

    _n_items: usize,
    _max_item: usize,
    _nodes: Vec<Box<N>>,
    _assigned_center: Vec<Vec<usize>>,
    mt: metrics::Metric, //compute metrics
    // _item2id: HashMap<i32, usize>,
    _nodes_tmp: Vec<N>,
}

impl<E: node::FloatElement, T: node::IdxType, N: node::Node<E = E, T = T>> IVFPQIndex<E, T, N> {
    pub fn new(dimension: usize, params: &IVFPQParams<E>) -> IVFPQIndex<E, T, N> {
        let n_sub = params.n_sub;
        let sub_bits = params.sub_bits;
        let n_kmeans_center = params.n_kmeans_center;
        let search_n_center = params.search_n_center;
        let train_epoch = params.train_epoch;

        let sub_dimension = dimension / n_sub;
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
            _dimension: dimension,
            _n_sub: n_sub,
            _sub_dimension: sub_dimension,
            _sub_bits: sub_bits,
            _sub_bytes: sub_bytes,
            _n_sub_center: n_center_per_sub,
            _code_bytes: code_bytes,
            _n_kmeans_center: n_kmeans_center,
            _search_n_center: search_n_center,
            _ivflist: ivflist,
            _train_epoch: train_epoch,
            _is_trained: false,
            _n_items: 0,
            _max_item: 100000,
            mt: metrics::Metric::Unknown,
            ..Default::default()
        }
    }

    fn init_item(&mut self, data: &N) -> usize {
        let cur_id = self._n_items;
        // self._item2id.insert(item, cur_id);
        self._nodes.push(Box::new(data.clone()));
        self._n_items += 1;
        cur_id
    }

    fn add_item(&mut self, data: &N) -> Result<usize, &'static str> {
        if data.len() != self._dimension {
            return Err("dimension is different");
        }
        // if self._item2id.contains_key(&item) {
        //     //to_do update point
        //     return Ok(self._item2id[&item]);
        // }

        if self._n_items > self._max_item {
            return Err("The number of elements exceeds the specified limit");
        }

        let insert_id = self.init_item(data);
        Ok(insert_id)
    }

    fn train(&mut self) {
        let n_item = self._n_items;
        let dimension = self._dimension;
        let n_center = self._n_kmeans_center;
        let n_epoch = self._train_epoch;
        let mut cluster = kmeans::Kmeans::<E>::new(dimension, n_center, self.mt);
        let mut data_vec: Vec<Vec<E>> = Vec::new();
        for node in self._nodes.iter() {
            data_vec.push(node.vectors().to_vec());
        }
        cluster.set_range(0, dimension);
        cluster.train(n_item, &data_vec, n_epoch);
        let mut assigned_center: Vec<usize> = Vec::new();
        cluster.search_data(n_item, &data_vec, &mut assigned_center);
        self._centers = cluster.centers().to_vec();
        (0..n_item).for_each(|i| {
            let center_id = assigned_center[i];
            self._ivflist[center_id].push(i);
        });
        for i in 0..n_center {
            let mut center_pq = PQIndex::<E, T>::new(
                self._dimension,
                &PQParams::default()
                    .n_sub(self._n_sub)
                    .sub_bits(self._sub_bits)
                    .train_epoch(self._train_epoch),
            );

            for j in 0..self._ivflist[i].len() {
                center_pq
                    .add_item(&self._nodes[self._ivflist[i][j]].clone())
                    .unwrap();
            }
            center_pq.set_residual(self._centers[i].to_vec());
            center_pq.train_center();
            self._pq_list.push(center_pq);
        }

        self._is_trained = true;
    }

    fn get_distance_from_vec_range(&self, x: &N, y: &[E], begin: usize, end: usize) -> E {
        return metrics::metric(&x.vectors()[begin..end], y, self.mt).unwrap();
    }

    fn search_knn_adc(
        &self,
        search_data: &N,
        k: usize,
    ) -> Result<BinaryHeap<Neighbor<E, usize>>, &'static str> {
        let mut top_centers: BinaryHeap<Neighbor<E, usize>> = BinaryHeap::new();
        let n_kmeans_center = self._n_kmeans_center;
        let dimension = self._dimension;
        for i in 0..n_kmeans_center {
            top_centers.push(Neighbor::new(
                i,
                -self.get_distance_from_vec_range(search_data, &self._centers[i], 0, dimension),
            ))
        }

        let mut top_candidate: BinaryHeap<Neighbor<E, usize>> = BinaryHeap::new();
        for _i in 0..self._search_n_center {
            let center = top_centers.pop().unwrap().idx();
            let mut ret = self._pq_list[center]
                .search_knn_adc(search_data, k)
                .unwrap();
            while !ret.is_empty() {
                let mut ret_peek = ret.pop().unwrap();
                ret_peek._idx = self._ivflist[center][ret_peek._idx];
                top_candidate.push(ret_peek);
                if top_candidate.len() > k {
                    top_candidate.pop();
                }
            }
        }
        Ok(top_candidate)
    }
}

impl<E: node::FloatElement, T: node::IdxType, N: node::Node<E = E, T = T>>
    ann_index::ANNIndex<E, T, N> for IVFPQIndex<E, T, N>
{
    fn build(&mut self, _mt: metrics::Metric) -> Result<(), &'static str> {
        self.mt = _mt;
        self.train();
        Result::Ok(())
    }
    fn add_node(&mut self, item: &N) -> Result<(), &'static str> {
        match self.add_item(item) {
            Err(err) => Err(err),
            _ => Ok(()),
        }
    }
    fn built(&self) -> bool {
        true
    }

    fn node_search_k(&self, item: &N, k: usize) -> Vec<(N, E)> {
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
                *self._nodes[result_idx[cur_id].0].clone(),
                result_idx[cur_id].1,
            ));
        }
        result
    }

    fn name(&self) -> &'static str {
        "IVFPQIndex"
    }

    fn dimension(&self) -> usize {
        self._dimension
    }
}

impl<
        E: node::FloatElement + DeserializeOwned,
        T: node::IdxType + DeserializeOwned,
        N: node::Node<E = E, T = T> + DeserializeOwned,
    > ann_index::SerializableIndex<E, T, N> for IVFPQIndex<E, T, N>
{
    fn load(path: &str) -> Result<Self, &'static str> {
        let file = File::open(path).unwrap_or_else(|_| panic!("unable to open file {:?}", path));
        let mut instance: IVFPQIndex<E, T> = bincode::deserialize_from(&file).unwrap();
        instance._nodes = instance
            ._nodes_tmp
            .iter()
            .map(|x| Box::new(x.clone()))
            .collect();
        instance._nodes_tmp.clear();
        for i in 0..instance._n_kmeans_center {
            instance._pq_list[i]._nodes = instance._pq_list[i]
                ._nodes_tmp
                .iter()
                .map(|x| Box::new(x.clone()))
                .collect();
            instance._pq_list[i]._nodes_tmp.clear();
        }
        Ok(instance)
    }

    fn dump(&mut self, path: &str) -> Result<(), &'static str> {
        self._nodes_tmp = self._nodes.iter().map(|x| *x.clone()).collect();
        for i in 0..self._n_kmeans_center {
            self._pq_list[i]._nodes_tmp =
                self._pq_list[i]._nodes.iter().map(|x| *x.clone()).collect();
        }
        let encoded_bytes = bincode::serialize(&self).unwrap();
        let mut file = File::create(path).unwrap();
        file.write_all(&encoded_bytes)
            .unwrap_or_else(|_| panic!("unable to write file {:?}", path));
        Result::Ok(())
    }
}
