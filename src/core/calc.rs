use crate::core::node::FloatElement;

pub fn get_norm<T>(vec1: &[T]) -> Result<T, &'static str>
where
    T: FloatElement,
{
    match dot(vec1, vec1) {
        Ok(val) => Ok(val.sqrt()),
        Err(err) => Err(err),
    }
}

pub fn dot<T>(vec1: &[T], vec2: &[T]) -> Result<T, &'static str>
where
    T: FloatElement,
{
    T::dot_product(vec1, vec2)
}

#[inline(always)]
pub fn same_dimension<T>(vec1: &[T], vec2: &[T]) -> Result<(), &'static str>
where
    T: FloatElement,
{
    if vec1.len() != vec2.len() {
        return Result::Err("different dimensions");
    }
    Result::Ok(())
}

pub fn split_imbalance<T>(vec1: &[T], vec2: &[T]) -> f64 {
    let ls = vec1.len() as f64;
    let rs = vec2.len() as f64;
    let f = ls / (ls + rs + 1e-9);
    if f > (1.0 - f) {
        f
    } else {
        1.0 - f
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::simd_metrics::SIMDOptmized;

    use rand::distributions::Standard;

    use rand::Rng;
    use std::time::SystemTime;
    fn make_normal_distribution_clustering(
        clustering_n: usize,
        node_n: usize,
        dimension: usize,
        range: f64,
    ) -> (
        Vec<Vec<f64>>, // center of cluster
        Vec<Vec<f64>>, // cluster data
    ) {
        let _rng = rand::thread_rng();

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
    fn test_dot() {
        let a = [1., 2., 3.];
        let b = [1., 2., 3.];
        assert_eq!(dot(&a, &b).unwrap(), 14.0);
    }

    #[test]
    fn bench_dot() {
        let dimension = 8024;
        let nodes_every_cluster = 600;
        let node_n = 50;
        let (_, nso) =
            make_normal_distribution_clustering(node_n, nodes_every_cluster, dimension, 100000.0);
        println!("hello world {:?}", nso.len());
        let ns: Vec<Vec<f32>> = nso
            .iter()
            .map(|x| x.iter().map(|p| *p as f32).collect())
            .collect();

        {
            let base_start = SystemTime::now();
            let sumbase = ns
                .iter()
                .map(|nsx| {
                    // dot(&nsx, &nsx);
                    // nsx.iter().zip(nsx).map(|(p, q)| p * q).sum::<f32>()
                    nsx.iter()
                        .zip(nsx)
                        .map(|(p, q)| (p - q).powi(2))
                        .sum::<f32>()
                })
                .sum::<f32>();
            let base_since_the_epoch = SystemTime::now()
                .duration_since(base_start)
                .expect("Time went backwards");
            println!(
                "test for {:?} times, base use {:?} millisecond {:?}",
                ns.len(),
                base_since_the_epoch.as_millis(),
                sumbase
            );
        }

        {
            let base_start = SystemTime::now();
            let sumsimd = ns
                .iter()
                .map(|nsx| f32::euclidean_distance(nsx, nsx).unwrap())
                .sum::<f32>();
            let base_since_the_epoch = SystemTime::now()
                .duration_since(base_start)
                .expect("Time went backwards");
            println!(
                "test for {:?} times, simd use {:?} millisecond, {:?}",
                ns.len(),
                base_since_the_epoch.as_millis(),
                sumsimd
            );
        }

        let b = 25;
        println!(
            "{:?}, {:?}",
            f32::dot_product(&ns[b], &ns[b]),
            dot(&ns[b], &ns[b]).unwrap()
        );
    }
}
