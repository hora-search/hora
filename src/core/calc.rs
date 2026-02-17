//! Vector math utilities: dot product, norm, dimension check.

use crate::core::node::FloatElement;

/// L2 norm of a vector.
pub fn get_norm<T>(vec1: &[T]) -> Result<T, &'static str>
where
    T: FloatElement,
{
    dot(vec1, vec1).map(|val| val.sqrt())
}

/// Dot product of two vectors (uses SIMD when available).
pub fn dot<T>(vec1: &[T], vec2: &[T]) -> Result<T, &'static str>
where
    T: FloatElement,
{
    T::dot_product(vec1, vec2)
}

/// Ensures two vectors have the same length.
#[inline(always)]
pub fn same_dimension<T>(vec1: &[T], vec2: &[T]) -> Result<(), &'static str>
where
    T: FloatElement,
{
    if vec1.len() != vec2.len() {
        return Err("different dimensions");
    }
    Ok(())
}

/// Computes imbalance factor between two lengths (for splitting).
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
    use crate::core::calc::dot;
    #[test]
    fn test_dot() {
        let a = [1., 2., 3.];
        let b = [1., 2., 3.];
        assert_eq!(dot::<f32>(&a, &b).unwrap(), 14.0);
    }
}
