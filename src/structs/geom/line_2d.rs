use crate::structs::geom::point_2d::Point2D;
#[cfg(feature = "helpers")]
use crate::structs::geom::PrintDesmos;
use crate::structs::AsType;
use arrayvec::ArrayString;
use core::ops::Neg;
use core::ops::{Add, Mul};
use core::ops::{Div, Sub};

/// A 2D line
#[derive(Debug, Clone, Copy)]
pub struct Line2D<T> {
    /// The 2 points of the line
    pub points: [Point2D<T>; 2],
}

impl<T> PartialEq for Line2D<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        (self.points[0] == other.points[0] && self.points[1] == other.points[1])
            || (self.points[0] == other.points[1] && self.points[1] == other.points[0])
    }
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
            + AsType<f32>
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
            let zero = T::from_type(0.0);
            let one = T::from_type(1.0);
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

    /// Get the length of the line in manhattan distance.
    /// This is favourable in situations where square root is unnecessary.
    pub fn length_manhattan(&self) -> T
    where
        T: Copy + Sub<Output = T> + PartialOrd + Neg<Output = T> + AsType<f32> + Add<Output = T>,
    {
        let dx = {
            let dx = self.points[0].x - self.points[1].x;
            if dx < T::from_type(0.0) {
                -dx
            } else {
                dx
            }
        };
        let dy = {
            let dy = self.points[0].y - self.points[1].y;
            if dy < T::from_type(0.0) {
                -dy
            } else {
                dy
            }
        };
        dx + dy
    }

    /// Returns the overlap of 2 lines
    pub fn overlap(&self, other: &Self) -> Option<Line2D<T>>
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
        let a0 = self.points[0];
        let a1 = self.points[1];
        let b0 = other.points[0];
        let b1 = other.points[1];

        // Direction of line A
        let d = a1 - a0;
        let d_dot = d.dot(&d);
        if d_dot == T::from_type(0.0) {
            return None; // self is a degenerate line
        }

        // Project all endpoints onto line A (relative to a0)
        let t = |p: Point2D<T>| -> T {
            let v = p - a0;
            v.dot(&d) / d_dot
        };

        let ta0 = T::from_type(0.0);
        let ta1 = T::from_type(1.0);
        let tb0 = t(b0);
        let tb1 = t(b1);

        // Sort projections to define intervals
        let (min_a, max_a) = if ta0 < ta1 { (ta0, ta1) } else { (ta1, ta0) };
        let (min_b, max_b) = if tb0 < tb1 { (tb0, tb1) } else { (tb1, tb0) };

        // Compute overlap interval
        let start_t = if min_a > min_b { min_a } else { min_b };
        let end_t = if max_a < max_b { max_a } else { max_b };

        if start_t >= end_t {
            return None;
        }

        // Reconstruct overlapping points along line A
        let start = a0 + d * start_t;
        let end = a0 + d * end_t;

        Some(Line2D::new(start, end))
    }

    /// Subtracts the overlap of 2 lines
    ///
    /// The remaining lines after subtraction are returned.
    ///
    /// Two values are returned if the line was subtracted in the middle.
    /// Otherwise, the left or right part of the line is returned if only partially covered.
    ///
    /// No lines are returned if the line being removed was bigger (started before, ended after) the source line.
    pub fn subtract(&self, other: &Self) -> (Option<Line2D<T>>, Option<Line2D<T>>)
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
        let diff = self.overlap(other);
        if let None = diff {
            return (Some(*self), None);
        }
        let diff = diff.unwrap();
        let start_is_different = {
            let p1 = diff.earlier();
            let p2 = self.earlier();
            p1.x != p2.x || p1.y != p2.y
        };
        let end_is_different = {
            let p1 = diff.later();
            let p2 = self.later();
            p1.x != p2.x || p1.y != p2.y
        };
        match (start_is_different, end_is_different) {
            (false, false) => {
                // This happens when the difference is the same as the line
                // So the entire line is subtracted, resulting in no lines
                (None, None)
            }
            (false, true) => {
                // We only take the first half
                (Some(Line2D::new(*diff.later(), *self.later())), None)
            }
            (true, false) => {
                // We only take the second half
                (Some(Line2D::new(*self.earlier(), *diff.earlier())), None)
            }
            (true, true) => {
                // We have 2 lines split by the overlap
                let first_half = Line2D::new(*self.earlier(), *diff.earlier());
                let second_half = Line2D::new(*diff.later(), *self.later());
                (Some(first_half), Some(second_half))
            }
        }
    }
}

#[cfg(feature = "helpers")]
impl<T: core::fmt::Display> PrintDesmos for Line2D<T> {
    fn to_string_desmos(&self) -> ArrayString<1024> {
        use core::fmt::Write;

        let mut ret = ArrayString::new();
        write!(
            &mut ret,
            "{}, {}",
            self.points[0].to_string_desmos(),
            self.points[1].to_string_desmos()
        )
        .unwrap();
        ret
    }
}

