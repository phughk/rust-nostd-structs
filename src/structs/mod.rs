use core::ops::Add;

/// A n-dimensional point that is used in the spatial data structures
///
/// Unit is the base type used in the dimensions
/// SumType is the larger unit in case of overflow
/// S is the constant number of dimensions
#[derive(PartialEq, Clone)]
#[cfg_attr(test, derive(Debug))]
pub struct NDimensionalPoint<Unit, SumType, const S: usize>
where
    Unit: Copy + PartialEq + Add<Output = SumType> + PartialOrd,
    SumType: Copy + PartialOrd,
{
    dimensions: [Unit; S],
}

impl<Unit, SumType, const S: usize> NDimensionalPoint<Unit, SumType, S>
where
    Unit: Copy + PartialEq + Add<Output = SumType> + PartialOrd,
    SumType: Copy + PartialOrd,
{
    /// Create a new n-dimensional spatial point
    pub fn new(vals: [Unit; S]) -> Self {
        NDimensionalPoint { dimensions: vals }
    }

    /// Mutably set the dimension to a value for the point
    pub fn mut_set(&mut self, dimension: usize, value: Unit) {
        self.dimensions[dimension] = value;
    }

    /// Set the value of a dimension by copying values
    pub fn copy_set(&self, dimension: usize, value: Unit) -> NDimensionalPoint<Unit, SumType, S> {
        let mut copied = self.dimensions;
        copied[dimension] = value;
        NDimensionalPoint { dimensions: copied }
    }

    pub fn dimension(&self, dimension: usize) -> &Unit {
        &self.dimensions[dimension]
    }

    pub fn dimension_mut(&mut self, dimension: usize) -> &mut Unit {
        &mut self.dimensions[dimension]
    }
}

// Copy is manually implemented because derive copy doesnt work for slices
impl<Unit, SumType, const S: usize> Copy for NDimensionalPoint<Unit, SumType, S>
where
    Unit: Copy + PartialEq + Add<Output = SumType> + PartialOrd,
    SumType: Copy + PartialOrd,
{
}

/// An Axis Aligned Bounding Box (AABB) is a type of shape that is perfectly aligned with it's axes.
///
/// Examples of such shapes include rectangles for 2D, and cubes for 3D. There is the added
/// constraint that they can not be skewed, but must be perfectly aligned with axis.
#[derive(PartialEq, Copy, Clone)]
#[cfg_attr(test, derive(Debug))]
pub struct AxisAlignedBoundingBox<Unit, SumType, const S: usize>
where
    Unit: Copy + PartialEq + Add<Output = SumType> + PartialOrd + Into<SumType>,
    SumType: Copy + PartialOrd,
{
    origin: NDimensionalPoint<Unit, SumType, S>,
    widths: [Unit; S],
}

impl<Unit, SumType, const S: usize> AxisAlignedBoundingBox<Unit, SumType, S>
where
    Unit: Copy + PartialEq + Add<Output = SumType> + PartialOrd + Into<SumType>,
    SumType: Copy + PartialOrd,
{
    /// Creates a new bounding box
    pub fn new(origin: NDimensionalPoint<Unit, SumType, S>, widths: [Unit; S]) -> Self {
        AxisAlignedBoundingBox { origin, widths }
    }

    /// Returns a reference to the point of origin of this bound
    pub fn origin(&self) -> &NDimensionalPoint<Unit, SumType, S> {
        return &self.origin;
    }

    /// Returns a mutable reference to the point of origin of this bound
    pub fn mut_origin(&mut self) -> &mut NDimensionalPoint<Unit, SumType, S> {
        return &mut self.origin;
    }

    /// Checks if this AABB intersects with another AABB.
    pub fn intersects(&self, other: &AxisAlignedBoundingBox<Unit, SumType, S>) -> bool {
        for i in 0..S {
            let self_min = self.origin.dimension(i);
            let self_max = *self_min + self.widths[i];

            let other_min = other.origin.dimension(i);
            let other_max = *other_min + other.widths[i];

            if self_max <= (*other_min).into() || other_max <= (*self_min).into() {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod test {
    use core::cell::RefCell;
    use core::ops::{Add, Deref};

    use crate::structs::{AxisAlignedBoundingBox, NDimensionalPoint};

    #[test]
    fn can_compare_ndimensional_point() {
        let mut left: NDimensionalPoint<i32, i32, 2> = NDimensionalPoint::new([1, 2]);

        let right = NDimensionalPoint::new([3, 4]);

        assert_ne!(left, right);

        left.mut_set(0, 5);
        left.mut_set(1, 6);
        let right = right.copy_set(0, 5).copy_set(1, 6);

        assert_eq!(left, right);
    }

    #[test]
    fn can_compare_aabb() {
        let point1: NDimensionalPoint<i32, i32, 3> = NDimensionalPoint::new([1, 2, 3]);
        let aabb1 = AxisAlignedBoundingBox::new(point1, [1, 1, 1]);

        let point2: NDimensionalPoint<i32, i32, 3> = NDimensionalPoint::new([1, 2, 3]);
        let mut aabb2 = AxisAlignedBoundingBox::new(point2, [1, 1, 1]);

        assert_eq!(aabb1, aabb2);

        aabb2.mut_origin().mut_set(0, 4);
        assert_ne!(aabb1, aabb2);
    }

    #[derive(PartialEq, Copy, Clone, PartialOrd)]
    struct SomeStruct {
        x: i32,
        y: i32,
    }

    impl Add for SomeStruct {
        type Output = SomeStruct;

        fn add(self, rhs: Self) -> Self::Output {
            SomeStruct {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            }
        }
    }

    #[test]
    fn can_reference() {
        let mut some_struct = SomeStruct { x: 5, y: 6 };
        let mut point: NDimensionalPoint<SomeStruct, SomeStruct, 1> =
            NDimensionalPoint::new([some_struct]);
        assert_eq!(point.dimension(0).x, 5);

        point.dimension_mut(0).x = 7;
        assert_eq!(point.dimension(0).x, 7);
    }
}
