//! This module provides the `EitherOf` enum, a utility type that represents a value
//! that can be one of two variants: `Left` or `Right`.
//!
//! This is useful for scenarios where a value can have one of two possible types, often used as a lightweight
//! alternative to `Result` or for decision trees in functional programming.
//!
//! The `EitherOf` enum has the following scenarios of use:
//! - `Left`: A variant for holding a value of type `L`.
//! - `Right`: A variant for holding a value of type `R`.
//!
//! # Public API
//! This module provides numerous utility methods for creating, inspecting, and
//! transforming instances of `EitherOf`.
//!
//! ## Creation
//! - [`EitherOf::new_left`]: Creates an `EitherOf` value in the `Left` variant.
//! - [`EitherOf::new_right`]: Creates an `EitherOf` value in the `Right` variant.
//!
//! ## Inspection
//! - [`EitherOf::is_left`]: Returns `true` if the value is `Left`.
//! - [`EitherOf::is_right`]: Returns `true` if the value is `Right`.
//! - [`EitherOf::Left`]: Returns a reference to the left value if it exists.
//! - [`EitherOf::Right`]: Returns a reference to the right value if it exists.
//! - [`EitherOf::opt2`]: Returns a tuple of `Option` references to either the left
//!   or the right value, depending on the variant.
//!
//! ## Default Values
//! - [`EitherOf::left_or`]: Returns the left value or a provided default.
//! - [`EitherOf::right_or`]: Returns the right value or a provided default.
//! - [`EitherOf::left_or_else`]: Returns the left value or computes a default using
//!   a closure.
//! - [`EitherOf::right_or_else`]: Returns the right value or computes a default
//!   using a closure.
//!
//! ## Unwrapping
//! - [`EitherOf::unwrap_left`]: Extracts the left value, panicking if the value is
//!   a `Right`.
//! - [`EitherOf::unwrap_right`]: Extracts the right value, panicking if the value is
//!   a `Left`.
//!
//! ## Transformation
//! - [`EitherOf::swap`]: Swaps the `Left` variant for `Right` and vice versa.
//! - [`EitherOf::map_left`]: Applies a function to transform the `Left` value.
//! - [`EitherOf::map_right`]: Applies a function to transform the `Right` value.
//! - [`EitherOf::map`]: Applies separate functions to transform either the `Left`
//!   or `Right` value depending on the variant.
//!
//! ## Examples
//! Usage of the `EitherOf` enum looks like this:
//!
//! ```rust
//! use any_of::{EitherOf, LeftOrRight, Swap};
//!
//! let left_value: EitherOf<i32, &str> = EitherOf::new_left(10);
//! assert!(left_value.is_left());
//! assert_eq!(left_value.left(), Some(&10));
//!
//! let right_value: EitherOf<i32, &str> = EitherOf::new_right("Hello");
//! assert!(right_value.is_right());
//! assert_eq!(right_value.right(), Some(&"Hello"));
//!
//! // Swapping sides
//! let swapped = right_value.swap();
//! assert!(swapped.is_left());
//! ```
//!

use crate::concepts::{Map, Unwrap};
use crate::{BothOf, LeftOrRight, Swap};
use core::ops::{Not, Shr};

/// The `EitherOf` enum is a utility type that can hold a value of one of two variants: `Left(L)` or `Right(R)`.
///
/// This type is exported as `any_of::EitherOf`.
///
/// It serves as a straightforward alternative to `Result`, providing a way to perform operations
/// on values of two possible types. Unlike `Result`, it does not imply any specific meaning
/// to the variants.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum EitherOf<L, R = L> {
    Left(L),
    Right(R),
}

impl<L, R> EitherOf<L, R> {
    /// Creates a new `EitherOf` value in the `Left` variant.
    ///
    /// ## Arguments
    ///
    /// * `left` - The value to be stored in the `Left` variant.
    ///
    /// ## Returns
    ///
    /// A new instance of `EitherOf` with the value in the `Left` variant.
    pub fn new_left(left: L) -> Self {
        Self::Left(left)
    }

    /// Creates a new `EitherOf` value in the `Right` variant.
    ///
    /// ## Arguments
    ///
    /// * `right` - The value to be stored in the `Right` variant.
    ///
    /// ## Returns
    ///
    /// A new instance of `EitherOf` with the value in the `Right` variant.
    pub fn new_right(right: R) -> Self {
        Self::Right(right)
    }
}

impl<L, R> LeftOrRight<L, R> for EitherOf<L, R> {
    fn left(&self) -> Option<&L> {
        match self {
            Self::Left(l) => Some(l),
            Self::Right(_) => None,
        }
    }

    fn right(&self) -> Option<&R> {
        match self {
            Self::Left(_) => None,
            Self::Right(r) => Some(r),
        }
    }
}

impl<L, R> Not for EitherOf<L, R> {
    type Output = EitherOf<R, L>;

    /// Swaps the variants of the `EitherOf`, turning a `Left` into a `Right` and vice versa.
    ///
    /// ## Returns
    ///
    /// An `EitherOf` with the variants swapped. The `Left` value becomes `Right`,
    /// and the `Right` value becomes `Left`.
    fn not(self) -> Self::Output {
        match self {
            Self::Left(l) => EitherOf::<R, L>::Right(l),
            Self::Right(r) => EitherOf::<R, L>::Left(r),
        }
    }
}
impl<L, R> Swap<L, R> for EitherOf<L, R> {
    type Output = <Self as Not>::Output;
}

impl<L, R> Map<L, R> for EitherOf<L, R> {
    type Output<L2, R2> = EitherOf<L2, R2>;

    /// Transforms the `L` or `R` value using separate functions, depending
    /// on the variant.
    ///
    /// # Type Parameters
    /// - `L2`: The resulting type of the transformed `left` value.
    /// - `R2`: The resulting type of the transformed `right` value.
    /// - `FL`: The type of the function used to transform the `left` value.
    /// - `FR`: The type of the function used to transform the `right` value.
    ///
    /// # Arguments
    /// - `fl`: A function that takes the `left` value and transforms it into a value of type `L2`.
    /// - `fr`: A function that takes the `right` value and transforms it into a value of type `R2`.
    ///
    /// ## Returns
    ///
    /// An `EitherOf` where the `Left` or `Right` value has been transformed
    /// by the corresponding function.
    fn map<FL, FR, L2, R2>(self, fl: FL, fr: FR) -> Self::Output<L2, R2>
    where
        FL: FnOnce(L) -> L2,
        FR: FnOnce(R) -> R2,
    {
        self >> (fl, fr).into()
    }
}

impl<L, R, FL, FR, L2, R2> Shr<BothOf<FL, FR>> for EitherOf<L, R>
where
    FL: FnOnce(L) -> L2,
    FR: FnOnce(R) -> R2,
{
    type Output = EitherOf<L2, R2>;

    fn shr(self, rhs: BothOf<FL, FR>) -> Self::Output {
        match self {
            Self::Left(l) => EitherOf::<L2, R2>::Left((rhs.left)(l)),
            Self::Right(r) => EitherOf::<L2, R2>::Right((rhs.right)(r)),
        }
    }
}

impl<L, R> Unwrap<L, R> for EitherOf<L, R> {
    fn left_or_else(self, f: impl FnOnce() -> L) -> L {
        match self {
            Self::Left(l) => l,
            Self::Right(_) => f(),
        }
    }

    fn right_or_else(self, f: impl FnOnce() -> R) -> R {
        match self {
            Self::Left(_) => f(),
            Self::Right(r) => r,
        }
    }
}
