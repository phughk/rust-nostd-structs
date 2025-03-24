use crate::structs::geom::point_2d::Point2D;
use crate::structs::geom::{Line2D, Polygon2D};

/// Methods for handling a shape in 2D space
pub trait Shape2D<const SZ: usize, T> {
    /// Rotate the shape by a number of degrees using f32 calculations
    fn rotate_deg(self, degrees: f64) -> Self;
    /// Rotate the shape by a number of degrees using f32 calculations
    fn rotate_deg_mut(&mut self, degrees: f64);

    /// Rotate the shape by a number of radians using f32 calculations
    fn rotate_rad(self, radians: T) -> Self;
    /// Rotate the shape by a number of radians using f32 calculations
    fn rotate_rad_mut(&mut self, radians: T);

    /// Get the area of the shape using f32 calculations
    fn surface(&self) -> T;

    /// Center-point of the shape
    fn center(&self) -> Point2D<T>;

    /// Closest point on the shape to a given point
    fn closest_point(&self, point: Point2D<T>) -> Point2D<T>;

    /// True if the provide point is in the shape
    fn point_in_shape(&self) -> bool;

    /// Axis-aligned bounding box of the shape
    fn axis_aligned_bounding_box(&self) -> Polygon2D<4, T>;

    /// Convex hull of the shape
    fn convex_hull<const NewSZ: usize>(&self) -> Polygon2D<NewSZ, T>;

    /// Convex hull from 2 shapes
    fn convex_hull_with_other_shape<
        const NEW_SZ: usize,
        const OTHER_SZ: usize,
        SHAPE: Shape2D<OTHER_SZ, T>,
    >(
        &self,
        other_shape: SHAPE,
    ) -> Polygon2D<NEW_SZ, T>;

    /// Points of the shape
    fn points(&self) -> &[Point2D<T>];

    /// Edges of the shape
    fn edges(&self) -> [Line2D<T>; SZ];
}
