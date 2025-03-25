use crate::structs::geom::Point2D;
use crate::structs::trig::{cos_degrees, sin_degrees};
use core::ops::{Add, Mul, Sub};

pub(super) fn rotate_deg_mut<
    T: Into<f64> + Copy + Sub<Output = T> + Mul<Output = T> + From<f32> + Add<Output = T>,
>(
    points: &mut [Point2D<T>],
    point2d: Point2D<T>,
    degrees: T,
) {
    let cos_theta = T::from(cos_degrees(degrees.into()) as f32);
    let sin_theta = T::from(sin_degrees(degrees.into()) as f32);

    // Rotate each point around the provided point
    for p in points.iter_mut() {
        let dx = p.x - point2d.x;
        let dy = p.y - point2d.y;

        let rotated_x = dx * cos_theta - dy * sin_theta + point2d.x;
        let rotated_y = dx * sin_theta + dy * cos_theta + point2d.y;
        p.x = rotated_x;
        p.y = rotated_y;
    }
}

pub(super) fn polygon_area_shoelace<T>(points: &[Point2D<T>]) -> T
where
    T: Mul<Output = T> + Sub<Output = T> + Copy + From<f32> + Add<Output = T> + PartialOrd,
{
    let n = points.len();
    let zero = T::from(0.0);
    if n < 3 {
        return zero;
    }

    let mut sum = T::from(0.0);
    for i in 0..n {
        let p1 = &points[i];
        let p2 = &points[(i + 1) % n];
        sum = sum + (p1.x * p2.y - p2.x * p1.y);
    }
    let sum = {
        if sum < zero {
            zero - sum
        } else {
            sum
        }
    };
    let half = T::from(0.5);

    half * sum
}
