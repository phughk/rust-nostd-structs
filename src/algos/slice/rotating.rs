use core::cmp::min;

/// Given a slice of items, such as characters, return 2 slices that can be used to rotate the
/// slice within a specific size.
///
/// This is useful to have rotating text of a given size (such as displaying 100 characters in a
/// 30-character wide display.
///
/// If the text is shorter than the width, then weird behaviour can happen, but it's tested that
/// it won't crash. Best case is to not invoke if the text is shorter. Or modify this code to
/// account for spacing and additional slices
pub fn rotate_slice<T>(source: &[T], frame: usize, step: usize, width: usize) -> (&[T], &[T]) {
    // Find the animation index
    let animation_index = frame / step;
    // Normalise the animation index (as it can wrap around)
    let animation_index = animation_index % source.len();
    // First see if we can fit everything into the first slice
    if animation_index + width < source.len() {
        (&source[animation_index..animation_index + width], &[])
    } else {
        let remaining_width = min(source.len(), width - (source.len() - animation_index));
        (&source[animation_index..], &source[..remaining_width])
    }
}

#[cfg(test)]
mod test {
    use crate::algos::slice::rotating::rotate_slice;
    use std::string::String;

    #[test]
    fn test_rotate_slice() {
        let input = "text rotate".as_bytes();
        let (first, second) = rotate_slice(&input, 0, 2, 4);
        assert_eq!(first, "text".as_bytes());
        assert_eq!(second, &[]);
        let (first, second) = rotate_slice(&input, 1, 2, 4);
        assert_eq!(first, "text".as_bytes());
        assert_eq!(second, &[]);
        let (first, second) = rotate_slice(&input, 2, 2, 4);
        assert_eq!(first, "ext ".as_bytes());
        assert_eq!(second, &[]);
        let (first, second) = rotate_slice(&input, 14, 2, 4);
        assert_eq!(first, "tate".as_bytes());
        assert_eq!(second, &[]);
        let (first, second) = rotate_slice(&input, 16, 2, 4);
        assert_eq!(first, "ate".as_bytes());
        assert_eq!(second, "t".as_bytes());
        let (first, second) = rotate_slice(&input, 18, 2, 4);
        assert_eq!(first, "te".as_bytes());
        assert_eq!(second, "te".as_bytes());
        let (first, second) = rotate_slice(&input, 20, 2, 4);
        assert_eq!(first, "e".as_bytes());
        assert_eq!(second, "tex".as_bytes());
        let (first, second) = rotate_slice(&input, 22, 2, 4);
        assert_eq!(first, "text".as_bytes());
        assert_eq!(second, "".as_bytes());
    }

    #[test]
    fn input_shorter_than_width() {
        let input = "This is 20 chars....".as_bytes();
        assert_eq!(input.len(), 20);
        let (first, second) = rotate_slice(input, 960, 30, 29);
        assert_eq!(String::from_utf8_lossy(first), "hars....");
        assert_eq!(String::from_utf8_lossy(second), "This is 20 chars....");
    }
}
