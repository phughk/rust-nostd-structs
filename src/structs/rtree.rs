use std::marker::PhantomData;

/// An R-Tree is a tree data structure used for spatial access methods,
/// i.e., for indexing multi-dimensional information such as geographical coordinates,
/// rectangles or polygons.
///
pub struct RTree<const Sz: usize, const DimSz: usize, T, D>
where
    T: Dimensionable<DimSz, D>,
    D: PartialOrd + Clone + Copy,
{
    data: [Option<T>; Sz],
    remaining_capacity: usize,
    phantom_data: PhantomData<D>,
    levels: usize,
    page_size: usize,
}

/// Allows for arbitrary dimensions
pub trait Dimensionable<const DimSz: usize, D>: Clone + Copy
where
    D: PartialOrd + Clone + Copy,
{
    fn dimensions(&self) -> &[D; DimSz];
}

pub const fn calc_rtree_size(levels: usize, page_size: usize) -> usize {
    assert!(levels > 0);
    assert!(page_size > 0);
    let mut total_nodes = 0;
    let mut level = 1; // Start at level 1
    let mut power = page_size; // page_size^1

    while level <= levels {
        total_nodes += power;
        power *= page_size;
        level += 1;
    }

    total_nodes
}

impl<const Sz: usize, const DimSz: usize, T, D> RTree<Sz, DimSz, T, D>
where
    T: Dimensionable<DimSz, D>,
    D: PartialOrd + Clone + Copy,
{
    pub fn new(levels: usize, page_size: usize) -> Self {
        let calc = calc_rtree_size(levels, page_size);
        assert_eq!(calc, Sz);
        RTree {
            data: [None; Sz],
            remaining_capacity: Sz,
            levels,
            page_size,
            phantom_data: PhantomData,
        }
    }

    pub const fn new_const(levels: usize, page_size: usize) -> Self {
        let calc = calc_rtree_size(levels, page_size);
        assert_eq!(calc, Sz);
        RTree {
            data: [None; Sz],
            remaining_capacity: Sz,
            levels,
            page_size,
            phantom_data: PhantomData,
        }
    }

    pub fn push(&mut self, item: T) -> Result<(), ()> {
        if self.remaining_capacity == 0 {
            return Err(());
        }
        todo!()
    }
}

#[cfg(test)]
mod test {
    use crate::structs::rtree::calc_rtree_size;
    use std::vec::Vec;
    use std::{format, vec};

    #[test]
    fn test_calc() {
        #[derive(Clone, Debug, PartialEq)]
        struct TestCase {
            levels: usize,
            page_size: usize,
            expected: usize,
            actual: usize,
        }
        let mut cases = vec![];
        for levels in 1..10 {
            for page_size in 1..10 {
                let actual = calc_rtree_size(levels, page_size);
                let expected = (1..=levels)
                    .into_iter()
                    .map(|level| page_size.pow(level as u32))
                    .sum();
                cases.push(TestCase {
                    levels,
                    page_size,
                    expected,
                    actual,
                });
            }
        }
        for case in cases.clone() {
            assert_eq!(
                case.expected,
                case.actual,
                "Cases: \n{}\nCase: {:?}",
                cases
                    .iter()
                    .map(|c| format!("{:?}", c))
                    .collect::<Vec<_>>()
                    .join("\n"),
                case,
            );
        }
    }
}
