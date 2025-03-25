//! Algorithms that are useful for handling slices of data (such as characters of text, or bytes)

mod rotating;
mod sort;
mod subslice;

pub use rotating::rotate_slice;
pub use sort::insertion_sort_by;
pub use subslice::find_fitting_subslice;
