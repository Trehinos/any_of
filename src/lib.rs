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
#![no_std]

/// The `(T, U)` tuple.
pub type Couple<T, U> = (T, U);

/// A shortcut for `Couple<T, T>`.
pub type Pair<T> = Couple<T, T>;

pub mod both;
pub mod either;

use core::ops::{BitAnd, BitOr, Not};

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

    /// Creates an `AnyOf::Neither` variant.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use any_of::AnyOf;
    ///
    /// let neither: AnyOf<i32, &str> = AnyOf::new_neither();
    /// assert!(neither.is_neither());
    /// ```
    pub fn new_neither() -> Self {
        Self::new(None, None)
    }

    /// Creates an `AnyOf::Either` variant containing a `Left` value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use any_of::AnyOf;
    ///
    /// let left: AnyOf<i32, &str> = AnyOf::new_left(42);
    /// assert!(left.is_left());
    /// ```
    pub fn new_left(left: L) -> Self {
        Self::new(Some(left), None)
    }

    /// Creates an `AnyOf::Either` variant containing a `Right` value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use any_of::AnyOf;
    ///
    /// let right: AnyOf<i32, &str> = AnyOf::new_right("Hello");
    /// assert!(right.is_right());
    /// ```
    pub fn new_right(right: R) -> Self {
        Self::new(None, Some(right))
    }

    /// Creates an `AnyOf::Both` variant containing both a `Left` and `Right` value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use any_of::AnyOf;
    ///
    /// let both: AnyOf<i32, &str> = AnyOf::new_both(42, "Hello");
    /// assert!(both.is_both());
    /// ```
    pub fn new_both(l: L, r: R) -> Self {
        Self::new(Some(l), Some(r))
    }

    /// Converts the `AnyOf` variant to a `Both` struct.
    ///
    /// # Panics
    ///
    /// This function will panic if `self` is not an `AnyOf::Both` variant.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use any_of::AnyOf;
    ///
    /// let both = AnyOf::new_both(42, "Hello").into_both();
    /// assert_eq!(both.left, 42);
    /// assert_eq!(both.right, "Hello");
    /// ```
    pub fn into_both(self) -> Both<L, R> {
        match self {
            AnyOf::Both(b) => b,
            _ => panic!("Can only convert Either::Both to Both"),
        }
    }

    /// Creates an `AnyOf::Both` variant from a `Both` struct.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use any_of::{AnyOf, Both};
    ///
    /// let both_struct = Both { left: 42, right: "Hello" };
    /// let both = AnyOf::from_both(both_struct);
    /// assert!(both.is_both());
    /// ```
    pub fn from_both(both: Both<L, R>) -> Self {
        Self::new(Some(both.left), Some(both.right))
    }

    /// Converts the `AnyOf` variant to an `Either` struct.
    ///
    /// # Panics
    ///
    /// This function will panic if `self` is not an `AnyOf::Either` variant.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use any_of::{AnyOf, Either};
    ///
    /// let either: Either<i32, ()> = AnyOf::new_left(42).into_either();
    /// assert_eq!(either, Either::Left(42));
    /// ```
    pub fn into_either(self) -> Either<L, R> {
        if let AnyOf::Either(e) = self {
            e
        } else {
            panic!("Can only convert Either::Either to Either");
        }
    }

    /// Converts the current `AnyOf` variant into a pair of options containing `Either` variants.
    ///
    /// # Returns
    ///
    /// - `(Some(Left), None)` for `AnyOf::Either(Either::Left)`.
    /// - `(None, Some(Right))` for `AnyOf::Either(Either::Right)`.
    /// - `(Some(Left), Some(Right))` for `AnyOf::Both`.
    /// - `(None, None)` for `AnyOf::Neither`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use any_of::{AnyOf, Either};
    ///
    /// let any_of = AnyOf::new_both(42, "Hello");
    /// let (left, right) = any_of.to_either_pair();
    /// assert_eq!(left, Some(Either::Left(42)));
    /// assert_eq!(right, Some(Either::Right("Hello")));
    /// ```
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

    /// Creates an `AnyOf` variant from an `Either` struct.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use any_of::{AnyOf, Either};
    ///
    /// let either: Either<i32, ()> = Either::Left(42);
    /// let any_of = AnyOf::from_either(either);
    /// assert!(any_of.is_left());
    /// ```
    pub fn from_either(either: Either<L, R>) -> Self {
        Self::Either(either)
    }

    /// True if [Either::Left] or [AnyOf::Both].
    pub fn has_left(&self) -> bool {
        matches!(self, Self::Either(Either::Left(_)) | Self::Both(_))
    }
    /// True if [Either::Left].
    pub fn is_left(&self) -> bool {
        matches!(self, Self::Either(Either::Left(_)))
    }
    /// True if [Either::Right] or [AnyOf::Both].
    pub fn has_right(&self) -> bool {
        matches!(self, Self::Either(Either::Right(_)) | Self::Both(_))
    }
    /// True if [Either::Right].
    pub fn is_right(&self) -> bool {
        matches!(self, Self::Either(Either::Right(_)))
    }
    /// True if not [AnyOf::Neither].
    pub fn is_any(&self) -> bool {
        matches!(
            self,
            Self::Either(Either::Left(_)) | Self::Either(Either::Right(_)) | Self::Both(_)
        )
    }
    /// True if [AnyOf::Either].
    pub fn is_one(&self) -> bool {
        matches!(self, Self::Either(_))
    }
    /// True if [AnyOf::Both]
    pub fn is_both(&self) -> bool {
        matches!(self, Self::Both(_))
    }
    /// True if [AnyOf::Neither]
    pub fn is_neither(&self) -> bool {
        matches!(self, Self::Neither)
    }
    /// True if not [AnyOf::Either]
    pub fn is_neither_or_both(&self) -> bool {
        matches!(self, Self::Neither | Self::Both(_))
    }

    /// Returns `Some((&L, &R))` if `self.is_both()` is true, or `None`.
    pub fn both_or_none(&self) -> Option<Couple<&L, &R>> {
        Some((self.left()?, self.right()?))
    }

    /// Returns the left and/or right part(s) of this instance.
    ///
    /// Returns :
    /// * (None, None) if this instance is [Self::Neither],
    /// * (Some(&L), None) if this instance is [Either::Left]
    /// * (None, Some(&R)) if this instance is [Either::Right]
    /// * (Some(&L), Some(&R)) if this instance is [Self::Both]
    pub fn any(&self) -> Couple<Option<&L>, Option<&R>> {
        (self.left(), self.right())
    }

    /// Returns `Some(&L)` if `self.has_left()` is true, or `None`.
    pub fn left(&self) -> Option<&L> {
        match self {
            Self::Neither => None,
            Self::Either(Either::Left(l)) => Some(l),
            Self::Either(Either::Right(_)) => None,
            Self::Both(Both { left: l, .. }) => Some(l),
        }
    }

    /// Returns `Some(&R)` if `self.has_right()` is true, or `None`.
    pub fn right(&self) -> Option<&R> {
        match self {
            Self::Neither => None,
            Self::Either(Either::Left(_)) => None,
            Self::Either(Either::Right(r)) => Some(r),
            Self::Both(Both { right: r, .. }) => Some(r),
        }
    }

    /// Returns the left value if present, or computes it with the provided function.
    pub fn left_or_else(self, f: impl FnOnce() -> L) -> L {
        match self {
            Self::Neither => f(),
            Self::Either(Either::Left(l)) => l,
            Self::Either(Either::Right(_)) => f(),
            Self::Both(Both { left: l, .. }) => l,
        }
    }

    /// Returns the right value if present, or computes it with the provided function.
    pub fn right_or_else(self, f: impl FnOnce() -> R) -> R {
        match self {
            Self::Neither => f(),
            Self::Either(Either::Left(_)) => f(),
            Self::Either(Either::Right(r)) => r,
            Self::Both(Both { right: r, .. }) => r,
        }
    }

    /// Returns both values if present, or computes them with the provided function.
    pub fn both_or_else(self, f: impl FnOnce() -> Both<L, R>) -> Both<L, R> {
        match self {
            Self::Neither => f(),
            Self::Either(Either::Left(l)) => Both::new(l, f().right),
            Self::Either(Either::Right(r)) => Both::new(f().left, r),
            Self::Both(b) => b,
        }
    }

    /// Returns the left value if present, or the provided default value.
    pub fn left_or(self, other: L) -> L {
        self.left_or_else(|| other)
    }

    /// Returns the right value if present, or the provided default value.
    pub fn right_or(self, other: R) -> R {
        self.right_or_else(|| other)
    }

    /// Returns both values if present, or the provided default values.
    pub fn both_or(self, other: Both<L, R>) -> Both<L, R> {
        self.both_or_else(|| other)
    }

    /// Unwraps and returns the left value, panicking if not available.
    pub fn unwrap_left(self) -> L {
        self.left_or_else(|| panic!("Can only unwrap left of Either::Left or Either::Both"))
    }

    /// Unwraps and returns the right value, panicking if not available.
    pub fn unwrap_right(self) -> R {
        self.right_or_else(|| panic!("Can only unwrap right of Either::Right or Either::Both"))
    }

    /// Unwraps and returns both values, panicking if not available.
    pub fn unwrap_both(self) -> Both<L, R> {
        self.both_or_else(|| panic!("Can only unwrap both of Either::Both"))
    }

    /// Filters to keep only the left value if present, otherwise returns Self::Neither.
    pub fn filter_left(self) -> Self {
        match self {
            Self::Neither => Self::Neither,
            Self::Either(Either::Left(l)) => Self::Either(Either::Left(l)),
            Self::Either(Either::Right(_)) => Self::Neither,
            Self::Both(Both { left: l, .. }) => Self::Either(Either::Left(l)),
        }
    }

    /// Filters to keep only the right value if present, otherwise returns Self::Neither.
    pub fn filter_right(self) -> Self {
        match self {
            Self::Neither => Self::Neither,
            Self::Either(Either::Left(_)) => Self::Neither,
            Self::Either(Either::Right(r)) => Self::Either(Either::Right(r)),
            Self::Both(Both { right: r, .. }) => Self::Either(Either::Right(r)),
        }
    }

    /// Adds or replaces the right value, keeping or constructing the instance accordingly.
    pub fn with_right(self, right: R) -> Self {
        match self {
            Self::Neither => Self::Either(Either::Right(right)),
            Self::Either(Either::Left(l)) => Self::Both(Both { left: l, right }),
            Self::Either(Either::Right(_)) => Self::Either(Either::Right(right)),
            Self::Both(Both { left: l, .. }) => Self::Both(Both { left: l, right }),
        }
    }

    /// Adds or replaces the left value, keeping or constructing the instance accordingly.
    pub fn with_left(self, left: L) -> Self {
        match self {
            Self::Neither => Self::Either(Either::Left(left)),
            Self::Either(Either::Left(_)) => Self::Either(Either::Left(left)),
            Self::Either(Either::Right(r)) => Self::Both(Both { left, right: r }),
            Self::Both(Both { right: r, .. }) => Self::Both(Both { left, right: r }),
        }
    }

    /// Swaps (! operator) the left and right components, creating a new `AnyOf` with reversed types.
    ///
    /// # Returns
    ///
    /// A new `AnyOf<R, L>` instance where the left and right components have been swapped.
    ///
    /// - If `self` is `Neither`, the result will also be `Neither`.
    /// - If `self` is an `Either::Left`, the result will contain the value as an `Either::Right`.
    /// - If `self` is an `Either::Right`, the result will contain the value as an `Either::Left`.
    /// - If `self` is a `Both`, the left and right values are swapped in the result.
    pub fn swap(self) -> AnyOf<R, L> {
        match self {
            Self::Neither => AnyOf::<R, L>::Neither,
            Self::Either(e) => AnyOf::<R, L>::Either(e.swap()),
            Self::Both(b) => AnyOf::<R, L>::Both(b.swap()),
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
    /// An `AnyOf` where the `Left` value has been transformed. The `Right` value,
    /// if present, remains unchanged. If self is `Neither`, this function returns `Neither`.
    pub fn map_left<FL, L2>(self, f: FL) -> AnyOf<L2, R>
    where
        FL: FnOnce(L) -> L2,
    {
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
    /// An `AnyOf` where the `Right` value has been transformed. The `Left` value,
    /// if present, remains unchanged. If self is `Neither`, this function returns `Neither`.
    pub fn map_right<FR, R2>(self, f: FR) -> AnyOf<L, R2>
    where
        FR: FnOnce(R) -> R2,
    {
        self.map(|l| l, f)
    }

    /// Transforms the `Left` and `Right` value using separate functions, depending
    /// on the variant.
    ///
    /// ## Arguments
    ///
    /// * `fl` - A function to transform the left value.
    /// * `fr` - A function to transform the right value.
    ///
    /// ## Returns
    ///
    /// An `AnyOf` where the `Left` and `Right` value has been transformed
    /// by the corresponding function. If self is `Neither`, this function returns `Neither`.
    pub fn map<FL, FR, L2, R2>(self, fl: FL, fr: FR) -> AnyOf<L2, R2>
    where
        FL: FnOnce(L) -> L2,
        FR: FnOnce(R) -> R2,
    {
        match self {
            Self::Neither => AnyOf::<L2, R2>::Neither,
            Self::Either(e) => AnyOf::<L2, R2>::Either(e.map(fl, fr)),
            Self::Both(b) => AnyOf::<L2, R2>::Both(b.map(fl, fr)),
        }
    }

    /// Combines (`&` operator) two `Either` values into a single one.
    ///
    /// ## General rules
    ///
    /// * `Neither` is always substituted by the other operand,
    /// * `Both` :
    ///     * as left operand : substitutes the other operand,
    ///     * as right operand : completes the other operand,
    /// * `Left` or `Right` :
    ///     * `L & R` or `R & L` combines to an instance of `Both`,
    ///     * `L & l` or `r & R` selects the operand placed on the correct side of the operator :
    ///         * left**Left** + right**Left** = left**Left**
    ///         * left**Right** + right**Right** = right**Right**
    ///
    /// ## All cases
    ///
    /// * Neither cases :
    ///     * Neither & Neither = Neither
    ///     * Neither & **other** = other
    ///     * **self** & Neither = self
    /// * Trivial cases :
    ///     * **Left(x)** & Left(y) = Left(x)
    ///     * Right(x) & **Right(y)** = Right(y)
    ///     * **Both(x, y)** & other = Both(x, y)
    /// * Combined cases :
    ///     * Left(x) & Right(y) = Both(x, y)
    ///     * Right(x) & Left(y) = Both(y, x)
    ///     * Left(x) & Both(_, y) = Both(x, y)
    ///     * Right(x) & Both(y, _) = Both(y, x)
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

    /// Filters (`|` operator) the current `AnyOf` instance using another `AnyOf` instance.
    ///
    /// ## General rules
    ///
    /// The filtering behavior depends on the specific variants of `self` and `other`:
    ///
    /// * `other == Neither` filters nothing (self is returned),
    /// * `other == Both` filters all (`Neither` is returned),
    /// * `Left` and `Right` as `other` filters the corresponding side of `self`.
    ///
    /// ## All cases
    ///
    /// - **Neither cases**:
    ///     * `Neither | other = Neither`
    ///     * `Left(x) | Left(y) = Neither`
    ///     * `Right(x) | Right(y) = Neither`
    ///     * `other | Both(x, y) = Neither`
    /// - **Trivial case**:
    ///     * `other | Neither = other`
    /// - **Filtered cases**:
    ///     * `Left(x) | Right(y) = Left(x)`
    ///     * `Right(x) | Left(y) = Right(x)`
    ///     * `Both(x, y) | Right(y) = Left(x)`
    ///     * `Both(x, y) | Left(y) = Right(y)`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use any_of::{AnyOf, Both};
    /// use any_of::either::Either;
    ///
    /// let both = AnyOf::Both(Both { left: 5, right: 10 });
    /// let left_only = AnyOf::Either(Either::Left(5));
    /// let right_only = AnyOf::Either(Either::Right(10));
    /// let neither: AnyOf<i32, i32> = AnyOf::Neither;
    ///
    /// // Filtering Both with Right results in Left
    /// assert_eq!(both | right_only, left_only);
    ///
    /// // Filtering Both with Left results in Right
    /// assert_eq!(both | left_only, right_only);
    ///
    /// // Filtering with Neither doesn't affect the original value
    /// assert_eq!(both | neither, both);
    /// assert_eq!(left_only | neither, left_only);
    /// assert_eq!(right_only | neither, right_only);
    ///
    /// // Filtering with Both always results in Neither
    /// assert_eq!(both | both, neither);
    /// assert_eq!(left_only | both, neither);
    /// assert_eq!(right_only | both, neither);
    /// ```
    pub fn filter(self, other: Self) -> Self {
        match other {
            AnyOf::Neither => self,
            AnyOf::Either(Either::Left(_)) => match self {
                AnyOf::Either(Either::Left(_)) => AnyOf::Neither,
                AnyOf::Either(Either::Right(r)) => AnyOf::Either(Either::Right(r)),
                AnyOf::Both(Both { right: r, .. }) => AnyOf::Either(Either::Right(r)),
                _ => self,
            },
            AnyOf::Either(Either::Right(_)) => match self {
                AnyOf::Either(Either::Left(l)) => AnyOf::Either(Either::Left(l)),
                AnyOf::Either(Either::Right(_)) => AnyOf::Neither,
                AnyOf::Both(Both { left: l, .. }) => AnyOf::Either(Either::Left(l)),
                _ => self,
            },
            AnyOf::Both(_) => AnyOf::Neither,
        }
    }
}

impl<L, R> BitAnd for AnyOf<L, R> {
    type Output = Self;

    /// See : [Self::combine].
    fn bitand(self, rhs: Self) -> Self::Output {
        self.combine(rhs)
    }
}

impl<L, R> BitOr for AnyOf<L, R> {
    type Output = Self;

    /// See : [Self::filter].
    fn bitor(self, rhs: Self) -> Self::Output {
        self.filter(rhs)
    }
}

impl<L, R> Not for AnyOf<L, R> {
    type Output = AnyOf<R, L>;

    /// See : [Self::swap].
    fn not(self) -> Self::Output {
        self.swap()
    }
}

pub mod any_of_x;

#[cfg(test)]
mod tests;