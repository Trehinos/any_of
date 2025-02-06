//! # any_of
//!
//! `any_of` is a flexible and lightweight Rust crate designed to handle scenarios
//! where a variable may have one out of several possible values, or none at all.
//! It provides the `AnyOf` enum with variants to represent these possibilities.
//!
//! ## Features
//! - **Enum Variants**:
//!   - `Neither` to represent the absence of values.
//!   - `Either` to represent a single value (`Left` or `Right`).
//!   - `Both` to represent a combination of values (`Left` and `Right`).
//! - **Convenient Methods**: Create, transform, and check the state of `AnyOf` values
//!   easily with utility functions like `new`, `is_neither`, `is_both`, `map_left`, etc.
//! - **Extensible with Type Parameters**: Allows customization of the `Left` and `Right` types.
//!
//! ## Example Usage
//!
//! ```rust
//! use any_of::{AnyOf, either::Either, both::Both};
//!
//! let neither: AnyOf<i32, &str> = AnyOf::Neither;
//! let neither: AnyOf<i32, &str> = AnyOf::new(None, None);
//! let left: AnyOf<i32, &str> = AnyOf::Either(Either::Left(42));
//! let left: AnyOf<i32, &str> = AnyOf::new(Some(42), None);
//! let both: AnyOf<i32, &str> = AnyOf::Both(Both { left: 42, right: "Hello" });
//! let both: AnyOf<i32, &str> = AnyOf::new(Some(42), Some("Hello"));
//!
//! assert!(neither.is_neither());
//! assert!(left.is_left());
//! assert!(both.is_both());
//!
//! assert!(neither.map_right(|r| r).is_neither());
//! assert!(left.map_right(|r| r).is_neither());
//! assert!(left.map_left(|l| l).is_left());
//! assert!(both.map_left(|l| l).is_left());
//! ```
//!
//! ## Crate Features
//! - `std` feature: Enables standard library support for types that require it.
//!
//! ## Use Cases
//! - Representing optional or branching data in a concise manner.
//! - Handling dynamic states with variants like `Neither`, `Either`, and `Both`.
//! - Reducing boilerplate for mapping and transforming multiple optional values.
//!
//! ## Notable methods :
//! - [AnyOf::new]
//! - [AnyOf::combine]
//! - [AnyOf::unwrap_left] and [AnyOf::left]
//! - [AnyOf::unwrap_right] and [AnyOf::right]
//! - [AnyOf::unwrap_both] and [AnyOf::both_or_none]
//!
#![cfg_attr(not(feature = "std"), no_std)]

/// The `(T, U)` tuple. 
pub type Couple<T, U> = (T, U);

/// A shortcut for `Couple<T, T>`.
pub type Pair<T> = Couple<T, T>;

pub mod either;
pub mod both;

use core::ops::Add;
#[cfg(feature = "std")]
use std::ops::Add;
pub use crate::both::Both;
pub use crate::either::Either;

/// Represents a type that can hold one of several variants: `Neither`, `Either` (with `Left` or `Right`),
/// or `Both` (containing both `Left` and `Right` values).
///
/// This enum allows for representing a flexible combination of values, or the absence of any values.
///
/// # Variants
///
/// - `Neither`: Represents the absence of both `Left` and `Right` values.
/// - `Either`: Represents a value that is either `Left` or `Right`.
/// - `Both`: Represents a combination of both `Left` and `Right` values.
///
/// # Type Parameters
///
/// - `L`: The type of the `Left` value.
/// - `R`: The type of the `Right` value. Defaults to the same type as `L` if not specified.
///
/// # Examples
///
/// ```rust
/// use any_of::{AnyOf, either::Either, both::Both};
///
/// let neither: AnyOf<i32, &str> = AnyOf::Neither;
/// let neither: AnyOf<i32, &str> = AnyOf::new(None, None);
/// let left: AnyOf<i32, &str> = AnyOf::Either(Either::Left(42));
/// let left: AnyOf<i32, &str> = AnyOf::new(Some(42), None);
/// let both: AnyOf<i32, &str> = AnyOf::Both(Both { left: 42, right: "Hello" });
/// let both: AnyOf<i32, &str> = AnyOf::new(Some(42), Some("Hello"));
///
/// assert!(neither.is_neither());
/// assert!(left.is_left());
/// assert!(both.is_both());
///
/// assert!(neither.map_right(|r| r).is_neither());
/// assert!(left.map_right(|r| r).is_neither());
/// assert!(left.map_left(|l| l).is_left());
/// assert!(both.map_left(|l| l).is_left());
///
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum AnyOf<L, R = L> {
    Neither,
    Either(Either<L, R>),
    Both(Both<L, R>),
}

