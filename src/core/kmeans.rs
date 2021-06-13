#![allow(dead_code)]
use crate::core::metrics;
use crate::core::node;
use metrics::metric;
use rand::prelude::*;
use rayon::prelude::*;
use std::sync::Mutex;

#[derive(Default, Debug)]
pub struct Kmeans<E: node::FloatElement> {
    _dimension: usize,
    _n_center: usize,
    _centers: Vec<Vec<E>>,
    _data_range_begin: usize,
    _data_range_end: usize,
    _has_residual: bool,
    _residual: Vec<E>,
    mt: metrics::Metric, //compute metrics
}

impl<E: node::FloatElement> Kmeans<E> {
    pub fn new(dimension: usize, n_center: usize, mt: metrics::Metric) -> Kmeans<E> {
        Kmeans {
            _dimension: dimension,
            _n_center: n_center,
            _data_range_begin: 0,
            _data_range_end: dimension,
            mt,
            ..Default::default()
        }
    }

    pub fn centers(&self) -> &Vec<Vec<E>> {
        &self._centers
    }

    pub fn get_distance_from_vec(&self, x: &[E], y: &[E]) -> E {
        let mut z = x[self._data_range_begin..self._data_range_end].to_vec();
        if self._has_residual {
            (0..self._data_range_end - self._data_range_begin)
                .for_each(|i| z[i] -= self._residual[i + self._data_range_begin]);
        }
        return metric(&z, y, self.mt).unwrap();
    }

    pub fn set_residual(&mut self, residual: Vec<E>) {
        self._has_residual = true;
        self._residual = residual;
    }

    pub fn init_center(&mut self, batch_size: usize, batch_data: &[Vec<E>]) {
        let dimension = self._dimension;
        let n_center = self._n_center;
        let begin = self._data_range_begin;
        let mut mean_center: Vec<E> = vec![E::from_f32(0.0).unwrap(); dimension];

        (0..batch_size).for_each(|i| {
            let cur_data = &batch_data[i];
            (0..dimension).for_each(|j| {
                if self._has_residual {
                    mean_center[j] += cur_data[begin + j] - self._residual[begin + j];
                } else {
                    mean_center[j] += cur_data[begin + j];
                }
            });
        });

        (0..dimension).for_each(|i| {
            mean_center[i] /= E::from_usize(batch_size).unwrap();
        });

        let mut new_centers: Vec<Vec<E>> = Vec::with_capacity(n_center);
        (0..n_center).for_each(|i| {
            let mut cur_center: Vec<E> = Vec::new();
            (0..dimension).for_each(|j| {
                let mut val = mean_center[j];
                if i & (1 << j) == 1 {
                    val += E::from_f32(1.0).unwrap();
                } else {
                    val -= E::from_f32(1.0).unwrap();
                }
                cur_center.push(val);
            });
            new_centers.push(cur_center);
        });
        self._centers = new_centers;
    }

    pub fn update_center(
        &mut self,
        batch_size: usize,
        batch_data: &[Vec<E>],
        assigned_center: &[usize],
    ) -> Vec<usize> {
        let dimension = self._dimension;
        let n_center = self._n_center;
        let begin = self._data_range_begin;
        let mut new_centers: Vec<Vec<E>> = Vec::with_capacity(n_center);
        (0..n_center).for_each(|_| {
            new_centers.push(vec![E::from_f32(0.0).unwrap(); dimension]);
        });
        let mut n_assigned_per_center: Vec<usize> = vec![0; n_center];
        (0..batch_size).for_each(|i| {
            let cur_data = &batch_data[i];
            let cur_center = assigned_center[i];
            n_assigned_per_center[cur_center] += 1;
            (0..dimension).for_each(|j| {
                if self._has_residual {
                    new_centers[cur_center][j] += cur_data[begin + j] - self._residual[begin + j];
                } else {
                    new_centers[cur_center][j] += cur_data[begin + j];
                }
            });
        });

        (0..n_center).for_each(|i| {
            if n_assigned_per_center[i] == 0 {
                return;
            }
            (0..dimension).for_each(|j| {
                new_centers[i][j] /= E::from_usize(n_assigned_per_center[i]).unwrap();
            });
        });
        self._centers = new_centers;
        n_assigned_per_center
    }

    pub fn search_data(
        &mut self,
        batch_size: usize,
        batch_data: &Vec<Vec<E>>,
        assigned_center: &mut Vec<usize>,
    ) {
        let n_center = self._n_center;
        let _dimension = self._dimension;
        (0..batch_size).for_each(|i| {
            let mut nearist_center_id: usize = 0;
            (1..n_center).for_each(|j| {
                let cur_center = &self._centers[j];
                let nearist_center = &self._centers[nearist_center_id];
                if self.get_distance_from_vec(&batch_data[i], cur_center)
                    < self.get_distance_from_vec(&batch_data[i], nearist_center)
                {
                    nearist_center_id = j;
                }
            });
            assigned_center.push(nearist_center_id);
        });
    }

    pub fn split_center(
        &mut self,
        batch_size: usize,
        n_assigned_per_center: &mut Vec<usize>,
    ) -> Result<(), &'static str> {
        let dimension = self._dimension;
        let n_center = self._n_center;

        if batch_size == 0 {
            return Err("None to assigned impossible split center");
        }

