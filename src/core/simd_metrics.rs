pub trait SIMDOptmized<T = Self> {
    fn dot_product(a: &[T], b: &[T]) -> Result<T, &'static str>;
    fn manhattan_distance(a: &[T], b: &[T]) -> Result<T, &'static str>;
    fn euclidean_distance(a: &[T], b: &[T]) -> Result<T, &'static str>;
}

// CPU feature detection for runtime optimization
#[cfg(all(feature = "simd", target_arch = "x86_64"))]
mod cpu_features {
    use std::sync::atomic::{AtomicU8, Ordering};
    
    static AVX2_DETECTED: AtomicU8 = AtomicU8::new(0); // 0: unknown, 1: yes, 2: no
    static AVX512F_DETECTED: AtomicU8 = AtomicU8::new(0);
    
    #[inline]
    pub fn has_avx2() -> bool {
        let val = AVX2_DETECTED.load(Ordering::Relaxed);
        if val == 0 {
            let has = is_x86_feature_detected!("avx2");
            AVX2_DETECTED.store(if has { 1 } else { 2 }, Ordering::Relaxed);
            has
        } else {
            val == 1
        }
    }
    
    #[inline]
    pub fn has_avx512f() -> bool {
        let val = AVX512F_DETECTED.load(Ordering::Relaxed);
        if val == 0 {
            let has = is_x86_feature_detected!("avx512f");
            AVX512F_DETECTED.store(if has { 1 } else { 2 }, Ordering::Relaxed);
            has
        } else {
            val == 1
        }
    }
}

// x86_64 SIMD implementation with runtime CPU feature detection
#[cfg(all(feature = "simd", target_arch = "x86_64"))]
mod x86_64_simd {
    use super::SIMDOptmized;
    use crate::core::calc::same_dimension;
    use std::arch::x86_64::*;

    // Optimized horizontal sum for AVX256
    #[target_feature(enable = "avx")]
    unsafe fn horizontal_sum_ps_avx256(sum: __m256) -> f32 {
        let sum_high = _mm256_extractf128_ps(sum, 1);
        let sum_low = _mm256_castps256_ps128(sum);
        let sum_128 = _mm_add_ps(sum_high, sum_low);
        let sum_64 = _mm_add_ps(sum_128, _mm_movehl_ps(sum_128, sum_128));
        let sum_32 = _mm_add_ss(sum_64, _mm_shuffle_ps(sum_64, sum_64, 0x55));
        _mm_cvtss_f32(sum_32)
    }

    #[target_feature(enable = "avx")]
    unsafe fn horizontal_sum_pd_avx256(sum: __m256d) -> f64 {
        let sum_high = _mm256_extractf128_pd(sum, 1);
        let sum_low = _mm256_castpd256_pd128(sum);
        let sum_128 = _mm_add_pd(sum_high, sum_low);
        let sum_64 = _mm_add_sd(sum_128, _mm_unpackhi_pd(sum_128, sum_128));
        _mm_cvtsd_f64(sum_64)
    }

    // f32 implementation
    impl SIMDOptmized for f32 {
        fn dot_product(a: &[f32], b: &[f32]) -> Result<f32, &'static str> {
            assert_eq!(a.len(), b.len());
            
            // Try AVX512 first (if available)
            #[cfg(target_feature = "avx512f")]
            if is_x86_feature_detected!("avx512f") {
                return unsafe { dot_product_avx512(a, b) };
            }
            
            // Try AVX2/AVX
            if is_x86_feature_detected!("avx2") {
                return unsafe { dot_product_avx256(a, b) };
            }
            
            // Fallback to SSE
            if is_x86_feature_detected!("sse") {
                return unsafe { dot_product_sse(a, b) };
            }
            
            // Scalar fallback
            Ok(-(a.iter().zip(b).map(|(p, q)| p * q).sum::<f32>()))
        }

        fn manhattan_distance(a: &[f32], b: &[f32]) -> Result<f32, &'static str> {
            assert_eq!(a.len(), b.len());
            
            if is_x86_feature_detected!("avx2") {
                return unsafe { manhattan_distance_avx256(a, b) };
            }
            
            if is_x86_feature_detected!("sse") {
                return unsafe { manhattan_distance_sse(a, b) };
            }
            
