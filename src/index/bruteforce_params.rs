#![allow(dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BruteForceParams {}

impl BruteForceParams {}

impl Default for BruteForceParams {
    fn default() -> Self {
        BruteForceParams {}
    }
}
