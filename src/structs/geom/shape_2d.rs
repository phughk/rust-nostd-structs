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
        T: AsType<f64>
            + Copy
            + Sub<Output = T>
            + Mul<Output = T>
            + AsType<f32>
            + Add<Output = T>
            + PartialOrd,
    {
        self.rotate_deg_mut(point, degrees);
        self
    }

    /// Rotate the shape by a number of degrees using f32 calculations
    fn rotate_deg_mut(&mut self, point: Point2D<T>, degrees: T)
    where
        T: AsType<f64>
            + Copy
            + Sub<Output = T>
            + Mul<Output = T>
            + AsType<f32>
            + Add<Output = T>
            + PartialOrd;

    /// Rotate the shape by a number of radians using f32 calculations
    fn rotate_rad(mut self, point: Point2D<T>, radians: T) -> Self
    where
        Self: Sized,
        T: AsType<f64>
            + Copy
            + Sub<Output = T>
            + Mul<Output = T>
            + AsType<f32>
            + Add<Output = T>
            + PartialOrd,
    {
        self.rotate_rad_mut(point, radians);
        self
    }

    /// Rotate the shape by a number of radians using f32 calculations
    fn rotate_rad_mut(&mut self, point: Point2D<T>, radians: T)
    where
        T: AsType<f64>
            + Copy
            + Sub<Output = T>
            + Mul<Output = T>
            + AsType<f32>
            + Add<Output = T>
            + PartialOrd,
    {
        let deg = crate::structs::trig::radians_to_degrees(radians);
        self.rotate_deg_mut(point, deg);
    }

    /// Get the area of the shape
    fn surface(&self) -> T
    where
        T: Mul<Output = T> + Sub<Output = T> + Copy + From<f32> + Add<Output = T> + PartialOrd,
    {
        super::misc::polygon_area_shoelace(self.points())
    }

    /// Center-point of the shape
    fn center(&self) -> Point2D<T>
    where
        T: Copy + Mul<Output = T> + Add<Output = T> + Sub<Output = T> + AsType<f32>,
    {
        let points = self.points();
        let n = points.len();
        assert!(n >= 3, "A polygon must have at least 3 points");

        let mut area = T::from_type(0.0);
        let mut cx = T::from_type(0.0);
        let mut cy = T::from_type(0.0);

        for i in 0..n {
            let p0 = &points[i];
            let p1 = &points[(i + 1) % n];

            let cross = p0.x * p1.y - p1.x * p0.y;
            area = area + cross;
            cx = cx + ((p0.x + p1.x) * cross);
            cy = cy + ((p0.y + p1.y) * cross);
        }

        area = area * T::from_type(0.5);
        let factor = T::from_type(1.0 / (6.0 * area.into_type()));

        Point2D {
            x: cx * factor,
            y: cy * factor,
        }
    }

    /// Closest point on the shape to a given point
    fn closest_point(&self, point: &Point2D<T>) -> Point2D<T>
    where
        T: Copy
            + PartialOrd
            + Mul<Output = T>
            + Add<Output = T>
            + Sub<Output = T>
            + AsType<f32>
            + Div<Output = T>
            + Mul<Output = T>
            + Add<Output = T>;

    /// True if the provide point is in the shape
    fn point_in_shape(&self, point: &Point2D<T>) -> bool
    where
        T: Copy
            + Sub<Output = T>
            + Mul<Output = T>
            + Add<Output = T>
            + Div<Output = T>
            + PartialOrd
            + AsType<f32>,
    {
        let points = self.points();
        let n = points.len();

        let mut inside = false;
        for i in 0..n {
            let a = &points[i];
            let b = &points[(i + 1) % n];

            let (px, py) = (point.x, point.y);
            let (x1, y1) = (a.x, a.y);
            let (x2, y2) = (b.x, b.y);

            let intersect = ((y1 > py) != (y2 > py))
                && (px < (x2 - x1) * (py - y1) / (y2 - y1 + T::from_type(f32::EPSILON)) + x1);
            if intersect {
                inside = !inside;
            }
        }

        inside
    }

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
    fn edges(&self) -> ArrayVec<Line2D<T>, SZ>
    where
        T: Copy;

    /// Create a projection of the shape onto a point
    fn project_onto_point(&self, point: &Point2D<T>) -> Line2D<T>
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
        let center = self.center();
        let line = LinearEquation::from_2_points((center.x, center.y), (point.x, point.y));
        let target_orth = line.orthogonal_at_point(point.x, point.y);
        self.project_onto_plane(&target_orth)
    }

    /// Project this polygon onto a plane creating a line of it's projected surface
    fn project_onto_plane(&self, plane: &LinearEquation<T>) -> Line2D<T>
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
        let points = self.points();
        let mut start = None;
        let mut end = None;
        for point in points {
            let p = plane.project_onto(point.x, point.y);
            let p = Point2D::new(p.0, p.1);
            if start.is_none() {
                start = Some(p);
            }
            if end.is_none() {
                end = Some(p);
            }
            if let Some(s) = &start {
                if &p < s {
                    start = Some(p);
                }
            }
            if let Some(e) = &end {
                if &p > e {
                    end = Some(p);
                }
            }
        }
        let start = start.unwrap();
        let end = end.unwrap();
        Line2D::new(start, end)
    }

    /// Project this polygon onto another shape
    fn project_onto_shape<const N: usize, SHAPE: Shape2D<N, T>>(&self, other: &SHAPE) -> Line2D<T>
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
        let _other_projection = project_onto_plane(other_points, &plane);

        // Get the 2D line of this polygon onto the surface
        let self_points = self.points();
        let _self_projection = project_onto_plane(self_points, &plane);

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
