//! Functionality to help working with slices

/// Given a list of items (such as options in a menu), and a selection index, find a subslice that prioritises the selection being in the middle
pub fn find_fitting_subslice<A>(options: &[A], selection: usize, height: usize) -> (&[A], usize) {
    // Handle edge case where the entire list fits within the given height
    if options.len() <= height {
        return (options, 0);
    }

    let half_height = height / 2;

    // Calculate the start of the subslice, trying to keep the selection centered
    let mut start = if selection > half_height {
        selection - half_height
    } else {
        0
    };

    // Ensure the subslice fits within the bounds of the options array
    let end = if start + height > options.len() {
        options.len()
    } else {
        start + height
    };

    // Adjust the start if necessary, in case the subslice is smaller than height
    if end - start < height {
        start = end.saturating_sub(height);
    }

    (&options[start..end], start)
}

#[cfg(test)]
mod tests {
    use crate::algos::slice::subslice::find_fitting_subslice;

    #[test]
    pub fn test_find_subslice() {
        let data = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(
            find_fitting_subslice(&data, 0, 5),
            (&[1, 2, 3, 4, 5] as &[i32], 0)
        );
        assert_eq!(
            find_fitting_subslice(&data, 4, 5),
            (&[3, 4, 5, 6, 7] as &[i32], 2)
        );
        assert_eq!(
            find_fitting_subslice(&data, 7, 5),
            (&[5, 6, 7, 8, 9] as &[i32], 4)
        );
        assert_eq!(
            find_fitting_subslice(&data, 2, 4),
            (&[1, 2, 3, 4] as &[i32], 0)
        );
        assert_eq!(
            find_fitting_subslice(&[0, 1, 2], 2, 20),
            (&[0, 1, 2] as &[i32], 0)
        );
    }
}
