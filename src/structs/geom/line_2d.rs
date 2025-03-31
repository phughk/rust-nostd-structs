use crate::structs::algebra::LinearEquation;
use crate::structs::geom::point_2d::Point2D;
use crate::structs::AsType;
use core::ops::Neg;
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

    /// Returns the spatially earlier of the two points
    pub fn earlier(&self) -> &Point2D<T>
    where
        T: PartialOrd,
    {
        &self.points[self.earlier_index()]
    }

    /// Returns the spatially later of the two points
    pub fn later(&self) -> &Point2D<T>
    where
        T: PartialOrd,
    {
        &self.points[1 - self.earlier_index()]
    }

    fn earlier_index(&self) -> usize
    where
        T: PartialOrd,
    {
        if self.points[0].x < self.points[1].x {
            0usize
        } else if self.points[0].x > self.points[1].x {
            1
        } else {
            if self.points[0].y < self.points[1].y {
                0
            } else {
                1
            }
        }
    }

    /// Dot product of the 2 points on the line
    pub fn dot(&self) -> T
    where
        T: Mul<Output = T> + Add<Output = T> + Copy,
    {
        self.points[0].dot(&self.points[1])
    }

    /// Returns the remaining line and an additional line if the split happened in the middle
    /// No lines are returned if there is no overlap
    pub fn overlap(&self, other: &Self) -> (Option<Line2D<T>>, Option<Line2D<T>>)
    where
        T: Copy
            + Sub<Output = T>
            + Div<Output = T>
            + Mul<Output = T>
            + Add<Output = T>
            + Neg<Output = T>
            + PartialEq
            + PartialOrd
            + AsType<f32>,
    {
        let plane = LinearEquation::from_2_points(
            (self.points[0].x, self.points[0].y),
            (self.points[1].x, self.points[1].y),
        );
        let projected_start = plane.project_onto(other.points[0].x, other.points[0].y);
        let projected_end = plane.project_onto(other.points[1].x, other.points[1].y);
        let projected = Line2D::new(
            Point2D::new(projected_start.0, projected_start.1),
            Point2D::new(projected_end.0, projected_end.1),
        );
        let sizes = [
            self.points[0].x,
            self.points[1].x,
            projected.points[0].x,
            projected.points[1].x,
        ];
        let mut smallest = sizes[0];
        let mut largest = sizes[0];
        for candidate in 1..4 {
            if sizes[candidate] < smallest {
                smallest = sizes[candidate];
            }
            if sizes[candidate] > largest {
                largest = sizes[candidate];
            }
        }
        let starts_after = smallest < self.earlier().x;
        let ends_before = largest > self.later().x;
        match (starts_after, ends_before) {
            (true, true) => (
                // The entire line is inside the other line
                Some(Line2D {
                    points: self.points,
                }),
                None,
            ),
            (false, false) => (
                // The other line splits this in 2
                Some(Line2D {
                    points: [*self.earlier(), *projected.earlier()],
                }),
                Some(Line2D {
                    points: [*projected.later(), *self.later()],
                }),
            ),
            (true, false) => (
                Some(Line2D {
                    points: [*self.earlier(), *projected.earlier()],
                }),
                None,
            ),
            (false, true) => todo!(),
        }
    }
}