impl<L, R> AnyOf<L, R> {
    /// Creates a new `AnyOf` instance based on the presence of `left` and `right` values.
    ///
    /// # Parameters
    ///
    /// - `left`: An `Option` containing the left value of type `L`, or `None` if absent.
    /// - `right`: An `Option` containing the right value of type `R`, or `None` if absent.
    ///
    /// # Returns
    ///
    /// - Returns `AnyOf::Neither` if both `left` and `right` are `None`.
    /// - Returns `AnyOf::Either(Either::Left)` if `left` has a value and `right` is `None`.
    /// - Returns `AnyOf::Either(Either::Right)` if `right` has a value and `left` is `None`.
    /// - Returns `AnyOf::Both` if both `left` and `right` have values.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use any_of::{AnyOf, either::Either, both::Both};
    ///
    /// let neither: AnyOf<i32, &str> = AnyOf::new(None, None);
    /// assert!(neither.is_neither());
    ///
    /// let left: AnyOf<i32, &str> = AnyOf::new(Some(42), None);
    /// assert!(left.is_left());
    ///
    /// let right: AnyOf<i32, &str> = AnyOf::new(None, Some("Hello"));
    /// assert!(right.is_right());
    ///
    /// let both: AnyOf<i32, &str> = AnyOf::new(Some(42), Some("Hello"));
    /// assert!(both.is_both());
    /// ```
    pub fn new(left: Option<L>, right: Option<R>) -> Self {
        match (left, right) {
            (None, None) => Self::Neither,
            (Some(l), None) => Self::Either(Either::Left(l)),
            (None, Some(r)) => Self::Either(Either::Right(r)),
            (Some(l), Some(r)) => Self::Both(Both { left: l, right: r }),
        }
    }
    pub fn new_neither() -> Self {
        Self::new(None, None)
    }
    pub fn new_left(left: L) -> Self {
        Self::new(Some(left), None)
    }
    pub fn new_right(right: R) -> Self {
        Self::new(None, Some(right))
    }
    pub fn new_both(l: L, r: R) -> Self {
        Self::new(Some(l), Some(r))
    }
    pub fn into_both(self) -> Both<L, R> {
        match self {
            AnyOf::Both(b) => b,
            _ => panic!("Can only convert Either::Both to Both"),
        }
    }
    pub fn from_both(both: Both<L, R>) -> Self {
        Self::new(Some(both.left), Some(both.right))
    }

    pub fn into_either(self) -> Either<L, R> {
        match self {
            AnyOf::Either(e) => match e {
                Either::Left(l) => Either::Left(l),
                Either::Right(r) => Either::Right(r),
            },
            _ => panic!("Can only convert Either::Either to Either"),
        }
    }

    /// Gets a [Either] pair from an Either.
    ///
    /// * Either::Neither => None, None
    /// * Either::Left(L) => Some(LeftOrRight::Left(L)), None
    /// * Either::Right(R) => None, Some(LeftOrRight::Right(R))
    /// * Either::Both(L, R) => Some(LeftOrRight::Left(L)), Some(LeftOrRight::Right(R))
    pub fn to_either_pair(&self) -> Pair<Option<Either<L, R>>>
    where
        L: Clone,
        R: Clone,
    {
        let both = self.any();
        let left = both.0.map(|l| Either::Left(l.clone()));
        let right = both.1.map(|r| Either::Right(r.clone()));
        (left, right)
    }

    pub fn from_either(either: Either<L, R>) -> Self {
        match either {
            Either::Left(l) => Self::Either(Either::Left(l)),
            Either::Right(r) => Self::Either(Either::Right(r)),
        }
    }

