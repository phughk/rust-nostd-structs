use crate::structs::geom::point_2d::Point2D;
use crate::structs::geom::{Line2D, Shape2D};
use arrayvec::ArrayVec;

/// An N-polygon in 2D space
#[cfg_attr(test, derive(Debug))]
pub struct Polygon2D<const N: usize, T> {
    /// Points of the polygon
    pub points: ArrayVec<Point2D<T>, N>,
}

impl<const N: usize, T: PartialEq> PartialEq for Polygon2D<N, T> {
    fn eq(&self, other: &Self) -> bool {
        let mut used = [false; N];
        for i in 0..N {
            for j in 0..N {
                if &self.points[i] == &other.points[j] && !used[j] {
                    used[j] = true;
                    break;
                }
            }
        }
        used.iter().all(|&x| x)
    }
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
        super::misc::polygon_area_shoelace(&self.points)
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

#[cfg(test)]
mod test {
    use crate::structs::geom::{Point2D, Polygon2D, Shape2D};
    use arrayvec::ArrayVec;

    #[test]
    fn test_rotate() {
        let mut polygon = Polygon2D {
            points: ArrayVec::from([
                Point2D::new(1.0, 1.0),
                Point2D::new(1.0, 5.0),
                Point2D::new(4.0, 4.0),
                Point2D::new(5.0, 4.0),
                Point2D::new(5.0, 1.0),
            ]),
        };

        polygon.rotate_deg_mut(Point2D::new(-1.0, -1.0), 90.0);

        // https://www.desmos.com/calculator/23qwxcfs2e
        assert_eq!(
            polygon,
            Polygon2D {
                points: ArrayVec::from([
                    Point2D::new(-3.0, 1.0),
                    Point2D::new(-7.0, 1.0),
                    Point2D::new(-6.0, 4.0),
                    Point2D::new(-6.0, 5.0),
                    Point2D::new(-3.0, 5.0),
                ])
            }
        )
    }
}

impl<const N: usize, T> Polygon2D<N, T> {
    pub fn resize<const M: usize>(self) -> Result<Polygon2D<M, T>, ()> {
        if self.points.len() > M {
            return Err(());
        }
        let mut points = ArrayVec::new();
        points.extend(self.points);
        Ok(Polygon2D { points })
    }
}
