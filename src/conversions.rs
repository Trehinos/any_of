//! This module provides type conversion implementations between the `EitherOf`, `BothOf`, `AnyOf`, and `Couple` types.
//! These conversions allow seamless interchangeability through the use of the `From` trait,
//! enabling ergonomic and straightforward transformations.
//!
//! # Implementations
//!
//! - Library types conversions :
//!     - `From<EitherOf<L, R>> for AnyOf<L, R>`: Converts an [EitherOf] type into an [AnyOf] type by internally delegating to [AnyOf::from_either].
//!     - `From<AnyOf<L, R>> for EitherOf<L, R>`: Converts [AnyOf] back into an [EitherOf] type using [AnyOf::into_either], ensuring proper conversion paths.
//!     - `From<BothOf<L, R>> for AnyOf<L, R>`: Turns a [BothOf] type into an [AnyOf] with the help of [AnyOf::from_both] for consistency.
//!     - `From<AnyOf<L, R>> for BothOf<L, R>`: Allows conversion of [AnyOf] back into [BothOf] using [AnyOf::into_both]. Useful when strong pair semantics are required.
//! - `Couple` (tuple) conversions :
//!     - `From<Couple<L, R>> for BothOf<L, R>`: Converts a [Couple] into a [BothOf], delegating to [BothOf::from_couple]. Handy for handling paired structures.
//!     - `From<Couple<L, R>> for AnyOf<L, R>`: Provides indirect conversion of a [Couple] into an [AnyOf], leveraging the `From<BothOf<L, R>>` for additional flexibility.
//!     - `From<BothOf<L, R>> for Couple<L, R>`: Turns [BothOf] back into [Couple] using [BothOf::into_couple]. This is useful for extracting paired values without additional logic.
//!     - `From<AnyOf<L, R>> for Couple<L, R>`: Turns [AnyOf] back into [Couple], using [AnyOf::into_both] and `BothOf::into()`.
//! - `Any` (tuple of options) conversions :
//!     - `From<Any<L, R>> for AnyOf<L, R>`: Maps higher-level [Opt2] structures into [AnyOf] using [AnyOf::from_opt2].
//!     - `From<Any<L, R>> for BothOf<L, R>`: Attempts to create a [BothOf] from an [Opt2], ensuring both values are present, or panics otherwise. Good for stricter assumptions.
//!     - `From<Any<L, R>> for EitherOf<L, R>`: Converts [Opt2] into [EitherOf], raising errors when invalid configurations like both missing or both present are encountered.
//!     - `From<AnyOf<L, R>> for Any<L, R>`: Takes an [AnyOf]<L, R> and returns an [Opt2] pair with cloned optional values.
//!     - `From<BothOf<L, R>> for Any<L, R>`: Converts [BothOf]<L, R> into [Opt2] with cloned left and right values.
//!     - `From<EitherOf<L, R>> for Any<L, R>`: Converts [EitherOf]<L, R> into [Opt2] by cloning applicable values.
//! 
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
//! let both: BothOf<i32, String> = BothOf { left: 42, right: "hello".to_string() };
//! let any_of_again: AnyOf<i32, String> = AnyOf::from(both);
//! let back_to_both: BothOf<i32, String> = BothOf::from(any_of_again);
//! ```

use crate::{Opt2, AnyOf, BothOf, Couple, EitherOf, LeftOrRight};

impl<L, R> From<EitherOf<L, R>> for AnyOf<L, R> {
    /// Converts an [EitherOf] type into an [AnyOf] type by internally delegating to [AnyOf::from_either].
    fn from(value: EitherOf<L, R>) -> Self {
        Self::from_either(value)
    }
}

impl<L, R> From<AnyOf<L, R>> for EitherOf<L, R> {
    /// Converts [AnyOf] back into an [EitherOf] type using [AnyOf::into_either], ensuring proper conversion paths.
    ///
    /// # Panics
    ///
    /// This function will panic if `value` is not an `Either` variant.
    fn from(value: AnyOf<L, R>) -> Self {
        value.into_either()
    }
}

