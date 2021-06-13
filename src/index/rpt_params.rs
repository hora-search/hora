#![allow(dead_code)]
use crate::core::ann_index;
use crate::core::arguments;
use crate::core::metrics;
use crate::core::neighbor;
use crate::core::node;
use fixedbitset::FixedBitSet;
#[cfg(feature = "without_std")]
use hashbrown::HashSet;
use rand::prelude::*;
use rayon::prelude::*;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::LinkedList;

use crate::core::kmeans;
#[cfg(not(feature = "without_std"))]
use std::collections::HashSet;
use std::collections::VecDeque;

use std::fs::File;
use std::io::Write;
use std::sync::{Arc, Mutex};

#[derive(Debug, Serialize, Deserialize)]
pub struct BPTParams {
    pub tree_num: i32,
    pub candidate_size: i32,
}

impl BPTParams {
    pub fn tree_num(mut self, new_tree_num: i32) -> Self {
        self.tree_num = new_tree_num;
        self
    }
    pub fn candidate_size(mut self, new_candidate_size: i32) -> Self {
        self.candidate_size = new_candidate_size;
        self
    }
}

impl Default for BPTParams {
    fn default() -> Self {
        BPTParams {
            tree_num: 0,
            candidate_size: 0,
        }
    }
}
