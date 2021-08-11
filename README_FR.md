<div align="center">
  <img src="asset/logo.svg" width="70%"/>
</div>

# Hora

**[[Homepage](http://horasearch.com/)]** **[[Document](https://horasearch.com/doc)]** **[[Examples](https://horasearch.com/doc/example.html)]**

**_Hora Search Everywhere!_**

Hora est un **algorithme de recherche du voisin le plus proche approximatif** ([wiki](https://en.wikipedia.org/wiki/Nearest_neighbor_search)). Nous implémentons tout le code dans `Rust🦀` pour une fiabilité, une abstraction de haut niveau et des vitesses élevées comparables à `C++`.

Hora, **`「ほら」`** en japonais, sonne comme `[hōlə]`, et signifie `Wow`, `Vous voyez !` ou ` Regardez ça ! `. Le nom est inspiré d'une célèbre chanson japonaise **`「小さな恋のうた」`**.

# Démos

**👩 Face-Match [[online demo](https://horasearch.com/#Demos)], Essaye!**

<div align="center">
  <img src="asset/demo3.gif" width="100%"/>
</div>

**🍷 Recherche de commentaires sur le vin de rêve [[online demo](https://horasearch.com/#Demos)], Essaye!**

<div align="center">
  <img src="asset/demo2.gif" width="100%"/>
</div>

# Principales caractéristiques

- **Performant** ⚡️

  - **SIMD-Accelerated ([packed_simd](https://github.com/rust-lang/packed_simd))**
  - **Implémentation d'algorithme stable**
  - **Multiple threads design**

- **Prend en charge plusieurs langages de programmation Lib** ☄️

  - `Python`
  - `Javascript`
  - `Java`
  - `Go` (WIP)
  - `Ruby` (WIP)
  - `Swift` (WIP)
  - `R` (WIP)
  - `Julia` (WIP)
  - **Peut également être utilisé comme un service**

- **Prend en charge plusieurs index** 🚀

  - `Hierarchical Navigable Small World Graph Index (HNSWIndex)` ([details](https://arxiv.org/abs/1603.09320))
  - `Satellite System Graph (SSGIndex)` ([details](https://arxiv.org/abs/1907.06146))
  - `Product Quantization Inverted File(PQIVFIndex)` ([details](https://lear.inrialpes.fr/pubs/2011/JDS11/jegou_searching_with_quantization.pdf))
  - `Random Projection Tree(RPTIndex)` (LSH, WIP)
  - `BruteForce (BruteForceIndex)` (naive implementation with SIMD)

- **Portable** 💼

  - Supports `WebAssembly`
  - Supports `Windows`, `Linux` and `OS X`
  - Supports `IOS` and `Android` (WIP)
  - Supports `no_std` (WIP, partial)
  - Pas de dépendances lourdes, telles que `BLAS`

- **Fiabilité** 🔒

  - Le compilateur `Rust` sécurise tout le code
  - Mémoire gérée par `Rust` pour toutes les bibliothèques de langage telles que `Python's`
  - Large couverture de test

- **Prend en charge plusieurs distances** 🧮

  - `Distance du produit de point`
    - ![equation](https://latex.codecogs.com/gif.latex?D%28x%2Cy%29%20%3D%20%5Csum%7B%28x*y%29%7D)
  - `Distance euclidienne`
    - ![equation](https://latex.codecogs.com/gif.latex?D%28x%2Cy%29%20%3D%20%5Csqrt%7B%5Csum%7B%28x-y%29%5E2%7D%7D)
  - `Distance de Manhattan`
    - ![equation](https://latex.codecogs.com/gif.latex?D%28x%2Cy%29%20%3D%20%5Csum%7B%7C%28x-y%29%7C%7D)
  - `Similitude de cosinus`
    - ![equation](https://latex.codecogs.com/gif.latex?D%28x%2Cy%29%20%3D%20%5Cfrac%7Bx%20*y%7D%7B%7C%7Cx%7C%7C*%7C%7Cy%7C%7C%7D)

- **Productive** ⭐
  - Bien documenté
  - API élégante, simple et facile à apprendre

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

# Repères

<img src="asset/fashion-mnist-784-euclidean_10_euclidean.png"/>

by `aws t2.medium (CPU: Intel(R) Xeon(R) CPU E5-2686 v4 @ 2.30GHz)` [more information](https://github.com/hora-search/ann-benchmarks)

# Exemples

**`Rust` exemple** [[more info](https://github.com/hora-search/hora/tree/main/examples)]

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

et merci @vaaaaanquish pour cette recherche complète d'images de rouille pure [exemple](https://github.com/vaaaaanquish/rust-ann-search-example), Pour plus d'informations sur cet exemple, vous pouvez cliquer sur [Pure Rustな近似最近 horaを用いた画像検索を実装する](https://vaaaaaanquish.hatenablog.com/entry/2021/08/10/065117)

**`Python` exemple** [[more info](https://github.com/hora-search/horapy)]

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

**`JavaScript` exemple** [[more info](https://github.com/hora-search/hora-wasm)]

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

**`Java` exemple** [[more info](https://github.com/hora-search/hora-java)]

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

# Feuille de route

- [ ] Couverture complète des tests
- [ ] Implémentez l'algorithme [EFANNA](http://arxiv.org/abs/1609.07228) pour obtenir une création de graphes KNN plus rapide
- [ ] Prise en charge Swift et exemple de déploiement iOS/macOS
- [ ] Support `R`
- [ ] support `mmap`

# Projets connexes et comparaison

- [Faiss](https://github.com/facebookresearch/faiss), [Annoy](https://github.com/spotify/annoy), [ScaNN](https://github.com/google-research/google-research/tree/master/scann):

  - **L'implémentation de `Hora` est fortement inspirée de ces bibliothèques.**
  - `Faiss` se concentre davantage sur la scène GPU, et `Hora` est plus léger que Faiss (**pas de dépendances lourdes)**.
  - `Hora` s'attend à prendre en charge plus de langues, et tout ce qui concerne les performances sera implémenté par Rust🦀.
  - `Annoy` ne prend en charge que l'algorithme `LSH (Random Projection)`.
  - `ScaNN` et `Faiss` sont moins conviviaux (par exemple, manque de documentation).
  - Hora is **ALL IN RUST** 🦀.

- [Milvus](https://github.com/milvus-io/milvus), [Vald](https://github.com/vdaas/vald), [Jina AI](https://github.com/jina-ai/jina)
  - `Milvus` et `Vald` prennent également en charge plusieurs langues, mais servent de service au lieu d'une bibliothèque
  - `Milvus` est construit sur certaines bibliothèques telles que `Faiss`, tandis que `Hora` est une bibliothèque avec tous les algorithmes implémentés elle-même

# Contribute

**Nous apprécions votre aide!**

Nous sommes ravis de votre participation, toutes les contributions sont les bienvenues, y compris les documentations et les tests.
Vous pouvez créer une `Pull Request` ou un `Issue` sur GitHub, et nous l'examinerons dès que possible.

Nous utilisons les problèmes GitHub pour suivre les suggestions et les bogues.

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