        (0..n_center).for_each(|i| {
            if n_assigned_per_center[i] == 0 {
                //rand pick split center
                let mut split_center_id = (i + 1) % n_center;
                loop {
                    let mut rng = rand::thread_rng();
                    let pick_percent =
                        n_assigned_per_center[split_center_id] as f64 / batch_size as f64;
                    if rng.gen_range(0.0..1.0) < pick_percent {
                        break;
                    }
                    split_center_id = (split_center_id + 1) % n_center;
                }
                const EPS: f32 = 1.0 / 1024.0;
                (0..dimension).for_each(|j| {
                    if j % 2 == 0 {
                        self._centers[i][j] =
                            self._centers[split_center_id][j] * E::from_f32(1.0 - EPS).unwrap();
                        self._centers[split_center_id][j] *= E::from_f32(1.0 + EPS).unwrap();
                    } else {
                        self._centers[i][j] =
                            self._centers[split_center_id][j] * E::from_f32(1.0 + EPS).unwrap();
                        self._centers[split_center_id][j] *= E::from_f32(1.0 - EPS).unwrap();
                    }
                });
                n_assigned_per_center[i] = n_assigned_per_center[split_center_id] / 2;
                n_assigned_per_center[split_center_id] -= n_assigned_per_center[i];
            }
        });
        Ok(())
    }

    pub fn train(&mut self, batch_size: usize, batch_data: &Vec<Vec<E>>, n_epoch: usize) {
        self.init_center(batch_size, batch_data);
        (0..n_epoch).for_each(|epoch| {
            let mut assigned_center: Vec<usize> = Vec::with_capacity(batch_size);
            self.search_data(batch_size, batch_data, &mut assigned_center);
            let mut n_assigned_per_center =
                self.update_center(batch_size, batch_data, &assigned_center);
            if epoch < n_epoch - 1 {
                self.split_center(batch_size, &mut n_assigned_per_center)
                    .unwrap();
            }
        });
    }

    pub fn set_range(&mut self, begin: usize, end: usize) {
        assert!(end - begin == self._dimension);
        self._data_range_begin = begin;
        self._data_range_end = end;
    }
}

pub fn general_kmeans<E: node::FloatElement, T: node::IdxType>(
    k: usize,
    epoch: usize,
    nodes: &[Box<node::Node<E, T>>],
    mt: metrics::Metric,
) -> Vec<usize> {
    if nodes.is_empty() {
        return Vec::new();
    }

    let mut rng = rand::thread_rng();
    let mut means = Vec::with_capacity(k);

    (0..k).for_each(|_i| {
        means.push(Box::new(nodes[rng.gen_range(0..nodes.len())].clone()));
    });

    (0..epoch).for_each(|_| {
        let cluster_count: Vec<Mutex<usize>> = (0..k).map(|_| Mutex::new(0)).collect();
        let mut cluster_features: Vec<Mutex<Vec<E>>> = (0..k)
            .map(|_| Mutex::new(vec![E::zero(); nodes[0].vectors().len()]))
            .collect();
        nodes.par_iter().zip(0..nodes.len()).for_each(|(node, _j)| {
            let mut idx = 0;
            let mut distance = E::max_value();
            for i in 0..means.len() {
                let _distance = node.metric(&means[i], mt).unwrap();
                if _distance < distance {
                    idx = i;
                    distance = _distance;
                }
            }
            cluster_features[idx]
                .lock()
                .unwrap()
                .iter_mut()
                .zip(node.vectors())
                .for_each(|(i, j)| *i += *j);
            *cluster_count[idx].lock().unwrap() += 1;
        });

        cluster_features
            .iter_mut()
            .zip(cluster_count)
            .for_each(|(features, cnt)| {
                features
                    .lock()
                    .unwrap()
                    .iter_mut()
                    .for_each(|f| *f /= E::from_usize(*cnt.lock().unwrap()).unwrap())
            });

        means
            .iter_mut()
            .zip(cluster_features)
            .for_each(|(mean, features)| mean.set_vectors(&features.lock().unwrap()));
    });

    means
        .iter()
        .map(|mean| {
            let mut mean_idx = 0;
            let mut mean_distance = E::max_value();
            nodes.iter().zip(0..nodes.len()).for_each(|(node, i)| {
                let distance = node.metric(&mean, mt).unwrap();
                if distance < mean_distance {
                    mean_idx = i;
                    mean_distance = distance;
                }
            });
            mean_idx
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

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
                base.push(n as f32);
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

    #[test]
    fn test_general_kmeans() {
        let dimension = 2;
        let nodes_every_cluster = 10;
        let node_n = 10;
        let (_, nso) =
            make_normal_distribution_clustering(node_n, nodes_every_cluster, dimension, 100000.0);
        println!("{:?}", nso);
        let ns: Vec<Vec<f32>> = nso
            .iter()
            .map(|x| x.iter().map(|p| *p as f32).collect())
            .collect();

        let nodes: Vec<Box<node::Node<f32, usize>>> = ns
            .iter()
            .zip(0..ns.len())
            .map(|(vs, idx)| Box::new(node::Node::new_with_idx(vs, idx)))
            .collect();
        println!(
            "{:?}",
            general_kmeans(node_n, 30, &nodes, metrics::Metric::Euclidean)
        );
    }
}
