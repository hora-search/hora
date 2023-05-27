#![allow(dead_code)]
use crate::core::metrics;
use crate::core::simd_metrics;
use core::{hash::Hash, iter::Sum};
use num::traits::{FromPrimitive, NumAssign};
use serde::{Deserialize, Serialize};

/// FloatElement trait, the generic of two primitive type `f32` and `f64`
///
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
    + simd_metrics::SIMDOptmized
{
    fn float_one() -> Self;

    fn float_two() -> Self;

    fn float_zero() -> Self;

    fn zero_patch_num() -> Self;
}

/// IdxType trait indicate the primitive type used for the data index
///
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
                1.0
            }

            fn float_zero() -> Self {
                0.0
            }

            fn zero_patch_num() -> Self {
                1.34e-6
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

pub trait Node: Clone + Send + Sync {
    type E;
    type T;
    fn new(vectors: &[Self::E]) -> Self;
    fn new_with_idx(vectors: &[Self::E], id: Self::T) -> Self;
    fn metric(
        &self,
        other: &impl Node<E = Self::E, T = Self::T>,
        t: metrics::Metric,
    ) -> Result<Self::E, &'static str>;
    fn vectors(&self) -> &Vec<Self::E>;
    fn mut_vectors(&mut self) -> &mut Vec<Self::E>;
    fn set_vectors(&mut self, v: &[Self::E]);
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn idx(&self) -> &Option<Self::T>;
    fn set_idx(&mut self, id: Self::T);
    fn valid_elements(vectors: &[Self::E]) -> bool;
}

/// Node is the main container for the point in the space
///
/// it contains a array of `FloatElement` and a index
///
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct MemoryNode<E: FloatElement, T: IdxType> {
    vectors: Vec<E>,
    idx: Option<T>, // data id, it can be any type;
}

impl<E: FloatElement, T: IdxType> Node for MemoryNode<E, T> {
    type E = E;
    type T = T;
    /// new without idx
    ///
    /// new a point without a idx
    fn new(vectors: &[E]) -> MemoryNode<E, T> {
        MemoryNode::<E, T>::valid_elements(vectors);
        MemoryNode {
            vectors: vectors.to_vec(),
            idx: Option::None,
        }
    }

    /// new with idx
    ///
    /// new a point with a idx
    fn new_with_idx(vectors: &[E], id: T) -> MemoryNode<E, T> {
        let mut n = MemoryNode::new(vectors);
        n.set_idx(id);
        n
    }

    /// calculate the point distance
    fn metric(&self, other: &impl Node, t: metrics::Metric) -> Result<E, &'static str> {
        metrics::metric(&self.vectors, &other.vectors(), t)
    }

    // return internal embeddings
    fn vectors(&self) -> &Vec<E> {
        &self.vectors
    }

    // return mut internal embeddings
    fn mut_vectors(&mut self) -> &mut Vec<E> {
        &mut self.vectors
    }

    // set internal embeddings
    fn set_vectors(&mut self, v: &[E]) {
        self.vectors = v.to_vec();
    }

    // internal embeddings length
    fn len(&self) -> usize {
        self.vectors.len()
    }

    fn is_empty(&self) -> bool {
        self.vectors.is_empty()
    }

    // return node's idx
    fn idx(&self) -> &Option<T> {
        &self.idx
    }

    fn set_idx(&mut self, id: T) {
        self.idx = Option::Some(id);
    }

    fn valid_elements(vectors: &[E]) -> bool {
        for e in vectors.iter() {
            if e.is_nan() || e.is_infinite() {
                //TODO: log
                panic!("invalid float element");
            }
        }
        true
    }
}

impl<E: FloatElement, T: IdxType> core::fmt::Display for MemoryNode<E, T> {
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
