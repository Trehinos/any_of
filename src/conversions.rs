//! This module provides type conversion implementations between the `Either`, `Both`,
//! and `AnyOf` types. These conversions allow seamless interchangeability through the use of the `From` trait,
//! enabling ergonomic and straightforward transformations.
//!
//! # Implementations
//!
//! - `From<Either<L, R>> for AnyOf<L, R>`: Converts an `Either` type into an `AnyOf` type.
//! - `From<AnyOf<L, R>> for Either<L, R>`: Converts an `AnyOf` type back into an `Either` type.
//! - `From<Both<L, R>> for AnyOf<L, R>`: Converts a `Both` type into an `AnyOf` type.
//! - `From<AnyOf<L, R>> for Both<L, R>`: Converts an `AnyOf` type back into a `Both` type.
//!
//! # Examples
//!
//! ```rust
//! use any_of::{AnyOf, Both, BothOf, EitherOf, Left};
//!
//! let either: EitherOf<i32, String> = Left(42);
//! let any_of: AnyOf<i32, String> = AnyOf::from(either);
//!
//! let either: EitherOf<i32, String> = Left(42);
//! let any_of_again: AnyOf<i32, String> = AnyOf::from(either);
//! let back_to_either: EitherOf<i32, String> = EitherOf::from(any_of_again);
//!
//! let both: Both<i32, String> = BothOf { left: 42, right: "hello".to_string() };
//! let any_of_again: AnyOf<i32, String> = AnyOf::from(both);
//! let back_to_both: Both<i32, String> = BothOf::from(any_of_again);
//! ```

use crate::{Any, AnyOf, BothOf, Couple, EitherOf, LeftOrRight};

impl<L, R> From<EitherOf<L, R>> for AnyOf<L, R> {
    /// See [Self::from_either].
    fn from(value: EitherOf<L, R>) -> Self {
        Self::from_either(value)
    }
}

impl<L, R> From<AnyOf<L, R>> for EitherOf<L, R> {
    /// See [AnyOf::into_either].
    fn from(value: AnyOf<L, R>) -> Self {
        value.into_either()
    }
}

impl<L, R> From<BothOf<L, R>> for AnyOf<L, R> {
    /// See [Self::from_both].
    fn from(value: BothOf<L, R>) -> Self {
        Self::from_both(value)
    }
}

impl<L, R> From<AnyOf<L, R>> for BothOf<L, R> {
    /// See [AnyOf::into_both].
    fn from(value: AnyOf<L, R>) -> Self {
        value.into_both()
    }
}

impl<L, R> From<Couple<L, R>> for BothOf<L, R> {
    fn from(value: Couple<L, R>) -> Self {
        Self::from_couple(value)
    }
}

impl<L, R> From<Couple<L, R>> for AnyOf<L, R> {
    fn from(value: Couple<L, R>) -> Self {
        Self::from(BothOf::from(value))
    }
}

impl<L, R> From<BothOf<L, R>> for Couple<L, R> {
    fn from(value: BothOf<L, R>) -> Self {
        value.into_couple()
    }
}

impl<L, R> From<AnyOf<L, R>> for Couple<L, R> {
    fn from(value: AnyOf<L, R>) -> Self {
        value.into_both().into()
    }
}

impl<L, R> From<Any<L, R>> for AnyOf<L, R> {
    fn from(value: Any<L, R>) -> Self {
        Self::from_any(value)
    }
}

impl<L, R> From<Any<L, R>> for BothOf<L, R> {
    fn from(value: Any<L, R>) -> Self {
        let (left, right) = value;
        Self::new(
            left.expect("Missing left value"),
            right.expect("Missing right value"),
        )
    }
}

impl<L, R> From<Any<L, R>> for EitherOf<L, R> {
    fn from(value: Any<L, R>) -> Self {
        let (left, right) = value;
        if let Some(left_value) = left {
            if right.is_some() {
                panic!("Cannot convert an Any type into an Either type because both values are present.");
            }
            EitherOf::Left(left_value)
        } else {
            EitherOf::Right(right.expect(
                "Cannot convert an Any type into an Either type because no value is present.",
            ))
        }
    }
}

impl<L: Clone, R: Clone> From<AnyOf<L, R>> for Any<L, R> {
    fn from(value: AnyOf<L, R>) -> Self {
        (value.left().cloned(), value.right().cloned())
    }
}

impl<L: Clone, R: Clone> From<BothOf<L, R>> for Any<L, R> {
    fn from(value: BothOf<L, R>) -> Self {
        (value.left().cloned(), value.right().cloned())
    }
}

impl<L: Clone, R: Clone> From<EitherOf<L, R>> for Any<L, R> {
    fn from(value: EitherOf<L, R>) -> Self {
        (value.left().cloned(), value.right().cloned())
    }
}
