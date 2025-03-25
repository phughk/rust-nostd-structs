use crate::structs::trig::{trig_from_lut, AsType};
use core::ops::{Add, Mul};

/// Get the sin value from literal degrees, so 45u8 for 45 degrees and 360u8 is the same as 0u8
pub fn cos_degrees<T>(degrees: T) -> T
where
    T: AsType<f32> + Add<Output = T> + Mul<Output = T> + Copy + PartialOrd,
{
    trig_from_lut(degrees, &super::cos_lut::COS_LUT)
}

#[cfg(test)]
mod test {
    use crate::structs::trig::cos_degrees;

    #[test]
    fn test_cos_degrees() {
        const _FRAC_1_SQRT_2_F64_LUT: f64 = 0.7071067812;
        const FRAC_1_SQRT_2_F32_LUT: f64 = 0.7071067690849304;
        let cases = [
            (0.0, 1.0),
            (45.0, FRAC_1_SQRT_2_F32_LUT),
            (90.0, 0.0),
            (135.0, -FRAC_1_SQRT_2_F32_LUT),
            (180.0, -1.0),
            (225.0, -FRAC_1_SQRT_2_F32_LUT),
            (270.0, 0.0),
            (315.0, FRAC_1_SQRT_2_F32_LUT),
            (360.0, 1.0),
        ];
        for case in cases {
            assert_eq!(cos_degrees(case.0), case.1, "{}", case.0)
        }
    }
}