            Ok(a.iter().zip(b).map(|(p, q)| (p - q).abs()).sum::<f32>())
        }

        fn euclidean_distance(a: &[f32], b: &[f32]) -> Result<f32, &'static str> {
            same_dimension(a, b)?;
            
            if is_x86_feature_detected!("avx2") {
                return unsafe { euclidean_distance_avx256(a, b) };
            }
            
            if is_x86_feature_detected!("sse") {
                return unsafe { euclidean_distance_sse(a, b) };
            }
            
            Ok(a.iter().zip(b).map(|(p, q)| (p - q).powi(2)).sum::<f32>())
        }
    }

    // AVX256 implementations for f32
    #[target_feature(enable = "avx2")]
    unsafe fn dot_product_avx256(a: &[f32], b: &[f32]) -> Result<f32, &'static str> {
        let chunks = a.chunks_exact(8);
        let remainder = chunks.remainder();
        let b_chunks = b.chunks_exact(8);
        let b_remainder = b_chunks.remainder();
        
        let mut sum = _mm256_setzero_ps();
        for (a_chunk, b_chunk) in chunks.zip(b_chunks) {
            let a_vec = _mm256_loadu_ps(a_chunk.as_ptr());
            let b_vec = _mm256_loadu_ps(b_chunk.as_ptr());
            let prod = _mm256_mul_ps(a_vec, b_vec);
            sum = _mm256_add_ps(sum, prod);
        }
        
        let simd_sum = horizontal_sum_ps_avx256(sum);
        let scalar_sum: f32 = remainder.iter().zip(b_remainder).map(|(p, q)| p * q).sum();
        Ok(-(simd_sum + scalar_sum))
    }

    #[target_feature(enable = "avx2")]
    unsafe fn manhattan_distance_avx256(a: &[f32], b: &[f32]) -> Result<f32, &'static str> {
        let chunks = a.chunks_exact(8);
        let remainder = chunks.remainder();
        let b_chunks = b.chunks_exact(8);
        let b_remainder = b_chunks.remainder();
        
        let mut sum = _mm256_setzero_ps();
        let sign_mask = _mm256_set1_ps(-0.0);
        for (a_chunk, b_chunk) in chunks.zip(b_chunks) {
            let a_vec = _mm256_loadu_ps(a_chunk.as_ptr());
            let b_vec = _mm256_loadu_ps(b_chunk.as_ptr());
            let diff = _mm256_sub_ps(a_vec, b_vec);
            let abs_diff = _mm256_andnot_ps(sign_mask, diff);
            sum = _mm256_add_ps(sum, abs_diff);
        }
        
        let simd_sum = horizontal_sum_ps_avx256(sum);
        let scalar_sum: f32 = remainder.iter().zip(b_remainder).map(|(p, q)| (p - q).abs()).sum();
        Ok(simd_sum + scalar_sum)
    }

    #[target_feature(enable = "avx2")]
    unsafe fn euclidean_distance_avx256(a: &[f32], b: &[f32]) -> Result<f32, &'static str> {
        let chunks = a.chunks_exact(8);
        let remainder = chunks.remainder();
        let b_chunks = b.chunks_exact(8);
        let b_remainder = b_chunks.remainder();
        
        let mut sum = _mm256_setzero_ps();
        for (a_chunk, b_chunk) in chunks.zip(b_chunks) {
            let a_vec = _mm256_loadu_ps(a_chunk.as_ptr());
            let b_vec = _mm256_loadu_ps(b_chunk.as_ptr());
            let diff = _mm256_sub_ps(a_vec, b_vec);
            let sqr = _mm256_mul_ps(diff, diff);
            sum = _mm256_add_ps(sum, sqr);
        }
        
        let simd_sum = horizontal_sum_ps_avx256(sum);
        let scalar_sum: f32 = remainder.iter().zip(b_remainder).map(|(p, q)| (p - q).powi(2)).sum();
        Ok(scalar_sum + simd_sum)
    }

    // SSE implementations for f32
    #[target_feature(enable = "sse")]
    unsafe fn dot_product_sse(a: &[f32], b: &[f32]) -> Result<f32, &'static str> {
        let chunks = a.chunks_exact(4);
        let remainder = chunks.remainder();
        let b_chunks = b.chunks_exact(4);
        let b_remainder = b_chunks.remainder();
        
        let mut sum = _mm_setzero_ps();
        for (a_chunk, b_chunk) in chunks.zip(b_chunks) {
            let a_vec = _mm_loadu_ps(a_chunk.as_ptr());
            let b_vec = _mm_loadu_ps(b_chunk.as_ptr());
            let prod = _mm_mul_ps(a_vec, b_vec);
            sum = _mm_add_ps(sum, prod);
        }
        
        let sum_64 = _mm_add_ps(sum, _mm_movehl_ps(sum, sum));
        let sum_32 = _mm_add_ss(sum_64, _mm_shuffle_ps(sum_64, sum_64, 0x55));
        let simd_sum = _mm_cvtss_f32(sum_32);
        let scalar_sum: f32 = remainder.iter().zip(b_remainder).map(|(p, q)| p * q).sum();
        Ok(-(simd_sum + scalar_sum))
    }

    #[target_feature(enable = "sse")]
    unsafe fn manhattan_distance_sse(a: &[f32], b: &[f32]) -> Result<f32, &'static str> {
        let chunks = a.chunks_exact(4);
        let remainder = chunks.remainder();
        let b_chunks = b.chunks_exact(4);
        let b_remainder = b_chunks.remainder();
        
        let mut sum = _mm_setzero_ps();
        let sign_mask = _mm_set1_ps(-0.0);
        for (a_chunk, b_chunk) in chunks.zip(b_chunks) {
            let a_vec = _mm_loadu_ps(a_chunk.as_ptr());
            let b_vec = _mm_loadu_ps(b_chunk.as_ptr());
            let diff = _mm_sub_ps(a_vec, b_vec);
            let abs_diff = _mm_andnot_ps(sign_mask, diff);
            sum = _mm_add_ps(sum, abs_diff);
        }
        
        let sum_64 = _mm_add_ps(sum, _mm_movehl_ps(sum, sum));
        let sum_32 = _mm_add_ss(sum_64, _mm_shuffle_ps(sum_64, sum_64, 0x55));
        let simd_sum = _mm_cvtss_f32(sum_32);
        let scalar_sum: f32 = remainder.iter().zip(b_remainder).map(|(p, q)| (p - q).abs()).sum();
        Ok(simd_sum + scalar_sum)
    }

    #[target_feature(enable = "sse")]
    unsafe fn euclidean_distance_sse(a: &[f32], b: &[f32]) -> Result<f32, &'static str> {
        let chunks = a.chunks_exact(4);
        let remainder = chunks.remainder();
        let b_chunks = b.chunks_exact(4);
        let b_remainder = b_chunks.remainder();
        
        let mut sum = _mm_setzero_ps();
        for (a_chunk, b_chunk) in chunks.zip(b_chunks) {
            let a_vec = _mm_loadu_ps(a_chunk.as_ptr());
            let b_vec = _mm_loadu_ps(b_chunk.as_ptr());
            let diff = _mm_sub_ps(a_vec, b_vec);
            let sqr = _mm_mul_ps(diff, diff);
            sum = _mm_add_ps(sum, sqr);
        }
        
        let sum_64 = _mm_add_ps(sum, _mm_movehl_ps(sum, sum));
        let sum_32 = _mm_add_ss(sum_64, _mm_shuffle_ps(sum_64, sum_64, 0x55));
        let simd_sum = _mm_cvtss_f32(sum_32);
        let scalar_sum: f32 = remainder.iter().zip(b_remainder).map(|(p, q)| (p - q).powi(2)).sum();
        Ok(scalar_sum + simd_sum)
    }

    // f64 implementation
    impl SIMDOptmized for f64 {
        fn dot_product(a: &[f64], b: &[f64]) -> Result<f64, &'static str> {
            assert_eq!(a.len(), b.len());
            
            if is_x86_feature_detected!("avx2") {
                return unsafe { dot_product_avx256_f64(a, b) };
            }
            
            if is_x86_feature_detected!("sse2") {
                return unsafe { dot_product_sse2_f64(a, b) };
            }
            
            Ok(-(a.iter().zip(b).map(|(p, q)| p * q).sum::<f64>()))
        }

        fn manhattan_distance(a: &[f64], b: &[f64]) -> Result<f64, &'static str> {
            assert_eq!(a.len(), b.len());
            
            if is_x86_feature_detected!("avx2") {
                return unsafe { manhattan_distance_avx256_f64(a, b) };
            }
            
            if is_x86_feature_detected!("sse2") {
                return unsafe { manhattan_distance_sse2_f64(a, b) };
            }
            
            Ok(a.iter().zip(b).map(|(p, q)| (p - q).abs()).sum::<f64>())
        }

        fn euclidean_distance(a: &[f64], b: &[f64]) -> Result<f64, &'static str> {
            same_dimension(a, b)?;
            
            if is_x86_feature_detected!("avx2") {
                return unsafe { euclidean_distance_avx256_f64(a, b) };
            }
            
            if is_x86_feature_detected!("sse2") {
                return unsafe { euclidean_distance_sse2_f64(a, b) };
            }
            
            Ok(a.iter().zip(b).map(|(p, q)| (p - q).powi(2)).sum::<f64>())
        }
    }

    #[target_feature(enable = "avx2")]
    unsafe fn dot_product_avx256_f64(a: &[f64], b: &[f64]) -> Result<f64, &'static str> {
        let chunks = a.chunks_exact(4);
        let remainder = chunks.remainder();
        let b_chunks = b.chunks_exact(4);
        let b_remainder = b_chunks.remainder();
        
        let mut sum = _mm256_setzero_pd();
        for (a_chunk, b_chunk) in chunks.zip(b_chunks) {
            let a_vec = _mm256_loadu_pd(a_chunk.as_ptr());
            let b_vec = _mm256_loadu_pd(b_chunk.as_ptr());
            let prod = _mm256_mul_pd(a_vec, b_vec);
            sum = _mm256_add_pd(sum, prod);
        }
        
        let simd_sum = horizontal_sum_pd_avx256(sum);
        let scalar_sum: f64 = remainder.iter().zip(b_remainder).map(|(p, q)| p * q).sum();
        Ok(-(simd_sum + scalar_sum))
    }

    #[target_feature(enable = "avx2")]
    unsafe fn manhattan_distance_avx256_f64(a: &[f64], b: &[f64]) -> Result<f64, &'static str> {
        let chunks = a.chunks_exact(4);
        let remainder = chunks.remainder();
        let b_chunks = b.chunks_exact(4);
        let b_remainder = b_chunks.remainder();
        
        let mut sum = _mm256_setzero_pd();
        let sign_mask = _mm256_set1_pd(-0.0);
        for (a_chunk, b_chunk) in chunks.zip(b_chunks) {
            let a_vec = _mm256_loadu_pd(a_chunk.as_ptr());
            let b_vec = _mm256_loadu_pd(b_chunk.as_ptr());
            let diff = _mm256_sub_pd(a_vec, b_vec);
            let abs_diff = _mm256_andnot_pd(sign_mask, diff);
            sum = _mm256_add_pd(sum, abs_diff);
        }
        
        let simd_sum = horizontal_sum_pd_avx256(sum);
        let scalar_sum: f64 = remainder.iter().zip(b_remainder).map(|(p, q)| (p - q).abs()).sum();
        Ok(simd_sum + scalar_sum)
    }

    #[target_feature(enable = "avx2")]
    unsafe fn euclidean_distance_avx256_f64(a: &[f64], b: &[f64]) -> Result<f64, &'static str> {
        let chunks = a.chunks_exact(4);
        let remainder = chunks.remainder();
        let b_chunks = b.chunks_exact(4);
        let b_remainder = b_chunks.remainder();
        
        let mut sum = _mm256_setzero_pd();
        for (a_chunk, b_chunk) in chunks.zip(b_chunks) {
            let a_vec = _mm256_loadu_pd(a_chunk.as_ptr());
            let b_vec = _mm256_loadu_pd(b_chunk.as_ptr());
            let diff = _mm256_sub_pd(a_vec, b_vec);
            let sqr = _mm256_mul_pd(diff, diff);
            sum = _mm256_add_pd(sum, sqr);
        }
        
        let simd_sum = horizontal_sum_pd_avx256(sum);
        let scalar_sum: f64 = remainder.iter().zip(b_remainder).map(|(p, q)| (p - q).powi(2)).sum();
        Ok(scalar_sum + simd_sum)
    }

    #[target_feature(enable = "sse2")]
    unsafe fn dot_product_sse2_f64(a: &[f64], b: &[f64]) -> Result<f64, &'static str> {
        let chunks = a.chunks_exact(2);
        let remainder = chunks.remainder();
        let b_chunks = b.chunks_exact(2);
        let b_remainder = b_chunks.remainder();
        
        let mut sum = _mm_setzero_pd();
        for (a_chunk, b_chunk) in chunks.zip(b_chunks) {
            let a_vec = _mm_loadu_pd(a_chunk.as_ptr());
            let b_vec = _mm_loadu_pd(b_chunk.as_ptr());
            let prod = _mm_mul_pd(a_vec, b_vec);
            sum = _mm_add_pd(sum, prod);
        }
        
        let sum_64 = _mm_add_sd(sum, _mm_unpackhi_pd(sum, sum));
        let simd_sum = _mm_cvtsd_f64(sum_64);
        let scalar_sum: f64 = remainder.iter().zip(b_remainder).map(|(p, q)| p * q).sum();
        Ok(-(simd_sum + scalar_sum))
    }

    #[target_feature(enable = "sse2")]
    unsafe fn manhattan_distance_sse2_f64(a: &[f64], b: &[f64]) -> Result<f64, &'static str> {
        let chunks = a.chunks_exact(2);
        let remainder = chunks.remainder();
        let b_chunks = b.chunks_exact(2);
        let b_remainder = b_chunks.remainder();
        
        let mut sum = _mm_setzero_pd();
        let sign_mask = _mm_set1_pd(-0.0);
        for (a_chunk, b_chunk) in chunks.zip(b_chunks) {
            let a_vec = _mm_loadu_pd(a_chunk.as_ptr());
            let b_vec = _mm_loadu_pd(b_chunk.as_ptr());
            let diff = _mm_sub_pd(a_vec, b_vec);
            let abs_diff = _mm_andnot_pd(sign_mask, diff);
            sum = _mm_add_pd(sum, abs_diff);
        }
        
        let sum_64 = _mm_add_sd(sum, _mm_unpackhi_pd(sum, sum));
        let simd_sum = _mm_cvtsd_f64(sum_64);
        let scalar_sum: f64 = remainder.iter().zip(b_remainder).map(|(p, q)| (p - q).abs()).sum();
        Ok(simd_sum + scalar_sum)
    }

    #[target_feature(enable = "sse2")]
    unsafe fn euclidean_distance_sse2_f64(a: &[f64], b: &[f64]) -> Result<f64, &'static str> {
        let chunks = a.chunks_exact(2);
        let remainder = chunks.remainder();
        let b_chunks = b.chunks_exact(2);
        let b_remainder = b_chunks.remainder();
        
        let mut sum = _mm_setzero_pd();
        for (a_chunk, b_chunk) in chunks.zip(b_chunks) {
            let a_vec = _mm_loadu_pd(a_chunk.as_ptr());
            let b_vec = _mm_loadu_pd(b_chunk.as_ptr());
            let diff = _mm_sub_pd(a_vec, b_vec);
            let sqr = _mm_mul_pd(diff, diff);
            sum = _mm_add_pd(sum, sqr);
        }
        
        let sum_64 = _mm_add_sd(sum, _mm_unpackhi_pd(sum, sum));
        let simd_sum = _mm_cvtsd_f64(sum_64);
        let scalar_sum: f64 = remainder.iter().zip(b_remainder).map(|(p, q)| (p - q).powi(2)).sum();
        Ok(scalar_sum + simd_sum)
    }
}