    pub fn has_left(&self) -> bool {
        matches!(self, Self::Either(Either::Left(_)) | Self::Both(_))
    }
    pub fn is_left(&self) -> bool {
        matches!(self, Self::Either(Either::Left(_)))
    }
    pub fn has_right(&self) -> bool {
        matches!(self, Self::Either(Either::Right(_)) | Self::Both(_))
    }
    pub fn is_right(&self) -> bool {
        matches!(self, Self::Either(Either::Right(_)))
    }
    pub fn is_any(&self) -> bool {
        matches!(
            self,
            Self::Either(Either::Left(_)) | Self::Either(Either::Right(_)) | Self::Both(_)
        )
    }
    pub fn is_one(&self) -> bool {
        matches!(
            self,
            Self::Either(Either::Left(_)) | Self::Either(Either::Right(_))
        )
    }
    pub fn is_both(&self) -> bool {
        matches!(self, Self::Both(_))
    }
    pub fn is_neither(&self) -> bool {
        matches!(self, Self::Neither)
    }
    pub fn is_neither_or_both(&self) -> bool {
        matches!(self, Self::Neither | Self::Both(_))
    }
    pub fn both_or_none(&self) -> Option<Couple<&L, &R>> {
        Some((self.left()?, self.right()?))
    }
    pub fn any(&self) -> Couple<Option<&L>, Option<&R>> {
        (self.left(), self.right())
    }
    pub fn left(&self) -> Option<&L> {
        match self {
            Self::Neither => None,
            Self::Either(Either::Left(l)) => Some(l),
            Self::Either(Either::Right(_)) => None,
            Self::Both(Both { left: l, .. }) => Some(l),
        }
    }
    pub fn right(&self) -> Option<&R> {
        match self {
            Self::Neither => None,
            Self::Either(Either::Left(_)) => None,
            Self::Either(Either::Right(r)) => Some(r),
            Self::Both(Both { right: r, .. }) => Some(r),
        }
    }

    pub fn left_or_else(self, f: impl FnOnce() -> L) -> L {
        match self {
            Self::Neither => f(),
            Self::Either(Either::Left(l)) => l,
            Self::Either(Either::Right(_)) => f(),
            Self::Both(Both { left: l, .. }) => l,
        }
    }
    pub fn right_or_else(self, f: impl FnOnce() -> R) -> R {
        match self {
            Self::Neither => f(),
            Self::Either(Either::Left(_)) => f(),
            Self::Either(Either::Right(r)) => r,
            Self::Both(Both { right: r, .. }) => r,
        }
    }
    pub fn both_or_else(self, f: impl FnOnce() -> Both<L, R>) -> Both<L, R> {
        match self {
            Self::Neither => f(),
            Self::Either(Either::Left(l)) => Both::new(l, f().right),
            Self::Either(Either::Right(r)) => Both::new(f().left, r),
            Self::Both(b) => b,
        }
    }
    pub fn left_or(self, other: L) -> L {
        self.left_or_else(|| other)
    }
    pub fn right_or(self, other: R) -> R {
        self.right_or_else(|| other)
    }
    pub fn both_or(self, other: Both<L, R>) -> Both<L, R> {
        self.both_or_else(|| other)
    }
    pub fn unwrap_left(self) -> L {
        self.left_or_else(|| panic!("Can only unwrap left of Either::Left or Either::Both"))
    }
    pub fn unwrap_right(self) -> R {
        self.right_or_else(|| panic!("Can only unwrap right of Either::Right or Either::Both"))
    }
    pub fn unwrap_both(self) -> Both<L, R> {
        self.both_or_else(|| panic!("Can only unwrap both of Either::Both"))
    }
    pub fn filter_left(self) -> Self {
        match self {
            Self::Neither => Self::Neither,
            Self::Either(Either::Left(l)) => Self::Either(Either::Left(l)),
            Self::Either(Either::Right(_)) => Self::Neither,
            Self::Both(Both { left: l, .. }) => Self::Either(Either::Left(l)),
        }
    }
    pub fn filter_right(self) -> Self {
        match self {
            Self::Neither => Self::Neither,
            Self::Either(Either::Left(_)) => Self::Neither,
            Self::Either(Either::Right(r)) => Self::Either(Either::Right(r)),
            Self::Both(Both { right: r, .. }) => Self::Either(Either::Right(r)),
        }
    }
    pub fn with_right(self, right: R) -> Self {
        match self {
            Self::Neither => Self::Either(Either::Right(right)),
            Self::Either(Either::Left(l)) => Self::Both(Both { left: l, right }),
            Self::Either(Either::Right(_)) => Self::Either(Either::Right(right)),
            Self::Both(Both { left: l, .. }) => Self::Both(Both { left: l, right }),
        }
    }
    pub fn with_left(self, left: L) -> Self {
        match self {
            Self::Neither => Self::Either(Either::Left(left)),
            Self::Either(Either::Left(_)) => Self::Either(Either::Left(left)),
            Self::Either(Either::Right(r)) => Self::Both(Both { left, right: r }),
            Self::Both(Both { right: r, .. }) => Self::Both(Both { left, right: r }),
        }
    }
    pub fn swap(self) -> AnyOf<R, L> {
        match self {
            Self::Neither => AnyOf::<R, L>::Neither,
            Self::Either(Either::Left(l)) => AnyOf::<R, L>::Either(Either::Right(l)),
            Self::Either(Either::Right(r)) => AnyOf::<R, L>::Either(Either::Left(r)),
            Self::Both(Both { left: l, right: r }) => {
                AnyOf::<R, L>::Both(Both { left: r, right: l })
            }
        }
    }
    pub fn map_left<FL, L2>(self, f: FL) -> AnyOf<L2, R>
    where
        FL: FnOnce(L) -> L2,
    {
        self.map(f, |r| r)
    }
    pub fn map_right<FR, R2>(self, f: FR) -> AnyOf<L, R2>
    where
        FR: FnOnce(R) -> R2,
    {
        self.map(|l| l, f)
    }

