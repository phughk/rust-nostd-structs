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
    fn superset(data: &[Self]) -> Self;
}

impl<const DimSz: usize, D> Dimensionable<DimSz, D> for [D; DimSz]
where
    D: PartialOrd + Clone + Copy,
{
    fn dimensions(&self) -> &[D; DimSz] {
        self
    }

    fn superset(data: &[Self]) -> Self {
        let mut superset = [data[0]; DimSz];
        for d in data.iter() {
            for (i, dim) in d.iter().enumerate() {
                if *dim > superset[i] {
                    superset[i] = *dim;
                }
            }
        }
        superset
    }
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
        assert!(calc == Sz, "RTree size is not equal to calculated size");
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
        let mut best_fit = 0;
        for level in 0..self.levels {
            let mut level_start = if level == 0 {
                0
            } else {
                calc_rtree_size(level, self.page_size)
            };

            level_start += best_fit * self.page_size;
            for (index, entry) in self.data[level_start..level_start + self.page_size]
                .iter_mut()
                .enumerate()
            {
                if entry.is_none() {
                    *entry = Some(item);
                    self.remaining_capacity -= 1;
                    return Ok(());
                }
                best_fit = index;
            }
        }
        unreachable!("RTree is full, but remaining_capacity is not zero");
    }
}

#[cfg(test)]
mod test {
    use crate::structs::rtree::{calc_rtree_size, RTree};
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

    #[test]
    fn test_rtree() {
        const sz: usize = calc_rtree_size(2, 2);
        assert_eq!(sz, 6);
        let mut tree: RTree<sz, 2, [u16; 2], _> = RTree::new_const(2, 2);
        assert_eq!(tree.remaining_capacity, 6);
        tree.push([1, 2]).unwrap();
        assert_eq!(tree.remaining_capacity, 5);
        tree.push([2, 3]).unwrap();
        assert_eq!(tree.remaining_capacity, 4);
        tree.push([3, 4]).unwrap();
        assert_eq!(tree.remaining_capacity, 3);
        tree.push([4, 5]).unwrap();
        assert_eq!(tree.remaining_capacity, 2);
        tree.push([5, 6]).unwrap();
        assert_eq!(tree.remaining_capacity, 1);
        tree.push([6, 7]).unwrap();
        assert_eq!(tree.remaining_capacity, 0);
        assert!(tree.push([7, 8]).is_err());
    }
}
