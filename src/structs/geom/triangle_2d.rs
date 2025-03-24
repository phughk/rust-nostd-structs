use crate::structs::geom::point_2d::Point2D;
use crate::structs::geom::{Line2D, Polygon2D, Shape2D};
use crate::structs::trig::{cos_degrees, sin_degrees};

/// A triangle in 2D space
pub struct Triangle2D<T> {
    /// Points of the triangle
    pub points: [Point2D<T>; 3],
}

impl Shape2D<3, f32> for Triangle2D<f32> {
    fn rotate_deg(self, degrees: f64) -> Self {
        let cos_theta = cos_degrees(degrees) as f32;
        let sin_theta = sin_degrees(degrees) as f32;

        // Compute the centroid (average of all three points)
        let cx = (self.points[0].x + self.points[1].x + self.points[2].x) / 3.0;
        let cy = (self.points[0].y + self.points[1].y + self.points[2].y) / 3.0;

        // Rotate each point around the centroid
        let rotated_points = self.points.map(|p| {
            let dx = p.x - cx;
            let dy = p.y - cy;

            let rotated_x = dx * cos_theta - dy * sin_theta + cx;
            let rotated_y = dx * sin_theta + dy * cos_theta + cy;

            Point2D {
                x: rotated_x,
                y: rotated_y,
            }
        });

        Triangle2D {
            points: rotated_points,
        }
    }

    fn rotate_deg_mut(&mut self, degrees: f64) {
        let cos_theta = cos_degrees(degrees) as f32;
        let sin_theta = sin_degrees(degrees) as f32;

        // Compute the centroid (average of all three points)
        let cx = (self.points[0].x + self.points[1].x + self.points[2].x) / 3.0;
        let cy = (self.points[0].y + self.points[1].y + self.points[2].y) / 3.0;

        // Rotate each point around the centroid
        for p in self.points.iter_mut() {
            let dx = p.x - cx;
            let dy = p.y - cy;

            let rotated_x = dx * cos_theta - dy * sin_theta + cx;
            let rotated_y = dx * sin_theta + dy * cos_theta + cy;
            p.x = rotated_x;
            p.y = rotated_y;
        }
    }

    fn rotate_rad(self, radians: f32) -> Self {
        todo!()
    }

    fn rotate_rad_mut(&mut self, radians: f32) {
        todo!()
    }

    fn surface(&self) -> f32 {
        todo!()
    }

    fn center(&self) -> Point2D<f32> {
        todo!()
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
    fn rotate_deg(self, degrees: f64) -> Self {
        let cos_theta = cos_degrees(degrees);
        let sin_theta = sin_degrees(degrees);

        // Compute the centroid (average of all three points)
        let cx = (self.points[0].x + self.points[1].x + self.points[2].x) / 3.0;
        let cy = (self.points[0].y + self.points[1].y + self.points[2].y) / 3.0;

        // Rotate each point around the centroid
        let rotated_points = self.points.map(|p| {
            let dx = p.x - cx;
            let dy = p.y - cy;

            let rotated_x = dx * cos_theta - dy * sin_theta + cx;
            let rotated_y = dx * sin_theta + dy * cos_theta + cy;

            Point2D {
                x: rotated_x,
                y: rotated_y,
            }
        });

        Triangle2D {
            points: rotated_points,
        }
    }

    fn rotate_deg_mut(&mut self, degrees: f64) {
        let cos_theta = cos_degrees(degrees);
        let sin_theta = sin_degrees(degrees);

        // Compute the centroid (average of all three points)
        let cx = (self.points[0].x + self.points[1].x + self.points[2].x) / 3.0;
        let cy = (self.points[0].y + self.points[1].y + self.points[2].y) / 3.0;

        // Rotate each point around the centroid
        for p in self.points.iter_mut() {
            let dx = p.x - cx;
            let dy = p.y - cy;

            p.x = dx * cos_theta - dy * sin_theta + cx;
            p.y = dx * sin_theta + dy * cos_theta + cy;
        }
    }

    fn rotate_rad(self, radians: f64) -> Self {
        todo!()
    }

    fn rotate_rad_mut(&mut self, radians: f64) {
        todo!()
    }

    fn surface(&self) -> f64 {
        todo!()
    }

    fn center(&self) -> Point2D<f64> {
        todo!()
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
