//! Simple HNSW demo: build index on random vectors and query k-NN.

use hora::core::ann_index::ANNIndex;
use rand::{thread_rng, Rng};
use rand_distr::{Distribution, Normal};

/// Builds an HNSW index on random Gaussian vectors and runs a sample k-NN query.
pub fn demo() {
    let n = 1_000_000;
    let dimension = 64;

    // Sample points from normal distribution
    let mut samples = Vec::with_capacity(n);
    let normal = Normal::new(0.0, 10.0).unwrap();
    for _i in 0..n {
        let mut sample = Vec::with_capacity(dimension);
        for _j in 0..dimension {
            sample.push(normal.sample(&mut rand::thread_rng()));
        }
        samples.push(sample);
    }

    // Build HNSW index
    let mut index = hora::index::hnsw_idx::HNSWIndex::<f32, usize>::new(
        dimension,
        &hora::index::hnsw_params::HNSWParams::<f32>::default(),
    );
    for (i, sample) in samples.iter().enumerate().take(n) {
        index.add(sample, i).unwrap();
    }
    index.build(hora::core::metrics::Metric::Euclidean).unwrap();

    // Query k nearest neighbors for a random point
    let mut rng = thread_rng();
    let target: usize = rng.gen_range(0..n);
    println!(
        "{:?} has neighbors: {:?}",
        target,
        index.search(&samples[target], 10)
    );
}
