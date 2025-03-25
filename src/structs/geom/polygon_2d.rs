use crate::algos::slice::insertion_sort_by;
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
        let n = self.points.len();
        assert!(n >= 3, "A polygon must have at least 3 points");

        let mut area = 0.0;
        let mut cx = 0.0;
        let mut cy = 0.0;

        for i in 0..n {
            let p0 = &self.points[i];
            let p1 = &self.points[(i + 1) % n];

            let cross = p0.x * p1.y - p1.x * p0.y;
            area += cross;
            cx += (p0.x + p1.x) * cross;
            cy += (p0.y + p1.y) * cross;
        }

        area *= 0.5;
        let factor = 1.0 / (6.0 * area);

        Point2D {
            x: cx * factor,
            y: cy * factor,
        }
    }

    fn closest_point(&self, point: Point2D<f32>) -> Point2D<f32> {
        if self.point_in_shape(point) {
            return point;
        }

        assert!(
            self.points.len() >= 2,
            "Polygon must have at least 2 points"
        );

        let mut closest = None;
        let mut min_dist = f32::MAX;

        for i in 0..self.points.len() {
            let a = self.points[i];
            let b = self.points[(i + 1) % self.points.len()];
            let candidate = Line2D::new(a, b).closest_point_on_segment(&point);
            let dist = candidate.distance_squared(&point);

            if dist < min_dist {
                min_dist = dist;
                closest = Some(candidate);
            }
        }

        closest.unwrap()
    }

    fn point_in_shape(&self, point: Point2D<f32>) -> bool {
        let mut inside = false;
        let n = self.points.len();

        for i in 0..n {
            let a = self.points[i];
            let b = self.points[(i + 1) % n];

            let (px, py) = (point.x, point.y);
            let (x1, y1) = (a.x, a.y);
            let (x2, y2) = (b.x, b.y);

            let intersect = ((y1 > py) != (y2 > py))
                && (px < (x2 - x1) * (py - y1) / (y2 - y1 + f32::EPSILON) + x1);
            if intersect {
                inside = !inside;
            }
        }

        inside
    }

    fn axis_aligned_bounding_box(&self) -> Polygon2D<4, f32> {
        todo!()
    }

    /// Graham's scan algorithm to find the convex hull of the polygon
    fn convex_hull_with_other_shape<
        const NEW_SZ: usize,
        const OTHER_SZ: usize,
        SHAPE: Shape2D<OTHER_SZ, f32>,
    >(
        &self,
        other_shape: SHAPE,
    ) -> Polygon2D<NEW_SZ, f32> {
        let mut points: ArrayVec<Point2D<f32>, NEW_SZ> = ArrayVec::new();
        for p in &self.points {
            points.push(*p);
        }
        for p in other_shape.points() {
            points.push(*p);
        }
        if points.len() <= 3 {
            return Polygon2D { points };
        }

        // 1. Find the lowest point
        let pivot_idx = points
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| {
                a.y.partial_cmp(&b.y)
                    .unwrap()
                    .then(a.x.partial_cmp(&b.x).unwrap())
            })
            .unwrap()
            .0;

        let pivot = points[pivot_idx];
        points.swap(0, pivot_idx);

        // 2. Sort points by polar angle w.r.t. pivot
        insertion_sort_by(&mut points[1..], |a, b| {
            let o = super::misc::orientation(&pivot, a, b);
            if o == 0.0 {
                // Collinear: closer one comes first
                let da = Point2D::new(a.x - pivot.x, a.y - pivot.y).hypotenuse();
                let db = Point2D::new(b.x - pivot.x, b.y - pivot.y).hypotenuse();
                da.partial_cmp(&db).unwrap()
            } else {
                o.partial_cmp(&0.0).unwrap().reverse() // CCW first
            }
        });

        // 3. Build the convex hull using a stack
        let mut stack: ArrayVec<Point2D<f32>, NEW_SZ> = ArrayVec::new();
        stack.extend([points[0], points[1]]);

        for &p in points.iter().skip(2) {
            while stack.len() >= 2
                && super::misc::orientation(&stack[stack.len() - 2], &stack[stack.len() - 1], &p)
                    <= 0.0
            {
                stack.pop();
            }
            stack.push(p);
        }

        Polygon2D { points: stack }
    }

    fn points(&self) -> &[Point2D<f32>] {
        self.points.as_slice()
    }

    fn edges(&self) -> ArrayVec<Line2D<f32>, N> {
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
