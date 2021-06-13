#![allow(dead_code)]
use crate::core::ann_index;
use crate::core::arguments;
use crate::core::kmeans;
use crate::core::metrics;
use crate::core::neighbor::Neighbor;
use crate::core::node;

use rayon::prelude::*;
use serde::de::DeserializeOwned;
use std::collections::BinaryHeap;

use serde::{Deserialize, Serialize};

use std::fs::File;

use std::io::Write;

#[derive(Debug, Serialize, Deserialize)]
pub struct PQParams<E: node::FloatElement> {
    pub n_sub: usize,
    pub sub_bits: usize,
    pub train_epoch: usize,
    pub e_type: E,
}

impl<E: node::FloatElement> PQParams<E> {
    pub fn n_sub(mut self, new_n_sub: usize) -> Self {
        self.n_sub = new_n_sub;
        self
    }

    pub fn sub_bits(mut self, new_sub_bits: usize) -> Self {
        self.sub_bits = new_sub_bits;
        self
    }

    pub fn train_epoch(mut self, new_train_epoch: usize) -> Self {
        self.train_epoch = new_train_epoch;
        self
    }
}

impl<E: node::FloatElement> Default for PQParams<E> {
    fn default() -> Self {
        PQParams {
            n_sub: 4,
            sub_bits: 4,
            train_epoch: 100,
            e_type: E::from_f32(0.0).unwrap(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IVFPQParams<E: node::FloatElement> {
    pub n_sub: usize,
    pub sub_bits: usize,
    pub n_kmeans_center: usize,
    pub search_n_center: usize,
    pub train_epoch: usize,
    pub e_type: E,
}

impl<E: node::FloatElement> IVFPQParams<E> {
    pub fn n_sub(mut self, new_n_sub: usize) -> Self {
        self.n_sub = new_n_sub;
        self
    }

    pub fn sub_bits(mut self, new_sub_bits: usize) -> Self {
        self.sub_bits = new_sub_bits;
        self
    }

    pub fn n_kmeans_center(mut self, new_n_kmeans_center: usize) -> Self {
        self.n_kmeans_center = new_n_kmeans_center;
        self
    }

    pub fn search_n_center(mut self, new_search_n_center: usize) -> Self {
        self.search_n_center = new_search_n_center;
        self
    }

    pub fn train_epoch(mut self, new_train_epoch: usize) -> Self {
        self.train_epoch = new_train_epoch;
        self
    }
}

impl<E: node::FloatElement> Default for IVFPQParams<E> {
    fn default() -> Self {
        IVFPQParams {
            n_sub: 25,
            sub_bits: 4,
            n_kmeans_center: 256,
            search_n_center: 8,
            train_epoch: 100,
            e_type: E::from_f32(0.0).unwrap(),
        }
    }
}