impl<L, R> From<BothOf<L, R>> for AnyOf<L, R> {
    /// Turns a [BothOf] type into an [AnyOf] with the help of [AnyOf::from_both] for consistency.
    fn from(value: BothOf<L, R>) -> Self {
        Self::from_both(value)
    }
}

impl<L, R> From<AnyOf<L, R>> for BothOf<L, R> {
    /// Allows conversion of [AnyOf] back into [BothOf] using [AnyOf::into_both]. Useful when strong pair semantics are required.
    ///
    /// # Panics
    ///
    /// This function will panic if `value` is not a `Both` variant.
    fn from(value: AnyOf<L, R>) -> Self {
        value.into_both()
    }
}

impl<L, R> From<Couple<L, R>> for BothOf<L, R> {
    /// Converts a [Couple] into a [BothOf], delegating to [AnyOf::from_couple]. Handy for handling paired structures.
    fn from(value: Couple<L, R>) -> Self {
        Self::from_couple(value)
    }
}

impl<L, R> From<Couple<L, R>> for AnyOf<L, R> {
    /// Provides indirect conversion of a [Couple] into an [AnyOf], leveraging the `From<BothOf<L, R>>` for additional flexibility.
    fn from(value: Couple<L, R>) -> Self {
        Self::from(BothOf::from(value))
    }
}

impl<L, R> From<BothOf<L, R>> for Couple<L, R> {
    /// Turns [BothOf] back into [Couple] using [BothOf::into_couple]. This is useful for extracting paired values without additional logic.
    fn from(value: BothOf<L, R>) -> Self {
        value.into_couple()
    }
}

impl<L, R> From<AnyOf<L, R>> for Couple<L, R> {
    /// Turns [AnyOf] back into [Couple], using [AnyOf::into_both] and [BothOf::into()].
    fn from(value: AnyOf<L, R>) -> Self {
        value.into_both().into()
    }
}

impl<L, R> From<Opt2<L, R>> for AnyOf<L, R> {
    /// Maps higher-level [Opt2] structures into [AnyOf] using [AnyOf::from_opt2].
    fn from(value: Opt2<L, R>) -> Self {
        Self::from_opt2(value)
    }
}

impl<L, R> From<Opt2<L, R>> for BothOf<L, R> {
    /// Attempts to create a [BothOf] from an [Opt2], ensuring both values are present, or panics otherwise. Good for stricter assumptions.
    ///
    /// # Panics
    ///
    /// If `value` is not `(Some(L), Some(R))`.
    fn from(value: Opt2<L, R>) -> Self {
        let (left, right) = value;
        Self::new(
            left.expect("Missing left value"),
            right.expect("Missing right value"),
        )
    }
}

impl<L, R> From<Opt2<L, R>> for EitherOf<L, R> {
    /// Converts [Opt2] into [EitherOf], raising errors when invalid configurations like both missing or both present are encountered.
    ///
    /// # Panics
    ///
    /// If `value` is `(Some(L), Some(R))` or if it is `(None, None)`.
    fn from(value: Opt2<L, R>) -> Self {
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

impl<L: Clone, R: Clone> From<AnyOf<L, R>> for Opt2<L, R> {
    /// Takes an [AnyOf]<L, R> and returns an [Opt2] pair with cloned optional values.
    fn from(value: AnyOf<L, R>) -> Self {
        (value.left().cloned(), value.right().cloned())
    }
}

impl<L: Clone, R: Clone> From<BothOf<L, R>> for Opt2<L, R> {
    /// Converts [BothOf]<L, R> into [Opt2] with cloned left and right values.
    fn from(value: BothOf<L, R>) -> Self {
        (value.left().cloned(), value.right().cloned())
    }
}

impl<L: Clone, R: Clone> From<EitherOf<L, R>> for Opt2<L, R> {
    /// Converts [EitherOf]<L, R> into [Opt2] by cloning applicable values.
    fn from(value: EitherOf<L, R>) -> Self {
        (value.left().cloned(), value.right().cloned())
    }
}
