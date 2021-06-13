#![allow(dead_code)]
use crate::core::ann_index;
use crate::core::arguments;
use crate::core::metrics;
use crate::core::neighbor;
use crate::core::node;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::collections::BinaryHeap;

#[derive(Debug, Serialize, Deserialize)]
pub struct BruteForceParams {}

impl BruteForceParams {}

impl Default for BruteForceParams {
    fn default() -> Self {
        BruteForceParams {}
    }
}