#[cfg(test)]
mod test {
    use crate::assert_float_equal_f64;
    #[cfg(feature = "helpers")]
    use crate::structs::geom::PrintDesmos;
    use crate::structs::geom::{Line2D, Point2D};
    use proptest::prelude::*;
    use std::println;

    #[test]
    fn test_overlap() {
        let l1 = Line2D::new(Point2D::new(-5.0, 1.0), Point2D::new(5.0, 1.0));
        let l2 = Line2D::new(Point2D::new(-7.0, -3.0), Point2D::new(4.0, 7.0));
        let overlap = l1.overlap(&l2);
        assert_eq!(
            overlap,
            Some(Line2D::new(Point2D::new(-5.0, 1.0), Point2D::new(4.0, 1.0)))
        );
    }

    #[test]
    fn test_overlap_vertical() {
        let l1 = Line2D::new(Point2D::new(-5.0, -10.0), Point2D::new(-5.0, 10.0));
        let l2 = Line2D::new(Point2D::new(-7.0, -3.0), Point2D::new(4.0, 7.0));
        let overlap = l1.overlap(&l2);
        assert_eq!(
            overlap,
            Some(Line2D::new(
                Point2D::new(-5.0, -3.0),
                Point2D::new(-5.0, 7.0)
            ))
        );
    }

    #[test]
    fn test_subtract_left() {
        let l1 = Line2D::new(Point2D::new(-5.0, 1.0), Point2D::new(5.0, 1.0));
        let l2 = Line2D::new(Point2D::new(-7.0, -3.0), Point2D::new(4.0, 7.0));
        let (sub1, sub2) = l1.subtract(&l2);
        assert_eq!(sub2, None);
        assert_eq!(
            sub1,
            Some(Line2D::new(Point2D::new(4.0, 1.0), Point2D::new(5.0, 1.0)))
        );
    }

    #[test]
    fn test_subtract_right() {
        let l1 = Line2D::new(Point2D::new(-5.0, 1.0), Point2D::new(5.0, 1.0));
        let l2 = Line2D::new(Point2D::new(-3.0, -3.0), Point2D::new(6.0, 7.0));
        let (sub1, sub2) = l1.subtract(&l2);
        assert_eq!(sub2, None);
        assert_eq!(
            sub1,
            Some(Line2D::new(
                Point2D::new(-5.0, 1.0),
                Point2D::new(-3.0, 1.0)
            ))
        );
    }

    #[test]
    fn test_subtract_middle() {
        let l1 = Line2D::new(Point2D::new(-5.0, 1.0), Point2D::new(5.0, 1.0));
        let l2 = Line2D::new(Point2D::new(-3.0, -3.0), Point2D::new(2.0, 7.0));
        let (sub1, sub2) = l1.subtract(&l2);
        assert_eq!(
            sub1,
            Some(Line2D::new(
                Point2D::new(-5.0, 1.0),
                Point2D::new(-3.0, 1.0)
            ))
        );
        assert_eq!(
            sub2,
            Some(Line2D::new(Point2D::new(2.0, 1.0), Point2D::new(5.0, 1.0)))
        )
    }

    #[test]
    fn test_subtract_vertical_left() {
        let l1 = Line2D::new(Point2D::new(-5.0, -3.0), Point2D::new(-5.0, 10.0));
        let l2 = Line2D::new(Point2D::new(-7.0, -4.0), Point2D::new(4.0, 7.0));
        let (sub1, sub2) = l1.subtract(&l2);
        assert_eq!(sub2, None);
        assert_eq!(
            sub1,
            Some(Line2D::new(
                Point2D::new(-5.0, 7.0),
                Point2D::new(-5.0, 10.0)
            ))
        );
    }

    #[test]
    fn test_subtract_vertical_right() {
        let l1 = Line2D::new(Point2D::new(5.0, 1.0), Point2D::new(5.0, 10.0));
        let l2 = Line2D::new(Point2D::new(-3.0, 4.0), Point2D::new(6.0, 12.0));
        let (sub1, sub2) = l1.subtract(&l2);
        assert_eq!(sub2, None);
        assert_eq!(
            sub1,
            Some(Line2D::new(Point2D::new(5.0, 1.0), Point2D::new(5.0, 4.0)))
        );
    }

    #[test]
    fn test_subtract_vertical_middle() {
        let l1 = Line2D::new(Point2D::new(5.0, -10.0), Point2D::new(5.0, 10.0));
        let l2 = Line2D::new(Point2D::new(-3.0, 4.0), Point2D::new(6.0, 7.0));
        let (sub1, sub2) = l1.subtract(&l2);
        assert_eq!(
            sub1,
            Some(Line2D::new(
                Point2D::new(5.0, -10.0),
                Point2D::new(5.0, 4.0)
            ))
        );
        assert_eq!(
            sub2,
            Some(Line2D::new(Point2D::new(5.0, 7.0), Point2D::new(5.0, 10.0)))
        );
    }

