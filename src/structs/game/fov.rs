//! Field of view tracking
//!

use crate::structs::algebra::LinearEquation;
use crate::structs::geom::{Line2D, Polygon2D, Shape2D};
use crate::structs::AsType;
use arrayvec::ArrayVec;
use core::ops::*;

/// A way for tracking field of view from one shape to another.
///
/// SZ is the size of maximum number of tracked fields
/// i.e. if a field of view gets split into 2, they need to be tracked separately.
///
/// When a field of view is split and there are not enough free slots, the smallest field of view is removed.
pub struct FieldOfView<const SZ: usize, T> {
    /// The remaining field of view lines indicating remaining visibility
    tracked_fields: ArrayVec<Line2D<T>, SZ>,
    /// The plane which we are plotting our blocking views onto
    /// It could be derived from the Polygon, but we shouldn't rely on it having that structure
    /// For example if we need to turn it into a convex hull (TODO)
    target_plane: LinearEquation<T>,
    /// The surface from the source to the target.
    /// We use this to determine if a blocking object is within the visibility range.
    /// This prevents calculating objects behind the target, behind the source, or in an irrelevant angle.
    surface: Polygon2D<4, T>,
}

impl<const SZ: usize, T> FieldOfView<SZ, T>
where
    T: Copy + PartialOrd,
{
    /// Create a new field of view from a source shape onto a target shape
    pub fn new<const SZ1: usize, S1: Shape2D<SZ1, T>, const SZ2: usize, S2: Shape2D<SZ2, T>>(
        source: &S1,
        target: &S2,
    ) -> FieldOfView<SZ, T>
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
        FieldOfView {
            tracked_fields,
            target_plane: tgt_plane,
            surface,
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
            match remaining {
                None => {
                    // We must remove this field of view as it was entirely blocked
                    self.tracked_fields.remove(i);
                    i -= 1;
                }
                Some(v) => {
                    let to_replace = self.tracked_fields.get_mut(i).unwrap();
                    *to_replace = v;
                }
            }
            i += 1;
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
}
