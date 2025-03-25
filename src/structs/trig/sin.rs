/// Get the sin value from literal degrees, so 45.0 for 45 degrees and sin(360.0) is the same as sin(0.0)
pub fn sin_degrees(mut degrees: f64) -> f64 {
    while degrees < 0.0 {
        degrees = 360.0 + degrees;
    }
    let degrees = (degrees * 100.0) as usize % 36000;
    super::sin_lut::SIN_LUT[degrees]
}

/// Get the sin value from literal radians, so 0.7853981633974483 for 45 degrees
pub fn sin_radians(radians: f64) -> f64 {
    let degrees = radians * (180.0 / core::f64::consts::PI);
    sin_degrees(degrees)
}

#[cfg(test)]
mod test {
    use crate::structs::trig::sin::sin_degrees;

    #[test]
    fn test_sin_degrees() {
        const FRAC_1_SQRT_2: f64 = 0.7071067812;
        let cases = [
            (0.0, 0.0),
            (45.0, FRAC_1_SQRT_2),
            (90.0, 1.0),
            (135.0, FRAC_1_SQRT_2),
            (180.0, 0.0),
            (225.0, -FRAC_1_SQRT_2),
            (270.0, -1.0),
            (315.0, -FRAC_1_SQRT_2),
            (360.0, 0.0),
        ];
        for case in cases {
            assert_eq!(sin_degrees(case.0), case.1, "{}", case.0)
        }
    }
}
