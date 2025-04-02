use super::misc;
use crate::structs::geom::point_2d::Point2D;
use crate::structs::geom::{Line2D, Shape2D};
use crate::structs::AsType;
use arrayvec::ArrayVec;
use core::ops::{Add, Div, Mul, Sub};

/// An N-polygon in 2D space
#[derive(Clone, Debug)]
pub struct Polygon2D<const N: usize, T> {
    /// Points of the polygon
    pub points: ArrayVec<Point2D<T>, N>,
}

impl<const N: usize, T: PartialEq> PartialEq for Polygon2D<N, T> {
    fn eq(&self, other: &Self) -> bool {
        assert!(self.points.len() > 2, "Polygon must have at least 3 points");
        assert!(
            other.points.len() > 2,
            "Polygon must have at least 3 points"
        );
        let sz = self.points.len();
        if sz != other.points.len() {
            return false;
        }
        let mut used = [false; N];
        for i in 0..sz {
            for j in 0..sz {
                if &self.points[i] == &other.points[j] && !used[j] {
                    used[j] = true;
                    break;
                }
            }
        }
        used[0..sz].iter().all(|&x| x)
    }
}

impl<const N: usize, T> Shape2D<N, T> for Polygon2D<N, T> {
    fn rotate_deg_mut(&mut self, point: Point2D<T>, degrees: T)
    where
        T: AsType<f64>
            + Copy
            + Sub<Output = T>
            + Mul<Output = T>
            + AsType<f32>
            + Add<Output = T>
            + PartialOrd,
    {
        misc::rotate_deg_mut(self.points.as_mut(), point, degrees)
    }

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
            + Add<Output = T>,
    {
        if self.point_in_shape(point) {
            return *point;
        }

        assert!(
            self.points.len() >= 2,
            "Polygon must have at least 2 points"
        );

        let mut closest = None;
        let mut min_dist = None;

        for i in 0..self.points.len() {
            let a = self.points[i];
            let b = self.points[(i + 1) % self.points.len()];
            let candidate = Line2D::new(a, b).closest_point_on_segment(&point);
            let dist = candidate.distance_squared(&point);
            if min_dist.is_none() {
                min_dist = Some(dist);
            }

            if dist < min_dist.unwrap() {
                min_dist = Some(dist);
                closest = Some(candidate);
            }
        }

        closest.unwrap()
    }

    fn points(&self) -> &[Point2D<T>] {
        self.points.as_slice()
    }

    fn edges(&self) -> ArrayVec<Line2D<T>, N>
    where
        T: Copy,
    {
        let mut edges = ArrayVec::new();
        for i in 0..self.points.len() {
            edges.push(Line2D::new(
                self.points[i],
                self.points[(i + 1) % self.points.len()],
            ));
        }
        edges
    }
}

impl<const N: usize, T> Polygon2D<N, T> {
    /// Resize the polygon to a new size
    pub fn resize<const M: usize>(self) -> Result<Polygon2D<M, T>, ()> {
        if self.points.len() > M {
            return Err(());
        }
        let mut points = ArrayVec::new();
        points.extend(self.points);
        Ok(Polygon2D { points })
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

    #[test]
    fn test_convex_hull() {
        let first = Polygon2D {
            points: ArrayVec::from([
                Point2D::new(0.0, 0.0),
                Point2D::new(4.0, 0.0),
                Point2D::new(4.0, 4.0),
                Point2D::new(0.0, 4.0),
            ]),
        };
        let second = Polygon2D {
            points: ArrayVec::from([
                Point2D::new(8.0, 10.0),
                Point2D::new(7.0, 12.0),
                Point2D::new(5.0, 9.0),
            ]),
        };

        let convex = first.convex_hull_with_other_shape::<7, 3, _>(&second);
        let mut points = ArrayVec::<Point2D<f32>, 7>::new();
        points.push(Point2D::new(0.0, 0.0));
        points.push(Point2D::new(4.0, 0.0));
        points.push(Point2D::new(8.0, 10.0));
        points.push(Point2D::new(7.0, 12.0));
        points.push(Point2D::new(0.0, 4.0));

        // https://www.desmos.com/calculator/cfplirl5bp
        assert_eq!(convex, Polygon2D { points });
    }
}
