//! Random number generator algorithms
pub mod lcg;

/// A trait for random number generators
pub trait RandomNumberGenerator {
    /// Generate the next random number
    fn next(&mut self) -> u64;
}
