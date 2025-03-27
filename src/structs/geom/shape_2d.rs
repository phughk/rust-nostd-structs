use super::misc::convex_hull;
use crate::structs::algebra::LinearEquation;
use crate::structs::geom::point_2d::Point2D;
use crate::structs::geom::{Line2D, Polygon2D};
use crate::structs::AsType;
use arrayvec::ArrayVec;
use core::ops::{Add, Div, Mul, Neg, Sub};

/// Methods for handling a shape in 2D space
pub trait Shape2D<const SZ: usize, T> {
    /// Rotate the shape by a number of degrees using f32 calculations
    fn rotate_deg(mut self, point: Point2D<T>, degrees: T) -> Self
    where
        Self: Sized,
    {
        self.rotate_rad_mut(point, degrees);
        self
    }

    /// Rotate the shape by a number of degrees using f32 calculations
    fn rotate_deg_mut(&mut self, point: Point2D<T>, degrees: T);

    /// Rotate the shape by a number of radians using f32 calculations
    fn rotate_rad(mut self, point: Point2D<T>, radians: T) -> Self
    where
        Self: Sized,
    {
        self.rotate_rad_mut(point, radians);
        self
    }

    /// Rotate the shape by a number of radians using f32 calculations
    fn rotate_rad_mut(&mut self, point: Point2D<T>, radians: T);

    /// Get the area of the shape
    fn surface(&self) -> T
    where
        T: Mul<Output = T> + Sub<Output = T> + Copy + From<f32> + Add<Output = T> + PartialOrd,
    {
        super::misc::polygon_area_shoelace(self.points())
    }

    /// Center-point of the shape
    fn center(&self) -> Point2D<T>;

    /// Closest point on the shape to a given point
    fn closest_point(&self, point: Point2D<T>) -> Point2D<T>;

    /// True if the provide point is in the shape
    fn point_in_shape(&self, point: Point2D<T>) -> bool;

    /// Axis-aligned bounding box of the shape
    fn axis_aligned_bounding_box(&self) -> Polygon2D<4, T>
    where
        T: PartialOrd + Copy,
    {
        let points = self.points();
        let p1 = &points[0];
        let mut min_x = p1.x;
        let mut max_x = p1.x;
        let mut min_y = p1.y;
        let mut max_y = p1.y;
        for i in 1..points.len() {
            let p = &points[i];
            if p.x < min_x {
                min_x = p.x;
            } else if p.x > max_x {
                max_x = p.x;
            }
            if p.y < min_y {
                min_y = p.y;
            } else if p.y > max_y {
                max_y = p.y;
            }
        }
        Polygon2D {
            points: ArrayVec::from([
                Point2D::new(min_x, min_y),
                Point2D::new(max_x, min_y),
                Point2D::new(max_x, max_y),
                Point2D::new(min_x, max_y),
            ]),
        }
    }

    /// Convex hull of this shape
    fn convex_hull(&self) -> Polygon2D<SZ, T>
    where
        T: Copy
            + PartialOrd
            + From<f32>
            + Sub<Output = T>
            + Mul<Output = T>
            + Add<Output = T>
            + Div<Output = T>,
    {
        let mut arr = ArrayVec::new();
        for p in self.points() {
            arr.push(*p);
        }
        convex_hull(arr)
    }

    /// Convex hull from 2 shapes
    fn convex_hull_with_other_shape<
        const NEW_SZ: usize,
        const OTHER_SZ: usize,
        SHAPE: Shape2D<OTHER_SZ, T>,
    >(
        &self,
        other_shape: &SHAPE,
    ) -> Polygon2D<NEW_SZ, T>
    where
        T: Copy
            + PartialOrd
            + From<f32>
            + Sub<Output = T>
            + Mul<Output = T>
            + Add<Output = T>
            + Div<Output = T>,
    {
        let mut arr = ArrayVec::new();
        for p in self.points() {
            arr.push(*p);
        }
        for p in other_shape.points() {
            arr.push(*p);
        }
        convex_hull(arr)
    }

    /// Points of the shape
    fn points(&self) -> &[Point2D<T>];

    /// Edges of the shape
    fn edges(&self) -> ArrayVec<Line2D<T>, SZ>;

    /// Project this polygon onto another shape
    fn project_onto_polygon<const N: usize, SHAPE: Shape2D<N, T>>(&self, other: &SHAPE) -> Line2D<T>
    where
        T: Copy
            + Sub<Output = T>
            + Div<Output = T>
            + Mul<Output = T>
            + Add<Output = T>
            + Neg<Output = T>
            + PartialEq
            + AsType<f32>
            + PartialOrd,
    {
        // Get a vector between the 2 shapes
        let self_center = self.center();
        let other_center = other.center();
        let line_between = LinearEquation::from_2_points(
            (self_center.x, self_center.y),
            (other_center.x, other_center.y),
        );

        // Derive the plane we are projecting onto
        let plane = line_between.orthogonal_at_point(other_center.x, other_center.y);

        // Get the 2D line of the target surface
        let other_points = other.points();
        let other_projection = project_onto_plane(other_points, &plane);

        // Get the 2D line of this polygon onto the surface
        let self_points = self.points();
        let self_projection = project_onto_plane(self_points, &plane);

        todo!()
    }
}

fn project_onto_plane<T>(points: &[Point2D<T>], plane: &LinearEquation<T>) -> Line2D<T>
where
    T: Copy
        + Sub<Output = T>
        + Div<Output = T>
        + Mul<Output = T>
        + Add<Output = T>
        + Neg<Output = T>
        + PartialEq
        + AsType<f32>
        + PartialOrd,
{
    let (x, _) = plane.project_onto(points[0].x, points[0].y);
    let mut x_min = x;
    let mut x_max = x;
    for i in 1..points.len() {
        let (x, _) = plane.project_onto(points[i].x, points[i].y);
        if x < x_min {
            x_min = x;
        } else if x > x_max {
            x_max = x;
        }
    }
    let y_min = plane.y(x_min).unwrap();
    let y_max = plane.y(x_max).unwrap();
    Line2D::new(Point2D::new(x_min, y_min), Point2D::new(x_max, y_max))
}
