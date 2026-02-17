//! Parameters for brute-force index (currently no tunables).

#![allow(dead_code)]

use serde::{Deserialize, Serialize};

/// Parameters for [`BruteForceIndex`](crate::index::bruteforce_idx::BruteForceIndex). Empty for now.
#[derive(Debug, Serialize, Deserialize)]
pub struct BruteForceParams {}

impl BruteForceParams {}

impl Default for BruteForceParams {
    fn default() -> Self {
        BruteForceParams {}
    }
}
