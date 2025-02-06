//! This module provides the `Either` enum, a utility type that represents a value
//! that can be one of two variants: `Left` or `Right`.
//!
//! This is useful for scenarios where a value can have one of two possible types, often used as a lightweight
//! alternative to `Result` or for decision trees in functional programming.
//!
//! The `Either` enum has the following scenarios of use:
//! - `Left`: A variant for holding a value of type `L`.
//! - `Right`: A variant for holding a value of type `R`.
//!
//! # Public API
//! This module provides numerous utility methods for creating, inspecting, and
//! transforming instances of `Either`.
//!
//! ## Creation
//! - [`Either::new_left`]: Creates an `Either` value in the `Left` variant.
//! - [`Either::new_right`]: Creates an `Either` value in the `Right` variant.
//!
//! ## Inspection
//! - [`Either::is_left`]: Returns `true` if the value is `Left`.
//! - [`Either::is_right`]: Returns `true` if the value is `Right`.
//! - [`Either::left`]: Returns a reference to the left value if it exists.
//! - [`Either::right`]: Returns a reference to the right value if it exists.
//! - [`Either::any`]: Returns a tuple of `Option` references to either the left
//!   or the right value, depending on the variant.
//!
//! ## Default Values
//! - [`Either::left_or`]: Returns the left value or a provided default.
//! - [`Either::right_or`]: Returns the right value or a provided default.
//! - [`Either::left_or_else`]: Returns the left value or computes a default using
//!   a closure.
//! - [`Either::right_or_else`]: Returns the right value or computes a default
//!   using a closure.
//!
//! ## Unwrapping
//! - [`Either::unwrap_left`]: Extracts the left value, panicking if the value is
//!   a `Right`.
//! - [`Either::unwrap_right`]: Extracts the right value, panicking if the value is
//!   a `Left`.
//!
//! ## Transformation
//! - [`Either::swap`]: Swaps the `Left` variant for `Right` and vice versa.
//! - [`Either::map_left`]: Applies a function to transform the `Left` value.
//! - [`Either::map_right`]: Applies a function to transform the `Right` value.
//! - [`Either::map`]: Applies separate functions to transform either the `Left`
//!   or `Right` value depending on the variant.
//!
//! ## Examples
//! Usage of the `Either` enum looks like this:
//!
//! ```rust
//! use any_of::Either;
//! use any_of::concepts::LeftOrRight;
//!
//! let left_value: Either<i32, &str> = Either::new_left(10);
//! assert!(left_value.is_left());
//! assert_eq!(left_value.left(), Some(&10));
//!
//! let right_value: Either<i32, &str> = Either::new_right("Hello");
//! assert!(right_value.is_right());
//! assert_eq!(right_value.right(), Some(&"Hello"));
//!
//! // Swapping sides
//! let swapped = right_value.swap();
//! assert!(swapped.is_left());
//! ```
//!

use core::ops::Not;
use crate::LeftOrRight;

/// The `Either` enum is a utility type that can hold a value of one of two variants: `Left(L)` or `Right(R)`.
///
/// It serves as a straightforward alternative to `Result`, providing a way to perform operations
/// on values of two possible types. Unlike `Result`, it does not imply any specific meaning
/// to the variants.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum Either<L, R> {
    Left(L),
    Right(R),
}

impl<L, R> Either<L, R> {
    /// Creates a new `Either` value in the `Left` variant.
    ///
    /// ## Arguments
    ///
    /// * `left` - The value to be stored in the `Left` variant.
    ///
    /// ## Returns
    ///
    /// A new instance of `Either` with the value in the `Left` variant.
    pub fn new_left(left: L) -> Self {
        Self::Left(left)
    }

    /// Creates a new `Either` value in the `Right` variant.
    ///
    /// ## Arguments
    ///
    /// * `right` - The value to be stored in the `Right` variant.
    ///
    /// ## Returns
    ///
    /// A new instance of `Either` with the value in the `Right` variant.
    pub fn new_right(right: R) -> Self {
        Self::Right(right)
    }

    /// Returns the left value or computes a default using the provided closure.
    ///
    /// ## Arguments
    ///
    /// * `f` - A closure that generates a default value if the variant is `Right`.
    ///
    /// ## Returns
    ///
    /// The left value if the variant is `Left`, otherwise the default value generated
    /// by the closure.
    pub fn left_or_else(self, f: impl FnOnce() -> L) -> L {
        match self {
            Self::Left(l) => l,
            Self::Right(_) => f(),
        }
    }

    /// Returns the right value or computes a default using the provided closure.
    ///
    /// ## Arguments
    ///
    /// * `f` - A closure that generates a default value if the variant is `Left`.
    ///
    /// ## Returns
    ///
    /// The right value if the variant is `Right`, otherwise the default value generated
    /// by the closure.
    pub fn right_or_else(self, f: impl FnOnce() -> R) -> R {
        match self {
            Self::Left(_) => f(),
            Self::Right(r) => r,
        }
    }

