<div align="center">
  <img src="asset/logo.svg" width="70%"/>
</div>

# Hora

**[[Homepage](http://horasearch.com/)]** **[[Document](https://horasearch.com/doc)]** **[[Examples](https://horasearch.com/doc/example.html)]**

**_Hora Search Everywhere!_**

**Hora** å®å¨åºäº **Rustð¦** å®ç°ï¼äºå®è¯æï¼**Rust** ç¡®å®éå¸¸éå¸¸å¿«ï¼å®å¨å¯ä»¥åª²ç¾ **C++** ï¼ä¸`Hora`ä½¿ç¨ **SIMD**è¿è¡äºå éï¼éåº¦éå¸¸å¿«â¡ï¸â¡ï¸â¡ï¸ï¼å·ä½éåº¦å¯ä»¥åèä¸é¢ç benchmark.

**Hora**, æ¥è¯­ä¸º **ãã»ãã**ï¼è¯»æ³å **[hÅlÉ]** ï¼æææ¯ **Wow**, **You see!** , **Look at that!** ã è¿ä¸ªåå­ççµææ¥èªæ¥æ¬èåæ­æ² **[ãå°ããªæã®ããã]( https://www.youtube.com/watch?v=u8EkSB9zSpE)** ã

# Demos

**ð© Face-Match [[online demo](https://horasearch.com/#Demos)], have a try!**

<div align="center">
  <img src="asset/demo3.gif" width="100%"/>
</div>

**ð· Dream wine comments search [[online demo](https://horasearch.com/#Demos)], have a try!**

<div align="center">
  <img src="asset/demo2.gif" width="100%"/>
</div>

# Features

- **Performant** â¡ï¸

  - **SIMD-Accelerated ([packed_simd](https://github.com/rust-lang/packed_simd))**
  - **Stable algorithm implementation**
  - **Multiple threads design**

- **Supports Multiple Languages** âï¸

  - `Python`
  - `Javascript`
  - `Java`
  - `Go` (WIP)
  - `Ruby` (WIP)
  - `Swift` (WIP)
  - `R` (WIP)
  - `Julia` (WIP)
  - **Can also be used as a service**

- **Supports Multiple Indexes** ð

  - `Hierarchical Navigable Small World Graph Index (HNSWIndex)` ([details](https://arxiv.org/abs/1603.09320))
  - `Satellite System Graph (SSGIndex)` ([details](https://arxiv.org/abs/1907.06146))
  - `Product Quantization Inverted File(PQIVFIndex)` ([details](https://lear.inrialpes.fr/pubs/2011/JDS11/jegou_searching_with_quantization.pdf))
  - `Random Projection Tree(RPTIndex)` (LSH, WIP)
  - `BruteForce (BruteForceIndex)` (naive implementation with SIMD)

- **Portable** ð¼
  - Supports `WebAssembly`
  - Supports `Windows`, `Linux` and `OS X`
  - Supports `IOS` and `Android` (WIP)
  - Supports `no_std` (WIP, partial)
  - **No** heavy dependencies, such as `BLAS`

- **Reliability** ð

  - `Rust` compiler secures all code
  - Memory managed by `Rust` for all language libraries such as `Python's`
  - Broad testing coverage

- **Supports Multiple Distances** ð§®

  - `Dot Product Distance`
    - ![equation](https://latex.codecogs.com/gif.latex?D%28x%2Cy%29%20%3D%20%5Csum%7B%28x*y%29%7D)
  - `Euclidean Distance`
    - ![equation](https://latex.codecogs.com/gif.latex?D%28x%2Cy%29%20%3D%20%5Csqrt%7B%5Csum%7B%28x-y%29%5E2%7D%7D)
  - `Manhattan Distance`
    - ![equation](https://latex.codecogs.com/gif.latex?D%28x%2Cy%29%20%3D%20%5Csum%7B%7C%28x-y%29%7C%7D)
  - `Cosine Similarity`
    - ![equation](https://latex.codecogs.com/gif.latex?D%28x%2Cy%29%20%3D%20%5Cfrac%7Bx%20*y%7D%7B%7C%7Cx%7C%7C*%7C%7Cy%7C%7C%7D)

- **Productive** â­
  - Well documented
  - Elegant, simple and easy to learn API

# Installation

**`Rust`**

in `Cargo.toml`

```toml
[dependencies]
hora = "0.1.1"
```

**`Python`**

```Bash
$ pip install horapy
```

**`Javascript (WebAssembly)`**

```Bash
$ npm i horajs
```

**`Building from source`**

```bash
$ git clone https://github.com/hora-search/hora
$ cargo build
```

# Benchmarks

<img src="asset/fashion-mnist-784-euclidean_10_euclidean.png"/>

by `aws t2.medium (CPU: Intel(R) Xeon(R) CPU E5-2686 v4 @ 2.30GHz)` [more information](https://github.com/hora-search/ann-benchmarks)

# Examples

**`Rust` example** [[more info](https://github.com/hora-search/hora/tree/main/examples)]

```Rust
use hora::core::ann_index::ANNIndex;
use rand::{thread_rng, Rng};
use rand_distr::{Distribution, Normal};

pub fn demo() {
    let n = 1000;
    let dimension = 64;

    // make sample points
    let mut samples = Vec::with_capacity(n);
    let normal = Normal::new(0.0, 10.0).unwrap();
    for _i in 0..n {
        let mut sample = Vec::with_capacity(dimension);
        for _j in 0..dimension {
            sample.push(normal.sample(&mut rand::thread_rng()));
        }
        samples.push(sample);
    }

    // init index
    let mut index = hora::index::hnsw_idx::HNSWIndex::<f32, usize>::new(
        dimension,
        &hora::index::hnsw_params::HNSWParams::<f32>::default(),
    );
    for (i, sample) in samples.iter().enumerate().take(n) {
        // add point
        index.add(sample, i).unwrap();
    }
    index.build(hora::core::metrics::Metric::Euclidean).unwrap();

    let mut rng = thread_rng();
    let target: usize = rng.gen_range(0..n);
    // 523 has neighbors: [523, 762, 364, 268, 561, 231, 380, 817, 331, 246]
    println!(
        "{:?} has neighbors: {:?}",
        target,
        index.search(&samples[target], 10) // search for k nearest neighbors
    );
}
```

æè°¢ @vaaaaanquish è¿ä¸ªå®æ´ççº¯ `Rust ð¦` å¾çæ£ç´¢ [example](https://github.com/vaaaaanquish/rust-ann-search-example), æ³äºè§£æ´å¤å¯ä»¥ç¹å» [Pure Rustãªè¿ä¼¼æè¿åæ¢ç´¢ã©ã¤ãã©ãªhoraãç¨ããç»åæ¤ç´¢ãå®è£ãã](https://vaaaaaanquish.hatenablog.com/entry/2021/08/10/065117)

**`Python` example** [[more info](https://github.com/hora-search/horapy)]

```Python
import numpy as np
from horapy import HNSWIndex

dimension = 50
n = 1000

# init index instance
index = HNSWIndex(dimension, "usize")

samples = np.float32(np.random.rand(n, dimension))
for i in range(0, len(samples)):
    # add node
    index.add(np.float32(samples[i]), i)

index.build("euclidean")  # build index

target = np.random.randint(0, n)
# 410 in Hora ANNIndex <HNSWIndexUsize> (dimension: 50, dtype: usize, max_item: 1000000, n_neigh: 32, n_neigh0: 64, ef_build: 20, ef_search: 500, has_deletion: False)
# has neighbors: [410, 736, 65, 36, 631, 83, 111, 254, 990, 161]
print("{} in {} \nhas neighbors: {}".format(
    target, index, index.search(samples[target], 10)))  # search

```

**`JavaScript` example** [[more info](https://github.com/hora-search/hora-wasm)]

```JavaScript
import * as horajs from "horajs";

const demo = () => {
    const dimension = 50;
    var bf_idx = horajs.BruteForceIndexUsize.new(dimension);
    // var hnsw_idx = horajs.HNSWIndexUsize.new(dimension, 1000000, 32, 64, 20, 500, 16, false);
    for (var i = 0; i < 1000; i++) {
        var feature = [];
        for (var j = 0; j < dimension; j++) {
            feature.push(Math.random());
        }
        bf_idx.add(feature, i); // add point 
    }
    bf_idx.build("euclidean"); // build index
    var feature = [];
    for (var j = 0; j < dimension; j++) {
        feature.push(Math.random());
    }
    console.log("bf result", bf_idx.search(feature, 10)); //bf result Uint32Array(10) [704, 113, 358, 835, 408, 379, 117, 414, 808, 826]
}

(async () => {
    await horajs.default();
    await horajs.init_env();
    demo();
})();
```

**`Java` example** [[more info](https://github.com/hora-search/hora-java)]

```Java
public void demo() {
    final int dimension = 2;
    final float variance = 2.0f;
    Random fRandom = new Random();

    BruteForceIndex bruteforce_idx = new BruteForceIndex(dimension); // init index instance

    List<float[]> tmp = new ArrayList<>();
    for (int i = 0; i < 5; i++) {
        for (int p = 0; p < 10; p++) {
            float[] features = new float[dimension];
            for (int j = 0; j < dimension; j++) {
                features[j] = getGaussian(fRandom, (float) (i * 10), variance);
            }
            bruteforce_idx.add("bf", features, i * 10 + p); // add point
            tmp.add(features);
          }
    }
    bruteforce_idx.build("bf", "euclidean"); // build index

    int search_index = fRandom.nextInt(tmp.size());
    // nearest neighbor search
    int[] result = bruteforce_idx.search("bf", 10, tmp.get(search_index));
    // [main] INFO com.hora.app.ANNIndexTest  - demo bruteforce_idx[7, 8, 0, 5, 3, 9, 1, 6, 4, 2]
    log.info("demo bruteforce_idx" + Arrays.toString(result));
}

private static float getGaussian(Random fRandom, float aMean, float variance) {
    float r = (float) fRandom.nextGaussian();
    return aMean + r * variance;
}
```

# Roadmap

- [ ] Full test coverage
- [ ] Implement [EFANNA](http://arxiv.org/abs/1609.07228) algorithm to achieve faster KNN graph building
- [ ] Swift support and iOS/macOS deployment example
- [ ] Support `R`
- [ ] support `mmap`

# Related Projects and Comparison

- [Faiss](https://github.com/facebookresearch/faiss), [Annoy](https://github.com/spotify/annoy), [ScaNN](https://github.com/google-research/google-research/tree/master/scann):
  - **`Hora` çå®ç°åå°è¿äºåºçå¼ºçå¯åã**
  - `Faiss` æ´ä¾§éäº GPU åºæ¯ï¼`Hora` æ¯ Faiss æ´è½»ï¼**æ éåº¦ä¾èµï¼**ã
  - `Hora` æå¾æ¯ææ´å¤çè¯­è¨ï¼ä¸æ§è½ç¸å³çä¸åé½ä¼ç± Rustð¦ å®ç°ã
  - `Annoy` åªæ¯æ `LSH (Random Projection)` ç®æ³ã
  - `ScaNN` å `Faiss` ä¸å¤ªç¨æ·åå¥½ï¼ï¼ä¾å¦ç¼ºä¹ææ¡£ï¼ã
  - Hora is **ALL IN RUST** ð¦.

- [Milvus](https://github.com/milvus-io/milvus), [Vald](https://github.com/vdaas/vald), [Jina AI](https://github.com/jina-ai/jina)
  - `Milvus` å `Vald` ä¹æ¯æå¤ç§è¯­è¨ï¼ä½ä½ä¸ºæå¡èä¸æ¯åº
  - `Milvus` æ¯å»ºç«å¨ä¸äºåºä¸çï¼æ¯å¦ `Faiss`ï¼è `Hora` æ¯ä¸ä¸ªåºï¼ææç®æ³é½æ¯èªå·±å®ç°ç

# Contribute

**We appreciate your help!**

æä»¬å¾é«å´æ¨çåä¸ï¼æ¬¢è¿ä»»ä½è´¡ç®ï¼åæ¬ææ¡£åæµè¯ã
æ¨å¯ä»¥å¨ GitHub ä¸åå»º `Pull Request` æ `Issue`ï¼æä»¬ä¼å°½å¿«å®¡æ ¸ã

æä»¬ä½¿ç¨ GitHub é®é¢æ¥è·è¸ªå»ºè®®åéè¯¯ã

#### Clone the repo

```bash
git clone https://github.com/hora-search/hora
```

#### Build

```bash
cargo build
```

#### Test

```bash
cargo test --lib
```

#### Try the changes

```bash
cd examples
cargo run
```

# License

The entire repository is licensed under the [Apache License](https://github.com/hora-search/hora/blob/main/LICENSE).
