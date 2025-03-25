/// Get the sin value from literal degrees, so 45u8 for 45 degrees and 360u8 is the same as 0u8
pub fn cos_degrees(mut degrees: f64) -> f64 {
    while degrees < 0.0 {
        degrees = 360.0 + degrees;
    }
    let degrees = (degrees * 100.0) as usize % 36000;
    super::cos_lut::COS_LUT[degrees]
}

/// Get the sin value from literal degrees, so 45u8 for 45 degrees and 360u8 is the same as 0u8
pub fn cos_radians(radians: f64) -> f64 {
    let degrees = radians * (180.0 / core::f64::consts::PI);
    cos_degrees(degrees)
}

#[cfg(test)]
mod test {
    use crate::structs::trig::cos_degrees;

    #[test]
    fn test_cos_degrees() {
        const FRAC_1_SQRT_2: f64 = 0.7071067812;
        let cases = [
            (0.0, 1.0),
            (45.0, FRAC_1_SQRT_2),
            (90.0, 0.0),
            (135.0, -FRAC_1_SQRT_2),
            (180.0, -1.0),
            (225.0, -FRAC_1_SQRT_2),
            (270.0, 0.0),
            (315.0, FRAC_1_SQRT_2),
            (360.0, 1.0),
        ];
        for case in cases {
            assert_eq!(cos_degrees(case.0), case.1, "{}", case.0)
        }
    }
}
