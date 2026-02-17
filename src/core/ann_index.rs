//! ANN index trait and serialization interface.

use crate::core::metrics;
use crate::core::node;
use serde::de::DeserializeOwned;

/// Trait for Approximate Nearest Neighbor (ANN) indexes.
///
/// Implementors support adding nodes, building the index, and querying for k nearest neighbors.
/// After construction, call [`build`](ANNIndex::build) to finalize the index for search.
///
/// # Example
///
/// ```ignore
/// let mut idx = Box::new(BruteForceIndex::<f32, usize>::new(dim, &params));
/// for (i, emb) in embs.iter().enumerate() {
///     idx.add_node(&Node::new_with_idx(emb, i)).unwrap();
/// }
/// idx.build(Metric::Euclidean).unwrap();
/// let nearest = idx.search(&embs[0], 10);
/// ```
pub trait ANNIndex<E: node::FloatElement, T: node::IdxType>: Send + Sync {
    /// Builds the index from all previously added nodes. May be expensive depending on the algorithm.
    fn build(&mut self, mt: metrics::Metric) -> Result<(), &'static str>;

    /// Adds a single node (vector + index) to the index.
    fn add_node(&mut self, item: &node::Node<E, T>) -> Result<(), &'static str>;

    /// Adds a node from a slice and index; delegates to `add_node`.
    fn add(&mut self, vs: &[E], idx: T) -> Result<(), &'static str> {
        self.add_node(&node::Node::new_with_idx(vs, idx))
    }

    /// Adds multiple nodes at once. Returns an error if `vss.len() != indices.len()`.
    fn madd(&mut self, vss: &[&[E]], indices: &[T]) -> Result<(), &'static str> {
        if vss.len() != indices.len() {
            return Err("vector count does not match index count");
        }
        for (vs, id) in vss.iter().zip(indices) {
            let n = node::Node::new_with_idx(vs, id.clone());
            if let Err(err) = self.add_node(&n) {
                return Err(err);
            }
        }
        Ok(())
    }

    /// Returns whether the index has been built (ready for search).
    fn built(&self) -> bool;

    /// Rebuilds the index with current nodes. Default implementation returns an error.
    fn rebuild(&mut self, _mt: metrics::Metric) -> Result<(), &'static str> {
        Err("not implemented")
    }

    /// Internal: search for k nearest neighbors given a node. Implementors override this.
    fn node_search_k(&self, item: &node::Node<E, T>, k: usize) -> Vec<(node::Node<E, T>, E)>;

    /// Returns k nearest neighbors as (node, distance). Panics if `item.len() != dimension()`.
    fn search_nodes(&self, item: &[E], k: usize) -> Vec<(node::Node<E, T>, E)> {
        assert_eq!(item.len(), self.dimension());
        self.node_search_k(&node::Node::new(item), k)
    }

    /// Returns indices of k nearest neighbors. Panics if `item.len() != dimension()`.
    fn search(&self, item: &[E], k: usize) -> Vec<T> {
        assert_eq!(item.len(), self.dimension());
        self.node_search_k(&node::Node::new(item), k)
            .into_iter()
            .map(|(n, _)| n.idx().as_ref().unwrap().clone())
            .collect::<Vec<T>>()
    }

    /// Human-readable index name (e.g. `"HNSWIndex"`).
    fn name(&self) -> &'static str;

    /// Number of nodes in the index. Default is 0.
    fn nodes_size(&self) -> usize {
        0
    }

    /// Clears all nodes and built index state.
    fn clear(&mut self) {}

    /// Optional statistics string for the index. Default: `"not implemented"`.
    fn idx_info(&self) -> String {
        "not implemented".to_string()
    }

    /// Vector dimension required by this index.
    fn dimension(&self) -> usize {
        0
    }
}

/// Index that can be serialized to and loaded from disk (e.g. bincode).
///
/// Use [`dump`](SerializableIndex::dump) to save and [`load`](SerializableIndex::load) to restore.
pub trait SerializableIndex<
    E: node::FloatElement + DeserializeOwned,
    T: node::IdxType + DeserializeOwned,
>: Send + Sync + ANNIndex<E, T>
{
    /// Loads the index from a file at `path`. Default returns an error.
    fn load(_path: &str) -> Result<Self, &'static str>
    where
        Self: Sized,
    {
        Err("empty implementation")
    }

    /// Saves the index to a file at `path`. Default returns an error.
    fn dump(&mut self, _path: &str) -> Result<(), &'static str> {
        Err("empty implementation")
    }
}
