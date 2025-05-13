//! Trigonometric functions
mod cos;
mod cos_lut;
mod sin;
mod sin_lut;
mod sqrt;
mod tan;
mod tan_lut;

use crate::structs::AsType;
use core::ops::{Add, Mul};
pub use cos::cos_degrees;
pub use sin::sin_degrees;
pub use sqrt::sqrt;
pub use tan::tan_degrees;

pub(super) fn trig_from_lut<T>(degrees_direct: T, lut: &[f32]) -> T
where
    T: AsType<f32> + Add<Output = T> + Mul<Output = T> + Copy + PartialOrd,
{
    let mut degrees: f32 = degrees_direct.into_type();
    while degrees < 0.0 {
        degrees = 360.0 + degrees;
    }
    let deg_index = (degrees * 100.0) as usize;
    let degrees = deg_index % lut.len();
    let v = lut[degrees];
    T::from_type(v)
}

/// Convert radians to degrees
pub fn radians_to_degrees<T>(radians: T) -> T
where
    T: AsType<f32> + Copy,
{
    let rad_f32 = radians.into_type();
    T::from_type(rad_f32 * (180.0 / core::f32::consts::PI))
}
