//! Node and element types for vector representations in the index.

#![allow(dead_code)]

use crate::core::metrics;
use crate::core::simd_metrics;
use core::{hash::Hash, iter::Sum};
use num::traits::{FromPrimitive, NumAssign};
use serde::{Deserialize, Serialize};

/// Element type for vector components: either `f32` or `f64`.
pub trait FloatElement:
    FromPrimitive
    + Sized
    + Default
    + num::Zero
    + num::traits::FloatConst
    + core::fmt::Debug
    + Clone
    + Copy
    + PartialEq
    + PartialOrd
    + NumAssign
    + num::Signed
    + num::Float
    + Sync
    + Send
    + Sum
    + Serialize
    + simd_metrics::SIMDOptimized
{
    fn float_one() -> Self;

    fn float_two() -> Self;

    fn float_zero() -> Self;

    fn zero_patch_num() -> Self;

    /// Returns the maximum finite value for this float type.
    fn max_value() -> Self;

    /// Converts a usize to this float type.
    fn from_usize(n: usize) -> Option<Self>;
}

/// Type used for node/data indices (e.g. `usize`, `i64`, `String`).
pub trait IdxType:
    Sized + Clone + Default + core::fmt::Debug + Eq + Ord + Sync + Send + Serialize + Hash
{
}

#[macro_export]
macro_rules! to_float_element {
    (  $x:ident  ) => {
        impl FloatElement for $x {
            fn float_one() -> Self {
                1.0
            }

            fn float_two() -> Self {
                2.0
            }

            fn float_zero() -> Self {
                0.0
            }

            fn zero_patch_num() -> Self {
                1.34e-6
            }

            fn max_value() -> Self {
                <$x>::MAX
            }

            fn from_usize(n: usize) -> Option<Self> {
                <$x as num::traits::FromPrimitive>::from_u64(n as u64)
            }
        }
    };
}

#[macro_export]
macro_rules! to_idx_type {
    (  $x:ident  ) => {
        impl IdxType for $x {}
    };
}

to_float_element!(f64);
to_float_element!(f32);
to_idx_type!(String);
to_idx_type!(usize);
to_idx_type!(i16);
to_idx_type!(i32);
to_idx_type!(i64);
to_idx_type!(i128);
to_idx_type!(u16);
to_idx_type!(u32);
to_idx_type!(u64);
to_idx_type!(u128);

/// A point in the metric space: embedding vector plus optional index.
///
/// Holds a slice of `FloatElement` (the vector) and an optional id of type `T`.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Node<E: FloatElement, T: IdxType> {
    /// Embedding vector.
    vectors: Vec<E>,
    /// Optional user-defined index (e.g. database id).
    idx: Option<T>,
}

impl<E: FloatElement, T: IdxType> Node<E, T> {
    /// Creates a node from a vector without an index.
    pub fn new(vectors: &[E]) -> Node<E, T> {
        Node::<E, T>::valid_elements(vectors);
        Node {
            vectors: vectors.to_vec(),
            idx: None,
        }
    }

    /// Creates a node from a vector with the given index.
    pub fn new_with_idx(vectors: &[E], id: T) -> Node<E, T> {
        let mut n = Node::new(vectors);
        n.set_idx(id);
        n
    }

    /// Computes the distance to another node under the given metric.
    pub fn metric(&self, other: &Node<E, T>, t: metrics::Metric) -> Result<E, &'static str> {
        metrics::metric(&self.vectors, &other.vectors, t)
    }

    /// Returns the embedding vector.
    pub fn vectors(&self) -> &Vec<E> {
        &self.vectors
    }

    /// Returns mutable reference to the embedding vector.
    pub fn mut_vectors(&mut self) -> &mut Vec<E> {
        &mut self.vectors
    }

    /// Replaces the embedding vector.
    pub fn set_vectors(&mut self, v: &[E]) {
        self.vectors = v.to_vec();
    }

    /// Returns the dimension (length of the vector).
    pub fn len(&self) -> usize {
        self.vectors.len()
    }

    /// Returns true if the vector is empty.
    pub fn is_empty(&self) -> bool {
        self.vectors.is_empty()
    }

    /// Returns the optional index of this node.
    pub fn idx(&self) -> &Option<T> {
        &self.idx
    }

    fn set_idx(&mut self, id: T) {
        self.idx = Some(id);
    }

    /// Panics if any element is NaN or infinite.
    fn valid_elements(vectors: &[E]) -> bool {
        for e in vectors.iter() {
            if e.is_nan() || e.is_infinite() {
                panic!("invalid float element: expected finite number");
            }
        }
        true
    }
}

impl<E: FloatElement, T: IdxType> core::fmt::Display for Node<E, T> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "(key: {:#?}, vectors: {:#?})", self.idx, self.vectors)
    }
}

// general method

#[cfg(test)]
#[test]
fn node_test() {
    // f64
    let v = vec![1.0, 1.0];
    let v2 = vec![2.0, 2.0];
    let n = Node::<f64, usize>::new(&v);
    let n2 = Node::<f64, usize>::new(&v2);
    assert_eq!(n.metric(&n2, metrics::Metric::Manhattan).unwrap(), 2.0);
}