// ARM NEON SIMD implementation
#[cfg(all(feature = "simd", any(target_arch = "aarch64", target_arch = "arm")))]
mod arm_neon_simd {
    use super::SIMDOptmized;
    use crate::core::calc::same_dimension;
    

    impl SIMDOptmized for f32 {
        fn dot_product(a: &[f32], b: &[f32]) -> Result<f32, &'static str> {
            assert_eq!(a.len(), b.len());
            
            #[cfg(target_arch = "aarch64")]
            {
                return unsafe { dot_product_neon(a, b) };
            }
            
            #[cfg(not(target_arch = "aarch64"))]
            {
                Ok(-(a.iter().zip(b).map(|(p, q)| p * q).sum::<f32>()))
            }
        }

        fn manhattan_distance(a: &[f32], b: &[f32]) -> Result<f32, &'static str> {
            assert_eq!(a.len(), b.len());
            
            #[cfg(target_arch = "aarch64")]
            {
                return unsafe { manhattan_distance_neon(a, b) };
            }
            
            #[cfg(not(target_arch = "aarch64"))]
            {
                Ok(a.iter().zip(b).map(|(p, q)| (p - q).abs()).sum::<f32>())
            }
        }

        fn euclidean_distance(a: &[f32], b: &[f32]) -> Result<f32, &'static str> {
            same_dimension(a, b)?;
            
            #[cfg(target_arch = "aarch64")]
            {
                return unsafe { euclidean_distance_neon(a, b) };
            }
            
            #[cfg(not(target_arch = "aarch64"))]
            {
                Ok(a.iter().zip(b).map(|(p, q)| (p - q).powi(2)).sum::<f32>())
            }
        }
    }

    #[cfg(target_arch = "aarch64")]
    #[target_feature(enable = "neon")]
    unsafe fn dot_product_neon(a: &[f32], b: &[f32]) -> Result<f32, &'static str> {
        use std::arch::aarch64::*;
        
        let chunks = a.chunks_exact(4);
        let remainder = chunks.remainder();
        let b_chunks = b.chunks_exact(4);
        let b_remainder = b_chunks.remainder();
        
        let mut sum = vdupq_n_f32(0.0);
        for (a_chunk, b_chunk) in chunks.zip(b_chunks) {
            let a_vec = vld1q_f32(a_chunk.as_ptr());
            let b_vec = vld1q_f32(b_chunk.as_ptr());
            let prod = vmulq_f32(a_vec, b_vec);
            sum = vaddq_f32(sum, prod);
        }
        
        // Horizontal sum
        let sum_high = vget_high_f32(sum);
        let sum_low = vget_low_f32(sum);
        let sum_64 = vadd_f32(sum_high, sum_low);
        let sum_32 = vadd_f32(sum_64, vrev64_f32(sum_64));
        let simd_sum = vget_lane_f32::<0>(sum_32) + vget_lane_f32::<1>(sum_32);
        
        let scalar_sum: f32 = remainder.iter().zip(b_remainder).map(|(p, q)| p * q).sum();
        Ok(-(simd_sum + scalar_sum))
    }

    #[cfg(target_arch = "aarch64")]
    #[target_feature(enable = "neon")]
    unsafe fn manhattan_distance_neon(a: &[f32], b: &[f32]) -> Result<f32, &'static str> {
        #[cfg(target_arch = "aarch64")]
        use std::arch::aarch64::*;
        
        let chunks = a.chunks_exact(4);
        let remainder = chunks.remainder();
        let b_chunks = b.chunks_exact(4);
        let b_remainder = b_chunks.remainder();
        
        let mut sum = vdupq_n_f32(0.0);
        for (a_chunk, b_chunk) in chunks.zip(b_chunks) {
            let a_vec = vld1q_f32(a_chunk.as_ptr());
            let b_vec = vld1q_f32(b_chunk.as_ptr());
            let diff = vsubq_f32(a_vec, b_vec);
            let abs_diff = vabsq_f32(diff);
            sum = vaddq_f32(sum, abs_diff);
        }
        
        let sum_high = vget_high_f32(sum);
        let sum_low = vget_low_f32(sum);
        let sum_64 = vadd_f32(sum_high, sum_low);
        let sum_32 = vadd_f32(sum_64, vrev64_f32(sum_64));
        let simd_sum = vget_lane_f32::<0>(sum_32) + vget_lane_f32::<1>(sum_32);
        
        let scalar_sum: f32 = remainder.iter().zip(b_remainder).map(|(p, q)| (p - q).abs()).sum();
        Ok(simd_sum + scalar_sum)
    }

    #[cfg(target_arch = "aarch64")]
    #[target_feature(enable = "neon")]
    unsafe fn euclidean_distance_neon(a: &[f32], b: &[f32]) -> Result<f32, &'static str> {
        #[cfg(target_arch = "aarch64")]
        use std::arch::aarch64::*;
        
        let chunks = a.chunks_exact(4);
        let remainder = chunks.remainder();
        let b_chunks = b.chunks_exact(4);
        let b_remainder = b_chunks.remainder();
        
        let mut sum = vdupq_n_f32(0.0);
        for (a_chunk, b_chunk) in chunks.zip(b_chunks) {
            let a_vec = vld1q_f32(a_chunk.as_ptr());
            let b_vec = vld1q_f32(b_chunk.as_ptr());
            let diff = vsubq_f32(a_vec, b_vec);
            let sqr = vmulq_f32(diff, diff);
            sum = vaddq_f32(sum, sqr);
        }
        
        let sum_high = vget_high_f32(sum);
        let sum_low = vget_low_f32(sum);
        let sum_64 = vadd_f32(sum_high, sum_low);
        let sum_32 = vadd_f32(sum_64, vrev64_f32(sum_64));
        let simd_sum = vget_lane_f32::<0>(sum_32) + vget_lane_f32::<1>(sum_32);
        
        let scalar_sum: f32 = remainder.iter().zip(b_remainder).map(|(p, q)| (p - q).powi(2)).sum();
        Ok(scalar_sum + simd_sum)
    }

    impl SIMDOptmized for f64 {
        fn dot_product(a: &[f64], b: &[f64]) -> Result<f64, &'static str> {
            assert_eq!(a.len(), b.len());
            Ok(-(a.iter().zip(b).map(|(p, q)| p * q).sum::<f64>()))
        }

        fn manhattan_distance(a: &[f64], b: &[f64]) -> Result<f64, &'static str> {
            assert_eq!(a.len(), b.len());
            Ok(a.iter().zip(b).map(|(p, q)| (p - q).abs()).sum::<f64>())
        }

        fn euclidean_distance(a: &[f64], b: &[f64]) -> Result<f64, &'static str> {
            same_dimension(a, b)?;
            Ok(a.iter().zip(b).map(|(p, q)| (p - q).powi(2)).sum::<f64>())
        }
    }
}

