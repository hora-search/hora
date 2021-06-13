use hora::core::metrics;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::distributions::Standard;
use rand::Rng;

fn make_normal_distribution_clustering(
    clustering_n: usize,
    node_n: usize,
    dimension: usize,
    range: f64,
) -> (
    Vec<Vec<f32>>, // center of cluster
    Vec<Vec<f32>>, // cluster data
) {
    let _rng = rand::thread_rng();

    let mut bases: Vec<Vec<f32>> = Vec::new();
    let mut ns: Vec<Vec<f32>> = Vec::new();
    for _i in 0..clustering_n {
        let mut rng = rand::thread_rng();
        let mut base: Vec<f32> = Vec::with_capacity(dimension);
        for _i in 0..dimension {
            let n: f64 = rng.gen::<f64>() * range; // base number
            base.push((n as f32));
        }

        let v_iter: Vec<f64> = rng
            .sample_iter(&Standard)
            .take(dimension * node_n)
            .collect::<Vec<f64>>()
            .clone();
        for _i in 0..node_n {
            let mut vec_item = Vec::with_capacity(dimension);
            for i in 0..dimension {
                let vv = (v_iter[_i * dimension..(_i + 1) * dimension][i] as f32) + base[i]; // add normal distribution noise
                vec_item.push(vv);
            }
            ns.push(vec_item);
        }
        bases.push(base);
    }

    (bases, ns)
}

fn metrics_dot_product_wrapper(nso: &[Vec<f32>]) {
    nso.iter().for_each(|n| {
        nso.iter().for_each(|m| {
            metrics::dot_product(n, m).unwrap();
        })
    })
}

fn metrics_euclidean_distance_wrapper(nso: &[Vec<f32>]) {
    nso.iter().for_each(|n| {
        nso.iter().for_each(|m| {
            metrics::euclidean_distance(n, m).unwrap();
        })
    })
}

fn metrics_manhattan_distance_wrapper(nso: &[Vec<f32>]) {
    nso.iter().for_each(|n| {
        nso.iter().for_each(|m| {
            metrics::manhattan_distance(n, m).unwrap();
        })
    })
}

fn metrics_benchmark(c: &mut Criterion) {
    let dimension = 64;
    let nodes_every_cluster = 10;
    let node_n = 50;
    let range = 100000.0;
    let (_, nso) =
        make_normal_distribution_clustering(node_n, nodes_every_cluster, dimension, range);

    c.bench_function("dot_product", |b| {
        b.iter(|| metrics_dot_product_wrapper(black_box(&nso)))
    });
    c.bench_function("euclidean", |b| {
        b.iter(|| metrics_euclidean_distance_wrapper(black_box(&nso)))
    });
    c.bench_function("manhattan", |b| {
        b.iter(|| metrics_manhattan_distance_wrapper(black_box(&nso)))
    });
}

criterion_group!(benches, metrics_benchmark);
criterion_main!(benches);