    #[test]
    fn test_subtract_larger() {
        let l1 = Line2D::new(Point2D::new(-5.0, -3.0), Point2D::new(5.0, 10.0));
        let l2 = Line2D::new(Point2D::new(-7.0, -4.0), Point2D::new(10.0, 20.0));
        let (sub1, sub2) = l1.subtract(&l2);
        assert_eq!(sub2, None);
        assert_eq!(sub1, None);
    }

    #[test]
    fn test_subtract_larger_vertical() {
        let l1 = Line2D::new(Point2D::new(-5.0, -3.0), Point2D::new(-5.0, 10.0));
        let l2 = Line2D::new(Point2D::new(-7.0, -4.0), Point2D::new(10.0, 20.0));
        let (sub1, sub2) = l1.subtract(&l2);
        assert_eq!(sub2, None);
        assert_eq!(sub1, None);
    }

    #[test]
    fn weird_case_for_dot_product() {
        let case = OverlapTest {
            p1x: 0.0,
            p1y: 0.0,
            p2x: 12.197849174953443,
            p2y: 24.83333321365357,
            tx: 1,
            ty: 14,
        };
        // TODO the data is correct, the test is invalid
        // Time to flip the logic and handle left|right|center
        test_prop_left_impl(case);
    }

    #[derive(Debug, Clone, Copy)]
    struct OverlapTest {
        p1x: f64,
        p1y: f64,
        p2x: f64,
        p2y: f64,
        // Percentage of tx/u8::MAX
        tx: u8,
        // Percentage of ty/u8::MAX
        ty: u8,
    }

    fn generate_overlap_test() -> BoxedStrategy<OverlapTest> {
        let p1x = -30.0f64..=30.0f64;
        let p1y = -30.0f64..=30.0f64;
        let p2x = -30.0f64..=30.0f64;
        let p2y = -30.0f64..=30.0f64;
        let tx = 1u8..u8::MAX;
        let ty = 1u8..u8::MAX;
        (p1x, p1y, p2x, p2y, tx, ty)
            .prop_filter(
                "Points cannot be the same",
                |(p1x, p1y, p2x, p2y, _tx, _ty)| p1x != p2x && p1y != p2y,
            )
            .prop_map(|(p1x, p1y, p2x, p2y, tx, ty)| OverlapTest {
                p1x,
                p1y,
                p2x,
                p2y,
                tx,
                ty,
            })
            .boxed()
    }

    fn test_prop_left_impl(case: OverlapTest) {
        let p1 = Point2D::new(case.p1x, case.p1y);
        let p2 = Point2D::new(case.p2x, case.p2y);
        let l1 = Line2D::new(p1, p2);
        let dx = (p2.x - p1.x).abs();
        let dy = (p2.y - p1.y).abs();
        let tx = (case.tx as f64 / u8::MAX as f64) * dx;
        let ty = (case.ty as f64 / u8::MAX as f64) * dy;
        let p3 = Point2D::new(p1.x + tx, p1.y + ty);
        let p4 = Point2D::new(p2.x + tx, p2.y + ty);
        let l2 = Line2D::new(p3, p4);
        let overlap = l1.overlap(&l2);
        let overlap = overlap.expect("There should be overlap");
        // Precision must be flexible
        let precision = {
            let vals = [
                p1.x.abs(),
                p1.y.abs(),
                p2.x.abs(),
                p2.y.abs(),
                p3.x.abs(),
                p3.y.abs(),
                p4.x.abs(),
                p4.y.abs(),
            ];
            let mut max = 0;
            for i in 0..vals.len() {
                if vals[i] > vals[max] {
                    max = i;
                }
            }
            // 5% of largest value
            vals[max] * 0.05
        };
        #[cfg(feature = "helpers")]
        {
            println!("Line 1: {}", l1.to_string_desmos());
            println!("Line 2: {}", l2.to_string_desmos());
            println!("Overlap: {}", overlap.to_string_desmos());
        }
        assert_float_equal_f64(overlap.points[0].x, p1.x, precision);
        assert_float_equal_f64(overlap.points[0].y, p1.y, precision);
        assert_float_equal_f64(overlap.points[1].x, p4.x, precision);
        assert_float_equal_f64(overlap.points[1].y, p4.y, precision);
    }

    #[test]
    fn test_prop_left_overlap_one() {
        let case = OverlapTest {
            p1x: 0.0,
            p1y: 0.0,
            p2x: 32.36723,
            p2y: -61.939075,
            tx: 1,
            ty: 1,
        };
        test_prop_left_impl(case);
    }

    proptest! {

        #[test]
        fn test_prop_left( case in generate_overlap_test() ) {
            test_prop_left_impl(case);
        }

    }
}
