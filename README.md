# Hora

approximate nearest neighbor search library in Rust

## Introduction

Hora, `ほら` in Japanese, sound like `hōlə`, means `You see!` or `Look at that!`.

## Key Features

* **Performant** ⚡️
  * SIMD acceleration
  * stuble algorithm implementation
  * multiple threads design

* **Multi Language Support** ☄️ 
  * `Python`
  * `Javascript`
  * `Java`
  * `Go`
  * `Ruby`
  * `Swift` (WIP)
  * `R` (WIP)
  * `Julia` (WIP)
  * and it also can serve as a service

* **Portable** 💼
  * `no_std` support (WIP)
  * `Windows`, `Linux` and `OS X` support
  * `IOS` and `Android` Support (WIP)
  * thanks for `LLVM`, Hora can be used in `x66` and `ARM` CPUs

* **security** 🔒
  * thanks for rust strict compiler
  * all language lib's Hora memory is managed by the Rust
  * full coverage testing

* **Multiple Index support** 🚀
  * `Hierarchical Navigable Small World Graph Index(HNSW)` ([reference](https://arxiv.org/abs/1603.09320))
  * `Satellite System Graph (SSG)` ([reference](https://arxiv.org/abs/1907.06146))
  * `Product Quantization Inverted File(PQIVF)` ([reference](https://lear.inrialpes.fr/pubs/2011/JDS11/jegou_searching_with_quantization.pdf))
  * `Random Projection Tree(RPT)`
  * `BruteForce`

* **Light** 💡
  * the whole library did not dependent any heavy library, such as `BLAS`

* **Configurable Compilation** 📕
  * Hora support some features, such as `SIMD`

* **Multiple Distances Support** 🧮
  * `Dot Product distance`
    * ![equation](https://latex.codecogs.com/gif.latex?D%28x%2Cy%29%20%3D%20%5Csum%7B%28x*y%29%7D)
  * `Euclidean distance`
    * ![equation](https://latex.codecogs.com/gif.latex?D%28x%2Cy%29%20%3D%20%5Csqrt%7B%5Csum%7B%28x-y%29%5E2%7D%7D)
  * `Manhattan distance`
    * ![equation](https://latex.codecogs.com/gif.latex?D%28x%2Cy%29%20%3D%20%5Csum%7B%7C%28x-y%29%7C%7D)
  * `cosine distance`
    * ![equation](https://latex.codecogs.com/gif.latex?D%28x%2Cy%29%20%3D%20%5Cfrac%7Bx%20*y%7D%7B%7C%7Cx%7C%7C*%7C%7Cy%7C%7C%7D)

* **Productive** ⭐
  * well documented
  * elegant and simple API, which is extremely easy to learn

## Installation

### rust

add this into `Cargo.toml`

```toml
[dependencies]
hora = "0.1.0"
```

### Python

```Bash
pip install hora
```

### Building from source

```bash
git clone https://github.com/hora-search/hora
cargo build
```

## Interface

All Index have already implement these method

```Rust
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
    fn node_search_k(
        &self,
        item: &node::Node<E, T>,
        k: usize,
        args: &arguments::Args,
    ) -> Vec<(node::Node<E, T>, E)>;

    /// search for k nearest neighbors and return full info
    ///
    /// it will return the all node's info including the original vectors, and the metric distance
    ///
    /// it require the item is the slice with the same dimension with index dimension, otherwise it will panic
    fn search_full(&self, item: &[E], k: usize) -> Vec<(node::Node<E, T>, E)> {
        assert_eq!(item.len(), self.dimension());
        self.node_search_k(&node::Node::new(item), k, &arguments::Args::new())
    }

    /// search for k nearest neighbors
    ///
    /// it only return the idx of the nearest node
    ///
    /// it require the item is the slice with the same dimension with index dimension, otherwise it will panic
    fn search(&self, item: &[E], k: usize) -> Vec<T> {
        assert_eq!(item.len(), self.dimension());
        self.node_search_k(&node::Node::new(item), k, &arguments::Args::new())
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

pub trait SerializableIndex<
    E: node::FloatElement + DeserializeOwned,
    T: node::IdxType + DeserializeOwned,
>: Send + Sync + ANNIndex<E, T>
{
    /// load file with path
    fn load(_path: &str, _args: &arguments::Args) -> Result<Self, &'static str>
    where
        Self: Sized,
    {
        Err("empty implementation")
    }

    /// dump the file into the path
    fn dump(&mut self, _path: &str, _args: &arguments::Args) -> Result<(), &'static str> {
        Err("empty implementation")
    }
}
```

## Benchmark

pic here

## Example

Rust usage example

```Rust

```

Python usage exmaple

```Python

```

## Related Project and Comparison

* [Faiss](https://github.com/facebookresearch/faiss): Facebook AI Similarity Search, which is the most popular ANN library currently
  * Diffrences: Faiss more focus on the GPU scene, and Hora is more light than Faiss

* Annoy

### Contribute

we are pretty gald to have you to participate, any contributions is welcome, including the documentations and tests.
you can do the  `Pull Requests`, `Issue` on the github, and we will review it as soon as possible.

We use GitHub issues for tracking suggestions and bugs.

To install for development:

#### clone the repo

```bash
git clone https://github.com/hora-search/hora
```

#### build

```bash
cargo build
```

#### try the changes

```bash
cd exmaples
cargo run
```

## License

The entire repo is under Apache License.
