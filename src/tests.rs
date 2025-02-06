use crate::AnyOf;
use crate::LeftOrRight;

mod utils {
    use core::ops::Add;

    pub fn mul2<T: Copy + Add<Output = T>>(a: T) -> T {
        a + a
    }
}

#[test]
fn test_any_of() {
    let any_of = AnyOf::new(Some(42u64), Some(1.1));
    let mul2 = any_of.map(utils::mul2, utils::mul2);
    assert_eq!(mul2.unwrap_left(), 84u64);
    assert_eq!(mul2.unwrap_right(), 2.2);
}
#[test]
fn test_any_of_right() {
    let any_of: AnyOf<u64, _> = AnyOf::new(None, Some(1.1));
    let mul2 = any_of.map(utils::mul2, utils::mul2);
    assert_eq!(mul2.left(), None);
    assert_eq!(mul2.unwrap_right(), 2.2);
}


#[test]
fn test_any_of_neither() {
    let any_of: AnyOf<u64, f64> = AnyOf::new(None, None);
    let mul2 = any_of.map(utils::mul2, utils::mul2);
    assert_eq!(mul2.left(), None);
    assert_eq!(mul2.right(), None);
}
