use crate::types::Size;

/// Returns the index of the first element in the slice `xs` which does not compare less
/// than `value`.
///
/// The index returned by this function may also be equivalent to the index of val, and not only
/// greater. The slice must be sorted.
pub fn lower_bound<T: PartialOrd>(xs: &[T], value: T) -> Size {
    // TODO use binary search instead of a linear scan
    for (i, x) in xs.iter().enumerate() {
        if x >= &value {
            return i;
        }
    }
    xs.len()
}

/// Returns the indxex of the first element in the slice `xs` which compares greater than
/// `value`.
///
/// Unlike lower_bound, the index returned by this function cannot be equivalent to the index
/// of `value`, only greater. The slice must be sorted.
pub fn upper_bound<T: PartialOrd>(xs: &[T], value: T) -> Size {
    // TODO use binary search instead of a linear scan
    for (i, x) in xs.iter().enumerate() {
        if x > &value {
            return i;
        }
    }
    xs.len()
}

// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use crate::maths::bounds::{lower_bound, upper_bound};

    #[test]
    fn test_lower_bound() {
        let xs = vec![1, 3, 7, 10, 15];
        assert_eq!(lower_bound(&xs, 0), 0);
        assert_eq!(lower_bound(&xs, 1), 0);
        assert_eq!(lower_bound(&xs, 3), 1);
        assert_eq!(lower_bound(&xs, 7), 2);
        assert_eq!(lower_bound(&xs, 8), 3);
        assert_eq!(lower_bound(&xs, 10), 3);
        assert_eq!(lower_bound(&xs, 11), 4);
        assert_eq!(lower_bound(&xs, 15), 4);
        assert_eq!(lower_bound(&xs, 20), 5);
    }

    #[test]
    fn test_upper_bound() {
        let xs = vec![1, 3, 7, 10, 15];
        assert_eq!(upper_bound(&xs, 0), 0);
        assert_eq!(upper_bound(&xs, 1), 1);
        assert_eq!(upper_bound(&xs, 3), 2);
        assert_eq!(upper_bound(&xs, 7), 3);
        assert_eq!(upper_bound(&xs, 8), 3);
        assert_eq!(upper_bound(&xs, 10), 4);
        assert_eq!(upper_bound(&xs, 11), 4);
        assert_eq!(upper_bound(&xs, 15), 5);
        assert_eq!(upper_bound(&xs, 20), 5);
    }
}
