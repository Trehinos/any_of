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
//! use any_of::{AnyOf, Both, Either};
//!
//! let either: Either<i32, String> = Either::Left(42);
//! let any_of: AnyOf<i32, String> = AnyOf::from(either);
//!
//! let both: Both<i32, String> = Both { left: 42, right: "hello".to_string() };
//! let any_of_again: AnyOf<i32, String> = AnyOf::from(both);
//! let back_to_either: Either<i32, String> = Either::from(any_of_again);
//!
//! let both: Both<i32, String> = Both { left: 42, right: "hello".to_string() };
//! let any_of_again: AnyOf<i32, String> = AnyOf::from(both);
//! let back_to_both: Both<i32, String> = Both::from(any_of_again);
//! ```

use crate::{AnyOf, Both, Either};

impl<L, R> From<Either<L, R>> for AnyOf<L, R> {
    /// See [Self::from_either].
    fn from(value: Either<L, R>) -> Self {
        Self::from_either(value)
    }
}

impl<L, R> From<AnyOf<L, R>> for Either<L, R> {
    /// See [AnyOf::into_either].
    fn from(value: AnyOf<L, R>) -> Self {
        value.into_either()
    }
}

impl<L, R> From<Both<L, R>> for AnyOf<L, R> {
    /// See [Self::from_both].
    fn from(value: Both<L, R>) -> Self {
        Self::from_both(value)
    }
}

impl<L, R> From<AnyOf<L, R>> for Both<L, R> {
    /// See [AnyOf::into_both].
    fn from(value: AnyOf<L, R>) -> Self {
        value.into_both()
    }
}