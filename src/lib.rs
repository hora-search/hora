pub mod core;
pub mod index;

#[cfg(test)]
mod tests {
    use super::*;

    use crate::core::ann_index::ANNIndex;
    use rand::distributions::Standard;
    use rand::Rng;
    use std::collections::HashSet;

    use std::sync::Arc;
    use std::sync::Mutex;
    fn make_normal_distribution_clustering(
        clustering_n: usize,
        node_n: usize,
        dimension: usize,
        range: f64,
    ) -> (
        Vec<Vec<f64>>, // center of cluster
        Vec<Vec<f64>>, // cluster data
    ) {
        let mut bases: Vec<Vec<f64>> = Vec::new();
        let mut ns: Vec<Vec<f64>> = Vec::new();
        for _i in 0..clustering_n {
            let mut rng = rand::thread_rng();
            let mut base: Vec<f64> = Vec::with_capacity(dimension);
            for _i in 0..dimension {
                let n: f64 = rng.gen::<f64>() * range; // base number
                base.push(n);
            }

            let v_iter: Vec<f64> = rng
                .sample_iter(&Standard)
                .take(dimension * node_n)
                .collect::<Vec<f64>>()
                .clone();
            for _i in 0..node_n {
                let mut vec_item = Vec::with_capacity(dimension);
                for i in 0..dimension {
                    let vv = v_iter[_i * dimension..(_i + 1) * dimension][i] + base[i]; // add normal distribution noise
                    vec_item.push(vv);
                }
                ns.push(vec_item);
            }
            bases.push(base);
        }

        (bases, ns)
    }

    #[test]
    fn test_all_index() {
        let dimension = 10;
        let nodes_every_cluster = 3;
        let node_n = 5000;

        let (_, ns) =
            make_normal_distribution_clustering(node_n, nodes_every_cluster, dimension, 100.0);
        let mut bf_idx = Box::new(index::bruteforce_idx::BruteForceIndex::<f64, usize>::new(
            dimension,
            &index::bruteforce_params::BruteForceParams::default(),
        ));
        // let bpt_idx = Box::new(
        //     index::bpt_idx::BPTIndex::<f64, usize>::new(dimension, &index::bpt_params::BPTParams::default()),
        // );
        let hnsw_idx = Box::new(index::hnsw_idx::HNSWIndex::<f64, usize>::new(
            dimension,
            &index::hnsw_params::HNSWParams::<f64>::default(),
        ));

        let pq_idx = Box::new(index::pq_idx::PQIndex::<f64, usize>::new(
            dimension,
            &index::pq_params::PQParams::<f64>::default(),
        ));
        let ssg_idx = Box::new(index::ssg_idx::SSGIndex::<f64, usize>::new(
            dimension,
            &index::ssg_params::SSGParams::default(),
        ));

        let mut indices: Vec<Box<dyn core::ann_index::ANNIndex<f64, usize>>> =
            vec![pq_idx, ssg_idx, hnsw_idx];
        let accuracy = Arc::new(Mutex::new(Vec::new()));
        for i in 0..indices.len() {
            make_idx_baseline(ns.clone(), &mut indices[i]);
            accuracy.lock().unwrap().push(0.);
        }
        make_idx_baseline(ns.clone(), &mut bf_idx);
        let test_time = 10;
        for _i in 0..test_time {
            let mut rng = rand::thread_rng();

            let target: usize = rng.gen_range(0..ns.len());
            let w = ns.get(target).unwrap();

            let base_set: HashSet<usize> = bf_idx
                .search_nodes(&w, 100)
                .iter()
                .map(|(n, _dist)| n.idx().unwrap())
                .collect();

            for j in 0..indices.len() {
                accuracy.lock().unwrap()[j] = 0.0;
                let result = indices[j].search_nodes(&w, 100);
                for (n, _dist) in result.iter() {
                    if base_set.contains(&n.idx().unwrap()) {
                        accuracy.lock().unwrap()[j] += 1.0;
                    }
                }
            }
        }
    }

    fn make_idx_baseline<
        E: core::node::FloatElement,
        T: core::ann_index::ANNIndex<E, usize> + ?Sized,
    >(
        embs: Vec<Vec<E>>,
        idx: &mut Box<T>,
    ) {
        for i in 0..embs.len() {
            idx.add_node(&core::node::Node::<E, usize>::new_with_idx(&embs[i], i))
                .unwrap();
        }
        idx.build(core::metrics::Metric::Euclidean).unwrap();
    }
}
