
/// A n-dimensional point that is used in the spatial data structures
pub struct NDimensionalPoint<U, const S: usize> where U: Default + Copy {
    dimensions: [U; S],
}

impl<U, const S: usize> NDimensionalPoint<U, S> where U: Default + Copy {
    /// Create a new n-dimensional spatial point
    pub fn new() -> Self {
        NDimensionalPoint {
            dimensions: [U::default(); S],
        }
    }
}

/// An Axis Aligned Bounding Box (AABB) is a type of shape that is perfectly aligned with it's axes.
///
/// Examples of such shapes include rectangles for 2D, and cubes for 3D. There is the added
/// constraint that they can not be skewed, but must be perfectly aligned with axis.
pub struct AxisAlignedBoundingBox<U, const S: usize> where U: Default + Copy {
    origin: NDimensionalPoint<U, S>,
    widths: [U; S],
}

impl<U, const S: usize> AxisAlignedBoundingBox<U, S> where U: Default + Copy {
    /// Creates a new bounding box
    pub fn new() -> Self {
        AxisAlignedBoundingBox {
            origin: NDimensionalPoint::new(),
            widths: [U::default(); S],
        }
    }
}