    /// Returns the left value or a provided default.
    ///
    /// ## Arguments
    ///
    /// * `other` - The default value to use if the variant is `Right`.
    ///
    /// ## Returns
    ///
    /// The left value if the variant is `Left`, otherwise the provided default value.
    pub fn left_or(self, other: L) -> L {
        self.left_or_else(|| other)
    }

    /// Returns the right value or a provided default.
    ///
    /// ## Arguments
    ///
    /// * `other` - The default value to use if the variant is `Left`.
    ///
    /// ## Returns
    ///
    /// The right value if the variant is `Right`, otherwise the provided default value.
    pub fn right_or(self, other: R) -> R {
        self.right_or_else(|| other)
    }

    /// Extracts the left value, panicking if the variant is `Right`.
    ///
    /// ## Panics
    ///
    /// Panics if the variant is `Right`.
    ///
    /// ## Returns
    ///
    /// The left value if the variant is `Left`.
    pub fn unwrap_left(self) -> L {
        self.left_or_else(|| panic!("Can only unwrap left of LeftOrRight::Left"))
    }

    /// Extracts the right value, panicking if the variant is `Left`.
    ///
    /// ## Panics
    ///
    /// Panics if the variant is `Left`.
    ///
    /// ## Returns
    ///
    /// The right value if the variant is `Right`.
    pub fn unwrap_right(self) -> R {
        self.right_or_else(|| panic!("Can only unwrap right of LeftOrRight::Right"))
    }

    /// Swaps the variants of the `Either`, turning a `Left` into a `Right` and vice versa.
    ///
    /// ## Returns
    ///
    /// An `Either` with the variants swapped. The `Left` value becomes `Right`,
    /// and the `Right` value becomes `Left`.
    pub fn swap(self) -> Either<R, L> {
        match self {
            Self::Left(l) => Either::<R, L>::Right(l),
            Self::Right(r) => Either::<R, L>::Left(r),
        }
    }

    /// Transforms the `Left` value using a provided function.
    ///
    /// ## Arguments
    ///
    /// * `f` - A function to transform the left value.
    ///
    /// ## Returns
    ///
    /// An `Either` where the `Left` value has been transformed. The `Right` value,
    /// if present, remains unchanged.
    pub fn map_left<FL: FnOnce(L) -> L2, L2>(self, f: FL) -> Either<L2, R> {
        self.map(f, |r| r)
    }

    /// Transforms the `Right` value using a provided function.
    ///
    /// ## Arguments
    ///
    /// * `f` - A function to transform the right value.
    ///
    /// ## Returns
    ///
    /// An `Either` where the `Right` value has been transformed. The `Left` value,
    /// if present, remains unchanged.
    pub fn map_right<FR: FnOnce(R) -> R2, R2>(self, f: FR) -> Either<L, R2> {
        self.map(|l| l, f)
    }

    /// Transforms the `Left` or `Right` value using separate functions, depending
    /// on the variant.
    ///
    /// ## Arguments
    ///
    /// * `fl` - A function to transform the left value.
    /// * `fr` - A function to transform the right value.
    ///
    /// ## Returns
    ///
    /// An `Either` where the `Left` or `Right` value has been transformed
    /// by the corresponding function.
    pub fn map<FL, FR, L2, R2>(self, fl: FL, fr: FR) -> Either<L2, R2>
    where
        FL: FnOnce(L) -> L2,
        FR: FnOnce(R) -> R2,
    {
        match self {
            Self::Left(l) => Either::<L2, R2>::Left(fl(l)),
            Self::Right(r) => Either::<L2, R2>::Right(fr(r)),
        }
    }
}

impl<L, R> LeftOrRight<L, R> for Either<L, R> {

    /// See [crate::LeftOrRight::is_left].
    fn is_left(&self) -> bool {
        matches!(self, Self::Left(_))
    }

    /// See [crate::LeftOrRight::is_right].
    fn is_right(&self) -> bool {
        matches!(self, Self::Right(_))
    }

    /// See [crate::LeftOrRight::any].
    fn any(&self) -> (Option<&L>, Option<&R>) {
        (self.left(), self.right())
    }

    /// See [crate::LeftOrRight::left].
    fn left(&self) -> Option<&L> {
        match self {
            Self::Left(l) => Some(l),
            Self::Right(_) => None,
        }
    }

    /// See [crate::LeftOrRight::right].
    fn right(&self) -> Option<&R> {
        match self {
            Self::Left(_) => None,
            Self::Right(r) => Some(r),
        }
    }

}

impl<L, R> Not for Either<L, R> {
    type Output = Either<R, L>;

    /// See : [Self::swap].
    fn not(self) -> Self::Output {
        self.swap()
    }
}
