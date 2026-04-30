<!--@nrg.languages=en,cn,fr,jp,kr,ru-->
<!--@nrg.defaultLanguage=en-->
<!--@nrg.fileNamePattern=README_<LANG>.md-->
<!--@nrg.defaultLanguageFileNamePattern=README.md-->
<div align="center"><!--en-->
  <img src="asset/logo.svg" width="70%"/><!--en-->
</div><!--en-->
<!--en-->
<div align="center"><!--en-->
  <h3>  English | <a href="https://github.com/hora-search/hora/blob/main/README_FR.md"> Français </a> | <a href="https://github.com/hora-search/hora/blob/main/README_JP.md"> 日本語 </a> | <a href="https://github.com/hora-search/hora/blob/main/README_KR.md">한국어</a> | <a href="https://github.com/hora-search/hora/blob/main/README_RU.md">Русский</a> | <a href="https://github.com/hora-search/hora/blob/main/README_CN.md">中文</a> </h3><!--en-->
</div><!--en-->
<!--en-->
# Hora<!--en-->
<!--en-->
**[[Homepage](http://horasearch.com/)]** **[[Document](https://horasearch.com/doc)]** **[[Examples](https://horasearch.com/doc/example.html)]**<!--en-->
<!--en-->
**_Hora Search Everywhere!_**<!--en-->
<!--en-->
Hora is an **approximate nearest neighbor search algorithm** ([wiki](https://en.wikipedia.org/wiki/Nearest_neighbor_search)) library. We implement all code in `Rust🦀` for reliability, high level abstraction and high speeds comparable to `C++`.<!--en-->
<!--en-->
Hora, **`「ほら」`** in Japanese, sounds like `[hōlə]`, and means `Wow`, `You see!` or `Look at that!`. The name is inspired by a famous Japanese song **`「小さな恋のうた」`**.<!--en-->
<!--en-->
# Demos<!--en-->
<!--en-->
**👩 Face-Match [[online demo](https://horasearch.com/#Demos)], have a try!**<!--en-->
<!--en-->
<div align="center"><!--en-->
  <img src="asset/demo3.gif" width="100%"/><!--en-->
</div><!--en-->
<!--en-->
**🍷 Dream wine comments search [[online demo](https://horasearch.com/#Demos)], have a try!**<!--en-->
<!--en-->
<div align="center"><!--en-->
  <img src="asset/demo2.gif" width="100%"/><!--en-->
</div><!--en-->
<!--en-->
# Features<!--en-->
<!--en-->
- **Performant** ⚡️<!--en-->
<!--en-->
  - **SIMD-Accelerated ([packed_simd](https://github.com/rust-lang/packed_simd))**<!--en-->
  - **Stable algorithm implementation**<!--en-->
  - **Multiple threads design**<!--en-->
<!--en-->
- **Supports Multiple Languages** ☄️<!--en-->
<!--en-->
  - `Python`<!--en-->
  - `Javascript`<!--en-->
  - `Java`<!--en-->
  - `Go` (WIP)<!--en-->
  - `Ruby` (WIP)<!--en-->
  - `Swift` (WIP)<!--en-->
  - `R` (WIP)<!--en-->
  - `Julia` (WIP)<!--en-->
  - **Can also be used as a service**<!--en-->
<!--en-->
- **Supports Multiple Indexes** 🚀<!--en-->
<!--en-->
  - `Hierarchical Navigable Small World Graph Index (HNSWIndex)` ([details](https://arxiv.org/abs/1603.09320))<!--en-->
  - `Satellite System Graph (SSGIndex)` ([details](https://arxiv.org/abs/1907.06146))<!--en-->
  - `Product Quantization Inverted File(PQIVFIndex)` ([details](https://lear.inrialpes.fr/pubs/2011/JDS11/jegou_searching_with_quantization.pdf))<!--en-->
  - `Random Projection Tree(RPTIndex)` (LSH, WIP)<!--en-->
  - `BruteForce (BruteForceIndex)` (naive implementation with SIMD)<!--en-->
<!--en-->
- **Portable** 💼<!--en-->
<!--en-->
  - Supports `WebAssembly`<!--en-->
  - Supports `Windows`, `Linux` and `OS X`<!--en-->
  - Supports `IOS` and `Android` (WIP)<!--en-->
  - Supports `no_std` (WIP, partial)<!--en-->
  - **No** heavy dependencies, such as `BLAS`<!--en-->
<!--en-->
- **Reliability** 🔒<!--en-->
<!--en-->
  - `Rust` compiler secures all code<!--en-->
  - Memory managed by `Rust` for all language libraries such as `Python's`<!--en-->
  - Broad testing coverage<!--en-->
<!--en-->
- **Supports Multiple Distances** 🧮<!--en-->
<!--en-->
  - `Dot Product Distance`<!--en-->
    - ![equation](https://latex.codecogs.com/gif.latex?D%28x%2Cy%29%20%3D%20%5Csum%7B%28x*y%29%7D)<!--en-->
  - `Euclidean Distance`<!--en-->
    - ![equation](https://latex.codecogs.com/gif.latex?D%28x%2Cy%29%20%3D%20%5Csqrt%7B%5Csum%7B%28x-y%29%5E2%7D%7D)<!--en-->
  - `Manhattan Distance`<!--en-->
    - ![equation](https://latex.codecogs.com/gif.latex?D%28x%2Cy%29%20%3D%20%5Csum%7B%7C%28x-y%29%7C%7D)<!--en-->
  - `Cosine Similarity`<!--en-->
    - ![equation](https://latex.codecogs.com/gif.latex?D%28x%2Cy%29%20%3D%20%5Cfrac%7Bx%20*y%7D%7B%7C%7Cx%7C%7C*%7C%7Cy%7C%7C%7D)<!--en-->
<!--en-->
- **Productive** ⭐<!--en-->
  - Well documented<!--en-->
  - Elegant, simple and easy to learn API<!--en-->
<!--en-->
# Installation<!--en-->
<!--en-->
**`Rust`**<!--en-->
<!--en-->
in `Cargo.toml`<!--en-->
<!--en-->
```toml<!--en-->
[dependencies]<!--en-->
hora = "0.1.1"<!--en-->
```<!--en-->
<!--en-->
**`Python`**<!--en-->
<!--en-->
```Bash<!--en-->
$ pip install horapy<!--en-->
```<!--en-->
<!--en-->
**`Javascript (WebAssembly)`**<!--en-->
<!--en-->
```Bash<!--en-->
$ npm i horajs<!--en-->
```<!--en-->
<!--en-->
**`Building from source`**<!--en-->
<!--en-->
```bash<!--en-->
$ git clone https://github.com/hora-search/hora<!--en-->
$ cargo build<!--en-->
```<!--en-->
<!--en-->
# Benchmarks<!--en-->
<!--en-->
<img src="asset/fashion-mnist-784-euclidean_10_euclidean.png"/><!--en-->
<!--en-->
by `aws t2.medium (CPU: Intel(R) Xeon(R) CPU E5-2686 v4 @ 2.30GHz)` [more information](https://github.com/hora-search/ann-benchmarks)<!--en-->
<!--en-->
# Examples<!--en-->
<!--en-->
**`Rust` example** [[more info](https://github.com/hora-search/hora/tree/main/examples)]<!--en-->
<!--en-->
```Rust<!--en-->
use hora::core::ann_index::ANNIndex;<!--en-->
use rand::{thread_rng, Rng};<!--en-->
use rand_distr::{Distribution, Normal};<!--en-->
<!--en-->
pub fn demo() {<!--en-->
    let n = 1000;<!--en-->
    let dimension = 64;<!--en-->
<!--en-->
    // make sample points<!--en-->
    let mut samples = Vec::with_capacity(n);<!--en-->
    let normal = Normal::new(0.0, 10.0).unwrap();<!--en-->
    for _i in 0..n {<!--en-->
        let mut sample = Vec::with_capacity(dimension);<!--en-->
        for _j in 0..dimension {<!--en-->
            sample.push(normal.sample(&mut rand::thread_rng()));<!--en-->
        }<!--en-->
        samples.push(sample);<!--en-->
    }<!--en-->
<!--en-->
    // init index<!--en-->
    let mut index = hora::index::hnsw_idx::HNSWIndex::<f32, usize>::new(<!--en-->
        dimension,<!--en-->
        &hora::index::hnsw_params::HNSWParams::<f32>::default(),<!--en-->
    );<!--en-->
    for (i, sample) in samples.iter().enumerate().take(n) {<!--en-->
        // add point<!--en-->
        index.add(sample, i).unwrap();<!--en-->
    }<!--en-->
    index.build(hora::core::metrics::Metric::Euclidean).unwrap();<!--en-->
<!--en-->
    let mut rng = thread_rng();<!--en-->
    let target: usize = rng.gen_range(0..n);<!--en-->
    // 523 has neighbors: [523, 762, 364, 268, 561, 231, 380, 817, 331, 246]<!--en-->
    println!(<!--en-->
        "{:?} has neighbors: {:?}",<!--en-->
        target,<!--en-->
        index.search(&samples[target], 10) // search for k nearest neighbors<!--en-->
    );<!--en-->
}<!--en-->
```<!--en-->
<!--en-->
thank @vaaaaanquish for this complete pure `Rust 🦀` image search [example](https://github.com/vaaaaanquish/rust-ann-search-example), For more information about this example, you can click [Pure Rust な近似最近傍探索ライブラリ hora を用いた画像検索を実装する](https://vaaaaaanquish.hatenablog.com/entry/2021/08/10/065117)<!--en-->
<!--en-->
**`Python` example** [[more info](https://github.com/hora-search/horapy)]<!--en-->
<!--en-->
```Python<!--en-->
import numpy as np<!--en-->
from horapy import HNSWIndex<!--en-->
<!--en-->
dimension = 50<!--en-->
n = 1000<!--en-->
<!--en-->
# init index instance<!--en-->
index = HNSWIndex(dimension, "usize")<!--en-->
<!--en-->
samples = np.float32(np.random.rand(n, dimension))<!--en-->
for i in range(0, len(samples)):<!--en-->
    # add node<!--en-->
    index.add(np.float32(samples[i]), i)<!--en-->
<!--en-->
index.build("euclidean")  # build index<!--en-->
<!--en-->
target = np.random.randint(0, n)<!--en-->
# 410 in Hora ANNIndex <HNSWIndexUsize> (dimension: 50, dtype: usize, max_item: 1000000, n_neigh: 32, n_neigh0: 64, ef_build: 20, ef_search: 500, has_deletion: False)<!--en-->
# has neighbors: [410, 736, 65, 36, 631, 83, 111, 254, 990, 161]<!--en-->
print("{} in {} \nhas neighbors: {}".format(<!--en-->
    target, index, index.search(samples[target], 10)))  # search<!--en-->
<!--en-->
```<!--en-->
<!--en-->
**`JavaScript` example** [[more info](https://github.com/hora-search/hora-wasm)]<!--en-->
<!--en-->
```JavaScript<!--en-->
import * as horajs from "horajs";<!--en-->
<!--en-->
const demo = () => {<!--en-->
    const dimension = 50;<!--en-->
    var bf_idx = horajs.BruteForceIndexUsize.new(dimension);<!--en-->
    // var hnsw_idx = horajs.HNSWIndexUsize.new(dimension, 1000000, 32, 64, 20, 500, 16, false);<!--en-->
    for (var i = 0; i < 1000; i++) {<!--en-->
        var feature = [];<!--en-->
        for (var j = 0; j < dimension; j++) {<!--en-->
            feature.push(Math.random());<!--en-->
        }<!--en-->
        bf_idx.add(feature, i); // add point<!--en-->
    }<!--en-->
    bf_idx.build("euclidean"); // build index<!--en-->
    var feature = [];<!--en-->
    for (var j = 0; j < dimension; j++) {<!--en-->
        feature.push(Math.random());<!--en-->
    }<!--en-->
    console.log("bf result", bf_idx.search(feature, 10)); //bf result Uint32Array(10) [704, 113, 358, 835, 408, 379, 117, 414, 808, 826]<!--en-->
}<!--en-->
<!--en-->
(async () => {<!--en-->
    await horajs.default();<!--en-->
    await horajs.init_env();<!--en-->
    demo();<!--en-->
})();<!--en-->
```<!--en-->
<!--en-->
**`Java` example** [[more info](https://github.com/hora-search/hora-java)]<!--en-->
<!--en-->
```Java<!--en-->
public void demo() {<!--en-->
    final int dimension = 2;<!--en-->
    final float variance = 2.0f;<!--en-->
    Random fRandom = new Random();<!--en-->
<!--en-->
    BruteForceIndex bruteforce_idx = new BruteForceIndex(dimension); // init index instance<!--en-->
<!--en-->
    List<float[]> tmp = new ArrayList<>();<!--en-->
    for (int i = 0; i < 5; i++) {<!--en-->
        for (int p = 0; p < 10; p++) {<!--en-->
            float[] features = new float[dimension];<!--en-->
            for (int j = 0; j < dimension; j++) {<!--en-->
                features[j] = getGaussian(fRandom, (float) (i * 10), variance);<!--en-->
            }<!--en-->
            bruteforce_idx.add("bf", features, i * 10 + p); // add point<!--en-->
            tmp.add(features);<!--en-->
          }<!--en-->
    }<!--en-->
    bruteforce_idx.build("bf", "euclidean"); // build index<!--en-->
<!--en-->
    int search_index = fRandom.nextInt(tmp.size());<!--en-->
    // nearest neighbor search<!--en-->
    int[] result = bruteforce_idx.search("bf", 10, tmp.get(search_index));<!--en-->
    // [main] INFO com.hora.app.ANNIndexTest  - demo bruteforce_idx[7, 8, 0, 5, 3, 9, 1, 6, 4, 2]<!--en-->
    log.info("demo bruteforce_idx" + Arrays.toString(result));<!--en-->
}<!--en-->
<!--en-->
private static float getGaussian(Random fRandom, float aMean, float variance) {<!--en-->
    float r = (float) fRandom.nextGaussian();<!--en-->
    return aMean + r * variance;<!--en-->
}<!--en-->
```<!--en-->
<!--en-->
# Roadmap<!--en-->
<!--en-->
- [ ] Full test coverage<!--en-->
- [ ] Implement [EFANNA](http://arxiv.org/abs/1609.07228) algorithm to achieve faster KNN graph building<!--en-->
- [ ] Swift support and iOS/macOS deployment example<!--en-->
- [ ] Support `R`<!--en-->
- [ ] support `mmap`<!--en-->
<!--en-->
# Related Projects and Comparison<!--en-->
<!--en-->
- [Faiss](https://github.com/facebookresearch/faiss), [Annoy](https://github.com/spotify/annoy), [ScaNN](https://github.com/google-research/google-research/tree/master/scann):<!--en-->
<!--en-->
  - **`Hora`'s implementation is strongly inspired by these libraries.**<!--en-->
  - `Faiss` focuses more on the GPU scenerio, and `Hora` is lighter than Faiss (**no heavy dependencies)**.<!--en-->
  - `Hora` expects to support more languages, and everything related to performance will be implemented by Rust🦀.<!--en-->
  - `Annoy` only supports the `LSH (Random Projection)` algorithm.<!--en-->
  - `ScaNN` and `Faiss` are less user-friendly, (e.g. lack of documentation).<!--en-->
  - Hora is **ALL IN RUST** 🦀.<!--en-->
<!--en-->
- [Milvus](https://github.com/milvus-io/milvus), [Vald](https://github.com/vdaas/vald), [Jina AI](https://github.com/jina-ai/jina)<!--en-->
  - `Milvus` and `Vald` also support multiple languages, but serve as a service instead of a library<!--en-->
  - `Milvus` is built upon some libraries such as `Faiss`, while `Hora` is a library with all the algorithms implemented itself<!--en-->
<!--en-->
# Contribute<!--en-->
<!--en-->
**We appreciate your participation!**<!--en-->
<!--en-->
We are glad to have you participate, any contributions are welcome, including documentations and tests.<!--en-->
You can create a `Pull Request` or `Issue` on GitHub, and we will review it as soon as possible.<!--en-->
<!--en-->
We use GitHub issues for tracking suggestions and bugs.<!--en-->
<!--en-->
#### Clone the repo<!--en-->
<!--en-->
```bash<!--en-->
git clone https://github.com/hora-search/hora<!--en-->
```<!--en-->
<!--en-->
#### Build<!--en-->
<!--en-->
```bash<!--en-->
cargo build<!--en-->
```<!--en-->
<!--en-->
#### Test<!--en-->
<!--en-->
```bash<!--en-->
cargo test --lib<!--en-->
```<!--en-->
<!--en-->
#### Try the changes<!--en-->
<!--en-->
```bash<!--en-->
cd examples<!--en-->
cargo run<!--en-->
```<!--en-->
<!--en-->
# License<!--en-->
<!--en-->
The entire repository is licensed under the [Apache License](https://github.com/hora-search/hora/blob/main/LICENSE).<!--en-->
<div align="center"><!--cn-->
  <img src="asset/logo.svg" width="70%"/><!--cn-->
</div><!--cn-->
<!--cn-->
# Hora<!--cn-->
<!--cn-->
**[[Homepage](http://horasearch.com/)]** **[[Document](https://horasearch.com/doc)]** **[[Examples](https://horasearch.com/doc/example.html)]**<!--cn-->
<!--cn-->
**_Hora Search Everywhere!_**<!--cn-->
<!--cn-->
**Hora** 完全基于 **Rust🦀** 实现，事实证明，**Rust** 确实非常非常快，完全可以媲美 **C++** ，且`Hora`使用 **SIMD**进行了加速，速度非常快⚡️⚡️⚡️，具体速度可以参考下面的 benchmark.<!--cn-->
<!--cn-->
**Hora**, 日语为 **「ほら」**，读法像 **[hōlə]** ，意思是 **Wow**, **You see!** , **Look at that!** 。 这个名字的灵感来自日本著名歌曲 **[「小さな恋のうた」]( https://www.youtube.com/watch?v=u8EkSB9zSpE)** 。<!--cn-->
<!--cn-->
# Demos<!--cn-->
<!--cn-->
**👩 Face-Match [[online demo](https://horasearch.com/#Demos)], have a try!**<!--cn-->
<!--cn-->
<div align="center"><!--cn-->
  <img src="asset/demo3.gif" width="100%"/><!--cn-->
</div><!--cn-->
<!--cn-->
**🍷 Dream wine comments search [[online demo](https://horasearch.com/#Demos)], have a try!**<!--cn-->
<!--cn-->
<div align="center"><!--cn-->
  <img src="asset/demo2.gif" width="100%"/><!--cn-->
</div><!--cn-->
<!--cn-->
# Features<!--cn-->
<!--cn-->
- **Performant** ⚡️<!--cn-->
<!--cn-->
  - **SIMD-Accelerated ([packed_simd](https://github.com/rust-lang/packed_simd))**<!--cn-->
  - **Stable algorithm implementation**<!--cn-->
  - **Multiple threads design**<!--cn-->
<!--cn-->
- **Supports Multiple Languages** ☄️<!--cn-->
<!--cn-->
  - `Python`<!--cn-->
  - `Javascript`<!--cn-->
  - `Java`<!--cn-->
  - `Go` (WIP)<!--cn-->
  - `Ruby` (WIP)<!--cn-->
  - `Swift` (WIP)<!--cn-->
  - `R` (WIP)<!--cn-->
  - `Julia` (WIP)<!--cn-->
  - **Can also be used as a service**<!--cn-->
<!--cn-->
- **Supports Multiple Indexes** 🚀<!--cn-->
<!--cn-->
  - `Hierarchical Navigable Small World Graph Index (HNSWIndex)` ([details](https://arxiv.org/abs/1603.09320))<!--cn-->
  - `Satellite System Graph (SSGIndex)` ([details](https://arxiv.org/abs/1907.06146))<!--cn-->
  - `Product Quantization Inverted File(PQIVFIndex)` ([details](https://lear.inrialpes.fr/pubs/2011/JDS11/jegou_searching_with_quantization.pdf))<!--cn-->
  - `Random Projection Tree(RPTIndex)` (LSH, WIP)<!--cn-->
  - `BruteForce (BruteForceIndex)` (naive implementation with SIMD)<!--cn-->
<!--cn-->
- **Portable** 💼<!--cn-->
  - Supports `WebAssembly`<!--cn-->
  - Supports `Windows`, `Linux` and `OS X`<!--cn-->
  - Supports `IOS` and `Android` (WIP)<!--cn-->
  - Supports `no_std` (WIP, partial)<!--cn-->
  - **No** heavy dependencies, such as `BLAS`<!--cn-->
<!--cn-->
- **Reliability** 🔒<!--cn-->
<!--cn-->
  - `Rust` compiler secures all code<!--cn-->
  - Memory managed by `Rust` for all language libraries such as `Python's`<!--cn-->
  - Broad testing coverage<!--cn-->
<!--cn-->
- **Supports Multiple Distances** 🧮<!--cn-->
<!--cn-->
  - `Dot Product Distance`<!--cn-->
    - ![equation](https://latex.codecogs.com/gif.latex?D%28x%2Cy%29%20%3D%20%5Csum%7B%28x*y%29%7D)<!--cn-->
  - `Euclidean Distance`<!--cn-->
    - ![equation](https://latex.codecogs.com/gif.latex?D%28x%2Cy%29%20%3D%20%5Csqrt%7B%5Csum%7B%28x-y%29%5E2%7D%7D)<!--cn-->
  - `Manhattan Distance`<!--cn-->
    - ![equation](https://latex.codecogs.com/gif.latex?D%28x%2Cy%29%20%3D%20%5Csum%7B%7C%28x-y%29%7C%7D)<!--cn-->
  - `Cosine Similarity`<!--cn-->
    - ![equation](https://latex.codecogs.com/gif.latex?D%28x%2Cy%29%20%3D%20%5Cfrac%7Bx%20*y%7D%7B%7C%7Cx%7C%7C*%7C%7Cy%7C%7C%7D)<!--cn-->
<!--cn-->
- **Productive** ⭐<!--cn-->
  - Well documented<!--cn-->
  - Elegant, simple and easy to learn API<!--cn-->
<!--cn-->
# Installation<!--cn-->
<!--cn-->
**`Rust`**<!--cn-->
<!--cn-->
in `Cargo.toml`<!--cn-->
<!--cn-->
```toml<!--cn-->
[dependencies]<!--cn-->
hora = "0.1.1"<!--cn-->
```<!--cn-->
<!--cn-->
**`Python`**<!--cn-->
<!--cn-->
```Bash<!--cn-->
$ pip install horapy<!--cn-->
```<!--cn-->
<!--cn-->
**`Javascript (WebAssembly)`**<!--cn-->
<!--cn-->
```Bash<!--cn-->
$ npm i horajs<!--cn-->
```<!--cn-->
<!--cn-->
**`Building from source`**<!--cn-->
<!--cn-->
```bash<!--cn-->
$ git clone https://github.com/hora-search/hora<!--cn-->
$ cargo build<!--cn-->
```<!--cn-->
<!--cn-->
# Benchmarks<!--cn-->
<!--cn-->
<img src="asset/fashion-mnist-784-euclidean_10_euclidean.png"/><!--cn-->
<!--cn-->
by `aws t2.medium (CPU: Intel(R) Xeon(R) CPU E5-2686 v4 @ 2.30GHz)` [more information](https://github.com/hora-search/ann-benchmarks)<!--cn-->
<!--cn-->
# Examples<!--cn-->
<!--cn-->
**`Rust` example** [[more info](https://github.com/hora-search/hora/tree/main/examples)]<!--cn-->
<!--cn-->
```Rust<!--cn-->
use hora::core::ann_index::ANNIndex;<!--cn-->
use rand::{thread_rng, Rng};<!--cn-->
use rand_distr::{Distribution, Normal};<!--cn-->
<!--cn-->
pub fn demo() {<!--cn-->
    let n = 1000;<!--cn-->
    let dimension = 64;<!--cn-->
<!--cn-->
    // make sample points<!--cn-->
    let mut samples = Vec::with_capacity(n);<!--cn-->
    let normal = Normal::new(0.0, 10.0).unwrap();<!--cn-->
    for _i in 0..n {<!--cn-->
        let mut sample = Vec::with_capacity(dimension);<!--cn-->
        for _j in 0..dimension {<!--cn-->
            sample.push(normal.sample(&mut rand::thread_rng()));<!--cn-->
        }<!--cn-->
        samples.push(sample);<!--cn-->
    }<!--cn-->
<!--cn-->
    // init index<!--cn-->
    let mut index = hora::index::hnsw_idx::HNSWIndex::<f32, usize>::new(<!--cn-->
        dimension,<!--cn-->
        &hora::index::hnsw_params::HNSWParams::<f32>::default(),<!--cn-->
    );<!--cn-->
    for (i, sample) in samples.iter().enumerate().take(n) {<!--cn-->
        // add point<!--cn-->
        index.add(sample, i).unwrap();<!--cn-->
    }<!--cn-->
    index.build(hora::core::metrics::Metric::Euclidean).unwrap();<!--cn-->
<!--cn-->
    let mut rng = thread_rng();<!--cn-->
    let target: usize = rng.gen_range(0..n);<!--cn-->
    // 523 has neighbors: [523, 762, 364, 268, 561, 231, 380, 817, 331, 246]<!--cn-->
    println!(<!--cn-->
        "{:?} has neighbors: {:?}",<!--cn-->
        target,<!--cn-->
        index.search(&samples[target], 10) // search for k nearest neighbors<!--cn-->
    );<!--cn-->
}<!--cn-->
```<!--cn-->
<!--cn-->
感谢 @vaaaaanquish 这个完整的纯 `Rust 🦀` 图片检索 [example](https://github.com/vaaaaanquish/rust-ann-search-example), 想了解更多可以点击 [Pure Rustな近似最近傍探索ライブラリhoraを用いた画像検索を実装する](https://vaaaaaanquish.hatenablog.com/entry/2021/08/10/065117)<!--cn-->
<!--cn-->
**`Python` example** [[more info](https://github.com/hora-search/horapy)]<!--cn-->
<!--cn-->
```Python<!--cn-->
import numpy as np<!--cn-->
from horapy import HNSWIndex<!--cn-->
<!--cn-->
dimension = 50<!--cn-->
n = 1000<!--cn-->
<!--cn-->
# init index instance<!--cn-->
index = HNSWIndex(dimension, "usize")<!--cn-->
<!--cn-->
samples = np.float32(np.random.rand(n, dimension))<!--cn-->
for i in range(0, len(samples)):<!--cn-->
    # add node<!--cn-->
    index.add(np.float32(samples[i]), i)<!--cn-->
<!--cn-->
index.build("euclidean")  # build index<!--cn-->
<!--cn-->
target = np.random.randint(0, n)<!--cn-->
# 410 in Hora ANNIndex <HNSWIndexUsize> (dimension: 50, dtype: usize, max_item: 1000000, n_neigh: 32, n_neigh0: 64, ef_build: 20, ef_search: 500, has_deletion: False)<!--cn-->
# has neighbors: [410, 736, 65, 36, 631, 83, 111, 254, 990, 161]<!--cn-->
print("{} in {} \nhas neighbors: {}".format(<!--cn-->
    target, index, index.search(samples[target], 10)))  # search<!--cn-->
<!--cn-->
```<!--cn-->
<!--cn-->
**`JavaScript` example** [[more info](https://github.com/hora-search/hora-wasm)]<!--cn-->
<!--cn-->
```JavaScript<!--cn-->
import * as horajs from "horajs";<!--cn-->
<!--cn-->
const demo = () => {<!--cn-->
    const dimension = 50;<!--cn-->
    var bf_idx = horajs.BruteForceIndexUsize.new(dimension);<!--cn-->
    // var hnsw_idx = horajs.HNSWIndexUsize.new(dimension, 1000000, 32, 64, 20, 500, 16, false);<!--cn-->
    for (var i = 0; i < 1000; i++) {<!--cn-->
        var feature = [];<!--cn-->
        for (var j = 0; j < dimension; j++) {<!--cn-->
            feature.push(Math.random());<!--cn-->
        }<!--cn-->
        bf_idx.add(feature, i); // add point <!--cn-->
    }<!--cn-->
    bf_idx.build("euclidean"); // build index<!--cn-->
    var feature = [];<!--cn-->
    for (var j = 0; j < dimension; j++) {<!--cn-->
        feature.push(Math.random());<!--cn-->
    }<!--cn-->
    console.log("bf result", bf_idx.search(feature, 10)); //bf result Uint32Array(10) [704, 113, 358, 835, 408, 379, 117, 414, 808, 826]<!--cn-->
}<!--cn-->
<!--cn-->
(async () => {<!--cn-->
    await horajs.default();<!--cn-->
    await horajs.init_env();<!--cn-->
    demo();<!--cn-->
})();<!--cn-->
```<!--cn-->
<!--cn-->
**`Java` example** [[more info](https://github.com/hora-search/hora-java)]<!--cn-->
<!--cn-->
```Java<!--cn-->
public void demo() {<!--cn-->
    final int dimension = 2;<!--cn-->
    final float variance = 2.0f;<!--cn-->
    Random fRandom = new Random();<!--cn-->
<!--cn-->
    BruteForceIndex bruteforce_idx = new BruteForceIndex(dimension); // init index instance<!--cn-->
<!--cn-->
    List<float[]> tmp = new ArrayList<>();<!--cn-->
    for (int i = 0; i < 5; i++) {<!--cn-->
        for (int p = 0; p < 10; p++) {<!--cn-->
            float[] features = new float[dimension];<!--cn-->
            for (int j = 0; j < dimension; j++) {<!--cn-->
                features[j] = getGaussian(fRandom, (float) (i * 10), variance);<!--cn-->
            }<!--cn-->
            bruteforce_idx.add("bf", features, i * 10 + p); // add point<!--cn-->
            tmp.add(features);<!--cn-->
          }<!--cn-->
    }<!--cn-->
    bruteforce_idx.build("bf", "euclidean"); // build index<!--cn-->
<!--cn-->
    int search_index = fRandom.nextInt(tmp.size());<!--cn-->
    // nearest neighbor search<!--cn-->
    int[] result = bruteforce_idx.search("bf", 10, tmp.get(search_index));<!--cn-->
    // [main] INFO com.hora.app.ANNIndexTest  - demo bruteforce_idx[7, 8, 0, 5, 3, 9, 1, 6, 4, 2]<!--cn-->
    log.info("demo bruteforce_idx" + Arrays.toString(result));<!--cn-->
}<!--cn-->
<!--cn-->
private static float getGaussian(Random fRandom, float aMean, float variance) {<!--cn-->
    float r = (float) fRandom.nextGaussian();<!--cn-->
    return aMean + r * variance;<!--cn-->
}<!--cn-->
```<!--cn-->
<!--cn-->
# Roadmap<!--cn-->
<!--cn-->
- [ ] Full test coverage<!--cn-->
- [ ] Implement [EFANNA](http://arxiv.org/abs/1609.07228) algorithm to achieve faster KNN graph building<!--cn-->
- [ ] Swift support and iOS/macOS deployment example<!--cn-->
- [ ] Support `R`<!--cn-->
- [ ] support `mmap`<!--cn-->
<!--cn-->
# Related Projects and Comparison<!--cn-->
<!--cn-->
- [Faiss](https://github.com/facebookresearch/faiss), [Annoy](https://github.com/spotify/annoy), [ScaNN](https://github.com/google-research/google-research/tree/master/scann):<!--cn-->
  - **`Hora` 的实现受到这些库的强烈启发。**<!--cn-->
  - `Faiss` 更侧重于 GPU 场景，`Hora` 比 Faiss 更轻（**无重度依赖）**。<!--cn-->
  - `Hora` 期待支持更多的语言，与性能相关的一切都会由 Rust🦀 实现。<!--cn-->
  - `Annoy` 只支持 `LSH (Random Projection)` 算法。<!--cn-->
  - `ScaNN` 和 `Faiss` 不太用户友好，（例如缺乏文档）。<!--cn-->
  - Hora is **ALL IN RUST** 🦀.<!--cn-->
<!--cn-->
- [Milvus](https://github.com/milvus-io/milvus), [Vald](https://github.com/vdaas/vald), [Jina AI](https://github.com/jina-ai/jina)<!--cn-->
  - `Milvus` 和 `Vald` 也支持多种语言，但作为服务而不是库<!--cn-->
  - `Milvus` 是建立在一些库上的，比如 `Faiss`，而 `Hora` 是一个库，所有算法都是自己实现的<!--cn-->
<!--cn-->
# Contribute<!--cn-->
<!--cn-->
**We appreciate your help!**<!--cn-->
<!--cn-->
我们很高兴您的参与，欢迎任何贡献，包括文档和测试。<!--cn-->
您可以在 GitHub 上创建 `Pull Request` 或 `Issue`，我们会尽快审核。<!--cn-->
<!--cn-->
我们使用 GitHub 问题来跟踪建议和错误。<!--cn-->
<!--cn-->
#### Clone the repo<!--cn-->
<!--cn-->
```bash<!--cn-->
git clone https://github.com/hora-search/hora<!--cn-->
```<!--cn-->
<!--cn-->
#### Build<!--cn-->
<!--cn-->
```bash<!--cn-->
cargo build<!--cn-->
```<!--cn-->
<!--cn-->
#### Test<!--cn-->
<!--cn-->
```bash<!--cn-->
cargo test --lib<!--cn-->
```<!--cn-->
<!--cn-->
#### Try the changes<!--cn-->
<!--cn-->
```bash<!--cn-->
cd examples<!--cn-->
cargo run<!--cn-->
```<!--cn-->
<!--cn-->
# License<!--cn-->
<!--cn-->
The entire repository is licensed under the [Apache License](https://github.com/hora-search/hora/blob/main/LICENSE).<!--cn-->
<div align="center"><!--fr-->
  <img src="asset/logo.svg" width="70%"/><!--fr-->
</div><!--fr-->
<!--fr-->
# Hora<!--fr-->
<!--fr-->
**[[Homepage](http://horasearch.com/)]** **[[Document](https://horasearch.com/doc)]** **[[Examples](https://horasearch.com/doc/example.html)]**<!--fr-->
<!--fr-->
**_Hora Search Everywhere!_**<!--fr-->
<!--fr-->
Hora est un **algorithme de recherche du voisin le plus proche approximatif** ([wiki](https://en.wikipedia.org/wiki/Nearest_neighbor_search)). Nous implémentons tout le code dans `Rust🦀` pour une fiabilité, une abstraction de haut niveau et des vitesses élevées comparables à `C++`.<!--fr-->
<!--fr-->
Hora, **`「ほら」`** en japonais, sonne comme `[hōlə]`, et signifie `Wow`, `Vous voyez !` ou ` Regardez ça ! `. Le nom est inspiré d'une célèbre chanson japonaise **`「小さな恋のうた」`**.<!--fr-->
<!--fr-->
# Démos<!--fr-->
<!--fr-->
**👩 Face-Match [[online demo](https://horasearch.com/#Demos)], Essaye!**<!--fr-->
<!--fr-->
<div align="center"><!--fr-->
  <img src="asset/demo3.gif" width="100%"/><!--fr-->
</div><!--fr-->
<!--fr-->
**🍷 Recherche de commentaires sur le vin de rêve [[online demo](https://horasearch.com/#Demos)], Essaye!**<!--fr-->
<!--fr-->
<div align="center"><!--fr-->
  <img src="asset/demo2.gif" width="100%"/><!--fr-->
</div><!--fr-->
<!--fr-->
# Principales caractéristiques<!--fr-->
<!--fr-->
- **Performant** ⚡️<!--fr-->
<!--fr-->
  - **SIMD-Accelerated ([packed_simd](https://github.com/rust-lang/packed_simd))**<!--fr-->
  - **Implémentation d'algorithme stable**<!--fr-->
  - **Multiple threads design**<!--fr-->
<!--fr-->
- **Prend en charge plusieurs langages de programmation Lib** ☄️<!--fr-->
<!--fr-->
  - `Python`<!--fr-->
  - `Javascript`<!--fr-->
  - `Java`<!--fr-->
  - `Go` (WIP)<!--fr-->
  - `Ruby` (WIP)<!--fr-->
  - `Swift` (WIP)<!--fr-->
  - `R` (WIP)<!--fr-->
  - `Julia` (WIP)<!--fr-->
  - **Peut également être utilisé comme un service**<!--fr-->
<!--fr-->
- **Prend en charge plusieurs index** 🚀<!--fr-->
<!--fr-->
  - `Hierarchical Navigable Small World Graph Index (HNSWIndex)` ([details](https://arxiv.org/abs/1603.09320))<!--fr-->
  - `Satellite System Graph (SSGIndex)` ([details](https://arxiv.org/abs/1907.06146))<!--fr-->
  - `Product Quantization Inverted File(PQIVFIndex)` ([details](https://lear.inrialpes.fr/pubs/2011/JDS11/jegou_searching_with_quantization.pdf))<!--fr-->
  - `Random Projection Tree(RPTIndex)` (LSH, WIP)<!--fr-->
  - `BruteForce (BruteForceIndex)` (naive implementation with SIMD)<!--fr-->
<!--fr-->
- **Portable** 💼<!--fr-->
<!--fr-->
  - Supports `WebAssembly`<!--fr-->
  - Supports `Windows`, `Linux` and `OS X`<!--fr-->
  - Supports `IOS` and `Android` (WIP)<!--fr-->
  - Supports `no_std` (WIP, partial)<!--fr-->
  - Pas de dépendances lourdes, telles que `BLAS`<!--fr-->
<!--fr-->
- **Fiabilité** 🔒<!--fr-->
<!--fr-->
  - Le compilateur `Rust` sécurise tout le code<!--fr-->
  - Mémoire gérée par `Rust` pour toutes les bibliothèques de langage telles que `Python's`<!--fr-->
  - Large couverture de test<!--fr-->
<!--fr-->
- **Prend en charge plusieurs distances** 🧮<!--fr-->
<!--fr-->
  - `Distance du produit de point`<!--fr-->
    - ![equation](https://latex.codecogs.com/gif.latex?D%28x%2Cy%29%20%3D%20%5Csum%7B%28x*y%29%7D)<!--fr-->
  - `Distance euclidienne`<!--fr-->
    - ![equation](https://latex.codecogs.com/gif.latex?D%28x%2Cy%29%20%3D%20%5Csqrt%7B%5Csum%7B%28x-y%29%5E2%7D%7D)<!--fr-->
  - `Distance de Manhattan`<!--fr-->
    - ![equation](https://latex.codecogs.com/gif.latex?D%28x%2Cy%29%20%3D%20%5Csum%7B%7C%28x-y%29%7C%7D)<!--fr-->
  - `Similitude de cosinus`<!--fr-->
    - ![equation](https://latex.codecogs.com/gif.latex?D%28x%2Cy%29%20%3D%20%5Cfrac%7Bx%20*y%7D%7B%7C%7Cx%7C%7C*%7C%7Cy%7C%7C%7D)<!--fr-->
<!--fr-->
- **Productive** ⭐<!--fr-->
  - Bien documenté<!--fr-->
  - API élégante, simple et facile à apprendre<!--fr-->
<!--fr-->
# Installation<!--fr-->
<!--fr-->
**`Rust`**<!--fr-->
<!--fr-->
in `Cargo.toml`<!--fr-->
<!--fr-->
```toml<!--fr-->
[dependencies]<!--fr-->
hora = "0.1.1"<!--fr-->
```<!--fr-->
<!--fr-->
**`Python`**<!--fr-->
<!--fr-->
```Bash<!--fr-->
$ pip install horapy<!--fr-->
```<!--fr-->
<!--fr-->
**`Javascript (WebAssembly)`**<!--fr-->
<!--fr-->
```Bash<!--fr-->
$ npm i horajs<!--fr-->
```<!--fr-->
<!--fr-->
**`Building from source`**<!--fr-->
<!--fr-->
```bash<!--fr-->
$ git clone https://github.com/hora-search/hora<!--fr-->
$ cargo build<!--fr-->
```<!--fr-->
<!--fr-->
# Repères<!--fr-->
<!--fr-->
<img src="asset/fashion-mnist-784-euclidean_10_euclidean.png"/><!--fr-->
<!--fr-->
by `aws t2.medium (CPU: Intel(R) Xeon(R) CPU E5-2686 v4 @ 2.30GHz)` [more information](https://github.com/hora-search/ann-benchmarks)<!--fr-->
<!--fr-->
# Exemples<!--fr-->
<!--fr-->
**`Rust` exemple** [[more info](https://github.com/hora-search/hora/tree/main/examples)]<!--fr-->
<!--fr-->
```Rust<!--fr-->
use hora::core::ann_index::ANNIndex;<!--fr-->
use rand::{thread_rng, Rng};<!--fr-->
use rand_distr::{Distribution, Normal};<!--fr-->
<!--fr-->
pub fn demo() {<!--fr-->
    let n = 1000;<!--fr-->
    let dimension = 64;<!--fr-->
<!--fr-->
    // make sample points<!--fr-->
    let mut samples = Vec::with_capacity(n);<!--fr-->
    let normal = Normal::new(0.0, 10.0).unwrap();<!--fr-->
    for _i in 0..n {<!--fr-->
        let mut sample = Vec::with_capacity(dimension);<!--fr-->
        for _j in 0..dimension {<!--fr-->
            sample.push(normal.sample(&mut rand::thread_rng()));<!--fr-->
        }<!--fr-->
        samples.push(sample);<!--fr-->
    }<!--fr-->
<!--fr-->
    // init index<!--fr-->
    let mut index = hora::index::hnsw_idx::HNSWIndex::<f32, usize>::new(<!--fr-->
        dimension,<!--fr-->
        &hora::index::hnsw_params::HNSWParams::<f32>::default(),<!--fr-->
    );<!--fr-->
    for (i, sample) in samples.iter().enumerate().take(n) {<!--fr-->
        // add point<!--fr-->
        index.add(sample, i).unwrap();<!--fr-->
    }<!--fr-->
    index.build(hora::core::metrics::Metric::Euclidean).unwrap();<!--fr-->
<!--fr-->
    let mut rng = thread_rng();<!--fr-->
    let target: usize = rng.gen_range(0..n);<!--fr-->
    // 523 has neighbors: [523, 762, 364, 268, 561, 231, 380, 817, 331, 246]<!--fr-->
    println!(<!--fr-->
        "{:?} has neighbors: {:?}",<!--fr-->
        target,<!--fr-->
        index.search(&samples[target], 10) // search for k nearest neighbors<!--fr-->
    );<!--fr-->
}<!--fr-->
```<!--fr-->
<!--fr-->
merci @vaaaaanquish pour cette recherche complète d'images de rouille pure [exemple] (https://github.com/vaaaaanquish/rust-ann-search-example), Pour plus d'informations sur cet exemple, veuillez cliquer sur [Pure Rustな近似最近傍 horaを用いた画像検索を実装する](https://vaaaaaanquish.hatenablog.com/entry/2021/08/10/065117)<!--fr-->
<!--fr-->
**`Python` exemple** [[more info](https://github.com/hora-search/horapy)]<!--fr-->
<!--fr-->
```Python<!--fr-->
import numpy as np<!--fr-->
from horapy import HNSWIndex<!--fr-->
<!--fr-->
dimension = 50<!--fr-->
n = 1000<!--fr-->
<!--fr-->
# init index instance<!--fr-->
index = HNSWIndex(dimension, "usize")<!--fr-->
<!--fr-->
samples = np.float32(np.random.rand(n, dimension))<!--fr-->
for i in range(0, len(samples)):<!--fr-->
    # add node<!--fr-->
    index.add(np.float32(samples[i]), i)<!--fr-->
<!--fr-->
index.build("euclidean")  # build index<!--fr-->
<!--fr-->
target = np.random.randint(0, n)<!--fr-->
# 410 in Hora ANNIndex <HNSWIndexUsize> (dimension: 50, dtype: usize, max_item: 1000000, n_neigh: 32, n_neigh0: 64, ef_build: 20, ef_search: 500, has_deletion: False)<!--fr-->
# has neighbors: [410, 736, 65, 36, 631, 83, 111, 254, 990, 161]<!--fr-->
print("{} in {} \nhas neighbors: {}".format(<!--fr-->
    target, index, index.search(samples[target], 10)))  # search<!--fr-->
<!--fr-->
```<!--fr-->
<!--fr-->
**`JavaScript` exemple** [[more info](https://github.com/hora-search/hora-wasm)]<!--fr-->
<!--fr-->
```JavaScript<!--fr-->
import * as horajs from "horajs";<!--fr-->
<!--fr-->
const demo = () => {<!--fr-->
    const dimension = 50;<!--fr-->
    var bf_idx = horajs.BruteForceIndexUsize.new(dimension);<!--fr-->
    // var hnsw_idx = horajs.HNSWIndexUsize.new(dimension, 1000000, 32, 64, 20, 500, 16, false);<!--fr-->
    for (var i = 0; i < 1000; i++) {<!--fr-->
        var feature = [];<!--fr-->
        for (var j = 0; j < dimension; j++) {<!--fr-->
            feature.push(Math.random());<!--fr-->
        }<!--fr-->
        bf_idx.add(feature, i); // add point<!--fr-->
    }<!--fr-->
    bf_idx.build("euclidean"); // build index<!--fr-->
    var feature = [];<!--fr-->
    for (var j = 0; j < dimension; j++) {<!--fr-->
        feature.push(Math.random());<!--fr-->
    }<!--fr-->
    console.log("bf result", bf_idx.search(feature, 10)); //bf result Uint32Array(10) [704, 113, 358, 835, 408, 379, 117, 414, 808, 826]<!--fr-->
}<!--fr-->
<!--fr-->
(async () => {<!--fr-->
    await horajs.default();<!--fr-->
    await horajs.init_env();<!--fr-->
    demo();<!--fr-->
})();<!--fr-->
```<!--fr-->
<!--fr-->
**`Java` exemple** [[more info](https://github.com/hora-search/hora-java)]<!--fr-->
<!--fr-->
```Java<!--fr-->
public void demo() {<!--fr-->
    final int dimension = 2;<!--fr-->
    final float variance = 2.0f;<!--fr-->
    Random fRandom = new Random();<!--fr-->
<!--fr-->
    BruteForceIndex bruteforce_idx = new BruteForceIndex(dimension); // init index instance<!--fr-->
<!--fr-->
    List<float[]> tmp = new ArrayList<>();<!--fr-->
    for (int i = 0; i < 5; i++) {<!--fr-->
        for (int p = 0; p < 10; p++) {<!--fr-->
            float[] features = new float[dimension];<!--fr-->
            for (int j = 0; j < dimension; j++) {<!--fr-->
                features[j] = getGaussian(fRandom, (float) (i * 10), variance);<!--fr-->
            }<!--fr-->
            bruteforce_idx.add("bf", features, i * 10 + p); // add point<!--fr-->
            tmp.add(features);<!--fr-->
          }<!--fr-->
    }<!--fr-->
    bruteforce_idx.build("bf", "euclidean"); // build index<!--fr-->
<!--fr-->
    int search_index = fRandom.nextInt(tmp.size());<!--fr-->
    // nearest neighbor search<!--fr-->
    int[] result = bruteforce_idx.search("bf", 10, tmp.get(search_index));<!--fr-->
    // [main] INFO com.hora.app.ANNIndexTest  - demo bruteforce_idx[7, 8, 0, 5, 3, 9, 1, 6, 4, 2]<!--fr-->
    log.info("demo bruteforce_idx" + Arrays.toString(result));<!--fr-->
}<!--fr-->
<!--fr-->
private static float getGaussian(Random fRandom, float aMean, float variance) {<!--fr-->
    float r = (float) fRandom.nextGaussian();<!--fr-->
    return aMean + r * variance;<!--fr-->
}<!--fr-->
```<!--fr-->
<!--fr-->
# Feuille de route<!--fr-->
<!--fr-->
- [ ] Couverture complète des tests<!--fr-->
- [ ] Implémentez l'algorithme [EFANNA](http://arxiv.org/abs/1609.07228) pour obtenir une création de graphes KNN plus rapide<!--fr-->
- [ ] Prise en charge Swift et exemple de déploiement iOS/macOS<!--fr-->
- [ ] Support `R`<!--fr-->
- [ ] support `mmap`<!--fr-->
<!--fr-->
# Projets connexes et comparaison<!--fr-->
<!--fr-->
- [Faiss](https://github.com/facebookresearch/faiss), [Annoy](https://github.com/spotify/annoy), [ScaNN](https://github.com/google-research/google-research/tree/master/scann):<!--fr-->
<!--fr-->
  - **L'implémentation de `Hora` est fortement inspirée de ces bibliothèques.**<!--fr-->
  - `Faiss` se concentre davantage sur la scène GPU, et `Hora` est plus léger que Faiss (**pas de dépendances lourdes)**.<!--fr-->
  - `Hora` s'attend à prendre en charge plus de langues, et tout ce qui concerne les performances sera implémenté par Rust🦀.<!--fr-->
  - `Annoy` ne prend en charge que l'algorithme `LSH (Random Projection)`.<!--fr-->
  - `ScaNN` et `Faiss` sont moins conviviaux (par exemple, manque de documentation).<!--fr-->
  - Hora is **ALL IN RUST** 🦀.<!--fr-->
<!--fr-->
- [Milvus](https://github.com/milvus-io/milvus), [Vald](https://github.com/vdaas/vald), [Jina AI](https://github.com/jina-ai/jina)<!--fr-->
  - `Milvus` et `Vald` prennent également en charge plusieurs langues, mais servent de service au lieu d'une bibliothèque<!--fr-->
  - `Milvus` est construit sur certaines bibliothèques telles que `Faiss`, tandis que `Hora` est une bibliothèque avec tous les algorithmes implémentés elle-même<!--fr-->
<!--fr-->
# Contribute<!--fr-->
<!--fr-->
**Nous apprécions votre aide!**<!--fr-->
<!--fr-->
Nous sommes ravis de votre participation, toutes les contributions sont les bienvenues, y compris les documentations et les tests.<!--fr-->
Vous pouvez créer une `Pull Request` ou un `Issue` sur GitHub, et nous l'examinerons dès que possible.<!--fr-->
<!--fr-->
Nous utilisons les problèmes GitHub pour suivre les suggestions et les bogues.<!--fr-->
<!--fr-->
#### Clone the repo<!--fr-->
<!--fr-->
```bash<!--fr-->
git clone https://github.com/hora-search/hora<!--fr-->
```<!--fr-->
<!--fr-->
#### Build<!--fr-->
<!--fr-->
```bash<!--fr-->
cargo build<!--fr-->
```<!--fr-->
<!--fr-->
#### Test<!--fr-->
<!--fr-->
```bash<!--fr-->
cargo test --lib<!--fr-->
```<!--fr-->
<!--fr-->
#### Try the changes<!--fr-->
<!--fr-->
```bash<!--fr-->
cd examples<!--fr-->
cargo run<!--fr-->
```<!--fr-->
<!--fr-->
# License<!--fr-->
<!--fr-->
The entire repository is licensed under the [Apache License](https://github.com/hora-search/hora/blob/main/LICENSE).<!--fr-->
<div align="center"><!--jp-->
  <img src="asset/logo.svg" width="70%"/><!--jp-->
</div><!--jp-->
<!--jp-->
# Hora<!--jp-->
<!--jp-->
**[[Homepage](http://horasearch.com/)]** **[[Document](https://horasearch.com/doc)]** **[[Examples](https://horasearch.com/doc/example.html)]**<!--jp-->
<!--jp-->
**_Hora Search Everywhere!_**<!--jp-->
<!--jp-->
Horaは**近似最近傍探索アルゴリズムライブラリ** [[Wikipedia](https://ja.wikipedia.org/wiki/%E6%9C%80%E8%BF%91%E5%82%8D%E6%8E%A2%E7%B4%A2)]です。 信頼性、高レベルの抽象化、および `C++`に匹敵する高速性を達成するために、すべてのコードを`Rust🦀`で実装しています。<!--jp-->
<!--jp-->
日本語で「ほら」は、`[hōlə]`のように聞こえます。この名前は、日本の歌「小さな恋のうた」の有名な歌詞「ほら あなたにとって大事な人ほど すぐそばにいるの」にちなんで付けられました。<!--jp-->
<!--jp-->
# デモ<!--jp-->
<!--jp-->
**👩 Face-Match [[online demo](https://horasearch.com/#Demos)]**<!--jp-->
<!--jp-->
<div align="center"><!--jp-->
  <img src="asset/demo3.gif" width="100%"/><!--jp-->
</div><!--jp-->
<!--jp-->
**🍷 Dream wine comments search [[online demo](https://horasearch.com/#Demos)]**<!--jp-->
<!--jp-->
<div align="center"><!--jp-->
  <img src="asset/demo2.gif" width="100%"/><!--jp-->
</div><!--jp-->
<!--jp-->
# 特徴<!--jp-->
<!--jp-->
- **性能** ⚡️<!--jp-->
<!--jp-->
  - **SIMD アクセラレーション ([packed_simd](https://github.com/rust-lang/packed_simd))**<!--jp-->
  - **安定したアルゴリズムの実装**<!--jp-->
  - **マルチスレッドデザイン**<!--jp-->
<!--jp-->
- **複数のプログラミング言語をサポート** ☄️<!--jp-->
<!--jp-->
  - `Python`<!--jp-->
  - `Javascript`<!--jp-->
  - `Java`<!--jp-->
  - `Go` (WIP)<!--jp-->
  - `Ruby` (WIP)<!--jp-->
  - `Swift` (WIP)<!--jp-->
  - `R` (WIP)<!--jp-->
  - `Julia` (WIP)<!--jp-->
  - **サービスとしても使用可能**<!--jp-->
<!--jp-->
- **複数のインデックスをサポート** 🚀<!--jp-->
<!--jp-->
  - `Hierarchical Navigable Small World Graph Index (HNSWIndex)` ([details](https://arxiv.org/abs/1603.09320))<!--jp-->
  - `Satellite System Graph (SSGIndex)` ([details](https://arxiv.org/abs/1907.06146))<!--jp-->
  - `Product Quantization Inverted File(PQIVFIndex)` ([details](https://lear.inrialpes.fr/pubs/2011/JDS11/jegou_searching_with_quantization.pdf))<!--jp-->
  - `Random Projection Tree(RPTIndex)` (LSH, WIP)<!--jp-->
  - `BruteForce (BruteForceIndex)` (SIMDを使った素朴な実装)<!--jp-->
<!--jp-->
- **移植性** 💼<!--jp-->
<!--jp-->
  - `WebAssembly`対応<!--jp-->
  - `Windows`、`Linux`および`OS X`に対応<!--jp-->
  - `iOS`および`Android`対応 (WIP)<!--jp-->
  - `no_std`対応 (WIP, partial)<!--jp-->
  - `BLAS`などの大きな依存関係は**ありません**<!--jp-->
<!--jp-->
- **信頼性** 🔒<!--jp-->
<!--jp-->
  - `Rust`コンパイラはすべてのコードを保護します<!--jp-->
  - `Python`などの全ての言語向けのライブラリで`Rust`によるメモリ管理<!--jp-->
  - 幅広いテスト範囲<!--jp-->
<!--jp-->
- **複数の距離をサポート** 🧮<!--jp-->
<!--jp-->
  - `Dot Product Distance`<!--jp-->
    - ![equation](https://latex.codecogs.com/gif.latex?D%28x%2Cy%29%20%3D%20%5Csum%7B%28x*y%29%7D)<!--jp-->
  - `Euclidean Distance`<!--jp-->
    - ![equation](https://latex.codecogs.com/gif.latex?D%28x%2Cy%29%20%3D%20%5Csqrt%7B%5Csum%7B%28x-y%29%5E2%7D%7D)<!--jp-->
  - `Manhattan Distance`<!--jp-->
    - ![equation](https://latex.codecogs.com/gif.latex?D%28x%2Cy%29%20%3D%20%5Csum%7B%7C%28x-y%29%7C%7D)<!--jp-->
  - `Cosine Similarity`<!--jp-->
    - ![equation](https://latex.codecogs.com/gif.latex?D%28x%2Cy%29%20%3D%20%5Cfrac%7Bx%20*y%7D%7B%7C%7Cx%7C%7C*%7C%7Cy%7C%7C%7D)<!--jp-->
<!--jp-->
- **生産性** ⭐<!--jp-->
  - 整備されたドキュメント<!--jp-->
  - エレガントかつシンプル、そして習得しやすいAPI<!--jp-->
<!--jp-->
# インストール<!--jp-->
<!--jp-->
`Cargo.toml`で<!--jp-->
<!--jp-->
```toml<!--jp-->
[dependencies]<!--jp-->
hora = "0.1.1"<!--jp-->
```<!--jp-->
<!--jp-->
**`Python`**<!--jp-->
<!--jp-->
```Bash<!--jp-->
$ pip install horapy<!--jp-->
```<!--jp-->
<!--jp-->
**`Javascript (WebAssembly)`**<!--jp-->
<!--jp-->
```Bash<!--jp-->
$ npm i horajs<!--jp-->
```<!--jp-->
<!--jp-->
**`ソースコードからビルド`**<!--jp-->
<!--jp-->
```bash<!--jp-->
$ git clone https://github.com/hora-search/hora<!--jp-->
$ cargo build<!--jp-->
```<!--jp-->
<!--jp-->
# ベンチマーク<!--jp-->
<!--jp-->
<img src="asset/fashion-mnist-784-euclidean_10_euclidean.png"/><!--jp-->
<!--jp-->
by `aws t2.medium (CPU: Intel(R) Xeon(R) CPU E5-2686 v4 @ 2.30GHz)` [more information](https://github.com/hora-search/ann-benchmarks)<!--jp-->
<!--jp-->
# Examples<!--jp-->
<!--jp-->
**`Rust`** [[詳細](https://github.com/hora-search/hora/tree/main/examples)]<!--jp-->
<!--jp-->
```Rust<!--jp-->
use hora::core::ann_index::ANNIndex;<!--jp-->
use rand::{thread_rng, Rng};<!--jp-->
use rand_distr::{Distribution, Normal};<!--jp-->
<!--jp-->
pub fn demo() {<!--jp-->
    let n = 1000;<!--jp-->
    let dimension = 64;<!--jp-->
<!--jp-->
    // make sample points<!--jp-->
    let mut samples = Vec::with_capacity(n);<!--jp-->
    let normal = Normal::new(0.0, 10.0).unwrap();<!--jp-->
    for _i in 0..n {<!--jp-->
        let mut sample = Vec::with_capacity(dimension);<!--jp-->
        for _j in 0..dimension {<!--jp-->
            sample.push(normal.sample(&mut rand::thread_rng()));<!--jp-->
        }<!--jp-->
        samples.push(sample);<!--jp-->
    }<!--jp-->
<!--jp-->
    // init index<!--jp-->
    let mut index = hora::index::hnsw_idx::HNSWIndex::<f32, usize>::new(<!--jp-->
        dimension,<!--jp-->
        &hora::index::hnsw_params::HNSWParams::<f32>::default(),<!--jp-->
    );<!--jp-->
    for (i, sample) in samples.iter().enumerate().take(n) {<!--jp-->
        // add point<!--jp-->
        index.add(sample, i).unwrap();<!--jp-->
    }<!--jp-->
    index.build(hora::core::metrics::Metric::Euclidean).unwrap();<!--jp-->
<!--jp-->
    let mut rng = thread_rng();<!--jp-->
    let target: usize = rng.gen_range(0..n);<!--jp-->
    // 523 has neighbors: [523, 762, 364, 268, 561, 231, 380, 817, 331, 246]<!--jp-->
    println!(<!--jp-->
        "{:?} has neighbors: {:?}",<!--jp-->
        target,<!--jp-->
        index.search(&samples[target], 10) // search for k nearest neighbors<!--jp-->
    );<!--jp-->
}<!--jp-->
```<!--jp-->
<!--jp-->
この完全な純粋な錆画像検索の[コード例](https://github.com/vaaaaanquish/rust-ann-search-example)を公開して下さった@vaaaaanquish様に感謝申し上げます。この例の詳細については、[Pure Rustな近似最近傍探索ライブラリhoraを用いた画像検索を実装する](https://vaaaaaanquish.hatenablog.com/entry/2021/08/10/065117)をご覧ください。<!--jp-->
<!--jp-->
**`Python`** [[詳細](https://github.com/hora-search/horapy)]<!--jp-->
<!--jp-->
```Python<!--jp-->
import numpy as np<!--jp-->
from horapy import HNSWIndex<!--jp-->
<!--jp-->
dimension = 50<!--jp-->
n = 1000<!--jp-->
<!--jp-->
# init index instance<!--jp-->
index = HNSWIndex(dimension, "usize")<!--jp-->
<!--jp-->
samples = np.float32(np.random.rand(n, dimension))<!--jp-->
for i in range(0, len(samples)):<!--jp-->
    # add node<!--jp-->
    index.add(np.float32(samples[i]), i)<!--jp-->
<!--jp-->
index.build("euclidean")  # build index<!--jp-->
<!--jp-->
target = np.random.randint(0, n)<!--jp-->
# 410 in Hora ANNIndex <HNSWIndexUsize> (dimension: 50, dtype: usize, max_item: 1000000, n_neigh: 32, n_neigh0: 64, ef_build: 20, ef_search: 500, has_deletion: False)<!--jp-->
# has neighbors: [410, 736, 65, 36, 631, 83, 111, 254, 990, 161]<!--jp-->
print("{} in {} \nhas neighbors: {}".format(<!--jp-->
    target, index, index.search(samples[target], 10)))  # search<!--jp-->
<!--jp-->
```<!--jp-->
<!--jp-->
**`JavaScript`** [[詳細](https://github.com/hora-search/hora-wasm)]<!--jp-->
<!--jp-->
```JavaScript<!--jp-->
import * as horajs from "horajs";<!--jp-->
<!--jp-->
const demo = () => {<!--jp-->
    const dimension = 50;<!--jp-->
    var bf_idx = horajs.BruteForceIndexUsize.new(dimension);<!--jp-->
    // var hnsw_idx = horajs.HNSWIndexUsize.new(dimension, 1000000, 32, 64, 20, 500, 16, false);<!--jp-->
    for (var i = 0; i < 1000; i++) {<!--jp-->
        var feature = [];<!--jp-->
        for (var j = 0; j < dimension; j++) {<!--jp-->
            feature.push(Math.random());<!--jp-->
        }<!--jp-->
        bf_idx.add(feature, i); // add point<!--jp-->
    }<!--jp-->
    bf_idx.build("euclidean"); // build index<!--jp-->
    var feature = [];<!--jp-->
    for (var j = 0; j < dimension; j++) {<!--jp-->
        feature.push(Math.random());<!--jp-->
    }<!--jp-->
    console.log("bf result", bf_idx.search(feature, 10)); //bf result Uint32Array(10) [704, 113, 358, 835, 408, 379, 117, 414, 808, 826]<!--jp-->
}<!--jp-->
<!--jp-->
(async () => {<!--jp-->
    await horajs.default();<!--jp-->
    await horajs.init_env();<!--jp-->
    demo();<!--jp-->
})();<!--jp-->
```<!--jp-->
<!--jp-->
**`Java`** [[詳細](https://github.com/hora-search/hora-java)]<!--jp-->
<!--jp-->
```Java<!--jp-->
public void demo() {<!--jp-->
    final int dimension = 2;<!--jp-->
    final float variance = 2.0f;<!--jp-->
    Random fRandom = new Random();<!--jp-->
<!--jp-->
    BruteForceIndex bruteforce_idx = new BruteForceIndex(dimension); // init index instance<!--jp-->
<!--jp-->
    List<float[]> tmp = new ArrayList<>();<!--jp-->
    for (int i = 0; i < 5; i++) {<!--jp-->
        for (int p = 0; p < 10; p++) {<!--jp-->
            float[] features = new float[dimension];<!--jp-->
            for (int j = 0; j < dimension; j++) {<!--jp-->
                features[j] = getGaussian(fRandom, (float) (i * 10), variance);<!--jp-->
            }<!--jp-->
            bruteforce_idx.add("bf", features, i * 10 + p); // add point<!--jp-->
            tmp.add(features);<!--jp-->
          }<!--jp-->
    }<!--jp-->
    bruteforce_idx.build("bf", "euclidean"); // build index<!--jp-->
<!--jp-->
    int search_index = fRandom.nextInt(tmp.size());<!--jp-->
    // nearest neighbor search<!--jp-->
    int[] result = bruteforce_idx.search("bf", 10, tmp.get(search_index));<!--jp-->
    // [main] INFO com.hora.app.ANNIndexTest  - demo bruteforce_idx[7, 8, 0, 5, 3, 9, 1, 6, 4, 2]<!--jp-->
    log.info("demo bruteforce_idx" + Arrays.toString(result));<!--jp-->
}<!--jp-->
<!--jp-->
private static float getGaussian(Random fRandom, float aMean, float variance) {<!--jp-->
    float r = (float) fRandom.nextGaussian();<!--jp-->
    return aMean + r * variance;<!--jp-->
}<!--jp-->
```<!--jp-->
<!--jp-->
# ロードマップ<!--jp-->
<!--jp-->
- [ ] 完全なテストカバレッジ<!--jp-->
- [ ] [EFANNA](http://arxiv.org/abs/1609.07228)アルゴリズムを実装して、より高速なKNNグラフ構築を実現する<!--jp-->
- [ ] SwiftのサポートとiOS/macOSのデプロイ例<!--jp-->
- [ ] `R`のサポート<!--jp-->
- [ ] `mmap`のサポート<!--jp-->
<!--jp-->
# 関連プロジェクトと比較<!--jp-->
<!--jp-->
- [Faiss](https://github.com/facebookresearch/faiss), [Annoy](https://github.com/spotify/annoy), [ScaNN](https://github.com/google-research/google-research/tree/master/scann):<!--jp-->
<!--jp-->
  - **Hora の実装は、これらのライブラリに強く影響を受けています。**<!--jp-->
  - `Faiss`はGPUの使用に重点を置いており、`Hora`はFaissよりも軽量です **(大きな依存関係はありません)**。<!--jp-->
  - `Hora`はより多くの言語をサポートすることを期待しており、パフォーマンスに関連する部分は全てRustで実装しています🦀<!--jp-->
  - `Annoy`は`LSH(Random Projection)`アルゴリズムのみをサポートします。<!--jp-->
  - `ScaN`と`Fats`はドキュメントなどの面においてユーザーフレンドリーではありません<!--jp-->
  - Hora is **ALL IN RUST** 🦀 (Horaは**全てRustで実装しています**🦀)<!--jp-->
<!--jp-->
- [Milvus](https://github.com/milvus-io/milvus), [Vald](https://github.com/vdaas/vald), [Jina AI](https://github.com/jina-ai/jina)<!--jp-->
  - `Milvus`と` Vald`も複数の言語をサポートしていますが、ライブラリではなくサービスとして機能します<!--jp-->
  - `Milvus`は`Faiss`などのいくつかのライブラリにの上に成り立っていますが、 `Hora`ではすべてのアルゴリズムが実装されています。<!--jp-->
<!--jp-->
# 貢献に参加する<!--jp-->
<!--jp-->
**We appreciate your participation!**<!--jp-->
<!--jp-->
皆様のご参加をお待ちしております。ドキュメントやテストなど、あらゆる貢献を歓迎します。<!--jp-->
GitHub でPull RequestsまたはIssuesを作成できます。できるだけ早く確認します。<!--jp-->
<!--jp-->
提案やバグを管理するためにGitHubのIssuesを使用します。<!--jp-->
<!--jp-->
#### リポジトリのclone<!--jp-->
<!--jp-->
```bash<!--jp-->
git clone https://github.com/hora-search/hora<!--jp-->
```<!--jp-->
<!--jp-->
#### ビルド<!--jp-->
<!--jp-->
```bash<!--jp-->
cargo build<!--jp-->
```<!--jp-->
<!--jp-->
#### テスト<!--jp-->
<!--jp-->
```bash<!--jp-->
cargo test --lib<!--jp-->
```<!--jp-->
<!--jp-->
#### 変更の確認<!--jp-->
<!--jp-->
```bash<!--jp-->
cd examples<!--jp-->
cargo run<!--jp-->
```<!--jp-->
<!--jp-->
# ライセンス<!--jp-->
<!--jp-->
このリポジトリは[Apache License](https://github.com/hora-search/hora/blob/main/LICENSE)でライセンスされています。<!--jp-->
<div align="center"><!--kr-->
  <img src="asset/logo.svg" width="70%"/><!--kr-->
</div><!--kr-->
<!--kr-->
# Hora<!--kr-->
<!--kr-->
**[[Homepage](http://horasearch.com/)]** **[[Document](https://horasearch.com/doc)]** **[[Examples](https://horasearch.com/doc/example.html)]**<!--kr-->
<!--kr-->
**_Hora Search Everywhere!_**<!--kr-->
<!--kr-->
Hora는 **근접 이웃 검색 알고리즘**([wiki](https://en.wikipedia.org/wiki/Nearest_neighbor_search)) 라이브러리입니다. 우리는 `C++` 에 필적하는 신뢰성, 높은 수준의 추상화 및 고속을 위해 `Rust🦀 `에서 모든 코드를 구현합니다.<!--kr-->
<!--kr-->
Hora, `「ほら」`는 일본어로 `[hōlə]`처럼 들리며 `와우`, `알겠습니다!` 또는 `저걸 봐!`를 의미합니다. 이름은 유명한 일본 노래 `「小さな恋のうた」`에서 영감을 받았습니다.<!--kr-->
<!--kr-->
# Demos<!--kr-->
<!--kr-->
**👩 Face-Match [[online demo](https://horasearch.com/#Demos)], have a try!**<!--kr-->
<!--kr-->
<div align="center"><!--kr-->
  <img src="asset/demo3.gif" width="100%"/><!--kr-->
</div><!--kr-->
<!--kr-->
**🍷 Dream wine comments search [[online demo](https://horasearch.com/#Demos)], have a try!**<!--kr-->
<!--kr-->
<div align="center"><!--kr-->
  <img src="asset/demo2.gif" width="100%"/><!--kr-->
</div><!--kr-->
<!--kr-->
# Features<!--kr-->
<!--kr-->
- **Performant** ⚡️<!--kr-->
<!--kr-->
  - **SIMD-Accelerated ([packed_simd](https://github.com/rust-lang/packed_simd))**<!--kr-->
  - **Stable algorithm implementation**<!--kr-->
  - **Multiple threads design**<!--kr-->
<!--kr-->
- **Supports Multiple Languages** ☄️<!--kr-->
<!--kr-->
  - `Python`<!--kr-->
  - `Javascript`<!--kr-->
  - `Java`<!--kr-->
  - `Go` (WIP)<!--kr-->
  - `Ruby` (WIP)<!--kr-->
  - `Swift` (WIP)<!--kr-->
  - `R` (WIP)<!--kr-->
  - `Julia` (WIP)<!--kr-->
  - **Can also be used as a service**<!--kr-->
<!--kr-->
- **Supports Multiple Indexes** 🚀<!--kr-->
<!--kr-->
  - `Hierarchical Navigable Small World Graph Index (HNSWIndex)` ([details](https://arxiv.org/abs/1603.09320))<!--kr-->
  - `Satellite System Graph (SSGIndex)` ([details](https://arxiv.org/abs/1907.06146))<!--kr-->
  - `Product Quantization Inverted File(PQIVFIndex)` ([details](https://lear.inrialpes.fr/pubs/2011/JDS11/jegou_searching_with_quantization.pdf))<!--kr-->
  - `Random Projection Tree(RPTIndex)` (LSH, WIP)<!--kr-->
  - `BruteForce (BruteForceIndex)` (naive implementation with SIMD)<!--kr-->
<!--kr-->
- **Portable** 💼<!--kr-->
  - Supports `WebAssembly`<!--kr-->
  - Supports `Windows`, `Linux` and `OS X`<!--kr-->
  - Supports `IOS` and `Android` (WIP)<!--kr-->
  - Supports `no_std` (WIP, partial)<!--kr-->
  - **No** heavy dependencies, such as `BLAS`<!--kr-->
<!--kr-->
- **Reliability** 🔒<!--kr-->
<!--kr-->
  - `Rust` compiler secures all code<!--kr-->
  - Memory managed by `Rust` for all language libraries such as `Python's`<!--kr-->
  - Broad testing coverage<!--kr-->
<!--kr-->
- **Supports Multiple Distances** 🧮<!--kr-->
<!--kr-->
  - `Dot Product Distance`<!--kr-->
    - ![equation](https://latex.codecogs.com/gif.latex?D%28x%2Cy%29%20%3D%20%5Csum%7B%28x*y%29%7D)<!--kr-->
  - `Euclidean Distance`<!--kr-->
    - ![equation](https://latex.codecogs.com/gif.latex?D%28x%2Cy%29%20%3D%20%5Csqrt%7B%5Csum%7B%28x-y%29%5E2%7D%7D)<!--kr-->
  - `Manhattan Distance`<!--kr-->
    - ![equation](https://latex.codecogs.com/gif.latex?D%28x%2Cy%29%20%3D%20%5Csum%7B%7C%28x-y%29%7C%7D)<!--kr-->
  - `Cosine Similarity`<!--kr-->
    - ![equation](https://latex.codecogs.com/gif.latex?D%28x%2Cy%29%20%3D%20%5Cfrac%7Bx%20*y%7D%7B%7C%7Cx%7C%7C*%7C%7Cy%7C%7C%7D)<!--kr-->
<!--kr-->
- **Productive** ⭐<!--kr-->
  - Well documented<!--kr-->
  - Elegant, simple and easy to learn API<!--kr-->
<!--kr-->
# Installation<!--kr-->
<!--kr-->
**`Rust`**<!--kr-->
<!--kr-->
in `Cargo.toml`<!--kr-->
<!--kr-->
```toml<!--kr-->
[dependencies]<!--kr-->
hora = "0.1.1"<!--kr-->
```<!--kr-->
<!--kr-->
**`Python`**<!--kr-->
<!--kr-->
```Bash<!--kr-->
$ pip install horapy<!--kr-->
```<!--kr-->
<!--kr-->
**`Javascript (WebAssembly)`**<!--kr-->
<!--kr-->
```Bash<!--kr-->
$ npm i horajs<!--kr-->
```<!--kr-->
<!--kr-->
**`Building from source`**<!--kr-->
<!--kr-->
```bash<!--kr-->
$ git clone https://github.com/hora-search/hora<!--kr-->
$ cargo build<!--kr-->
```<!--kr-->
<!--kr-->
# Benchmarks<!--kr-->
<!--kr-->
<img src="asset/fashion-mnist-784-euclidean_10_euclidean.png"/><!--kr-->
<!--kr-->
by `aws t2.medium (CPU: Intel(R) Xeon(R) CPU E5-2686 v4 @ 2.30GHz)` [more information](https://github.com/hora-search/ann-benchmarks)<!--kr-->
<!--kr-->
# Examples<!--kr-->
<!--kr-->
**`Rust` example** [[more info](https://github.com/hora-search/hora/tree/main/examples)]<!--kr-->
<!--kr-->
```Rust<!--kr-->
use hora::core::ann_index::ANNIndex;<!--kr-->
use rand::{thread_rng, Rng};<!--kr-->
use rand_distr::{Distribution, Normal};<!--kr-->
<!--kr-->
pub fn demo() {<!--kr-->
    let n = 1000;<!--kr-->
    let dimension = 64;<!--kr-->
<!--kr-->
    // make sample points<!--kr-->
    let mut samples = Vec::with_capacity(n);<!--kr-->
    let normal = Normal::new(0.0, 10.0).unwrap();<!--kr-->
    for _i in 0..n {<!--kr-->
        let mut sample = Vec::with_capacity(dimension);<!--kr-->
        for _j in 0..dimension {<!--kr-->
            sample.push(normal.sample(&mut rand::thread_rng()));<!--kr-->
        }<!--kr-->
        samples.push(sample);<!--kr-->
    }<!--kr-->
<!--kr-->
    // init index<!--kr-->
    let mut index = hora::index::hnsw_idx::HNSWIndex::<f32, usize>::new(<!--kr-->
        dimension,<!--kr-->
        &hora::index::hnsw_params::HNSWParams::<f32>::default(),<!--kr-->
    );<!--kr-->
    for (i, sample) in samples.iter().enumerate().take(n) {<!--kr-->
        // add point<!--kr-->
        index.add(sample, i).unwrap();<!--kr-->
    }<!--kr-->
    index.build(hora::core::metrics::Metric::Euclidean).unwrap();<!--kr-->
<!--kr-->
    let mut rng = thread_rng();<!--kr-->
    let target: usize = rng.gen_range(0..n);<!--kr-->
    // 523 has neighbors: [523, 762, 364, 268, 561, 231, 380, 817, 331, 246]<!--kr-->
    println!(<!--kr-->
        "{:?} has neighbors: {:?}",<!--kr-->
        target,<!--kr-->
        index.search(&samples[target], 10) // search for k nearest neighbors<!--kr-->
    );<!--kr-->
}<!--kr-->
```<!--kr-->
<!--kr-->
thank @vaaaaanquish for this complete pure rust image search [example](https://github.com/vaaaaanquish/rust-ann-search-example), For more information about this example, please can click [Pure Rustな近似最近傍探索ライブラリhoraを用いた画像検索を実装する](https://vaaaaaanquish.hatenablog.com/entry/2021/08/10/065117)<!--kr-->
<!--kr-->
**`Python` example** [[more info](https://github.com/hora-search/horapy)]<!--kr-->
<!--kr-->
```Python<!--kr-->
import numpy as np<!--kr-->
from horapy import HNSWIndex<!--kr-->
<!--kr-->
dimension = 50<!--kr-->
n = 1000<!--kr-->
<!--kr-->
# init index instance<!--kr-->
index = HNSWIndex(dimension, "usize")<!--kr-->
<!--kr-->
samples = np.float32(np.random.rand(n, dimension))<!--kr-->
for i in range(0, len(samples)):<!--kr-->
    # add node<!--kr-->
    index.add(np.float32(samples[i]), i)<!--kr-->
<!--kr-->
index.build("euclidean")  # build index<!--kr-->
<!--kr-->
target = np.random.randint(0, n)<!--kr-->
# 410 in Hora ANNIndex <HNSWIndexUsize> (dimension: 50, dtype: usize, max_item: 1000000, n_neigh: 32, n_neigh0: 64, ef_build: 20, ef_search: 500, has_deletion: False)<!--kr-->
# has neighbors: [410, 736, 65, 36, 631, 83, 111, 254, 990, 161]<!--kr-->
print("{} in {} \nhas neighbors: {}".format(<!--kr-->
    target, index, index.search(samples[target], 10)))  # search<!--kr-->
<!--kr-->
```<!--kr-->
<!--kr-->
**`JavaScript` example** [[more info](https://github.com/hora-search/hora-wasm)]<!--kr-->
<!--kr-->
```JavaScript<!--kr-->
import * as horajs from "horajs";<!--kr-->
<!--kr-->
const demo = () => {<!--kr-->
    const dimension = 50;<!--kr-->
    var bf_idx = horajs.BruteForceIndexUsize.new(dimension);<!--kr-->
    // var hnsw_idx = horajs.HNSWIndexUsize.new(dimension, 1000000, 32, 64, 20, 500, 16, false);<!--kr-->
    for (var i = 0; i < 1000; i++) {<!--kr-->
        var feature = [];<!--kr-->
        for (var j = 0; j < dimension; j++) {<!--kr-->
            feature.push(Math.random());<!--kr-->
        }<!--kr-->
        bf_idx.add(feature, i); // add point <!--kr-->
    }<!--kr-->
    bf_idx.build("euclidean"); // build index<!--kr-->
    var feature = [];<!--kr-->
    for (var j = 0; j < dimension; j++) {<!--kr-->
        feature.push(Math.random());<!--kr-->
    }<!--kr-->
    console.log("bf result", bf_idx.search(feature, 10)); //bf result Uint32Array(10) [704, 113, 358, 835, 408, 379, 117, 414, 808, 826]<!--kr-->
}<!--kr-->
<!--kr-->
(async () => {<!--kr-->
    await horajs.default();<!--kr-->
    await horajs.init_env();<!--kr-->
    demo();<!--kr-->
})();<!--kr-->
```<!--kr-->
<!--kr-->
**`Java` example** [[more info](https://github.com/hora-search/hora-java)]<!--kr-->
<!--kr-->
```Java<!--kr-->
public void demo() {<!--kr-->
    final int dimension = 2;<!--kr-->
    final float variance = 2.0f;<!--kr-->
    Random fRandom = new Random();<!--kr-->
<!--kr-->
    BruteForceIndex bruteforce_idx = new BruteForceIndex(dimension); // init index instance<!--kr-->
<!--kr-->
    List<float[]> tmp = new ArrayList<>();<!--kr-->
    for (int i = 0; i < 5; i++) {<!--kr-->
        for (int p = 0; p < 10; p++) {<!--kr-->
            float[] features = new float[dimension];<!--kr-->
            for (int j = 0; j < dimension; j++) {<!--kr-->
                features[j] = getGaussian(fRandom, (float) (i * 10), variance);<!--kr-->
            }<!--kr-->
            bruteforce_idx.add("bf", features, i * 10 + p); // add point<!--kr-->
            tmp.add(features);<!--kr-->
          }<!--kr-->
    }<!--kr-->
    bruteforce_idx.build("bf", "euclidean"); // build index<!--kr-->
<!--kr-->
    int search_index = fRandom.nextInt(tmp.size());<!--kr-->
    // nearest neighbor search<!--kr-->
    int[] result = bruteforce_idx.search("bf", 10, tmp.get(search_index));<!--kr-->
    // [main] INFO com.hora.app.ANNIndexTest  - demo bruteforce_idx[7, 8, 0, 5, 3, 9, 1, 6, 4, 2]<!--kr-->
    log.info("demo bruteforce_idx" + Arrays.toString(result));<!--kr-->
}<!--kr-->
<!--kr-->
private static float getGaussian(Random fRandom, float aMean, float variance) {<!--kr-->
    float r = (float) fRandom.nextGaussian();<!--kr-->
    return aMean + r * variance;<!--kr-->
}<!--kr-->
```<!--kr-->
<!--kr-->
# Roadmap<!--kr-->
<!--kr-->
- [ ] 전체 테스트 범위<!--kr-->
- [ ] 더 빠른 KNN 그래프 구축을 달성하기 위해 [EFANNA](http://arxiv.org/abs/1609.07228) 알고리즘 구현<!--kr-->
- [ ] Swift 지원 및 `iOS`/`macOS` 배포 예시<!--kr-->
- [ ] 지원 `R`<!--kr-->
- [ ] `mmap` 지원<!--kr-->
<!--kr-->
# Related Projects and Comparison<!--kr-->
<!--kr-->
- [Faiss](https://github.com/facebookresearch/faiss), [Annoy](https://github.com/spotify/annoy), [ScaNN](https://github.com/google-research/google-research/tree/master/scann):<!--kr-->
<!--kr-->
  - **`Hora`의 구현은 이러한 라이브러리에서 크게 영감을 받았습니다.**<!--kr-->
  - `Faiss`는 GPU 장면에 더 중점을 두고 `Hora`는 Faiss보다 가볍습니다(**중대한 종속성 없음)**.<!--kr-->
  - `Hora`는 더 많은 언어를 지원할 예정이며 성능과 관련된 모든 것은 Rust🦀에서 구현됩니다.<!--kr-->
  - `Annoy`는 ``LSH (Random Projection)` 알고리즘만 지원합니다.<!--kr-->
  - `ScaNN` 및 `Faiss`는 사용자 친화적이지 않습니다(예: 문서 부족).<!--kr-->
  - Hora is **ALL IN RUST** 🦀.<!--kr-->
<!--kr-->
- [Milvus](https://github.com/milvus-io/milvus), [Vald](https://github.com/vdaas/vald), [Jina AI](https://github.com/jina-ai/jina)<!--kr-->
  - 'Milvus'와 'Vald'도 여러 언어를 지원하지만 라이브러리 대신 서비스 역할을 합니다.<!--kr-->
  - 'Milvus'는 'Faiss'와 같은 일부 라이브러리를 기반으로 하는 반면, 'Hora'는 모든 알고리즘이 자체적으로 구현된 라이브러리입니다.<!--kr-->
<!--kr-->
# Contribute<!--kr-->
<!--kr-->
**We appreciate your help!**<!--kr-->
<!--kr-->
문서 및 테스트를 포함하여 모든 기여를 환영합니다.<!--kr-->
GitHub에서 `Pull Request` 또는 `Issue` 를 생성할 수 있으며 최대한 빨리 검토하겠습니다.<!--kr-->
<!--kr-->
제안 및 버그를 추적하기 위해 GitHub 문제를 사용합니다.<!--kr-->
<!--kr-->
#### Clone the repo<!--kr-->
<!--kr-->
```bash<!--kr-->
git clone https://github.com/hora-search/hora<!--kr-->
```<!--kr-->
<!--kr-->
#### Build<!--kr-->
<!--kr-->
```bash<!--kr-->
cargo build<!--kr-->
```<!--kr-->
<!--kr-->
#### Test<!--kr-->
<!--kr-->
```bash<!--kr-->
cargo test --lib<!--kr-->
```<!--kr-->
<!--kr-->
#### Try the changes<!--kr-->
<!--kr-->
```bash<!--kr-->
cd examples<!--kr-->
cargo run<!--kr-->
```<!--kr-->
<!--kr-->
# License<!--kr-->
<!--kr-->
The entire repository is licensed under the [Apache License](https://github.com/hora-search/hora/blob/main/LICENSE).<!--kr-->
<div align="center"><!--ru-->
  <img src="asset/logo.svg" width="70%"/><!--ru-->
</div><!--ru-->
<!--ru-->
# Hora<!--ru-->
<!--ru-->
**[[Homepage](http://horasearch.com/)]** **[[Document](https://horasearch.com/doc)]** **[[Examples](https://horasearch.com/doc/example.html)]**<!--ru-->
<!--ru-->
**_Hora Search Everywhere!_**<!--ru-->
<!--ru-->
Hora - это **приблизительный алгоритм поиска ближайшего соседа** ([wiki](https://en.wikipedia.org/wiki/Nearest_neighbor_search)) библиотека. Мы реализуем весь код на `Rust🦀 ` для надежности, высокого уровня абстракции и высокой скорости, сравнимой с `C++`.<!--ru-->
<!--ru-->
Hora, **`「ほら」`** на японском языке, звучит как `[hōlə]` и означает `Вау`,`Ты видишь!`Или`Посмотри на это!`. Название навеяно известной японской песней **`「小さな恋のうた」`**.<!--ru-->
<!--ru-->
# Демо<!--ru-->
<!--ru-->
**👩 Face-Match [[online demo](https://horasearch.com/#Demos)], попробуй!**<!--ru-->
<!--ru-->
<div align="center"><!--ru-->
  <img src="asset/demo3.gif" width="100%"/><!--ru-->
</div><!--ru-->
<!--ru-->
**🍷 Dream wine comments search [[online demo](https://horasearch.com/#Demos)], попробуй!**<!--ru-->
<!--ru-->
<div align="center"><!--ru-->
  <img src="asset/demo2.gif" width="100%"/><!--ru-->
</div><!--ru-->
<!--ru-->
# ключевая особенность<!--ru-->
<!--ru-->
- **Исполнитель** ⚡️<!--ru-->
<!--ru-->
  - **SIMD-Accelerated ([packed_simd](https://github.com/rust-lang/packed_simd))**<!--ru-->
  - **Быстрая реализация алгоритма**<!--ru-->
  - **Многопоточная конструкция**<!--ru-->
<!--ru-->
- **Поддерживает несколько языков программирования** ☄️<!--ru-->
<!--ru-->
  - `Python`<!--ru-->
  - `Javascript`<!--ru-->
  - `Java`<!--ru-->
  - `Go` (WIP)<!--ru-->
  - `Ruby` (WIP)<!--ru-->
  - `Swift` (WIP)<!--ru-->
  - `R` (WIP)<!--ru-->
  - `Julia` (WIP)<!--ru-->
  - **Также может использоваться как услуга**<!--ru-->
<!--ru-->
- **Поддерживает несколько индексов** 🚀<!--ru-->
<!--ru-->
  - `Hierarchical Navigable Small World Graph Index (HNSWIndex)` ([details](https://arxiv.org/abs/1603.09320))<!--ru-->
  - `Satellite System Graph (SSGIndex)` ([details](https://arxiv.org/abs/1907.06146))<!--ru-->
  - `Product Quantization Inverted File(PQIVFIndex)` ([details](https://lear.inrialpes.fr/pubs/2011/JDS11/jegou_searching_with_quantization.pdf))<!--ru-->
  - `Random Projection Tree(RPTIndex)` (LSH, WIP)<!--ru-->
  - `BruteForce (BruteForceIndex)` (naive implementation with SIMD)<!--ru-->
<!--ru-->
- **Портативный** 💼<!--ru-->
<!--ru-->
  - Supports `WebAssembly`<!--ru-->
  - Supports `Windows`, `Linux` and `OS X`<!--ru-->
  - Supports `IOS` and `Android` (WIP)<!--ru-->
  - Supports `no_std` (WIP, partial)<!--ru-->
  - Никаких тяжелых зависимостей, таких как `BLAS`<!--ru-->
<!--ru-->
- **Надежность** 🔒<!--ru-->
<!--ru-->
  - Компилятор `Rust` защищает весь код<!--ru-->
  - Память, управляемая `Rust` для всех языковых библиотек, таких как `Python`<!--ru-->
  - Broad testing coverage<!--ru-->
<!--ru-->
- **Широкий охват тестирования** 🧮<!--ru-->
<!--ru-->
  - `Расстояние точечного продукта`<!--ru-->
    - ![equation](https://latex.codecogs.com/gif.latex?D%28x%2Cy%29%20%3D%20%5Csum%7B%28x*y%29%7D)<!--ru-->
  - `Евклидово расстояние`<!--ru-->
    - ![equation](https://latex.codecogs.com/gif.latex?D%28x%2Cy%29%20%3D%20%5Csqrt%7B%5Csum%7B%28x-y%29%5E2%7D%7D)<!--ru-->
  - `Манхэттен Расстояние`<!--ru-->
    - ![equation](https://latex.codecogs.com/gif.latex?D%28x%2Cy%29%20%3D%20%5Csum%7B%7C%28x-y%29%7C%7D)<!--ru-->
  - `Косинусное подобие`<!--ru-->
    - ![equation](https://latex.codecogs.com/gif.latex?D%28x%2Cy%29%20%3D%20%5Cfrac%7Bx%20*y%7D%7B%7C%7Cx%7C%7C*%7C%7Cy%7C%7C%7D)<!--ru-->
<!--ru-->
- **Продуктивный** ⭐<!--ru-->
  - Хорошо задокументированы<!--ru-->
  - Элегантный, простой и легкий в освоении API<!--ru-->
<!--ru-->
# Монтаж<!--ru-->
<!--ru-->
**`Rust`**<!--ru-->
<!--ru-->
in `Cargo.toml`<!--ru-->
<!--ru-->
```toml<!--ru-->
[dependencies]<!--ru-->
hora = "0.1.1"<!--ru-->
```<!--ru-->
<!--ru-->
**`Python`**<!--ru-->
<!--ru-->
```Bash<!--ru-->
$ pip install horapy<!--ru-->
```<!--ru-->
<!--ru-->
**`Javascript (WebAssembly)`**<!--ru-->
<!--ru-->
```Bash<!--ru-->
$ npm i horajs<!--ru-->
```<!--ru-->
<!--ru-->
**`Building from source`**<!--ru-->
<!--ru-->
```bash<!--ru-->
$ git clone https://github.com/hora-search/hora<!--ru-->
$ cargo build<!--ru-->
```<!--ru-->
<!--ru-->
# Контрольный показатель<!--ru-->
<!--ru-->
<img src="asset/fashion-mnist-784-euclidean_10_euclidean.png"/><!--ru-->
<!--ru-->
by `aws t2.medium (CPU: Intel(R) Xeon(R) CPU E5-2686 v4 @ 2.30GHz)` [more information](https://github.com/hora-search/ann-benchmarks)<!--ru-->
<!--ru-->
# Примеры<!--ru-->
<!--ru-->
**`Rust` Примеры** [[more info](https://github.com/hora-search/hora/tree/main/examples)]<!--ru-->
<!--ru-->
```Rust<!--ru-->
use hora::core::ann_index::ANNIndex;<!--ru-->
use rand::{thread_rng, Rng};<!--ru-->
use rand_distr::{Distribution, Normal};<!--ru-->
<!--ru-->
pub fn demo() {<!--ru-->
    let n = 1000;<!--ru-->
    let dimension = 64;<!--ru-->
<!--ru-->
    // make sample points<!--ru-->
    let mut samples = Vec::with_capacity(n);<!--ru-->
    let normal = Normal::new(0.0, 10.0).unwrap();<!--ru-->
    for _i in 0..n {<!--ru-->
        let mut sample = Vec::with_capacity(dimension);<!--ru-->
        for _j in 0..dimension {<!--ru-->
            sample.push(normal.sample(&mut rand::thread_rng()));<!--ru-->
        }<!--ru-->
        samples.push(sample);<!--ru-->
    }<!--ru-->
<!--ru-->
    // init index<!--ru-->
    let mut index = hora::index::hnsw_idx::HNSWIndex::<f32, usize>::new(<!--ru-->
        dimension,<!--ru-->
        &hora::index::hnsw_params::HNSWParams::<f32>::default(),<!--ru-->
    );<!--ru-->
    for (i, sample) in samples.iter().enumerate().take(n) {<!--ru-->
        // add point<!--ru-->
        index.add(sample, i).unwrap();<!--ru-->
    }<!--ru-->
    index.build(hora::core::metrics::Metric::Euclidean).unwrap();<!--ru-->
<!--ru-->
    let mut rng = thread_rng();<!--ru-->
    let target: usize = rng.gen_range(0..n);<!--ru-->
    // 523 has neighbors: [523, 762, 364, 268, 561, 231, 380, 817, 331, 246]<!--ru-->
    println!(<!--ru-->
        "{:?} has neighbors: {:?}",<!--ru-->
        target,<!--ru-->
        index.search(&samples[target], 10) // search for k nearest neighbors<!--ru-->
    );<!--ru-->
}<!--ru-->
```<!--ru-->
<!--ru-->
**`Python` Примеры** [[more info](https://github.com/hora-search/horapy)]<!--ru-->
<!--ru-->
```Python<!--ru-->
import numpy as np<!--ru-->
from horapy import HNSWIndex<!--ru-->
<!--ru-->
dimension = 50<!--ru-->
n = 1000<!--ru-->
<!--ru-->
# init index instance<!--ru-->
index = HNSWIndex(dimension, "usize")<!--ru-->
<!--ru-->
samples = np.float32(np.random.rand(n, dimension))<!--ru-->
for i in range(0, len(samples)):<!--ru-->
    # add node<!--ru-->
    index.add(np.float32(samples[i]), i)<!--ru-->
<!--ru-->
index.build("euclidean")  # build index<!--ru-->
<!--ru-->
target = np.random.randint(0, n)<!--ru-->
# 410 in Hora ANNIndex <HNSWIndexUsize> (dimension: 50, dtype: usize, max_item: 1000000, n_neigh: 32, n_neigh0: 64, ef_build: 20, ef_search: 500, has_deletion: False)<!--ru-->
# has neighbors: [410, 736, 65, 36, 631, 83, 111, 254, 990, 161]<!--ru-->
print("{} in {} \nhas neighbors: {}".format(<!--ru-->
    target, index, index.search(samples[target], 10)))  # search<!--ru-->
<!--ru-->
```<!--ru-->
<!--ru-->
**`JavaScript` Примеры** [[more info](https://github.com/hora-search/hora-wasm)]<!--ru-->
<!--ru-->
```JavaScript<!--ru-->
import * as horajs from "horajs";<!--ru-->
<!--ru-->
const demo = () => {<!--ru-->
    const dimension = 50;<!--ru-->
    var bf_idx = horajs.BruteForceIndexUsize.new(dimension);<!--ru-->
    // var hnsw_idx = horajs.HNSWIndexUsize.new(dimension, 1000000, 32, 64, 20, 500, 16, false);<!--ru-->
    for (var i = 0; i < 1000; i++) {<!--ru-->
        var feature = [];<!--ru-->
        for (var j = 0; j < dimension; j++) {<!--ru-->
            feature.push(Math.random());<!--ru-->
        }<!--ru-->
        bf_idx.add(feature, i); // add point<!--ru-->
    }<!--ru-->
    bf_idx.build("euclidean"); // build index<!--ru-->
    var feature = [];<!--ru-->
    for (var j = 0; j < dimension; j++) {<!--ru-->
        feature.push(Math.random());<!--ru-->
    }<!--ru-->
    console.log("bf result", bf_idx.search(feature, 10)); //bf result Uint32Array(10) [704, 113, 358, 835, 408, 379, 117, 414, 808, 826]<!--ru-->
}<!--ru-->
<!--ru-->
(async () => {<!--ru-->
    await horajs.default();<!--ru-->
    await horajs.init_env();<!--ru-->
    demo();<!--ru-->
})();<!--ru-->
```<!--ru-->
<!--ru-->
**`Java` Примеры** [[more info](https://github.com/hora-search/hora-java)]<!--ru-->
<!--ru-->
```Java<!--ru-->
public void demo() {<!--ru-->
    final int dimension = 2;<!--ru-->
    final float variance = 2.0f;<!--ru-->
    Random fRandom = new Random();<!--ru-->
<!--ru-->
    BruteForceIndex bruteforce_idx = new BruteForceIndex(dimension); // init index instance<!--ru-->
<!--ru-->
    List<float[]> tmp = new ArrayList<>();<!--ru-->
    for (int i = 0; i < 5; i++) {<!--ru-->
        for (int p = 0; p < 10; p++) {<!--ru-->
            float[] features = new float[dimension];<!--ru-->
            for (int j = 0; j < dimension; j++) {<!--ru-->
                features[j] = getGaussian(fRandom, (float) (i * 10), variance);<!--ru-->
            }<!--ru-->
            bruteforce_idx.add("bf", features, i * 10 + p); // add point<!--ru-->
            tmp.add(features);<!--ru-->
          }<!--ru-->
    }<!--ru-->
    bruteforce_idx.build("bf", "euclidean"); // build index<!--ru-->
<!--ru-->
    int search_index = fRandom.nextInt(tmp.size());<!--ru-->
    // nearest neighbor search<!--ru-->
    int[] result = bruteforce_idx.search("bf", 10, tmp.get(search_index));<!--ru-->
    // [main] INFO com.hora.app.ANNIndexTest  - demo bruteforce_idx[7, 8, 0, 5, 3, 9, 1, 6, 4, 2]<!--ru-->
    log.info("demo bruteforce_idx" + Arrays.toString(result));<!--ru-->
}<!--ru-->
<!--ru-->
private static float getGaussian(Random fRandom, float aMean, float variance) {<!--ru-->
    float r = (float) fRandom.nextGaussian();<!--ru-->
    return aMean + r * variance;<!--ru-->
}<!--ru-->
```<!--ru-->
<!--ru-->
# Дорожная карта<!--ru-->
<!--ru-->
- [ ] Полное тестовое покрытие<!--ru-->
- [ ] Внедрить алгоритм [EFANNA](http://arxiv.org/abs/1609.07228) для более быстрого построения графа KNN.<!--ru-->
- [ ] Поддержка`Swift` и пример развертывания `iOS` / `macOS`<!--ru-->
- [ ] Поддержка `R`<!--ru-->
- [ ] поддержка `mmap`<!--ru-->
<!--ru-->
# Связанные проекты и сравнение<!--ru-->
<!--ru-->
- [Faiss](https://github.com/facebookresearch/faiss), [Annoy](https://github.com/spotify/annoy), [ScaNN](https://github.com/google-research/google-research/tree/master/scann):<!--ru-->
<!--ru-->
  - **Реализация `Hora` сильно вдохновлена этими библиотеками.**<!--ru-->
  - `Faiss` больше ориентирован на сценарий GPU, а `Hora` легче, чем `Faiss`.<!--ru-->
  - `Hora` рассчитывает поддерживать больше языков, и все, что связано с производительностью, будет реализовано Rust🦀.<!--ru-->
  - `Annoy` поддерживает только алгоритм `LSH (Random Projection)`.<!--ru-->
  - `ScaNN` и `Faiss` менее удобны для пользователя (например, отсутствие документации).<!--ru-->
  - Hora is **ALL IN RUST** 🦀.<!--ru-->
<!--ru-->
- [Milvus](https://github.com/milvus-io/milvus), [Vald](https://github.com/vdaas/vald), [Jina AI](https://github.com/jina-ai/jina)<!--ru-->
  - `Milvus` и `Vald` также поддерживают несколько языков, но служат в качестве службы, а не библиотеки.<!--ru-->
  - `Milvus` построен на некоторых библиотеках, таких как `Faiss`, а `Hora` - это библиотека со всеми реализованными алгоритмами.<!--ru-->
<!--ru-->
# Способствовать<!--ru-->
<!--ru-->
**We appreciate your help!**<!--ru-->
<!--ru-->
Мы рады, что вы участвуете, приветствуются любые взносы, включая документацию и тесты.<!--ru-->
Вы можете создать `Pull Request` или `Issue` на GitHub, и мы рассмотрим его как можно скорее.<!--ru-->
<!--ru-->
Мы используем проблемы GitHub для отслеживания предложений и ошибок.<!--ru-->
<!--ru-->
#### Clone the repo<!--ru-->
<!--ru-->
```bash<!--ru-->
git clone https://github.com/hora-search/hora<!--ru-->
```<!--ru-->
<!--ru-->
#### Build<!--ru-->
<!--ru-->
```bash<!--ru-->
cargo build<!--ru-->
```<!--ru-->
<!--ru-->
#### Test<!--ru-->
<!--ru-->
```bash<!--ru-->
cargo test --lib<!--ru-->
```<!--ru-->
<!--ru-->
#### Try the changes<!--ru-->
<!--ru-->
```bash<!--ru-->
cd examples<!--ru-->
cargo run<!--ru-->
```<!--ru-->
<!--ru-->
# License<!--ru-->
<!--ru-->
The entire repository is licensed under the [Apache License](https://github.com/hora-search/hora/blob/main/LICENSE).<!--ru-->
