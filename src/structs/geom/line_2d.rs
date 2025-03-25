use crate::structs::geom::point_2d::Point2D;
use core::ops::{Add, Mul};
use core::ops::{Div, Sub};

/// A 2D line
pub struct Line2D<T> {
    /// The 2 points of the line
    pub points: [Point2D<T>; 2],
}

impl<T> Line2D<T> {
    /// Create a new line from 2 points
    pub fn new(p1: Point2D<T>, p2: Point2D<T>) -> Self {
        Line2D { points: [p1, p2] }
    }

    /// Closest point on the line to a given point
    pub fn closest_point_on_segment(&self, point: &Point2D<T>) -> Point2D<T>
    where
        T: Copy
            + PartialOrd
            + From<u8>
            + Sub<Output = T>
            + Div<Output = T>
            + Mul<Output = T>
            + Add<Output = T>,
    {
        let a = &self.points[0];
        let b = &self.points[1];
        let ab = b - a;
        let ap = point - a;

        let t = ap.dot(&ab) / ab.dot(&ab);

        let t_clamped = {
            let zero = T::from(0u8);
            let one = T::from(1u8);
            if t < zero {
                zero
            } else if t > one {
                one
            } else {
                t
            }
        };

        Point2D {
            x: a.x + ab.x * t_clamped,
            y: a.y + ab.y * t_clamped,
        }
    }

    /// Dot product of the 2 points on the line
    pub fn dot(&self) -> T
    where
        T: Mul<Output = T> + Add<Output = T> + Copy,
    {
        self.points[0].dot(&self.points[1])
    }
}
