#![allow(dead_code)]

use crate::core::node;

#[cfg(feature = "without_std")]
use hashbrown::HashSet;

use serde::{Deserialize, Serialize};

#[cfg(not(feature = "without_std"))]
#[derive(Debug, Serialize, Deserialize)]
pub struct SSGParams<E: node::FloatElement> {
    pub angle: E,
    pub init_k: usize,
    pub index_size: usize,
    pub neighbor_neighbor_size: usize,
    pub root_size: usize,
}

impl<E: node::FloatElement> SSGParams<E> {
    pub fn angle(mut self, new_angle: f32) -> Self {
        self.angle = E::from_f32(new_angle).unwrap();
        self
    }
    pub fn init_k(mut self, new_init_k: usize) -> Self {
        self.init_k = new_init_k;
        self
    }
    pub fn index_size(mut self, new_index_size: usize) -> Self {
        self.index_size = new_index_size;
        self
    }
    pub fn neighbor_neighbor_size(mut self, new_neighbor_neighbor_size: usize) -> Self {
        self.neighbor_neighbor_size = new_neighbor_neighbor_size;
        self
    }
    pub fn root_size(mut self, new_root_size: usize) -> Self {
        self.root_size = new_root_size;
        self
    }
}

impl<E: node::FloatElement> Default for SSGParams<E> {
    fn default() -> Self {
        SSGParams {
            angle: E::from_f32(30.0).unwrap(),
            init_k: 100,
            index_size: 100,
            neighbor_neighbor_size: 100,
            root_size: 30,
        }
    }
}
