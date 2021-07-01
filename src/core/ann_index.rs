use crate::core::metrics;
use crate::core::node;

use serde::de::DeserializeOwned;

/// ANNIndex trait provide the all `Approximate Nearest Neighbor Search` problem required method
///
/// ANNIndex is the main trait that all `Approximate Nearest Neighbor Search` algorithm index have to implement
///
/// Initial a ANNIndex and call `.build` method, it will build up the index internal to speed up the ANN search.
///
///
/// Example:
///
/// ```
/// let mut bf_idx = Box::new(bf::bf::BruteForceIndex::<f32, usize>::new()); // use BruteForceIndex
/// for i in 0..embs.len() {
///    bf_idx.add_node(&core::node::Node::<E, usize>::new_with_idx(&embs[i], i)); // add index
/// }
/// idx.build(core::metrics::Metric::Euclidean).unwrap(); // build up index
/// println!("{embedding {}'s nearest neighbor is {}}", 0, bf_idx.search(embs[0]);
/// ```
///

pub trait ANNIndex<E: node::FloatElement, T: node::IdxType>: Send + Sync {
    /// build up the ANN index
    ///
    /// build up index with all node which have add into before, it will cost some time, and the time it cost depends on the algorithm
    /// return `Err(&'static str)` if there is something wrong with the building process, and the `static str` is the debug reason
    fn build(&mut self, mt: metrics::Metric) -> Result<(), &'static str>;

    /// add node internal method
    ///
    /// it will allocate a space in the heap(Vector), and init a `Node`
    /// return `Err(&'static str)` if there is something wrong with the adding process, and the `static str` is the debug reason
    fn add_node(&mut self, item: &node::Node<E, T>) -> Result<(), &'static str>;

    /// add node
    ///
    /// call `add_node()` internal
    fn add(&mut self, vs: &[E], idx: T) -> Result<(), &'static str> {
        self.add_node(&node::Node::new_with_idx(vs, idx))
    }

    /// add multiple node one time
    ///
    /// return `Err(&'static str)` if there is something wrong with the adding process, and the `static str` is the debug reason
    fn add_batch(&mut self, vss: &[&[E]], indices: &[T]) -> Result<(), &'static str> {
        if vss.len() != indices.len() {
            return Err("vector's size is different with index");
        }
        for idx in 0..vss.len() {
            let n = node::Node::new_with_idx(vss[idx], indices[idx].clone());
            if let Err(err) = self.add_node(&n) {
                return Err(err);
            }
        }
        Ok(())
    }

    /// return the index has already been built or not
    ///
    /// return `True` if the index has been built
    fn built(&self) -> bool;

    /// to rebuild the index with all nodes inside
    ///
    /// /// return `Err(&'static str)` if there is something wrong with the rebuilding process, and the `static str` is the debug reason
    fn rebuild(&mut self, _mt: metrics::Metric) -> Result<(), &'static str> {
        Err("not implement")
    }

    /// search for k nearest neighbors node internal method
    fn node_search_k(&self, item: &node::Node<E, T>, k: usize) -> Vec<(node::Node<E, T>, E)>;

    /// search for k nearest neighbors and return full info
    ///
    /// it will return the all node's info including the original vectors, and the metric distance
    ///
    /// it require the item is the slice with the same dimension with index dimension, otherwise it will panic
    fn search_full(&self, item: &[E], k: usize) -> Vec<(node::Node<E, T>, E)> {
        assert_eq!(item.len(), self.dimension());
        self.node_search_k(&node::Node::new(item), k)
    }

    /// search for k nearest neighbors
    ///
    /// it only return the idx of the nearest node
    ///
    /// it require the item is the slice with the same dimension with index dimension, otherwise it will panic
    fn search(&self, item: &[E], k: usize) -> Vec<T> {
        assert_eq!(item.len(), self.dimension());
        self.node_search_k(&node::Node::new(item), k)
            .iter()
            .map(|x| x.0.idx().as_ref().unwrap().clone())
            .collect::<Vec<T>>()
    }

    /// return the name of the Index
    /// format like this
    /// `HNSWIndex(Hierarchical Navigable Small World Index)`
    fn name(&self) -> &'static str;

    /// internal nodes' size
    fn nodes_size(&self) -> usize {
        0
    }

    /// clear all nodes and index built before
    fn clear(&mut self) {}

    /// return String of Index statistics informations
    fn idx_info(&self) -> String {
        "not implement".to_string()
    }

    /// return the dimension it require
    fn dimension(&self) -> usize {
        0
    }
}

/// SerializableIndex provide the `Serialization` and `Deserialization` method for the index
/// SerializableIndex is the main trait that all index have to implement
///
/// call `.dump` method to dump a binary format file in the disk, and the binary file include all nodes which have added into
/// call `.load' method to load a binary format file to load back the Index built before, and the Index loaded have all Nodes' info the binary file have
///
///
/// Example:
///
/// ```
/// let mut bf_idx = Box::new(bf::bf::BruteForceIndex::<f32, usize>::new()); // use BruteForceIndex
/// for i in 0..embs.len() {
///    bf_idx.add_node(&core::node::Node::<E, usize>::new_with_idx(&embs[i], i)); // add index
/// }
/// bf_idx.dump("bf_idx.idx", &arguments::Args::new());
/// let bf_idx2 = Box::new(bf::bf::BruteForceIndex::<f32, usize>::load("bf_idx.idx", &argument).unwrap());
/// ```
///
pub trait SerializableIndex<
    E: node::FloatElement + DeserializeOwned,
    T: node::IdxType + DeserializeOwned,
>: Send + Sync + ANNIndex<E, T>
{
    /// load file with path
    fn load(_path: &str) -> Result<Self, &'static str>
    where
        Self: Sized,
    {
        Err("empty implementation")
    }

    /// dump the file into the path
    fn dump(&mut self, _path: &str) -> Result<(), &'static str> {
        Err("empty implementation")
    }
}
