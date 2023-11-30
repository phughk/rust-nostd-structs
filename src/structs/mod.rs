/// A n-dimensional point that is used in the spatial data structures
///
/// U is the base unit
/// L is the larger unit in case of overflow
/// S is the constant number of dimensions
#[derive(PartialEq)]
#[cfg_attr(test, derive(Debug))]
pub struct NDimensionalPoint<U, const S: usize>
where
    U: Default + Copy + PartialEq,
{
    dimensions: [U; S],
}

impl<U, const S: usize> NDimensionalPoint<U, S>
where
    U: Default + Copy + PartialEq,
{
    /// Create a new n-dimensional spatial point
    pub fn new() -> Self {
        NDimensionalPoint {
            dimensions: [U::default(); S],
        }
    }

    /// Mutably set the dimension to a value for the point
    pub fn mut_set(&mut self, dimension: usize, value: U) {
        self.dimensions[dimension] = value;
    }

    /// Set the value of a dimension by copying values
    pub fn copy_set(&self, dimension: usize, value: U) -> NDimensionalPoint<U, S> {
        let mut copied = self.dimensions;
        copied[dimension] = value;
        NDimensionalPoint { dimensions: copied }
    }
}

/// An Axis Aligned Bounding Box (AABB) is a type of shape that is perfectly aligned with it's axes.
///
/// Examples of such shapes include rectangles for 2D, and cubes for 3D. There is the added
/// constraint that they can not be skewed, but must be perfectly aligned with axis.
#[derive(PartialEq)]
#[cfg_attr(test, derive(Debug))]
pub struct AxisAlignedBoundingBox<U, const S: usize>
where
    U: Default + Copy + PartialEq,
{
    origin: NDimensionalPoint<U, S>,
    widths: [U; S],
}

impl<U, const S: usize> AxisAlignedBoundingBox<U, S>
where
    U: Default + Copy + PartialEq,
{
    /// Creates a new bounding box
    pub fn new(origin: NDimensionalPoint<U, S>) -> Self {
        AxisAlignedBoundingBox {
            origin: origin,
            widths: [U::default(); S],
        }
    }

    /// Returns a reference to the point of origin of this bound
    pub fn origin(&self) -> &NDimensionalPoint<U, S> {
        return &self.origin;
    }

    /// Returns a mutable reference to the point of origin of this bound
    pub fn mut_origin(&mut self) -> &mut NDimensionalPoint<U, S> {
        return &mut self.origin;
    }
}

#[cfg(test)]
mod test {
    use crate::structs::{AxisAlignedBoundingBox, NDimensionalPoint};

    #[test]
    fn can_compare_ndimensional_point() {
        let mut left: NDimensionalPoint<i32, 2> = NDimensionalPoint::new();
        left.mut_set(0, 1);
        left.mut_set(1, 2);

        let right = NDimensionalPoint::new().copy_set(0, 3).copy_set(1, 4);

        assert_ne!(left, right);

        left.mut_set(0, 5);
        left.mut_set(1, 6);
        let right = right.copy_set(0, 5).copy_set(1, 6);

        assert_eq!(left, right);
    }

    #[test]
    fn can_compare_aabb() {
        let point1: NDimensionalPoint<i32, 3> = NDimensionalPoint::new()
            .copy_set(0, 1)
            .copy_set(1, 2)
            .copy_set(2, 3);
        let aabb1 = AxisAlignedBoundingBox::new(point1);

        let point2: NDimensionalPoint<i32, 3> = NDimensionalPoint::new()
            .copy_set(0, 1)
            .copy_set(1, 2)
            .copy_set(2, 3);
        let mut aabb2 = AxisAlignedBoundingBox::new(point2);

        assert_eq!(aabb1, aabb2);

        aabb2.mut_origin().mut_set(0, 4);
        assert_ne!(aabb1, aabb2);
    }
}
