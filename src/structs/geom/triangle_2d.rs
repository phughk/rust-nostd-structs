use crate::structs::geom::point_2d::Point2D;
use crate::structs::geom::{Line2D, Polygon2D, Shape2D};
use arrayvec::ArrayVec;

/// A triangle in 2D space
#[cfg_attr(test, derive(Debug))]
pub struct Triangle2D<T> {
    /// Points of the triangle
    pub points: [Point2D<T>; 3],
}

impl<T: PartialEq> PartialEq for Triangle2D<T> {
    fn eq(&self, other: &Self) -> bool {
        let mut used = [false; 3];
        for i in 0..3 {
            for j in 0..3 {
                if &self.points[i] == &other.points[j] && !used[j] {
                    used[j] = true;
                    break;
                }
            }
        }
        used.iter().all(|&x| x)
    }
}

impl Shape2D<3, f32> for Triangle2D<f32> {
    fn rotate_deg(mut self, point: Point2D<f32>, degrees: f32) -> Self {
        self.rotate_rad_mut(point, degrees);
        self
    }

    fn rotate_deg_mut(&mut self, point2d: Point2D<f32>, degrees: f32) {
        super::misc::rotate_deg_mut(&mut self.points, point2d, degrees);
    }

    fn rotate_rad(self, point2d: Point2D<f32>, radians: f32) -> Self {
        todo!()
    }

    fn rotate_rad_mut(&mut self, point2d: Point2D<f32>, radians: f32) {
        todo!()
    }

    fn surface(&self) -> f32 {
        let a = &self.points[0];
        let b = &self.points[1];
        let c = &self.points[2];
        0.5 * ((a.x * (b.y - c.y)) + (b.x * (c.y - a.y)) + (c.x * (a.y - b.y))).abs()
    }

    fn center(&self) -> Point2D<f32> {
        // Compute the centroid (average of all three points)
        let cx = (self.points[0].x + self.points[1].x + self.points[2].x) / 3.0;
        let cy = (self.points[0].y + self.points[1].y + self.points[2].y) / 3.0;
        Point2D::new(cx, cy)
    }

    fn closest_point(&self, point: Point2D<f32>) -> Point2D<f32> {
        if self.point_in_shape(point) {
            return point;
        }

        // Check edges
        let edges = self.edges();
        let ab = edges[0].closest_point_on_segment(&point);
        let bc = edges[1].closest_point_on_segment(&point);
        let ca = edges[2].closest_point_on_segment(&point);

        let d_ab = ab.distance_squared(&point);
        let d_bc = bc.distance_squared(&point);
        let d_ca = ca.distance_squared(&point);

        if d_ab < d_bc && d_ab < d_ca {
            ab
        } else if d_bc < d_ca {
            bc
        } else {
            ca
        }
    }

    fn point_in_shape(&self, point: Point2D<f32>) -> bool {
        let a = &self.points[0];
        let b = &self.points[1];
        let c = &self.points[2];

        let v0 = c - a;
        let v1 = b - a;
        let v2 = &point - a;

        let dot00 = v0.dot(&v0);
        let dot01 = v0.dot(&v1);
        let dot02 = v0.dot(&v2);
        let dot11 = v1.dot(&v1);
        let dot12 = v1.dot(&v2);

        let denom = dot00 * dot11 - dot01 * dot01;
        if denom != 0.0 {
            return true;
        }

        let inv_denom = 1.0 / denom;
        let u = (dot11 * dot02 - dot01 * dot12) * inv_denom;
        let v = (dot00 * dot12 - dot01 * dot02) * inv_denom;

        let point_in_triangle = u >= 0.0 && v >= 0.0 && (u + v) <= 1.0;
        point_in_triangle
    }

    fn axis_aligned_bounding_box(&self) -> Polygon2D<4, f32> {
        todo!()
    }

    fn as_convex_hull(self) -> Polygon2D<3, f32> {
        todo!()
    }

    fn convex_hull_with_other_shape<
        const NEW_SZ: usize,
        const OTHER_SZ: usize,
        SHAPE: Shape2D<OTHER_SZ, f32>,
    >(
        &self,
        other_shape: SHAPE,
    ) -> Polygon2D<NEW_SZ, f32> {
        todo!()
    }

    fn points(&self) -> &[Point2D<f32>] {
        &self.points
    }

    fn edges(&self) -> ArrayVec<Line2D<f32>, 3> {
        ArrayVec::from([
            Line2D::new(self.points[0], self.points[1]),
            Line2D::new(self.points[1], self.points[2]),
            Line2D::new(self.points[1], self.points[0]),
        ])
    }
}

impl Shape2D<3, f64> for Triangle2D<f64> {
    fn rotate_deg(mut self, point2d: Point2D<f64>, degrees: f64) -> Self {
        self.rotate_rad_mut(point2d, degrees);
        self
    }

    fn rotate_deg_mut(&mut self, point2d: Point2D<f64>, degrees: f64) {
        super::misc::rotate_deg_mut(&mut self.points, point2d, degrees);
    }

