/// A n-dimensional point that is used in the spatial data structures
///
/// U is the base unit
/// L is the larger unit in case of overflow
/// S is the constant number of dimensions
#[derive(PartialEq, Copy, Clone)]
#[cfg_attr(test, derive(Debug))]
pub struct NDimensionalPoint<U, const S: usize>
where
    U: Copy + PartialEq,
{
    dimensions: [U; S],
}

impl<U, const S: usize> NDimensionalPoint<U, S>
where
    U: Copy + PartialEq,
{
    /// Create a new n-dimensional spatial point
    pub fn new(vals: [U; S]) -> Self {
        NDimensionalPoint { dimensions: vals }
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

    pub fn dimension(&self, dimension: usize) -> &U {
        &self.dimensions[dimension]
    }
}

/// An Axis Aligned Bounding Box (AABB) is a type of shape that is perfectly aligned with it's axes.
///
/// Examples of such shapes include rectangles for 2D, and cubes for 3D. There is the added
/// constraint that they can not be skewed, but must be perfectly aligned with axis.
#[derive(PartialEq, Copy, Clone)]
#[cfg_attr(test, derive(Debug))]
pub struct AxisAlignedBoundingBox<U, const S: usize>
where
    U: Copy + PartialEq,
{
    origin: NDimensionalPoint<U, S>,
    widths: [U; S],
}

impl<U, const S: usize> AxisAlignedBoundingBox<U, S>
where
    U: Copy + PartialEq,
{
    /// Creates a new bounding box
    pub fn new(origin: NDimensionalPoint<U, S>, widths: [U; S]) -> Self {
        AxisAlignedBoundingBox { origin, widths }
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
    use alloc::boxed::Box;
    use core::cell::{RefCell, RefMut};
    use core::sync::atomic::{AtomicI8, Ordering};

    #[test]
    fn can_compare_ndimensional_point() {
        let mut left: NDimensionalPoint<i32, 2> = NDimensionalPoint::new([1, 2]);

        let right = NDimensionalPoint::new([3, 4]);

        assert_ne!(left, right);

        left.mut_set(0, 5);
        left.mut_set(1, 6);
        let right = right.copy_set(0, 5).copy_set(1, 6);

        assert_eq!(left, right);
    }

    #[test]
    fn can_compare_aabb() {
        let point1: NDimensionalPoint<i32, 3> = NDimensionalPoint::new([1, 2, 3]);
        let aabb1 = AxisAlignedBoundingBox::new(point1, [1, 1, 1]);

        let point2: NDimensionalPoint<i32, 3> = NDimensionalPoint::new([1, 2, 3]);
        let mut aabb2 = AxisAlignedBoundingBox::new(point2, [1, 1, 1]);

        assert_eq!(aabb1, aabb2);

        aabb2.mut_origin().mut_set(0, 4);
        assert_ne!(aabb1, aabb2);
    }

    #[test]
    fn can_reference() {
        #[derive(PartialEq, Copy, Clone)]
        struct SomeStruct {
            x: i32,
            y: i32,
        }
        let ref_cell = RefCell::new(SomeStruct { x: 5, y: 6 });
        let point: NDimensionalPoint<&RefCell<SomeStruct>, 1> = NDimensionalPoint::new([&ref_cell]);
        assert_eq!(point.dimension(0).borrow().x, 5);

        ref_cell.borrow_mut().x = 7;
        assert_eq!(point.dimension(0).borrow().x, 7);
    }
}
