//! This module contains the structs that are used as primitives within various data structures
//! and algorithms in this crate.
//!
//! The general idea is that some of the data structures and algorithms require a common interface
//! for accessing them. For example spatial algorithms and data structures will naturally operate
//! on data that is dimensional. For that, this crate provides {{structs::NDimensionalPoint}} and
//! {{structs::AxisAlignedBoundingBox}}. Users can then create these strucutures to whatever sizes
//! and dimensions they need, and conveniently apply them to the algorithms, without having to write
//! Into traits for tuples or arrays.

pub mod algebra;
pub mod geom;
pub mod lru_map;
pub mod trig;

pub use lru_map::LruMap;

use core::ops::Add;

/// A convenient way to cast between 2 types
pub trait AsType<T> {
    /// Convert from type T to Self
    fn from_type(t: T) -> Self;
    /// Convert from Self to type T
    fn into_type(self) -> T;
}

impl AsType<f32> for f32 {
    fn from_type(t: f32) -> Self {
        t
    }

    fn into_type(self) -> f32 {
        self
    }
}

impl AsType<f32> for f64 {
    fn from_type(t: f32) -> Self {
        t as f64
    }

    fn into_type(self) -> f32 {
        self as f32
    }
}

/// A n-dimensional point that is used in the spatial data structures
///
/// - **Unit** is the base type used in the dimensions
/// - **SumType** is the larger unit in case of overflow
/// - **S** is the constant number of dimensions
///
/// An example of how to create a point in 3D space, where x=1, y=2, z=3:
/// ```
/// use nostd_structs::structs::NDimensionalPoint;
///
/// // The precision is i32, and we specify a second i32 type, as that is the type we would need to
/// // prevent integer overflow. Obviously, i32::max_value() + i32::max_value() would be
/// // larger than i32. But the point is that we can specify the type not only for numbers,
/// // but for any struct we would provide.
/// let point = NDimensionalPoint::<i32, i32, 3>::new([1, 2, 3]);
///
/// assert_eq!(*point.dimension(0), 1);
/// assert_eq!(*point.dimension(1), 2);
/// assert_eq!(*point.dimension(2), 3);
/// ```
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

    /// Retrieve a specific dimension from the point
    pub fn dimension(&self, dimension: usize) -> &Unit {
        &self.dimensions[dimension]
    }

    /// Mutably borrow a specific dimension from the point
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

    /// Checks if this AABB intersects with another AABB exclusive of edges.
    pub fn intersects_exc(&self, other: &AxisAlignedBoundingBox<Unit, SumType, S>) -> bool {
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

    /// Checks if this AABB intersects with another AABB inclusive of edges.
    pub fn intersects_inc(&self, other: &AxisAlignedBoundingBox<Unit, SumType, S>) -> bool {
        for i in 0..S {
            let self_min = self.origin.dimension(i);
            let self_max = *self_min + self.widths[i];

            let other_min = other.origin.dimension(i);
            let other_max = *other_min + other.widths[i];

            if self_max < (*other_min).into() || other_max < (*self_min).into() {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod test {
    use core::ops::Add;

    use crate::structs::{AxisAlignedBoundingBox, NDimensionalPoint};

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

    #[test]
    fn can_reference() {
        let some_struct = SomeStruct { x: 5, y: 6 };
        let mut point: NDimensionalPoint<SomeStruct, SomeStruct, 1> =
            NDimensionalPoint::new([some_struct]);
        assert_eq!(point.dimension(0).x, 5);

        point.dimension_mut(0).x = 7;
        assert_eq!(point.dimension(0).x, 7);
    }

    #[test]
    fn check_intersects_inc() {
        let big = AxisAlignedBoundingBox::new(NDimensionalPoint::new([0, 0]), [10, 10]);
        let medium = AxisAlignedBoundingBox::new(NDimensionalPoint::new([5, 5]), [-5, -5]);
        let small = AxisAlignedBoundingBox::new(NDimensionalPoint::new([10, 10]), [-1, -1]);
        let left_medium = AxisAlignedBoundingBox::new(NDimensionalPoint::new([0, 0]), [5, 5]);
        let left_small = AxisAlignedBoundingBox::new(NDimensionalPoint::new([0, 0]), [1, 1]);

        assert!(big.intersects_inc(&medium));
        assert!(big.intersects_inc(&small));
        assert!(big.intersects_inc(&left_medium));
        assert!(!left_medium.intersects_inc(&small));
        assert!(!left_small.intersects_inc(&small));
    }

    #[test]
    fn check_intersects_exc() {
        let big = AxisAlignedBoundingBox::new(NDimensionalPoint::new([0, 0]), [10, 10]);
        let medium = AxisAlignedBoundingBox::new(NDimensionalPoint::new([5, 5]), [-5, -5]);
        let small = AxisAlignedBoundingBox::new(NDimensionalPoint::new([10, 10]), [-1, -1]);
        let left_medium = AxisAlignedBoundingBox::new(NDimensionalPoint::new([0, 0]), [5, 5]);
        let left_small = AxisAlignedBoundingBox::new(NDimensionalPoint::new([0, 0]), [1, 1]);

        assert!(!big.intersects_exc(&medium));
        assert!(!big.intersects_exc(&small));
        assert!(big.intersects_exc(&left_medium));
        assert!(!left_medium.intersects_exc(&small));
        assert!(!left_small.intersects_exc(&small));
    }
}