    fn rotate_rad(self, point2d: Point2D<f64>, radians: f64) -> Self {
        todo!()
    }

    fn rotate_rad_mut(&mut self, point2d: Point2D<f64>, radians: f64) {
        todo!()
    }

    fn surface(&self) -> f64 {
        let a = &self.points[0];
        let b = &self.points[1];
        let c = &self.points[2];
        0.5 * ((a.x * (b.y - c.y)) + (b.x * (c.y - a.y)) + (c.x * (a.y - b.y))).abs()
    }

    fn center(&self) -> Point2D<f64> {
        // Compute the centroid (average of all three points)
        let cx = (self.points[0].x + self.points[1].x + self.points[2].x) / 3.0;
        let cy = (self.points[0].y + self.points[1].y + self.points[2].y) / 3.0;
        Point2D::new(cx, cy)
    }

    fn closest_point(&self, point: Point2D<f64>) -> Point2D<f64> {
        if self.point_in_shape(point) {
            return point;
        }

        // Check edges
        let edges = self.edges();
        let ab = edges[0].closest_point_on_segment(&point);
        let bc = edges[1].closest_point_on_segment(&point);
        let ca = edges[2].closest_point_on_segment(&point);

        let d_ab = ab.distance_squared(&point);
        let d_bc = bc.distance_squared(&point);
        let d_ca = ca.distance_squared(&point);

        if d_ab < d_bc && d_ab < d_ca {
            ab
        } else if d_bc < d_ca {
            bc
        } else {
            ca
        }
    }

    fn point_in_shape(&self, point: Point2D<f64>) -> bool {
        let a = &self.points[0];
        let b = &self.points[1];
        let c = &self.points[2];

        let v0 = c - a;
        let v1 = b - a;
        let v2 = &point - a;

        let dot00 = v0.dot(&v0);
        let dot01 = v0.dot(&v1);
        let dot02 = v0.dot(&v2);
        let dot11 = v1.dot(&v1);
        let dot12 = v1.dot(&v2);

        let denom = dot00 * dot11 - dot01 * dot01;
        if denom != 0.0 {
            return true;
        }

        let inv_denom = 1.0 / denom;
        let u = (dot11 * dot02 - dot01 * dot12) * inv_denom;
        let v = (dot00 * dot12 - dot01 * dot02) * inv_denom;

        let point_in_triangle = u >= 0.0 && v >= 0.0 && (u + v) <= 1.0;
        point_in_triangle
    }

    fn axis_aligned_bounding_box(&self) -> Polygon2D<4, f64> {
        todo!()
    }

    fn as_convex_hull(self) -> Polygon2D<3, f64> {
        todo!()
    }

    fn convex_hull_with_other_shape<
        const NEW_SZ: usize,
        const OTHER_SZ: usize,
        SHAPE: Shape2D<OTHER_SZ, f64>,
    >(
        &self,
        other_shape: SHAPE,
    ) -> Polygon2D<NEW_SZ, f64> {
        todo!()
    }

    fn points(&self) -> &[Point2D<f64>] {
        &self.points
    }

    fn edges(&self) -> ArrayVec<Line2D<f64>, 3> {
        ArrayVec::from([
            Line2D::new(self.points[0], self.points[1]),
            Line2D::new(self.points[1], self.points[2]),
            Line2D::new(self.points[1], self.points[0]),
        ])
    }
}

#[cfg(test)]
mod test {
    use crate::structs::geom::{Point2D, Shape2D, Triangle2D};

    #[test]
    fn test_rotate_around_0() {
        let mut triangle = Triangle2D {
            points: [
                Point2D::new(0.0, 0.0),
                Point2D::new(4.0, 0.0),
                Point2D::new(0.0, 4.0),
            ],
        };
        triangle.rotate_deg_mut(Point2D::new(0.0, 0.0), 90.0);
        assert_eq!(
            triangle,
            Triangle2D {
                points: [
                    Point2D::new(0.0, 0.0),
                    Point2D::new(0.0, 4.0),
                    Point2D::new(-4.0, 0.0)
                ],
            }
        );
    }

    #[test]
    fn test_rotate_around_center() {
        let mut triangle = Triangle2D {
            points: [
                Point2D::new(-1.0, -1.0),
                Point2D::new(0.0, 1.0),
                Point2D::new(1.0, -1.0),
            ],
        };
        let center = triangle.center();
        triangle.rotate_deg_mut(center, 90.0);
        assert_eq!(
            triangle,
            Triangle2D {
                points: [
                    Point2D::new(0.6666666666666667, -1.3333333333333333),
                    Point2D::new(-1.3333333333333333, -0.3333333333333333),
                    Point2D::new(0.6666666666666667, 0.6666666666666667)
                ],
            }
        );
    }

    #[test]
    fn test_surface() {
        let triangle = Triangle2D {
            points: [
                Point2D::new(0.0, 0.0),
                Point2D::new(4.0, 0.0),
                Point2D::new(0.0, 4.0),
            ],
        };
        assert_eq!(triangle.surface(), 8.0);
    }
}
