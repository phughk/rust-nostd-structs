use crate::structs::geom::point_2d::Point2D;

/// An N-polygon in 2D space
pub struct Polygon2D<const N: usize, T> {
    /// Points of the polygon
    pub points: [Point2D<T>; N],
}
