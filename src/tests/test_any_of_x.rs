use crate::*;

#[test]
fn test_anyof4_methods() {
    let value: AnyOf4<i32, i32, i32, i32> = AnyOf4::new_left(AnyOf::new_left(1));
    assert_eq!(value.ll(), Some(&1));
    assert_eq!(value.lr(), None);
    assert_eq!(value.rl(), None);
    assert_eq!(value.rr(), None);
    assert_eq!(value.opt4(), (Some(&1), None, None, None));
}

#[test]
fn test_anyof4_constructors() {
    let via_new = AnyOf4::new4(Some(1), None::<i32>, None::<i32>, None::<i32>);
    let via_from: AnyOf4<i32, i32, i32, i32> = AnyOf4::from_opt4((Some(1), None, None, None));
    assert_eq!(via_new.ll(), Some(&1));
    assert_eq!(via_from, via_new);
}

#[test]
fn test_anyof8_opt8() {
    let value: AnyOf8<i32, i32, i32, i32, i32, i32, i32, i32> =
        AnyOf8::new_right(AnyOf4::new_right(AnyOf::new_right(8)));
    let opts = value.opt8();
    assert_eq!(opts.7, Some(&8));
}

#[test]
fn test_anyof8_constructors() {
    let via_new = AnyOf8::new8(
        Some(1), None::<i32>, None::<i32>, None::<i32>, None::<i32>, None::<i32>, None::<i32>, None::<i32>,
    );
    let via_from: AnyOf8<i32, i32, i32, i32, i32, i32, i32, i32> = AnyOf8::from_opt8((
        Some(1), None, None, None, None, None, None, None,
    ));
    assert_eq!(via_new.lll(), Some(&1));
    assert_eq!(via_from, via_new);
}

#[test]
fn test_anyof16_opt16() {
    let value: AnyOf16<i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32> =
        AnyOf16::new_left(AnyOf8::new_left(AnyOf4::new_left(AnyOf::new_left(1))));
    let opts = value.opt16();
    assert_eq!(opts.0, Some(&1));
}

#[test]
fn test_anyof16_constructors() {
    let via_new = AnyOf16::new16(
        Some(1), None::<i32>, None::<i32>, None::<i32>,
        None::<i32>, None::<i32>, None::<i32>, None::<i32>,
        None::<i32>, None::<i32>, None::<i32>, None::<i32>,
        None::<i32>, None::<i32>, None::<i32>, None::<i32>,
    );
    let via_from: AnyOf16<i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32> = AnyOf16::from_opt16((
        Some(1), None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None,
    ));
    assert_eq!(via_new.llll(), Some(&1));
    assert_eq!(via_from, via_new);
}