    pub fn map<FL, FR, L2, R2>(self, fl: FL, fr: FR) -> AnyOf<L2, R2>
    where
        FL: FnOnce(L) -> L2,
        FR: FnOnce(R) -> R2,
    {
        match self {
            Self::Neither => AnyOf::<L2, R2>::Neither,
            Self::Either(Either::Left(l)) => AnyOf::<L2, R2>::Either(Either::Left(fl(l))),
            Self::Either(Either::Right(r)) => AnyOf::<L2, R2>::Either(Either::Right(fr(r))),
            Self::Both(Both { left: l, right: r }) => AnyOf::<L2, R2>::Both(Both {
                left: fl(l),
                right: fr(r),
            }),
        }
    }

    /// Combines (`+` operator) two `Either` values into a single one.
    ///
    /// ## General rules
    ///
    /// * `Neither` is always substituted by the other operand,
    /// * `Both` :
    ///     * as left operand : substitutes the other operand,
    ///     * as right operand : completes the other operand,
    /// * `Left` or `Right` :
    ///     * `L+R` or `R+L` combines to an instance of `Both`,
    ///     * `L+l` or `r+R` selects the operand placed on the correct side of the operator :
    ///         * left**Left** + right**Left** = left**Left**
    ///         * left**Right** + right**Right** = right**Right**
    ///
    /// ## All cases
    ///
    /// * Neither cases :
    ///     * Neither + **other** = other
    ///     * **other** + Neither = other
    /// * Trivial cases :
    ///     * **Left(x)** + Left(y) = Left(x)
    ///     * Right(x) + **Right(y)** = Right(y)
    ///     * **Both(x, y)** + other = Both(x, y)
    /// * Merge cases :
    ///     * Left(x) + Right(y) = Both(x, y)
    ///     * Right(x) + Left(y) = Both(y, x)
    ///     * Left(x) + Both(_, y) = Both(x, y)
    ///     * Right(x) + Both(y, _) = Both(y, x)
    pub fn combine(self, other: Self) -> Self {
        match self {
            Self::Neither => other,
            Self::Either(Either::Left(l)) => match other {
                AnyOf::Neither => Self::Either(Either::Left(l)),
                AnyOf::Either(Either::Left(_)) => Self::Either(Either::Left(l)),
                AnyOf::Either(Either::Right(r)) => Self::Both(Both { left: l, right: r }),
                AnyOf::Both(Both { right: r, .. }) => Self::Both(Both { left: l, right: r }),
            },
            Self::Either(Either::Right(r)) => match other {
                AnyOf::Neither => Self::Either(Either::Right(r)),
                AnyOf::Either(Either::Left(l)) => Self::Both(Both { left: l, right: r }),
                AnyOf::Either(Either::Right(r2)) => Self::Either(Either::Right(r2)),
                AnyOf::Both(Both { left: l, .. }) => Self::Both(Both { left: l, right: r }),
            },
            Self::Both(b) => Self::Both(b),
        }
    }
}

//#[cfg(feature = "std")]
impl<L, R> Add for AnyOf<L, R> {
    type Output = ();

    fn add(self, rhs: Self) -> Self::Output {
        self.combine(rhs);
    }
}

pub mod any_of_n;