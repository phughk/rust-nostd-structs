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
#![feature(const_for)]
#![deny(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::missing_crate_level_docs)]
#![warn(rustdoc::private_doc_tests)]
#![deny(rustdoc::invalid_codeblock_attributes)]
#![deny(rustdoc::invalid_html_tags)]
#![deny(rustdoc::invalid_rust_codeblocks)]
#![deny(rustdoc::unescaped_backticks)]

extern crate alloc;

pub mod algos;
pub mod conversion;
pub mod structs;
