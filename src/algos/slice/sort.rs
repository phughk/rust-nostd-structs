/// Sorts the slice using the insertion sort algorithm.
pub fn insertion_sort_by<T, F>(slice: &mut [T], mut compare: F)
where
    F: FnMut(&T, &T) -> core::cmp::Ordering,
{
    let len = slice.len();
    for i in 1..len {
        let mut j = i;
        while j > 0 && compare(&slice[j], &slice[j - 1]) == core::cmp::Ordering::Less {
            slice.swap(j, j - 1);
            j -= 1;
        }
    }
}
