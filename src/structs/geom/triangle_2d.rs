use crate::structs::geom::point_2d::Point2D;
use crate::structs::geom::{Line2D, Polygon2D, Shape2D};
use crate::structs::trig::{cos_degrees, sin_degrees};

/// A triangle in 2D space
#[derive(PartialEq)]
#[cfg_attr(test, derive(Debug))]
pub struct Triangle2D<T> {
    /// Points of the triangle
    pub points: [Point2D<T>; 3],
}

impl Shape2D<3, f32> for Triangle2D<f32> {
    fn rotate_deg(self, point: Point2D<f32>, degrees: f32) -> Self {
        let cos_theta = cos_degrees(degrees as f64) as f32;
        let sin_theta = sin_degrees(degrees as f64) as f32;

        // Rotate each point around the centroid
        let rotated_points = self.points.map(|p| {
            let dx = p.x - point.x;
            let dy = p.y - point.y;

            let rotated_x = dx * cos_theta - dy * sin_theta + point.x;
            let rotated_y = dx * sin_theta + dy * cos_theta + point.y;

            Point2D {
                x: rotated_x,
                y: rotated_y,
            }
        });

        Triangle2D {
            points: rotated_points,
        }
    }

    fn rotate_deg_mut(&mut self, point2d: Point2D<f32>, degrees: f32) {
        let cos_theta = cos_degrees(degrees as f64) as f32;
        let sin_theta = sin_degrees(degrees as f64) as f32;

        // Rotate each point around the centroid
        for p in self.points.iter_mut() {
            let dx = p.x - point2d.x;
            let dy = p.y - point2d.y;

            let rotated_x = dx * cos_theta - dy * sin_theta + point2d.x;
            let rotated_y = dx * sin_theta + dy * cos_theta + point2d.y;
            p.x = rotated_x;
            p.y = rotated_y;
        }
    }

    fn rotate_rad(self, point2d: Point2D<f32>, radians: f32) -> Self {
        todo!()
    }

    fn rotate_rad_mut(&mut self, point2d: Point2D<f32>, radians: f32) {
        todo!()
    }

    fn surface(&self) -> f32 {
        todo!()
    }

    fn center(&self) -> Point2D<f32> {
        // Compute the centroid (average of all three points)
        let cx = (self.points[0].x + self.points[1].x + self.points[2].x) / 3.0;
        let cy = (self.points[0].y + self.points[1].y + self.points[2].y) / 3.0;
        Point2D::new(cx, cy)
    }

    fn closest_point(&self, point: Point2D<f32>) -> Point2D<f32> {
        todo!()
    }

    fn point_in_shape(&self) -> bool {
        todo!()
    }

    fn axis_aligned_bounding_box(&self) -> Polygon2D<4, f32> {
        todo!()
    }

    fn convex_hull<const NewSZ: usize>(&self) -> Polygon2D<NewSZ, f32> {
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
        todo!()
    }

    fn edges(&self) -> [Line2D<f32>; 3] {
        todo!()
    }
}

impl Shape2D<3, f64> for Triangle2D<f64> {
    fn rotate_deg(self, point2d: Point2D<f64>, degrees: f64) -> Self {
        let cos_theta = cos_degrees(degrees);
        let sin_theta = sin_degrees(degrees);

        // Rotate each point around the centroid
        let rotated_points = self.points.map(|p| {
            let dx = p.x - point2d.x;
            let dy = p.y - point2d.y;

            let rotated_x = dx * cos_theta - dy * sin_theta + point2d.x;
            let rotated_y = dx * sin_theta + dy * cos_theta + point2d.y;

            Point2D {
                x: rotated_x,
                y: rotated_y,
            }
        });

        Triangle2D {
            points: rotated_points,
        }
    }

    fn rotate_deg_mut(&mut self, point2d: Point2D<f64>, degrees: f64) {
        let cos_theta = cos_degrees(degrees);
        let sin_theta = sin_degrees(degrees);

        // Rotate each point around the centroid
        for p in self.points.iter_mut() {
            let dx = p.x - point2d.x;
            let dy = p.y - point2d.y;

            p.x = dx * cos_theta - dy * sin_theta + point2d.x;
            p.y = dx * sin_theta + dy * cos_theta + point2d.y;
        }
    }

    fn rotate_rad(self, point2d: Point2D<f64>, radians: f64) -> Self {
        todo!()
    }

    fn rotate_rad_mut(&mut self, point2d: Point2D<f64>, radians: f64) {
        todo!()
    }

    fn surface(&self) -> f64 {
        todo!()
    }

    fn center(&self) -> Point2D<f64> {
        // Compute the centroid (average of all three points)
        let cx = (self.points[0].x + self.points[1].x + self.points[2].x) / 3.0;
        let cy = (self.points[0].y + self.points[1].y + self.points[2].y) / 3.0;
        Point2D::new(cx, cy)
    }

    fn closest_point(&self, point: Point2D<f64>) -> Point2D<f64> {
        todo!()
    }

    fn point_in_shape(&self) -> bool {
        todo!()
    }

    fn axis_aligned_bounding_box(&self) -> Polygon2D<4, f64> {
        todo!()
    }

    fn convex_hull<const NewSZ: usize>(&self) -> Polygon2D<NewSZ, f64> {
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
        todo!()
    }

    fn edges(&self) -> [Line2D<f64>; 3] {
        todo!()
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
}
