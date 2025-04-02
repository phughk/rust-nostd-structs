//! Field of view tracking
//!

use crate::structs::algebra::LinearEquation;
use crate::structs::geom::{Line2D, Polygon2D, Shape2D};
use crate::structs::AsType;
use arrayvec::{ArrayString, ArrayVec};
use core::fmt::Write;
use core::ops::*;

/// A way for tracking line of sight from one shape to another.
///
/// SZ is the size of maximum number of tracked fields of view
/// i.e. if a field of view gets split into 2, they need to be tracked separately.
///
/// When a field of view is split and there are not enough free slots, the smallest field of view is removed.
pub struct LineOfSight<const SZ: usize, T> {
    /// The remaining field of view lines indicating remaining visibility
    pub tracked_fields: ArrayVec<Line2D<T>, SZ>,
    /// The plane which we are plotting our blocking views onto
    /// It could be derived from the Polygon, but we shouldn't rely on it having that structure
    /// For example if we need to turn it into a convex hull (TODO)
    pub target_plane: LinearEquation<T>,
    /// The surface from the source to the target.
    /// We use this to determine if a blocking object is within the visibility range.
    /// This prevents calculating objects behind the target, behind the source, or in an irrelevant angle.
    pub surface: Polygon2D<4, T>,
    /// The original manhattan distance of the line of sight
    /// We track this value to see if the line of sight was obscured at all
    pub original_distance: T,
}

impl<const SZ: usize, T> LineOfSight<SZ, T>
where
    T: Copy + PartialOrd,
{
    /// Create a new field of view from a source shape onto a target shape
    pub fn new<const SZ1: usize, S1: Shape2D<SZ1, T>, const SZ2: usize, S2: Shape2D<SZ2, T>>(
        source: &S1,
        target: &S2,
    ) -> LineOfSight<SZ, T>
    where
        S1: core::fmt::Debug,
        S2: core::fmt::Debug,
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
        #[cfg(debug_assertions)]
        {
            extern crate std;
            let mut s: ArrayString<2048> = ArrayString::new();
            write!(s, "Source: {:?}", source).unwrap();
            write!(s, "\nTarget: {:?}", target).unwrap();
            std::println!("{}", s);
        }
        let sc = source.center();
        let tc = target.center();
        let direction_plane = LinearEquation::from_2_points((sc.x, sc.y), (tc.x, tc.y));
        let src_plane = direction_plane.orthogonal_at_point(sc.x, sc.y);
        let src_line = source.project_onto_plane(&src_plane);
        let tgt_plane = direction_plane.orthogonal_at_point(tc.x, tc.y);
        let tgt_line = target.project_onto_plane(&tgt_plane);
        let fov_line = source.project_onto_plane(&tgt_plane);
        let tgt_line = tgt_line.overlap(&fov_line);
        // We can unwrap, because there should always be overlap of 2 planes across the same axis
        let tgt_line = tgt_line.unwrap();
        let surface = Polygon2D {
            points: ArrayVec::from([
                tgt_line.points[0],
                tgt_line.points[1],
                src_line.points[1],
                src_line.points[0],
            ]),
        };

        let mut tracked_fields = ArrayVec::new();
        tracked_fields.push(tgt_line);
        LineOfSight {
            tracked_fields,
            target_plane: tgt_plane,
            surface,
            original_distance: tgt_line.length_manhattan(),
        }
    }

    /// Update the field of view to reflect being blocked by an obstacle
    ///
    /// This will always modify the state.
    ///
    /// An error indicates the view is now blocked.
    /// No error means that blocking with the object retained some field of view.
    pub fn block_view<const SZ2: usize, S: Shape2D<SZ2, T>>(&mut self, other: &S) -> Result<(), ()>
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
        // If we have no more fields of view, then return error - field of view is blocked
        if self.tracked_fields.is_empty() {
            return Err(());
        }
        // Check if blocker is in FOV surface
        let mut blocked = false;
        for point in other.points() {
            if self.surface.point_in_shape(point) {
                // We have a point in the surface, so we can block
                blocked = true;
                break;
            }
        }
        if !blocked {
            // We have no point in the surface, so we can ignore this blocker
            return Ok(());
        }
        // We retain split fields separately, and add them later
        let mut blocked: ArrayVec<Line2D<T>, SZ> = ArrayVec::new();
        // Update the fields with any blocked view
        let mut i = 0;
        while i < self.tracked_fields.len() {
            let field_of_view = &self.tracked_fields[i];
            let blocked_view_line = other.project_onto_plane(&self.target_plane);
            let (remaining, other) = field_of_view.subtract(&blocked_view_line);
            if other.is_some() {
                blocked.push(other.unwrap())
            }
            let mut add_one = true;
            match remaining {
                None => {
                    // We must remove this field of view as it was entirely blocked
                    self.tracked_fields.remove(i);
                    add_one = false;
                }
                Some(v) => {
                    let to_replace = self.tracked_fields.get_mut(i).unwrap();
                    *to_replace = v;
                }
            }
            if add_one {
                i += 1;
            }
        }
        // Add all remaining lines
        for blocker in blocked {
            if !self.tracked_fields.is_full() {
                // We can add safely
                self.tracked_fields.push(blocker);
            } else {
                // We need to replace the smallest fov being tracked
                let (smallest, distance) = self
                    .tracked_fields
                    .iter()
                    // TODO is manhattan safe?
                    .map(|l| l.length_manhattan())
                    .enumerate()
                    .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
                    .unwrap();
                if blocker.length_manhattan() > distance {
                    let replace = self.tracked_fields.get_mut(smallest).unwrap();
                    *replace = blocker;
                }
            }
        }
        if self.tracked_fields.is_empty() {
            // We have no more fov to track, so the line of sight is blocked
            Err(())
        } else {
            Ok(())
        }
    }

    /// True, if the line of sight was at least partially blocked
    pub fn partially_blocked(&self) -> bool
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
        if self.tracked_fields.len() != 1 {
            return true;
        }
        self.tracked_fields[0].length_manhattan() < self.original_distance
    }

    /// True if the line of sight is completely blocked
    pub fn totally_blocked(&self) -> bool {
        self.tracked_fields.is_empty()
    }
}

#[cfg(test)]
mod test {
    use crate::structs::game::los::LineOfSight;
    use crate::structs::geom::{Point2D, Polygon2D};
    use arrayvec::ArrayVec;

    #[test]
    fn test_fov_subtract_with_overflow() {
        // Write a test that ends up with zero fov
        // i-=1 before removing the element would result in invalid usize = -1
        // Fixing without test currently
    }

    #[test]
    fn test_weird_case() {
        let source = Polygon2D {
            points: ArrayVec::from([
                Point2D { x: 13.7, y: -10.8 },
                Point2D {
                    x: 20.8,
                    y: -2.6000004,
                },
                Point2D {
                    x: 13.699999,
                    y: -1.4000003,
                },
                Point2D {
                    x: 19.599998,
                    y: 7.999999,
                },
            ]),
        };
        let target = Polygon2D {
            points: ArrayVec::from([
                Point2D { x: -7.9, y: 9.2 },
                Point2D {
                    x: -0.8000002,
                    y: 4.6,
                },
                Point2D { x: -5.5, y: 11.4 },
                Point2D {
                    x: 0.4000001,
                    y: 1.5999994,
                },
            ]),
        };
        let _los_should_not_fail: LineOfSight<5, f32> = LineOfSight::new(&source, &target);
    }
}