// WASM SIMD implementation
#[cfg(all(feature = "simd", target_arch = "wasm32"))]
mod wasm_simd {
    use super::SIMDOptmized;
    use crate::core::calc::same_dimension;
    use std::arch::wasm32::*;

    impl SIMDOptmized for f32 {
        fn dot_product(a: &[f32], b: &[f32]) -> Result<f32, &'static str> {
            assert_eq!(a.len(), b.len());
            
            let chunks = a.chunks_exact(4);
            let remainder = chunks.remainder();
            let b_chunks = b.chunks_exact(4);
            let b_remainder = b_chunks.remainder();
            
            unsafe {
                let mut sum = f32x4_splat(0.0);
                for (a_chunk, b_chunk) in chunks.zip(b_chunks) {
                    let a_vec = v128_load(a_chunk.as_ptr() as *const v128);
                    let b_vec = v128_load(b_chunk.as_ptr() as *const v128);
                    let prod = f32x4_mul(a_vec, b_vec);
                    sum = f32x4_add(sum, prod);
                }
                
                let simd_sum = f32x4_extract_lane::<0>(sum) + f32x4_extract_lane::<1>(sum) +
                              f32x4_extract_lane::<2>(sum) + f32x4_extract_lane::<3>(sum);
                let scalar_sum: f32 = remainder.iter().zip(b_remainder).map(|(p, q)| p * q).sum();
                Ok(-(simd_sum + scalar_sum))
            }
        }

