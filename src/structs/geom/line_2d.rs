use crate::structs::geom::point_2d::Point2D;

/// A 2D line
pub struct Line2D<T> {
    /// First point
    p1: Point2D<T>,
    /// Second point
    p2: Point2D<T>,
}
