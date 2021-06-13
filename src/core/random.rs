use rand::prelude::*;

pub trait Random<T> {
    fn kiss() -> T;
    fn flip() -> bool;
    fn index(n: usize) -> usize;
}

// TODO: use random
pub fn flip() -> bool {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..10) > 5
}

pub fn index(n: usize) -> usize {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..n)
}
