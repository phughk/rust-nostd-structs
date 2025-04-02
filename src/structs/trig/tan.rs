use super::{trig_from_lut, AsType};
use core::ops::{Add, Mul};

/// Calculate the tangent of an angle in degrees.
pub fn tan_degrees<T>(degrees: T) -> T
where
    T: AsType<f32> + Add<Output = T> + Mul<Output = T> + Copy + PartialOrd,
{
    trig_from_lut(degrees, &super::tan_lut::TAN_LUT)
}
