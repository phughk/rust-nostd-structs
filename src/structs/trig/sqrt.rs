use core::ops::{Add, Div, Mul};

/// Calculate the square root of a value using Newton-Raphson iteration.
/// The number of iterations determines the precision of the result.
pub fn sqrt<T>(val: T, iterations: usize) -> Result<T, ()>
where
    T: PartialOrd + From<f32> + Div<Output = T> + Mul<Output = T> + Add<Output = T> + Copy,
{
    let zero = T::from(0.0);
    if val < zero {
        return Err(());
    }
    if val == zero {
        return Ok(zero);
    }

    // Initial guess
    let two = T::from(2.0);
    let mut guess = val / two;

    // Iterate Newton-Raphson until convergence
    let half = T::from(0.5);
    for _ in 0..iterations {
        guess = half * (guess + val / guess);
    }
    Ok(guess)
}
