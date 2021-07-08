<div align="center">
  <img src="asset/logo.svg" width="70%"/>
</div>

# Hora

***Hora Search Everywhere!***

Hora, a approximate nearest neighbor search algorithm library, all code implemented in Rust 🦀, because we think rust code is safe, high level abstraction and the speed is as fast as c++.

Hora, **`ほら`** in Japanese, sound like `[hōlə]`, means `Wow`, `You see!` or `Look at that!`. The name is inspired by a famous Japanese song **`小さな恋のうた`**.

# Key Features

* **Performant** ⚡️
  * **SIMD-Accelerated ([packed_simd](https://github.com/rust-lang/packed_simd))**
  * **Stable Algorithm Implementation**
  * **Multiple Threads Design**

* **Multiple Languages Support** ☄️
  * `Python`
  * `Javascript`
  * `Java`
  * `Go` (WIP)
  * `Ruby` (WIP)
  * `Swift` (WIP)
  * `R` (WIP)
  * `Julia` (WIP)
  * **also can serve as a service**

* **Multiple Indexes Support** 🚀
  * `Hierarchical Navigable Small World Graph Index(HNSW)` ([detail](https://arxiv.org/abs/1603.09320))
  * `Satellite System Graph (SSG)` ([detail](https://arxiv.org/abs/1907.06146))
  * `Product Quantization Inverted File(PQIVF)` ([detail](https://lear.inrialpes.fr/pubs/2011/JDS11/jegou_searching_with_quantization.pdf))
  * `Random Projection Tree(RPT)` (LSH, WIP)
  * `BruteForce` (naive implementation with SIMD)

* **Portable** 💼
  * `no_std` support (in the future, not full support)
  * `Windows`, `Linux` and `OS X` Support
  * `IOS` and `Android` Support (WIP)
  * **without** any heavy library, such as `BLAS`

* **Security** 🔒
  * rust compiler guarantee all code
  * language lib like `Python lib`, the memory is managed by the Rust
  * great testing coverage

* **Multiple Distances Support** 🧮
  * `Dot Product Distance`
    * ![equation](https://latex.codecogs.com/gif.latex?D%28x%2Cy%29%20%3D%20%5Csum%7B%28x*y%29%7D)
  * `Euclidean Distance`
    * ![equation](https://latex.codecogs.com/gif.latex?D%28x%2Cy%29%20%3D%20%5Csqrt%7B%5Csum%7B%28x-y%29%5E2%7D%7D)
  * `Manhattan Distance`
    * ![equation](https://latex.codecogs.com/gif.latex?D%28x%2Cy%29%20%3D%20%5Csum%7B%7C%28x-y%29%7C%7D)
  * `Cosine Similarity`
    * ![equation](https://latex.codecogs.com/gif.latex?D%28x%2Cy%29%20%3D%20%5Cfrac%7Bx%20*y%7D%7B%7C%7Cx%7C%7C*%7C%7Cy%7C%7C%7D)

* **Productive** ⭐
  * well documented
  * elegant and simple API, which is extremely easy to learn

# Installation

### rust

add this into `Cargo.toml`

```toml
[dependencies]
hora = "0.1.0"
```

### Python

```Bash
$ pip install hora
```

### Building from source

```bash
$ git clone https://github.com/hora-search/hora
$ cargo build
```

# Benchmark

<img src="asset/fashion-mnist-784-euclidean_10_euclidean.png"/>

# Example

Rust usage example

```Rust

```

Python usage exmaple

```Python

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

    /// add node
    ///
    /// call `add_node()` internal
    fn add(&mut self, vs: &[E], idx: T) -> Result<(), &'static str>;

    /// add multiple node one time
    ///
    /// return `Err(&'static str)` if there is something wrong with the adding process, and the `static str` is the debug reason
    fn madd(&mut self, vss: &[&[E]], indices: &[T]) -> Result<(), &'static str>;

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

    /// search for k nearest neighbors
    ///
    /// it only return the idx of the nearest node
    ///
    /// it require the item is the slice with the same dimension with index dimension, otherwise it will panic
    fn search(&self, item: &[E], k: usize) -> Vec<T>;

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
    fn idx_info(&self) -> String;

    /// return the dimension it require
    fn dimension(&self) -> usize;
}

pub trait SerializableIndex<
    E: node::FloatElement + DeserializeOwned,
    T: node::IdxType + DeserializeOwned,
>: Send + Sync + ANNIndex<E, T>
{
    /// load file with path
    fn load(_path: &str, _args: &arguments::Args) -> Result<Self, &'static str>
    where
        Self: Sized;

    /// dump the file into the path
    fn dump(&mut self, _path: &str, _args: &arguments::Args) -> Result<(), &'static str>;
}
```

# Roadmap

- [ ] Full Coverage Test
- [ ] implement a [EFANNA](http://arxiv.org/abs/1609.07228) to achieve faster KNN buiding
- [ ] Swift Support and also IOS/Mac OS deployment example
- [ ] R Support
- [ ] mmap file support

# Related Project and Comparison

* [Faiss](https://github.com/facebookresearch/faiss), [Annoy](https://github.com/spotify/annoy), [ScaNN](https://github.com/google-research/google-research/tree/master/scann): 
  * **In fact `Hora`'s implementation is strongly inspired by these lib.**
  * `Faiss` more focus on the GPU scene, and `Hora` is more light than Faiss
  * `Hora` wish to support more language, and all the thing related to speed should be implemented by Rust🦀
  * `Annoy` only implement `LSH(Random Projection)` algorithm
  * `ScaNN` and `Faiss` is not easy to use, it's lack of document.
  * **ALL IN RUST** 🦀

* [Milvus](https://github.com/milvus-io/milvus), [Vald](https://github.com/vdaas/vald), [Jina AI](https://github.com/jina-ai/jina)
  * `Milvus` and `Vald` also support multiple languages, but it serve as a service, not a lib
  * `Milvus` is built upon some libs like `Faiss`, but `Hora` is a algorithm lib, all the algo is implemented by itself

# Contribute

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

#### test

```bash
cargo test --lib
```

#### try the changes

```bash
cd exmaples
cargo run
```

# License

The entire repo is under Apache License.
