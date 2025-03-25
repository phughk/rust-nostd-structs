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

    // Rotate each point around the centroid
    for p in points.iter_mut() {
        let dx = p.x - point2d.x;
        let dy = p.y - point2d.y;

        let rotated_x = dx * cos_theta - dy * sin_theta + point2d.x;
        let rotated_y = dx * sin_theta + dy * cos_theta + point2d.y;
        p.x = rotated_x;
        p.y = rotated_y;
    }
}
