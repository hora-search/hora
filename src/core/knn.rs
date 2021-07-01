#![allow(dead_code)]
use crate::core::metrics;
use crate::core::neighbor::Neighbor;
use crate::core::node::{FloatElement, IdxType, Node};
use fixedbitset::FixedBitSet;
use rand::seq::SliceRandom;
use rand::Rng;
use rayon::prelude::*;

use core::sync::mpsc;

use core::sync::{Arc, Mutex};
extern crate alloc;
use alloc::vec::Vec;
use alloc::boxed::Box;

pub fn naive_build_knn_graph<E: FloatElement, T: IdxType>(
    nodes: &[Box<Node<E, T>>],
    mt: metrics::Metric,
    k: usize,
    graph: &mut Vec<Vec<Neighbor<E, usize>>>, // TODO: not use this one
) {
    let tmp_graph = Arc::new(Mutex::new(graph));
    (0..nodes.len()).into_par_iter().for_each(|n| {
        let item = &nodes[n];
        let mut heap = BinaryHeap::with_capacity(k);
        (0..nodes.len()).for_each(|i| {
            if i == n {
                return;
            }
            heap.push(Neighbor::new(i, item.metric(&nodes[i], mt).unwrap()));
            if heap.len() > k {
                heap.pop();
            }
        });
        let mut tmp = Vec::with_capacity(heap.len());
        while !heap.is_empty() {
            tmp.push(heap.pop().unwrap());
        }

        tmp_graph.lock().unwrap()[n].clear();
        tmp_graph.lock().unwrap()[n] = tmp;
    });
}

pub struct NNDescentHandler<'a, E: FloatElement, T: IdxType> {
    nodes: &'a [Box<Node<E, T>>],
    graph: Vec<Arc<Mutex<BinaryHeap<Neighbor<E, usize>>>>>,
    mt: metrics::Metric,
    k: usize,
    visited_id: FixedBitSet,
    calculation_context: Vec<(Vec<usize>, Vec<usize>, Vec<usize>, Vec<usize>)>, // nn_new_neighbors, nn_old_neighbors, reversed_new_neighbors, reversed_old_neighbors
    rho: f32,
    cost: usize,
    s: usize,
    update_cnt: usize,
}

impl<'a, E: FloatElement, T: IdxType> NNDescentHandler<'a, E, T> {
    fn new(nodes: &'a [Box<Node<E, T>>], mt: metrics::Metric, k: usize, rho: f32) -> Self {
        NNDescentHandler {
            nodes,
            graph: Vec::new(), // TODO: as params
            mt,
            k,
            visited_id: FixedBitSet::with_capacity(nodes.len() * nodes.len()),
            calculation_context: Vec::new(),
            rho,
            cost: 0,
            s: (rho * k as f32) as usize,
            update_cnt: 0,
        }
    }

    fn update(
        &self,
        u1: usize,
        u2: usize,
        my_graph: &[Arc<Mutex<BinaryHeap<Neighbor<E, usize>>>>],
    ) -> bool {
        if u1 == u2 {
            return false;
        }

        self.update_nn_node(u1, u2, my_graph);
        self.update_nn_node(u2, u1, my_graph);
        true
    }

    fn update_nn_node(
        &self,
        me: usize,
        candidate: usize,
        my_graph: &[Arc<Mutex<BinaryHeap<Neighbor<E, usize>>>>],
    ) -> bool {
        let dist = self.nodes[me]
            .metric(&self.nodes[candidate], self.mt)
            .unwrap();
        if dist > my_graph[me].lock().unwrap().peek().unwrap().distance() {
            false
        } else {
            my_graph[me]
                .lock()
                .unwrap()
                .push(Neighbor::new(candidate, dist));
            if my_graph[me].lock().unwrap().len() > self.k {
                my_graph[me].lock().unwrap().pop();
            }
            true
        }
    }

    fn init(&mut self) {
        self.visited_id = FixedBitSet::with_capacity(self.nodes.len() * self.nodes.len());
        self.graph.clear();

        self.graph = (0..self.nodes.len())
            .into_par_iter()
            .map(|_i| {
                let mut v = BinaryHeap::with_capacity(self.k * 2);
                for _j in 0..self.k {
                    v.push(Neighbor::new(self.nodes.len(), E::max_value()));
                }
                Arc::new(Mutex::new(v))
            })
            .collect();

        self.calculation_context = (0..self.nodes.len())
            .into_par_iter()
            .map(|_i| {
                let mut nn_new_neighbors: Vec<usize> = Vec::with_capacity(self.s);
                let nn_old_neighbors: Vec<usize> = Vec::with_capacity(self.s);
                for _j in 0..self.s {
                    let rand_val = rand::thread_rng().gen_range(0..self.nodes.len());
                    nn_new_neighbors.push(rand_val);
                }

                let mut reversed_new_neighbors: Vec<usize> = Vec::with_capacity(self.s);
                let reversed_old_neighbors: Vec<usize> = Vec::with_capacity(self.s);
                for _j in 0..self.s {
                    let rand_val = rand::thread_rng().gen_range(0..self.nodes.len());
                    reversed_new_neighbors.push(rand_val);
                }
                (
                    nn_new_neighbors,
                    nn_old_neighbors,
                    reversed_new_neighbors,
                    reversed_old_neighbors,
                )
            })
            .collect();
    }