        fn manhattan_distance(a: &[f32], b: &[f32]) -> Result<f32, &'static str> {
            assert_eq!(a.len(), b.len());
            
            let chunks = a.chunks_exact(4);
            let remainder = chunks.remainder();
            let b_chunks = b.chunks_exact(4);
            let b_remainder = b_chunks.remainder();
            
            unsafe {
                let mut sum = f32x4_splat(0.0);
                for (a_chunk, b_chunk) in chunks.zip(b_chunks) {
                    let a_vec = v128_load(a_chunk.as_ptr() as *const v128);
                    let b_vec = v128_load(b_chunk.as_ptr() as *const v128);
                    let diff = f32x4_sub(a_vec, b_vec);
                    let abs_diff = f32x4_abs(diff);
                    sum = f32x4_add(sum, abs_diff);
                }
                
                let simd_sum = f32x4_extract_lane::<0>(sum) + f32x4_extract_lane::<1>(sum) +
                              f32x4_extract_lane::<2>(sum) + f32x4_extract_lane::<3>(sum);
                let scalar_sum: f32 = remainder.iter().zip(b_remainder).map(|(p, q)| (p - q).abs()).sum();
                Ok(simd_sum + scalar_sum)
            }
        }

        fn euclidean_distance(a: &[f32], b: &[f32]) -> Result<f32, &'static str> {
            same_dimension(a, b)?;
            
            let chunks = a.chunks_exact(4);
            let remainder = chunks.remainder();
            let b_chunks = b.chunks_exact(4);
            let b_remainder = b_chunks.remainder();
            
            unsafe {
                let mut sum = f32x4_splat(0.0);
                for (a_chunk, b_chunk) in chunks.zip(b_chunks) {
                    let a_vec = v128_load(a_chunk.as_ptr() as *const v128);
                    let b_vec = v128_load(b_chunk.as_ptr() as *const v128);
                    let diff = f32x4_sub(a_vec, b_vec);
                    let sqr = f32x4_mul(diff, diff);
                    sum = f32x4_add(sum, sqr);
                }
                
                let simd_sum = f32x4_extract_lane::<0>(sum) + f32x4_extract_lane::<1>(sum) +
                              f32x4_extract_lane::<2>(sum) + f32x4_extract_lane::<3>(sum);
                let scalar_sum: f32 = remainder.iter().zip(b_remainder).map(|(p, q)| (p - q).powi(2)).sum();
                Ok(scalar_sum + simd_sum)
            }
        }
    }

    impl SIMDOptmized for f64 {
        fn dot_product(a: &[f64], b: &[f64]) -> Result<f64, &'static str> {
            assert_eq!(a.len(), b.len());
            Ok(-(a.iter().zip(b).map(|(p, q)| p * q).sum::<f64>()))
        }

        fn manhattan_distance(a: &[f64], b: &[f64]) -> Result<f64, &'static str> {
            assert_eq!(a.len(), b.len());
            Ok(a.iter().zip(b).map(|(p, q)| (p - q).abs()).sum::<f64>())
        }

        fn euclidean_distance(a: &[f64], b: &[f64]) -> Result<f64, &'static str> {
            same_dimension(a, b)?;
            Ok(a.iter().zip(b).map(|(p, q)| (p - q).powi(2)).sum::<f64>())
        }
    }
}

