use crate::structs::geom::point_2d::Point2D;
use crate::structs::geom::{Line2D, Polygon2D};
use arrayvec::ArrayVec;

/// Methods for handling a shape in 2D space
pub trait Shape2D<const SZ: usize, T> {
    /// Rotate the shape by a number of degrees using f32 calculations
    fn rotate_deg(self, point: Point2D<T>, degrees: T) -> Self;
    /// Rotate the shape by a number of degrees using f32 calculations
    fn rotate_deg_mut(&mut self, point: Point2D<T>, degrees: T);

    /// Rotate the shape by a number of radians using f32 calculations
    fn rotate_rad(self, point: Point2D<T>, radians: T) -> Self;
    /// Rotate the shape by a number of radians using f32 calculations
    fn rotate_rad_mut(&mut self, point: Point2D<T>, radians: T);

    /// Get the area of the shape using f32 calculations
    fn surface(&self) -> T;

    /// Center-point of the shape
    fn center(&self) -> Point2D<T>;

    /// Closest point on the shape to a given point
    fn closest_point(&self, point: Point2D<T>) -> Point2D<T>;

    /// True if the provide point is in the shape
    fn point_in_shape(&self, point: Point2D<T>) -> bool;

    /// Axis-aligned bounding box of the shape
    fn axis_aligned_bounding_box(&self) -> Polygon2D<4, T>;

    /// Turns this polygon into a convex hull
    fn as_convex_hull(self) -> Polygon2D<SZ, T>;

    /// Convex hull from 2 shapes
    fn convex_hull_with_other_shape<
        const NEW_SZ: usize,
        const OTHER_SZ: usize,
        SHAPE: Shape2D<OTHER_SZ, T>,
    >(
        &self,
        other_shape: SHAPE,
    ) -> Polygon2D<NEW_SZ, T>;

    /// True, if this shape overlaps with another shape
    fn overlaps<OtherShape: Shape2D<OtherSz, T>, const OtherSz: usize>(
        &self,
        other_shape: &OtherShape,
    ) -> bool {
        let center = self.center();
        let mut radius = 0.0;
        for point in self.points() {
            let dist = center.distance(point);
            if dist < radius {
                radius = dist;
            }
        }
        // Now check distance of other shape to this shape
        for point in other_shape.points() {
            center.distance_squared()
            let dist = center.distance(point);
            if dist < radius {
                return true;
            }
        }
        false
    }

    /// Points of the shape
    fn points(&self) -> &[Point2D<T>];

    /// Edges of the shape
    fn edges(&self) -> ArrayVec<Line2D<T>, SZ>;
}
