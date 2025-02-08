#[cfg(test)]
mod tests {
    use crate::{AnyOf, Both, Either, LeftOrRight, Map, Swap, Unwrap};

    #[test]
    fn test_new() {
        let neither: AnyOf<i32, &str> = AnyOf::new(None, None);
        assert!(neither.is_neither());

        let left: AnyOf<i32, &str> = AnyOf::new(Some(42), None);
        assert!(left.is_left());

        let right: AnyOf<i32, &str> = AnyOf::new(None, Some("Hello"));
        assert!(right.is_right());

        let both: AnyOf<i32, &str> = AnyOf::new(Some(42), Some("World"));
        assert!(both.is_both());
    }

    #[test]
    fn test_new_variants() {
        let neither: AnyOf<i32, &str> = AnyOf::new_neither();
        assert!(neither.is_neither());

        let left: AnyOf<i32, &str> = AnyOf::new_left(42);
        assert!(left.is_left());

        let right: AnyOf<i32, &str> = AnyOf::new_right("Hello");
        assert!(right.is_right());

        let both: AnyOf<i32, &str> = AnyOf::new_both(42, "World");
        assert!(both.is_both());
    }

    #[test]
    fn test_has_left_right() {
        let both = AnyOf::new_both(42, "Hello");
        let left: AnyOf<i32, &str> = AnyOf::new_left(42);
        let right: AnyOf<i32, &str> = AnyOf::new_right("Hello");
        let neither: AnyOf<i32, &str> = AnyOf::new_neither();

        assert!(both.has_left());
        assert!(both.has_right());
        assert!(left.has_left());
        assert!(!left.has_right());
        assert!(right.has_right());
        assert!(!right.has_left());
        assert!(!neither.has_left());
        assert!(!neither.has_right());
    }

    #[test]
    fn test_filter() {
        let both = AnyOf::new_both(42, "World");
        let left_filter = AnyOf::new_left(100);
        let right_filter = AnyOf::new_right("Hello");

        let filtered_left = both.filter(left_filter);
        let filtered_right = both.filter(right_filter);

        assert_eq!(filtered_left, AnyOf::new_right("World"));
        assert_eq!(filtered_right, AnyOf::new_left(42));
    }

    #[test]
    fn test_map() {
        let both: AnyOf<i32, &str> = AnyOf::new_both(2, "text");
        let transformed: AnyOf<i32, usize> = both.map(|left| left * 3, |right| right.len());

        assert!(transformed.is_both());
        assert_eq!(transformed.both_or_none().unwrap().0, &6);
        assert_eq!(transformed.both_or_none().unwrap().1, &4);
    }

    #[test]
    fn test_combine() {
        let left = AnyOf::new_left(42);
        let right = AnyOf::new_right("World");
        let neither = AnyOf::<i32, &str>::new_neither();
        let both = AnyOf::new_both(100, "Hello");

        assert_eq!(left.combine(neither), left);
        assert_eq!(neither.combine(right), right);
        assert_eq!(left.combine(right), AnyOf::new_both(42, "World"));
        assert_eq!(left.combine(both), AnyOf::new_both(42, "Hello"));
    }

    #[test]
    fn test_swap_and_not() {
        let both = Both::new(42, "text");
        let swapped = both.swap();

        assert_eq!(swapped.left, "text");
        assert_eq!(swapped.right, 42);

        let not_both: Both<&str, i32> = !both;

        assert_eq!(not_both.left, "text");
        assert_eq!(not_both.right, 42);
    }

    #[test]
    fn test_either() {
        let either_left: Either<i32, &str> = Either::new_left(42);
        let either_right: Either<i32, &str> = Either::new_right("Hello");

        assert!(either_left.is_left());
        assert!(!either_left.is_right());
        assert!(either_right.is_right());
        assert!(!either_right.is_left());

        let swapped_left: Either<&str, i32> = either_left.swap();
        let swapped_right: Either<&str, i32> = either_right.swap();

        assert_eq!(swapped_left, Either::new_right(42));
        assert_eq!(swapped_right, Either::new_left("Hello"));
    }

    #[test]
    fn test_both_methods() {
        let both = Both::new(50, "value");

        let couple = both.into_couple();
        assert_eq!(couple, (50, "value"));

        let left = both.into_left();
        assert_eq!(left, Either::Left(50));

        let right = both.into_right();
        assert_eq!(right, Either::Right("value"));
    }

    #[test]
    fn test_unwrap() {
        let both = Both::new(1, "text");
        assert_eq!(both.left_or_else(|| 0), 1);
        assert_eq!(both.right_or_else(|| "default"), "text");
    }
}