    fn iterate_nn(&self) -> (usize, FixedBitSet) {
        let my_graph = &self.graph;
        let length = self.nodes.len();
        // let (sender, receiver) = mpsc::channel();

        // cc += (0..self.nodes.len())
        self.calculation_context
            .par_iter()
            .map(
                |(
                    nn_new_neighbors,
                    nn_old_neighbors,
                    reversed_new_neighbors,
                    reversed_old_neighbors,
                )| {
                    let mut flags = FixedBitSet::with_capacity(length * length);
                    let mut ccc: usize = 0;
                    for j in 0..nn_new_neighbors.len() {
                        for k in j..nn_new_neighbors.len() {
                            if self.update(nn_new_neighbors[j], nn_new_neighbors[k], my_graph) {
                                ccc += 1;
                            }
                            flags.insert(nn_new_neighbors[j] * length + nn_new_neighbors[k]);
                            flags.insert(nn_new_neighbors[k] * length + nn_new_neighbors[j]);
                        }
                    }

                    nn_new_neighbors.iter().for_each(|j| {
                        nn_old_neighbors.iter().for_each(|k| {
                            if self.update(*j, *k, my_graph) {
                                ccc += 1;
                            }
                            flags.insert(j * length + k);
                            flags.insert(k * length + j);
                        })
                    });

                    for j in 0..reversed_new_neighbors.len() {
                        for k in j..reversed_new_neighbors.len() {
                            if reversed_new_neighbors[j] >= reversed_new_neighbors[k] {
                                continue;
                            }
                            if self.update(
                                reversed_new_neighbors[j],
                                reversed_new_neighbors[k],
                                my_graph,
                            ) {
                                ccc += 1;
                            }
                            flags.insert(
                                reversed_new_neighbors[j] * length + reversed_new_neighbors[k],
                            );
                            flags.insert(
                                reversed_new_neighbors[k] * length + reversed_new_neighbors[j],
                            );
                        }
                    }
                    reversed_new_neighbors.iter().for_each(|j| {
                        reversed_old_neighbors.iter().for_each(|k| {
                            if self.update(*j, *k, my_graph) {
                                ccc += 1;
                            }
                            flags.insert(j * length + k);
                            flags.insert(k * length + j);
                        })
                    });

                    nn_new_neighbors.iter().for_each(|j| {
                        reversed_old_neighbors.iter().for_each(|k| {
                            if self.update(*j, *k, my_graph) {
                                ccc += 1;
                            }
                            flags.insert(j * length + k);
                            flags.insert(k * length + j);
                        })
                    });

                    nn_new_neighbors.iter().for_each(|j| {
                        reversed_new_neighbors.iter().for_each(|k| {
                            if self.update(*j, *k, my_graph) {
                                ccc += 1;
                            }
                            flags.insert(j * length + k);
                            flags.insert(k * length + j);
                        })
                    });

                    nn_old_neighbors.iter().for_each(|j| {
                        reversed_new_neighbors.iter().for_each(|k| {
                            if self.update(*j, *k, my_graph) {
                                ccc += 1;
                            }
                            flags.insert(j * length + k);
                            flags.insert(k * length + j);
                        })
                    });
                    (ccc, flags)
                },
            )
            .reduce(
                || (0, FixedBitSet::with_capacity(length * length)),
                |(ccc1, mut flags1), (ccc2, flags2)| {
                    flags1.union_with(&flags2);
                    (ccc1 + ccc2, flags1)
                },
            )
    }

