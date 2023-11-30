trait Collection<E> {
    fn add(&self, e: E);
    fn remove(&mut self, e: E);
    fn len(&self) -> usize;
    fn capacity(&self) -> usize;
    fn clear(&mut self);
    fn contains(&self, e: E) -> bool;
}

trait OrderedCollection<E>: Collection<E> {
    fn delete(&self, i: usize);
    fn insert(&self, e: E, i: usize);
    fn get(&self, i: usize) -> E;
}

trait SpatialCollection {

}

struct NDimensionalPoint<U, const S: usize> where U: Default + Copy {
    dimensions: [U; S],
}

impl<U, const S: usize> NDimensionalPoint<U, S> where U: Default + Copy {
    fn new() -> Self {
        NDimensionalPoint {
            dimensions: [U::default(); S],
        }
    }
}

struct AxisAlignedBoundingBox<U, const S: usize> where U: Default + Copy {
    focal_point: NDimensionalPoint<U, S>,
    widths: [U; S],
}

impl<U, const S: usize> AxisAlignedBoundingBox<U, S> where U: Default + Copy {
    fn new() -> Self {
        AxisAlignedBoundingBox {
            focal_point: NDimensionalPoint::new(),
            widths: [U::default(); S],
        }
    }
}