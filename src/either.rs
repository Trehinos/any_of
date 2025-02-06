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
//! # Examples
//! Usage of the `Either` enum looks like this:
//!
//! ```rust
//! use any_of::Either;
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

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum Either<L, R> {
    Left(L),
    Right(R),
}

impl<L, R> Either<L, R> {
    pub fn new_left(left: L) -> Self {
        Self::Left(left)
    }
    pub fn new_right(right: R) -> Self {
        Self::Right(right)
    }

    pub fn is_left(&self) -> bool {
        matches!(self, Self::Left(_))
    }

    pub fn is_right(&self) -> bool {
        matches!(self, Self::Right(_))
    }

    pub fn any(&self) -> (Option<&L>, Option<&R>) {
        match self {
            Self::Left(l) => (Some(l), None),
            Self::Right(r) => (None, Some(r)),
        }
    }
    pub fn left(&self) -> Option<&L> {
        match self {
            Self::Left(l) => Some(l),
            Self::Right(_) => None,
        }
    }

    pub fn right(&self) -> Option<&R> {
        match self {
            Self::Left(_) => None,
            Self::Right(r) => Some(r),
        }
    }

    pub fn left_or_else(self, f: impl FnOnce() -> L) -> L {
        match self {
            Self::Left(l) => l,
            Self::Right(_) => f(),
        }
    }
    pub fn right_or_else(self, f: impl FnOnce() -> R) -> R {
        match self {
            Self::Left(_) => f(),
            Self::Right(r) => r,
        }
    }
    pub fn left_or(self, other: L) -> L {
        self.left_or_else(|| other)
    }
    pub fn right_or(self, other: R) -> R {
        self.right_or_else(|| other)
    }
    pub fn unwrap_left(self) -> L {
        self.left_or_else(|| panic!("Can only unwrap left of LeftOrRight::Left"))
    }
    pub fn unwrap_right(self) -> R {
        self.right_or_else(|| panic!("Can only unwrap right of LeftOrRight::Right"))
    }
    pub fn swap(self) -> Either<R, L> {
        match self {
            Self::Left(l) => Either::<R, L>::Right(l),
            Self::Right(r) => Either::<R, L>::Left(r),
        }
    }
    pub fn map_left<FL: FnOnce(L) -> L2, L2>(self, f: FL) -> Either<L2, R> {
        self.map(f, |r| r)
    }
    pub fn map_right<FR: FnOnce(R) -> R2, R2>(self, f: FR) -> Either<L, R2> {
        self.map(|l| l, f)
    }
    pub fn map<FL, FR, L2, R2>(self, f: FL, g: FR) -> Either<L2, R2>
    where
        FL: FnOnce(L) -> L2,
        FR: FnOnce(R) -> R2,
    {
        match self {
            Self::Left(l) => Either::<L2, R2>::Left(f(l)),
            Self::Right(r) => Either::<L2, R2>::Right(g(r)),
        }
    }
}
