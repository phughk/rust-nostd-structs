use crate::algos::slice::insertion_sort_by;
use crate::structs::geom::{Point2D, Polygon2D};
use crate::structs::trig::{cos_degrees, sin_degrees};
use crate::structs::AsType;
use arrayvec::ArrayVec;
use core::ops::{Add, Div, Mul, Sub};

pub(super) fn rotate_deg_mut<
    T: AsType<f64>
        + Copy
        + Sub<Output = T>
        + Mul<Output = T>
        + AsType<f32>
        + Add<Output = T>
        + PartialOrd,
>(
    points: &mut [Point2D<T>],
    point2d: Point2D<T>,
    degrees: T,
) {
    let cos_theta = cos_degrees(degrees);
    let sin_theta = sin_degrees(degrees);

    // Rotate each point around the provided point
    for p in points.iter_mut() {
        let dx = p.x - point2d.x;
        let dy = p.y - point2d.y;

        let rotated_x = dx * cos_theta - dy * sin_theta + point2d.x;
        let rotated_y = dx * sin_theta + dy * cos_theta + point2d.y;
        p.x = rotated_x;
        p.y = rotated_y;
    }
}

pub(super) fn polygon_area_shoelace<T>(points: &[Point2D<T>]) -> T
where
    T: Mul<Output = T> + Sub<Output = T> + Copy + From<f32> + Add<Output = T> + PartialOrd,
{
    let n = points.len();
    let zero = T::from(0.0);
    if n < 3 {
        return zero;
    }

    let mut sum = T::from(0.0);
    for i in 0..n {
        let p1 = &points[i];
        let p2 = &points[(i + 1) % n];
        sum = sum + (p1.x * p2.y - p2.x * p1.y);
    }
    let sum = {
        if sum < zero {
            zero - sum
        } else {
            sum
        }
    };
    let half = T::from(0.5);

    half * sum
}

pub fn orientation<T: Sub<Output = T> + Copy + Mul<Output = T>>(
    a: &Point2D<T>,
    b: &Point2D<T>,
    c: &Point2D<T>,
) -> T {
    let ab = b - a;
    let ac = c - a;
    ab.cross(&ac)
}

pub fn convex_hull<const SZ: usize, T>(mut points: ArrayVec<Point2D<T>, SZ>) -> Polygon2D<SZ, T>
where
    T: Copy
        + PartialOrd
        + From<f32>
        + Sub<Output = T>
        + Mul<Output = T>
        + Add<Output = T>
        + Div<Output = T>,
{
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
    let zero = T::from(0.0);
    insertion_sort_by(&mut points[1..], |a, b| {
        let o = orientation(&pivot, a, b);
        if o == zero {
            // Collinear: closer one comes first
            let da = Point2D::new(a.x - pivot.x, a.y - pivot.y).hypotenuse();
            let db = Point2D::new(b.x - pivot.x, b.y - pivot.y).hypotenuse();
            da.partial_cmp(&db).unwrap()
        } else {
            o.partial_cmp(&zero).unwrap().reverse() // CCW first
        }
    });

    // 3. Build the convex hull using a stack
    let mut stack: ArrayVec<Point2D<T>, SZ> = ArrayVec::new();
    stack.extend([points[0], points[1]]);

    for &p in points.iter().skip(2) {
        while stack.len() >= 2
            && orientation(&stack[stack.len() - 2], &stack[stack.len() - 1], &p) <= zero
        {
            stack.pop();
        }
        stack.push(p);
    }

    Polygon2D { points: stack }
}