    fn iterate(&mut self) -> usize {
        self.update_cnt = 0;
        self.cost = 0;

        let (cc, flags) = self.iterate_nn();
        self.visited_id.union_with(&flags);

        //         s.send(flags).unwrap();
        //         ccc
        //     })
        //     .sum::<usize>();

        // receiver.iter().for_each(|flags| {
        //     flags.iter().for_each(|j| {
        //         self.visited_id.set(*j, true);
        //     });
        // });

        self.graph.par_iter().for_each(|graph| {
            while graph.lock().unwrap().len() > self.k {
                graph.lock().unwrap().pop();
            }
        });

        self.cost += cc;
        let mut t = 0;

        let (sender2, receiver2) = mpsc::channel();
        // let pending_status2: Vec<(usize, usize, Vec<usize>, Vec<usize>, Vec<usize>)> = (0..self
        //     .nodes
        //     .len())
        t += (0..self.nodes.len())
            .into_par_iter()
            .map_with(sender2, |s, i| {
                // .map(|i| {
                let mut nn_new_neighbors = Vec::with_capacity(self.graph[i].lock().unwrap().len());
                let mut nn_old_neighbors = Vec::with_capacity(self.graph[i].lock().unwrap().len());
                let mut flags = Vec::with_capacity(self.graph[i].lock().unwrap().len());
                let graph_item: Vec<Neighbor<E, usize>> =
                    self.graph[i].lock().unwrap().clone().into_vec();

                let mut tt: usize = 0;

                for (j, the_graph_item) in graph_item.iter().enumerate().take(self.k) {
                    if the_graph_item.idx() == self.nodes.len() {
                        // init value, pass
                        continue;
                    }
                    if self
                        .visited_id
                        .contains(self.nodes.len() * i + the_graph_item.idx())
                    {
                        nn_new_neighbors.push(j);
                    } else {
                        nn_old_neighbors.push(the_graph_item.idx());
                    }
                }

                tt += nn_new_neighbors.len();

                if nn_new_neighbors.len() > self.s {
                    let mut rng = rand::thread_rng();
                    nn_new_neighbors.shuffle(&mut rng);
                    nn_new_neighbors = nn_new_neighbors[self.s..].to_vec();
                }

                nn_new_neighbors.iter_mut().for_each(|j| {
                    flags.push(i * self.nodes.len() + graph_item[*j].idx());
                    *j = graph_item[*j].idx();
                });

                s.send((i, nn_new_neighbors, nn_old_neighbors, flags))
                    .unwrap();
                tt
                // (i, tt, nn_new_neighbors, nn_old_neighbors, flags)
            })
            // .collect();
            .sum::<usize>();

        // t += pending_status2
        //     .iter()
        //     .map(|(i, tt, nn_new_neighbors, nn_old_neighbors, flags)| {
        //         self.nn_new_neighbors[*i] = nn_new_neighbors.to_vec();
        //         self.nn_old_neighbors[*i] = nn_old_neighbors.to_vec();
        //         flags.iter().for_each(|j| {
        //             self.visited_id.set(*j, false);
        //         });
        //         tt
        //     })
        //     .sum::<usize>();

        receiver2
            .iter()
            .for_each(|(i, nn_new_neighbors, nn_old_neighbors, flags)| {
                self.calculation_context[i].0 = nn_new_neighbors;
                self.calculation_context[i].1 = nn_old_neighbors;
                flags.iter().for_each(|j| {
                    self.visited_id.set(*j, false);
                });
            });

        let reversed_new_neighbors = vec![Arc::new(Mutex::new(Vec::new())); self.nodes.len()];
        let reversed_old_neighbors = vec![Arc::new(Mutex::new(Vec::new())); self.nodes.len()];

        (0..self.nodes.len()).into_par_iter().for_each(|i| {
            for e in 0..self.calculation_context[i].1.len() {
                reversed_old_neighbors[self.calculation_context[i].1[e]]
                    .lock()
                    .unwrap()
                    .push(i);
            }
            for e in 0..self.calculation_context[i].0.len() {
                reversed_new_neighbors[self.calculation_context[i].0[e]]
                    .lock()
                    .unwrap()
                    .push(i);
            }
        });

        (0..self.nodes.len()).into_par_iter().for_each(|i| {
            let mut rng = rand::thread_rng();
            if reversed_old_neighbors[i].lock().unwrap().len() > self.s {
                reversed_old_neighbors[i].lock().unwrap().shuffle(&mut rng);
                reversed_old_neighbors[i].lock().unwrap().resize(self.s, 0);
            }
            if reversed_new_neighbors[i].lock().unwrap().len() > self.s {
                reversed_new_neighbors[i].lock().unwrap().shuffle(&mut rng);
                reversed_new_neighbors[i].lock().unwrap().resize(self.s, 0);
            }
        });

        (0..self.nodes.len()).for_each(|i| {
            self.calculation_context[i].2 = reversed_new_neighbors[i]
                .lock()
                .unwrap()
                .iter()
                .copied()
                .collect();
            self.calculation_context[i].3 = reversed_old_neighbors[i]
                .lock()
                .unwrap()
                .iter()
                .copied()
                .collect();
        });

        t
    }

    fn graph(&self) -> Vec<Vec<Neighbor<E, usize>>> {
        let mut graph: Vec<Vec<Neighbor<E, usize>>> = Vec::with_capacity(self.graph.len());
        for iter in self.graph.iter() {
            graph.push(iter.lock().unwrap().iter().cloned().collect());
        }
        graph
    }