// Scalar fallback for unsupported platforms
#[cfg(not(all(feature = "simd", any(target_arch = "x86_64", target_arch = "aarch64", target_arch = "arm", target_arch = "wasm32"))))]
mod scalar_fallback {
    use super::SIMDOptmized;
    use crate::core::calc::same_dimension;

    impl SIMDOptmized for f32 {
        fn dot_product(a: &[f32], b: &[f32]) -> Result<f32, &'static str> {
            assert_eq!(a.len(), b.len());
            Ok(-(a.iter().zip(b).map(|(p, q)| p * q).sum::<f32>()))
        }

        fn manhattan_distance(a: &[f32], b: &[f32]) -> Result<f32, &'static str> {
            assert_eq!(a.len(), b.len());
            Ok(a.iter().zip(b).map(|(p, q)| (p - q).abs()).sum::<f32>())
        }

        fn euclidean_distance(a: &[f32], b: &[f32]) -> Result<f32, &'static str> {
            same_dimension(a, b)?;
            Ok(a.iter().zip(b).map(|(p, q)| (p - q).powi(2)).sum::<f32>())
        }
    }

    impl SIMDOptmized for f64 {
        fn dot_product(a: &[f64], b: &[f64]) -> Result<f64, &'static str> {
            assert_eq!(a.len(), b.len());
            Ok(-(a.iter().zip(b).map(|(p, q)| p * q).sum::<f64>()))
        }

        fn manhattan_distance(a: &[f64], b: &[f64]) -> Result<f64, &'static str> {
            assert_eq!(a.len(), b.len());
            Ok(a.iter().zip(b).map(|(p, q)| (p - q).abs()).sum::<f64>())
        }

        fn euclidean_distance(a: &[f64], b: &[f64]) -> Result<f64, &'static str> {
            same_dimension(a, b)?;
            Ok(a.iter().zip(b).map(|(p, q)| (p - q).powi(2)).sum::<f64>())
        }
    }
}
