//! Nostd Structs contains data structures and algorithms that are intended to be used in a
//! minimal environment.
//!
//! This is useful for cases where you are compiling without access to the standard library, such as
//! in embedded environments.
//!
//! An additional benefit is that, since the memory allocators tend to be tied to the standard
//! library, the data structures in this crate are stack allocated, and do not require a heap.
//! This has performance benefits and leads to predictable memory usage, at the cost of not using
//! memory dynamically.
#![no_std]
#![deny(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::missing_crate_level_docs)]
#![warn(rustdoc::private_doc_tests)]
#![deny(rustdoc::invalid_codeblock_attributes)]
#![deny(rustdoc::invalid_html_tags)]
#![deny(rustdoc::invalid_rust_codeblocks)]
#![deny(rustdoc::unescaped_backticks)]

#[cfg(test)]
extern crate std;

pub mod algos;
pub mod conversion;
pub mod structs;

#[cfg(test)]
/// Compare 2 floats accounting for Nan and precision.
/// Precision is the maximum difference between the two floats
pub fn assert_float_equal_f32(left: f32, right: f32, precision: f32) {
    if left.is_nan() || right.is_nan() {
        assert!(left.is_nan() && right.is_nan(), "{} != {}", left, right);
        return;
    }
    let diff = (left - right).abs();
    claim::assert_lt!(
        diff,
        precision,
        "{} != {} with precision {}",
        left,
        right,
        precision
    );
}

#[cfg(test)]
/// Compare 2 floats accounting for Nan and precision.
/// Precision is the maximum difference between the two floats
pub fn assert_float_equal_f64(left: f64, right: f64, precision: f64) {
    if left.is_nan() || right.is_nan() {
        assert!(left.is_nan() && right.is_nan(), "{} != {}", left, right);
        return;
    }
    let diff = (left - right).abs();
    claim::assert_lt!(
        diff,
        precision,
        "{} != {} with precision {}",
        left,
        right,
        precision
    );
}
