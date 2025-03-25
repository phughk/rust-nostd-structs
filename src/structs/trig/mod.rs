//! Trigonometric functions
mod cos;
mod cos_lut;
mod sin;
mod sin_lut;
mod tan;
mod tan_lut;

pub use cos::cos_degrees;
pub use cos::cos_radians;
pub use sin::sin_degrees;
pub use sin::sin_radians;
pub use tan::tan_degrees;
pub use tan::tan_radians;
