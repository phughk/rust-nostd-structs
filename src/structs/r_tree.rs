use crate::structs::AxisAlignedBoundingBox;

/// An R-Tree is a data structure that is used to store spatial data.
///
/// It can be used to answer which objects are close in proximity to other objects.
/// This is useful in collision detection and path finding.
#[derive(Copy, Clone)]
pub struct RTree<'tree, 'aabb, UNIT, const DIMENSIONS: usize, const LAYER_SZ: usize>
where
    UNIT: Copy + PartialOrd + Default,
{
    children: [RTreeNode<'tree, 'aabb, UNIT, DIMENSIONS, LAYER_SZ>; LAYER_SZ],
}

/// An R-Tree consists of layers of elements. The elements in the layer are this object.
/// The R-Tree owns it's layers, but it borrows the data it points to.
#[derive(Copy, Clone)]
enum RTreeNode<'tree, 'aabb, UNIT, const DIMENSIONS: usize, const LAYER_SZ: usize>
where
    UNIT: Copy + PartialOrd + Default,
{
    Unused,
    Leaf([&'aabb AxisAlignedBoundingBox<UNIT, DIMENSIONS>; DIMENSIONS]),
    Node(&'tree RTree<'tree, 'aabb, UNIT, DIMENSIONS, LAYER_SZ>),
}

impl<'tree, 'aabb, UNIT, const DIMENSIONS: usize, const LAYER_SZ: usize>
    RTree<'tree, 'aabb, UNIT, DIMENSIONS, LAYER_SZ>
where
    UNIT: Copy + PartialOrd + Default,
{
    /// Creates a new R-Tree with the given capacity.
    pub fn new() -> Self {
        Self {
            children: [RTreeNode::Unused; LAYER_SZ],
        }
    }

    pub fn add(&mut self, bounding_box: &AxisAlignedBoundingBox<UNIT, DIMENSIONS>) {
        panic!("Not implemented yet")
    }

    pub fn len(&self) -> usize {
        panic!("Not implemented yet")
    }

    /// Checks if an object contains another
    pub fn contains(&self) -> bool {
        panic!("Not implemented yet")
    }

    /// Populates a provided slice with the bounding boxes that are contained in the given bounding box.
    pub fn contents(&self, container: &mut [AxisAlignedBoundingBox<UNIT, DIMENSIONS>]) {
        panic!("Not implemented yet")
    }
}

#[cfg(test)]
mod test {
    use crate::structs::NDimensionalPoint;

    #[test]
    fn can_add_bounds() {
        let mut rtree = super::RTree::<i32, 2, 2>::new();
        assert_eq!(rtree.len(), 0);

        let bounding_box = super::AxisAlignedBoundingBox::<i32, 2>::new(
            NDimensionalPoint::new().copy_set(0, 5).copy_set(1, 6),
        );
        rtree.add(&bounding_box);
        assert_eq!(rtree.len(), 1);
    }

    #[test]
    fn can_not_overfill() {}
}
