/// Calculate the tangent of an angle in degrees.
pub fn tan_degrees(mut degrees: f64) -> f64 {
    while degrees < 0.0 {
        degrees = 360.0 + degrees;
    }
    let degrees = (degrees * 100.0) as usize % 36000;
    super::tan_lut::TAN_LUT[degrees]
}

/// Get the tan value from literal degrees, so 45u8 for 45 degrees and 360u8 is the same as 0u8
pub fn tan_radians(radians: f64) -> f64 {
    let degrees = radians * (180.0 / core::f64::consts::PI);
    tan_degrees(degrees)
}
