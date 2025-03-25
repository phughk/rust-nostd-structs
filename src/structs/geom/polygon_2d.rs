use crate::structs::geom::point_2d::Point2D;
use crate::structs::geom::{Line2D, Shape2D};

/// An N-polygon in 2D space
pub struct Polygon2D<const N: usize, T> {
    /// Points of the polygon
    pub points: [Point2D<T>; N],
}

impl<const N: usize> Shape2D<N, f32> for Polygon2D<N, f32> {
    fn rotate_deg(mut self, point: Point2D<f32>, degrees: f32) -> Self {
        self.rotate_deg_mut(point, degrees);
        self
    }

    fn rotate_deg_mut(&mut self, point: Point2D<f32>, degrees: f32) {
        super::misc::rotate_deg_mut(&mut self.points, point, degrees);
    }

    fn rotate_rad(self, point: Point2D<f32>, radians: f32) -> Self {
        todo!()
    }

    fn rotate_rad_mut(&mut self, point: Point2D<f32>, radians: f32) {
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

    fn point_in_shape(&self, point: Point2D<f32>) -> bool {
        todo!()
    }

    fn axis_aligned_bounding_box(&self) -> Polygon2D<4, f32> {
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

    fn edges(&self) -> [Line2D<f32>; N] {
        todo!()
    }
}
