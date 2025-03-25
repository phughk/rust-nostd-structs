use core::ops::Add;
use core::ops::{Mul, Sub};

/// A point in n-dimensional space.
#[derive(Clone, Copy)]
#[cfg_attr(test, derive(Debug))]
pub struct Point2D<T> {
    /// X-axis value
    pub x: T,
    /// Y-axis value
    pub y: T,
}

impl<T> PartialEq for &Point2D<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        &self.x == &other.x && &self.y == &other.y
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

impl<T: Sub<Output = T> + Copy> Sub for &Point2D<T> {
    type Output = Point2D<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Point2D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T> Point2D<T> {
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

    /// Dot product of two points
    pub fn dot(&self, other: &Self) -> T
    where
        T: Mul<Output = T> + Add<Output = T> + Copy,
    {
        let x = self.x * other.x;
        let y = self.y * other.y;
        x + y
    }
}
