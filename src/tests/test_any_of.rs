use crate::*;

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
    let both = BothOf::new(42, "text");
    let swapped = both.swap();

    assert_eq!(swapped.left, "text");
    assert_eq!(swapped.right, 42);

    let not_both: BothOf<&str, i32> = !both;

    assert_eq!(not_both.left, "text");
    assert_eq!(not_both.right, 42);
}

#[test]
fn test_either() {
    let either_left: EitherOf<i32, &str> = EitherOf::new_left(42);
    let either_right: EitherOf<i32, &str> = EitherOf::new_right("Hello");

    assert!(either_left.is_left());
    assert!(!either_left.is_right());
    assert!(either_right.is_right());
    assert!(!either_right.is_left());

    let swapped_left: EitherOf<&str, i32> = either_left.swap();
    let swapped_right: EitherOf<&str, i32> = either_right.swap();

    assert_eq!(swapped_left, EitherOf::new_right(42));
    assert_eq!(swapped_right, EitherOf::new_left("Hello"));
}

#[test]
fn test_unwrap() {
    let both = BothOf::new(1, "text");
    assert_eq!(both.left_or_else(|| 0), 1);
    assert_eq!(both.right_or_else(|| "default"), "text");
}

#[test]
fn test_from_both() {
    let both = BothOf {
        left: 42,
        right: "Hello",
    };
    let any_of = AnyOf::from_both(both);
    assert!(any_of.is_both());
}

#[test]
fn test_from_opt2() {
    let any = (Some(42), None);
    let any_of: AnyOf<i32, &str> = AnyOf::from_opt2(any);
    assert!(any_of.is_left());
}

#[test]
fn test_from_either() {
    let either: EitherOf<i32, &str> = Left(42);
    let any_of = AnyOf::from_either(either);
    assert!(any_of.is_left());
}

#[test]
fn test_into_both() {
    let any_of = AnyOf::new_both(42, "World");
    let both = any_of.into_both();
    assert_eq!(both.left, 42);
    assert_eq!(both.right, "World");
}

#[test]
fn test_into_either() {
    let any_of: AnyOf<i32, &str> = AnyOf::new_left(42);
    let either = any_of.into_either();
    assert_eq!(either, Left(42));
}

#[test]
fn test_to_either_pair() {
    let any_of = AnyOf::new_both(42, "Hello");
    let (left, right) = any_of.to_either_pair();
    assert_eq!(left, Some(Left(42)));
    assert_eq!(right, Some(Right("Hello")));
}

#[test]
fn test_is_either_and_is_neither_or_both() {
    let neither: AnyOf<i32, &str> = AnyOf::new_neither();
    let left: AnyOf<i32, &str> = AnyOf::new_left(42);
    let right: AnyOf<i32, &str> = AnyOf::new_right("World");
    let both: AnyOf<i32, &str> = AnyOf::new_both(42, "World");

    assert!(!neither.is_either());
    assert!(left.is_either());
    assert!(right.is_either());
    assert!(!both.is_either());

    assert!(neither.is_neither_or_both());
    assert!(!left.is_neither_or_both());
    assert!(!right.is_neither_or_both());
    assert!(both.is_neither_or_both());
}

#[test]
fn test_both_methods() {
    let both = AnyOf::new_both(42, "Hello");

    assert_eq!(both.both_or_none(), Some((&42, &"Hello")));
    assert_eq!(
        both.both_or_else(|| BothOf::new(0, "Default")),
        BothOf::new(42, "Hello")
    );

    let neither: AnyOf<i32, &str> = AnyOf::new_neither();
    assert_eq!(
        neither.both_or_else(|| BothOf::new(0, "Default")),
        BothOf::new(0, "Default")
    );

    let left = AnyOf::new_left(42);
    assert_eq!(
        left.both_or(BothOf::new(0, "Default")),
        BothOf::new(42, "Default")
    );
}

#[test]
fn test_filter_left_and_filter_right() {
    let any_of = AnyOf::new_both(42, "Hello");

    let left_only = any_of.filter_left();
    assert!(left_only.is_left());
    assert_eq!(left_only.into_either(), Left(42));

    let right_only = any_of.filter_right();
    assert!(right_only.is_right());
    assert_eq!(right_only.into_either(), Right("Hello"));
}

#[test]
fn test_with_left_and_with_right() {
    let neither: AnyOf<i32, &str> = AnyOf::new_neither();

    let with_left = neither.with_left(42);
    assert!(with_left.is_left());
    assert_eq!(with_left.into_either(), Left(42));

    let with_right = neither.with_right("World");
    assert!(with_right.is_right());
    assert_eq!(with_right.into_either(), Right("World"));

    let both = with_left.with_right("Hello");
    assert!(both.is_both());
    let both_values = both.both_or_else(|| BothOf::new(0, "Default"));
    assert_eq!(both_values, BothOf::new(42, "Hello"));
}
