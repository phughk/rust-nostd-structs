#[cfg(feature = "helpers")]
use crate::structs::geom::PrintDesmos;
use crate::structs::trig::sqrt;
use crate::structs::AsType;
use core::cmp::Ordering;
use core::ops::{Add, Div};
use core::ops::{Mul, Sub};

/// A point in n-dimensional space.
#[derive(Debug, Clone, Copy)]
pub struct Point2D<T> {
    /// X-axis value
    pub x: T,
    /// Y-axis value
    pub y: T,
}

impl<T> PartialEq for Point2D<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        &self.x == &other.x && &self.y == &other.y
    }
}

impl<T> PartialOrd for Point2D<T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let x_cmp = self.x.partial_cmp(&other.x);
        if x_cmp == Some(Ordering::Equal) {
            self.y.partial_cmp(&other.y)
        } else {
            x_cmp
        }
    }
}

impl<T: Copy> Point2D<T> {
    /// New point from an array of data
    pub const fn new_array(data: [T; 2]) -> Self {
        Point2D {
            x: data[0],
            y: data[1],
        }
    }

    /// New point from x and y values
    pub const fn new(x: T, y: T) -> Self {
        Point2D { x, y }
    }
}

impl<T: Sub<Output = T> + Copy> Sub for Point2D<T> {
    type Output = Point2D<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Point2D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T: Sub<Output = T> + Copy> Sub for &Point2D<T> {
    type Output = Point2D<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Point2D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T> Mul<T> for Point2D<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Point2D<T>;

    /// Scalar multiplication
    fn mul(self, rhs: T) -> Self::Output {
        Point2D {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T> Add<Point2D<T>> for Point2D<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Point2D<T>;

    fn add(self, rhs: Point2D<T>) -> Self::Output {
        Point2D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T> Point2D<T> {
    /// Convert the point to a tuple
    pub fn as_tuple(&self) -> (T, T)
    where
        T: Copy,
    {
        (self.x, self.y)
    }

    /// Distance between two points, without square root
    pub fn distance_squared(&self, other: &Self) -> T
    where
        T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Copy,
    {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let x = dx * dx;
        let y = dy * dy;
        x + y
    }

    /// Chebyshev distance, which is has a worst case accuracy of 7%
    pub fn distance_chebyshev(&self, other: &Self) -> T
    where
        T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Copy + PartialOrd + AsType<f32>,
    {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        if dx > dy {
            dx + T::from_type(0.5) * dy
        } else {
            dy + T::from_type(0.5) * dx
        }
    }

    /// Hypotenuse of the point.
    /// The hypotenuse is the distance of a point from (0,0)
    pub fn hypotenuse(&self) -> T
    where
        T: PartialOrd
            + From<f32>
            + Div<Output = T>
            + Mul<Output = T>
            + Add<Output = T>
            + Sub<Output = T>
            + Copy,
    {
        let d = self.distance_squared(&Point2D {
            x: T::from(0.0),
            y: T::from(0.0),
        });
        sqrt(d, 20).unwrap()
    }

    /// Dot product of two points
    pub fn dot(&self, other: &Self) -> T
    where
        T: Mul<Output = T> + Add<Output = T> + Copy,
    {
        let x = self.x * other.x;
        let y = self.y * other.y;
        x + y
    }

    /// Cross product of two points
    pub fn cross(&self, other: &Self) -> T
    where
        T: Mul<Output = T> + Sub<Output = T> + Copy,
    {
        self.x * other.y - self.y * other.x
    }

    /// Translate the point by a given distance
    pub fn translate(&self, dx: T, dy: T) -> Self
    where
        T: Add<Output = T> + Copy,
    {
        Point2D {
            x: self.x + dx,
            y: self.y + dy,
        }
    }
}

#[cfg(feature = "helpers")]
impl<T: core::fmt::Display> PrintDesmos for Point2D<T> {
    fn to_string_desmos(&self) -> ArrayString<1024> {
        use core::fmt::Write;
        let mut s = ArrayString::new();
        core::write!(&mut s, "({}, {})", self.x, self.y).unwrap();
        s
    }
}
