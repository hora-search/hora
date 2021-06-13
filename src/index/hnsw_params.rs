#![allow(dead_code)]

use crate::core::node;

#[cfg(feature = "without_std")]
use hashbrown::HashMap;
#[cfg(feature = "without_std")]
use hashbrown::HashSet;

use serde::{Deserialize, Serialize};

#[cfg(not(feature = "without_std"))]
#[cfg(not(feature = "without_std"))]
#[derive(Debug, Serialize, Deserialize)]
pub struct HNSWParams<E: node::FloatElement> {
    pub max_item: usize,
    pub n_neighbor: usize,
    pub n_neighbor0: usize,
    pub max_level: usize,
    pub ef_build: usize,
    pub ef_search: usize,
    pub has_deletion: bool,
    pub e_type: E,
}

impl<E: node::FloatElement> HNSWParams<E> {
    pub fn max_item(mut self, new_max_item: usize) -> Self {
        self.max_item = new_max_item;
        self
    }

    pub fn n_neighbor(mut self, new_n_neighbor: usize) -> Self {
        self.n_neighbor = new_n_neighbor;
        self
    }

    pub fn n_neighbor0(mut self, new_n_neighbor0: usize) -> Self {
        self.n_neighbor0 = new_n_neighbor0;
        self
    }

    pub fn ef_build(mut self, new_ef_build: usize) -> Self {
        self.ef_build = new_ef_build;
        self
    }

    pub fn ef_search(mut self, new_ef_search: usize) -> Self {
        self.ef_search = new_ef_search;
        self
    }

    pub fn has_deletion(mut self, new_has_deletion: bool) -> Self {
        self.has_deletion = new_has_deletion;
        self
    }
}

impl<E: node::FloatElement> Default for HNSWParams<E> {
    fn default() -> Self {
        HNSWParams {
            max_item: 1000000,
            n_neighbor: 32,
            n_neighbor0: 64,
            max_level: 20,
            ef_build: 500,
            ef_search: 16,
            has_deletion: false,
            e_type: E::from_f32(0.0).unwrap(),
        }
    }
}
