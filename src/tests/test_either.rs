use crate::*;

#[test]
fn test_new_left() {
    let either: EitherOf<i32, &str> = EitherOf::new_left(42);
    assert!(either.is_left());
    assert_eq!(either.left(), Some(&42));
    assert_eq!(either.right(), None);
}

#[test]
fn test_new_right() {
    let either: EitherOf<i32, &str> = EitherOf::new_right("Hello");
    assert!(either.is_right());
    assert_eq!(either.right(), Some(&"Hello"));
    assert_eq!(either.left(), None);
}

#[test]
fn test_is_left() {
    let either: EitherOf<i32, &str> = EitherOf::new_left(10);
    assert!(either.is_left());
    assert!(!either.is_right());
}

#[test]
fn test_is_right() {
    let either: EitherOf<i32, &str> = EitherOf::new_right("World");
    assert!(either.is_right());
    assert!(!either.is_left());
}

#[test]
fn test_opt2() {
    let either: EitherOf<i32, &str> = EitherOf::new_left(25);
    assert_eq!(either.opt2(), (Some(&25), None));
    let either: EitherOf<i32, &str> = EitherOf::new_right("Option");
    assert_eq!(either.opt2(), (None, Some(&"Option")));
}

#[test]
fn test_swap() {
    let either: EitherOf<i32, &str> = EitherOf::new_left(100);
    let swapped = either.swap();
    assert!(swapped.is_right());
    assert_eq!(swapped.right(), Some(&100));

    let either: EitherOf<i32, &str> = EitherOf::new_right("Swapped");
    let swapped = either.swap();
    assert!(swapped.is_left());
    assert_eq!(swapped.left(), Some(&"Swapped"));
}

#[test]
fn test_map() {
    let either: EitherOf<i32, &str> = EitherOf::new_left(50);
    let mapped = either.map(|l| l * 2, |r| r.len());
    assert!(mapped.is_left());
    assert_eq!(mapped.left(), Some(&100));

    let either: EitherOf<i32, &str> = EitherOf::new_right("MapTest");
    let mapped = either.map(|l| l * 2, |r| r.len());
    assert!(mapped.is_right());
    assert_eq!(mapped.right(), Some(&7));
}

#[test]
fn test_map_left() {
    let either: EitherOf<i32, &str> = EitherOf::new_left(30);
    let mapped = either.map_left(|l| l + 10);
    assert!(mapped.is_left());
    assert_eq!(mapped.left(), Some(&40));
}

#[test]
fn test_map_right() {
    let either: EitherOf<i32, &str> = EitherOf::new_right("Mapping");
    let mapped = either.map_right(|r| r.len());
    assert!(mapped.is_right());
    assert_eq!(mapped.right(), Some(&7));
}

#[test]
fn test_left_or_else() {
    let either: EitherOf<i32, &str> = EitherOf::new_right("Fallback");
    let value = either.left_or_else(|| 99);
    assert_eq!(value, 99);
}

#[test]
fn test_right_or_else() {
    let either: EitherOf<i32, &str> = EitherOf::new_left(123);
    let value = either.right_or_else(|| "Default");
    assert_eq!(value, "Default");
}

#[test]
fn test_left_or_default() {
    let either: EitherOf<i32, &str> = EitherOf::new_right("Default");
    let value = either.left_or_default();
    assert_eq!(value, 0);
}

#[test]
fn test_right_or_default() {
    let either: EitherOf<i32, &str> = EitherOf::new_left(123);
    let value = either.right_or_default();
    assert_eq!(value, "");
}

#[test]
fn test_expect_left() {
    let either: EitherOf<i32, &str> = EitherOf::new_left(42);
    let value = either.expect_left("Expected left value");
    assert_eq!(value, 42);
}

#[test]
#[should_panic(expected = "Expected right value")]
fn test_expect_right_panics() {
    let either: EitherOf<i32, &str> = EitherOf::new_left(42);
    either.expect_right("Expected right value");
}

#[test]
fn test_unwrap_left() {
    let either: EitherOf<i32, &str> = EitherOf::new_left(36);
    let value = either.unwrap_left();
    assert_eq!(value, 36);
}

#[test]
#[should_panic(expected = "called `unwrap_left` on `LeftOrRight` value that is `Right`")]
fn test_unwrap_left_panics() {
    let either: EitherOf<i32, &str> = EitherOf::new_right("Failure");
    either.unwrap_left();
}

#[test]
fn test_unwrap_right() {
    let either: EitherOf<i32, &str> = EitherOf::new_right("Success");
    let value = either.unwrap_right();
    assert_eq!(value, "Success");
}

#[test]
#[should_panic(expected = "called `unwrap_right` on `LeftOrRight` value that is `Left`")]
fn test_unwrap_right_panics() {
    let either: EitherOf<i32, &str> = EitherOf::new_left(101);
    either.unwrap_right();
}