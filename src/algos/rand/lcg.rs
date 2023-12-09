//! Linear congruential generator.
//!
//! This is a simple random number generator that is not cryptographically secure.

/// Linear congruential generator.
///
/// You can use this to generate random numbers by providing a seed only.
/// For the numbers to seem random, the seed must come from an unpredictable source.
/// Some examples include user input, timing of events, clocks, other sensors that have entropy.
///
/// To use the random number generator, you can do the following:
/// ```
/// use nostd_structs::algos::rand::lcg::LcgRng;
/// let my_name = "Hugh Kaznowski";
/// // Generate Seed
/// let simple_hash = |seed: &str| -> u64 {
///    let mut hash = 0;
///     for c in seed.chars() {
///        hash = c as u64 + ((hash << 5) - hash);
///    }
///   hash
/// };
/// let seed = simple_hash(my_name);
/// // Create RNG
/// let mut rng = LcgRng::new(seed);
/// // Generate random numbers
/// assert_ne!(rng.next(), rng.next());
/// ```
pub struct LcgRng {
    state: u64,
}

impl LcgRng {
    /// Create a new random number generator with a seed
    pub fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    /// Generate the next random number
    pub fn next(&mut self) -> u64 {
        let a: u64 = 1664525;
        let c = 1013904223;
        let m = 2u64.pow(32);
        self.state = (a.wrapping_mul(self.state) + c) % m;
        self.state
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::collections::BTreeSet;

    #[test]
    fn test_lcg() {
        let mut rng = LcgRng::new(0);
        assert_eq!(rng.next(), 1013904223);
        assert_eq!(rng.next(), 1196435762);
        assert_eq!(rng.next(), 3519870697);
        assert_eq!(rng.next(), 2868466484);
        assert_eq!(rng.next(), 1649599747);
    }

    #[test]
    fn doesnt_overflow() {
        let mut rng = LcgRng::new(0);
        let mut used = BTreeSet::new();
        for _ in 0..1000000 {
            let val = rng.next();
            assert!(!used.contains(&val));
            used.insert(val);
        }
    }
}
