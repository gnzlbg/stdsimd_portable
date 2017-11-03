#![feature(cfg_target_feature)]
#![allow(non_camel_case_types)]
#![feature(const_fn)]

#[macro_use]
extern crate stdsimd;

use std::ops::{Add, Sub, Mul};
use stdsimd::simd as st;
use stdsimd::vendor;

#[macro_use]
mod macros;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct u64x2(st::u64x2);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct u32x4(st::u32x4);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct f64x2(st::f64x2);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct f32x4(st::f32x4);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct bool32ix4(st::u32x4);

impl_from!(u32x4, st::u32x4);
impl_from!(u64x2, st::u64x2);
impl_from!(f32x4, st::f32x4);
impl_from!(f64x2, st::f64x2);
impl_from!(bool32ix4, st::u32x4);

impl_new!(u32x4, u32, v0, v1, v2, v3);
impl_new!(u64x2, u64, v0, v1);
impl_new!(f32x4, f32, v0, v1, v2, v3);
impl_new!(f64x2, f64, v0, v1);

impl_zeros!(u32x4, 0, 0, 0, 0);
impl_zeros!(u64x2, 0, 0);
impl_zeros!(f32x4, 0., 0., 0., 0.);
impl_zeros!(f64x2, 0., 0.);

impl_ones!(u32x4, 1, 1, 1, 1);
impl_ones!(u64x2, 1, 1);
impl_ones!(f32x4, 1., 1., 1., 1.);
impl_ones!(f64x2, 1., 1.);

impl_get!(u32x4, u32);
impl_get!(u64x2, u64);
impl_get!(f32x4, f32);
impl_get!(f64x2, f64);

impl_set!(u32x4, u32);
impl_set!(u64x2, u64);
impl_set!(f32x4, f32);
impl_set!(f64x2, f64);

impl_add!(u32x4, "sse2", _mm_add_epi32);
impl_add!(u64x2, "sse", _mm_add_epi64);
impl_add!(f32x4, "sse", _mm_add_ps);
impl_add!(f64x2, "sse2", _mm_add_pd);

impl_sub!(u32x4, "sse2", _mm_sub_epi32);
impl_sub!(u64x2, "sse", _mm_sub_epi64);
impl_sub!(f32x4, "sse", _mm_sub_ps);
impl_sub!(f64x2, "sse2", _mm_sub_pd);

// not implemented on stdsimd yet:
//impl_mul!(u32x4, "sse41", _mm_mullo_epi32);
//impl_mul!(u64x2, "avx512", _mm_mullo_epi64);
impl_mul!(f32x4, "sse", _mm_mul_ps);
impl_mul!(f64x2, "sse2", _mm_mul_pd);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn arithmetic_f32x4() {
        let a = f32x4::ones();
        let b = a;
        let c = a + b;
        assert_eq!(c, f32x4::new(2.0, 2.0, 2.0, 2.0));
        let d = c * c;
        assert_eq!(d, f32x4::new(4.0, 4.0, 4.0, 4.0));
    }

    #[test]
    fn arithmetic_f64x2() {
        let a = f64x2::ones();
        let b = a;
        let c = a + b;
        assert_eq!(c, f64x2::new(2.0, 2.0));
        let d = c * c;
        assert_eq!(d, f64x2::new(4.0, 4.0));
    }
}