    fn cost(&self) -> &usize {
        &self.cost
    }

    fn ths_update_cnt(&self) -> &usize {
        &self.update_cnt
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     use crate::core::node;
//     use rand::distributions::{Distribution, Standard};
//     use rand::Rng;
//     use std::collections::HashMap;
//     use std::collections::HashSet;

//     use std::iter::FromIterator;
//     use std::time::SystemTime;
//     fn make_normal_distribution_clustering(
//         clustering_n: usize,
//         node_n: usize,
//         dimension: usize,
//         range: f64,
//     ) -> (
//         Vec<Vec<f64>>, // center of cluster
//         Vec<Vec<f64>>, // cluster data
//     ) {
//         let mut bases: Vec<Vec<f64>> = Vec::new();
//         let mut ns: Vec<Vec<f64>> = Vec::new();
//         for _i in 0..clustering_n {
//             let mut rng = rand::thread_rng();
//             let mut base: Vec<f64> = Vec::with_capacity(dimension);
//             for _i in 0..dimension {
//                 let n: f64 = rng.gen::<f64>() * range; // base number
//                 base.push(n);
//             }

//             let v_iter: Vec<f64> = rng
//                 .sample_iter(&Standard)
//                 .take(dimension * node_n)
//                 .collect::<Vec<f64>>()
//                 .clone();
//             for _i in 0..node_n {
//                 let mut vec_item = Vec::with_capacity(dimension);
//                 for i in 0..dimension {
//                     let vv = v_iter[_i * dimension..(_i + 1) * dimension][i] + base[i]; // add normal distribution noise
//                     vec_item.push(vv);
//                 }
//                 ns.push(vec_item);
//             }
//             bases.push(base);
//         }

//         (bases, ns)
//     }

//     #[test]
//     fn knn_nn_descent() {
//         let dimension = 2;
//         let nodes_every_cluster = 10;
//         let node_n = 1000;
//         let (_, ns) =
//             make_normal_distribution_clustering(node_n, nodes_every_cluster, dimension, 10000000.0);

//         let mut data = Vec::new();
//         for i in 0..ns.len() {
//             data.push(Box::new(node::Node::new_with_idx(&ns[i], i)));
//         }

//         let mut graph: Vec<Vec<Neighbor<f64, usize>>> = vec![Vec::new(); data.len()];
//         let base_start = SystemTime::now();
//         naive_build_knn_graph::<f64, usize>(&data, metrics::Metric::Euclidean, 100, &mut graph);
//         let base_since_the_epoch = SystemTime::now()
//             .duration_since(base_start)
//             .expect("Time went backwards");
//         println!(
//             "test for {:?} times, base use {:?} millisecond",
//             ns.len(),
//             base_since_the_epoch.as_millis()
//         );

//         let base_start = SystemTime::now();
//         let mut nn_descent_handler =
//             NNDescentHandler::new(&data, metrics::Metric::Euclidean, 100, 0.2);
//         nn_descent_handler.init();

//         let try_times = 8;
//         let mut ground_truth: HashMap<usize, HashSet<usize>> = HashMap::new();
//         for i in 0..graph.len() {
//             ground_truth.insert(i, HashSet::from_iter(graph[i].iter().map(|x| x.idx())));
//         }
//         // let guard = pprof::ProfilerGuard::new(100).unwrap();
//         for _p in 0..try_times {
//             let cc = nn_descent_handler.iterate();
//             let mut error = 0;
//             for i in 0..nn_descent_handler.graph.len() {
//                 let nn_descent_handler_val: Vec<Neighbor<f64, usize>> = nn_descent_handler.graph[i]
//                     .lock()
//                     .unwrap()
//                     .iter()
//                     .cloned()
//                     .collect();
//                 for j in 0..nn_descent_handler_val.len() {
//                     if !ground_truth[&i].contains(&nn_descent_handler_val[j].idx()) {
//                         error += 1;
//                     }
//                 }
//             }
//             println!(
//                 "error {} /{:?} cc {:?} cost {:?} update_cnt {:?}",
//                 error,
//                 data.len() * 10,
//                 cc,
//                 nn_descent_handler.cost(),
//                 nn_descent_handler.ths_update_cnt(),
//             );
//         }
//         // if let Ok(report) = guard.report().build() {
//         //     let file = File::create("flamegraph.svg").unwrap();
//         //     report.flamegraph(file).unwrap();
//         // };

//         let base_since_the_epoch = SystemTime::now()
//             .duration_since(base_start)
//             .expect("Time went backwards");
//         println!(
//             "test for {:?} times, base use {:?} millisecond",
//             ns.len(),
//             base_since_the_epoch.as_millis()
//         );
//     }
// }
