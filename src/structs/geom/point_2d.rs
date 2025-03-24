/// A point in n-dimensional space.
pub struct Point2D<T> {
    /// X-axis value
    pub x: T,
    /// Y-axis value
    pub y: T,
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
