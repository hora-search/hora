extern crate num;
use crate::core::node;
use core::cmp::Ordering;

/// internal temp structure
///
#[derive(Default, Clone, PartialEq, Debug)]
pub struct Neighbor<E: node::FloatElement, T: node::IdxType> {
    pub _idx: T,
    pub _distance: E,
}

impl<E: node::FloatElement, T: node::IdxType> Neighbor<E, T> {
    pub fn new(idx: T, distance: E) -> Neighbor<E, T> {
        Neighbor {
            _idx: idx,
            _distance: distance,
        }
    }

    pub fn idx(&self) -> T {
        self._idx.clone()
    }

    pub fn distance(&self) -> E {
        self._distance
    }
}

impl<E: node::FloatElement, T: node::IdxType> Ord for Neighbor<E, T> {
    fn cmp(&self, other: &Neighbor<E, T>) -> Ordering {
        self._distance.partial_cmp(&other._distance).unwrap()
    }
}

impl<E: node::FloatElement, T: node::IdxType> PartialOrd for Neighbor<E, T> {
    fn partial_cmp(&self, other: &Neighbor<E, T>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<E: node::FloatElement, T: node::IdxType> Eq for Neighbor<E, T> {}
