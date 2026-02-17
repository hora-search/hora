//! Neighbor entry used in search results (index + distance).

use crate::core::node;
use core::cmp::Ordering;

/// A candidate neighbor: index and distance for ordering in heaps.
#[derive(Default, Clone, PartialEq, Debug)]
pub struct Neighbor<E: node::FloatElement, T: node::IdxType> {
    /// Index of the neighbor.
    id: T,
    /// Distance to the query (used for ordering; smaller is closer).
    dist: E,
}

impl<E: node::FloatElement, T: node::IdxType> Neighbor<E, T> {
    pub fn new(idx: T, distance: E) -> Neighbor<E, T> {
        Neighbor {
            id: idx,
            dist: distance,
        }
    }

    pub fn idx(&self) -> T {
        self.id.clone()
    }

    pub fn distance(&self) -> E {
        self.dist
    }
}

impl<E: node::FloatElement, T: node::IdxType> Ord for Neighbor<E, T> {
    fn cmp(&self, other: &Neighbor<E, T>) -> Ordering {
        self.dist.partial_cmp(&other.dist).unwrap()
    }
}

impl<E: node::FloatElement, T: node::IdxType> PartialOrd for Neighbor<E, T> {
    fn partial_cmp(&self, other: &Neighbor<E, T>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<E: node::FloatElement, T: node::IdxType> Eq for Neighbor<E, T> {}
