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
//! use any_of::{AnyOf, Both , BothOf, Either , Left, LeftOrRight, Map, Neither};
//!
//! let neither: AnyOf<i32, &str> = AnyOf::new(None, None);
//! let neither: AnyOf<i32, &str> = Neither;
//!
//! let left: AnyOf<i32, &str> = AnyOf::new(Some(42), None);
//! let left: AnyOf<i32, &str> = Either(Left(42));
//!
//! let both: AnyOf<i32, &str> = AnyOf::new(Some(42), Some("Hello"));
//! let both: AnyOf<i32, &str> = Both(BothOf { left: 42, right: "Hello" });
//!
//! assert!(neither.is_neither());
//! assert!(left.is_left());
//! assert!(both.is_both());
//!
//! assert!(neither.map_right(|r| r).is_neither());
//! assert!(left.map_right(|r| r).is_left());
//! assert!(left.map_left(|l| l).is_left());
//! assert!(both.map_left(|l| l).is_both());
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
//! - [AnyOf::filter]
//! - [AnyOf::map]
//! - [AnyOf::swap]
//! - [AnyOf::unwrap_left] and [AnyOf::left]
//! - [AnyOf::unwrap_right] and [AnyOf::right]
//! - [AnyOf::unwrap_both] and [AnyOf::both_or_none]
//!
//! ## Exported elements :
//! - Enum cases : [Left], [Right], [Both], [Either], [Neither],
//! - Traits : [LeftOrRight], [Unwrap], [Map], [Swap],
//! - Types : [Couple], [Pair], [EitherOf], [BothOf], [AnyOf], [AnyOf4], [AnyOf8], [AnyOf16]
//!
#![no_std]

pub mod concepts;

pub mod either;

pub mod both;

use core::ops::{Add, Not, Shr, Sub};

pub use crate::{
    any_of_x::{AnyOf16, AnyOf4, AnyOf8},
    both::BothOf,
    concepts::{Any, Couple, LeftOrRight, Map, Pair, Swap, Unwrap},
    either::EitherOf,
    either::EitherOf::{Left, Right},
    AnyOf::{Both, Either, Neither},
};

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
/// use any_of::{AnyOf, EitherOf, BothOf, LeftOrRight, Map, Right, Left, Neither, Either, Both};
///
/// let neither: AnyOf<i32, &str> = Neither;
/// let neither: AnyOf<i32, &str> = AnyOf::new(None, None);
/// let left: AnyOf<i32, &str> = Either(Left(42));
/// let left: AnyOf<i32, &str> = AnyOf::new(Some(42), None);
/// let both: AnyOf<i32, &str> = Both(BothOf { left: 42, right: "Hello" });
/// let both: AnyOf<i32, &str> = AnyOf::new(Some(42), Some("Hello"));
///
/// assert!(neither.is_neither());
/// assert!(left.is_left());
/// assert!(both.is_both());
///
/// assert!(neither.map_right(|r| r).is_neither());
/// assert!(left.map_right(|r| r).is_left());
/// assert!(left.map_left(|l| l).is_left());
/// assert!(both.map_left(|l| l).is_both());
///
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum AnyOf<L, R = L> {
    Neither,
    Either(EitherOf<L, R>),
    Both(BothOf<L, R>),
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
    /// - Returns `Neither` if both `left` and `right` are `None`.
    /// - Returns `Either(Left)` if `left` has a value and `right` is `None`.
    /// - Returns `Either(Right)` if `right` has a value and `left` is `None`.
    /// - Returns `Both` if both `left` and `right` have values.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use any_of::{AnyOf, Either, Both, LeftOrRight};
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
            (None, None) => Neither,
            (Some(l), None) => Either(Left(l)),
            (None, Some(r)) => Either(Right(r)),
            (Some(l), Some(r)) => Both(BothOf { left: l, right: r }),
        }
    }

    /// Creates an `Neither` variant.
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

    /// Creates an `Either` variant containing a `Left` value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use any_of::{AnyOf, LeftOrRight};
    ///
    /// let left: AnyOf<i32, &str> = AnyOf::new_left(42);
    /// assert!(left.is_left());
    /// ```
    pub fn new_left(left: L) -> Self {
        Self::new(Some(left), None)
    }

    /// Creates an `Either` variant containing a `Right` value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use any_of::{AnyOf, LeftOrRight};
    ///
    /// let right: AnyOf<i32, &str> = AnyOf::new_right("Hello");
    /// assert!(right.is_right());
    /// ```
    pub fn new_right(right: R) -> Self {
        Self::new(None, Some(right))
    }

    /// Creates an `Both` variant containing both a `Left` and `Right` value.
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

    /// Creates an `Both` variant from a `Both` struct.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use any_of::{AnyOf, BothOf};
    ///
    /// let both_struct = BothOf { left: 42, right: "Hello" };
    /// let both = AnyOf::from_both(both_struct);
    /// assert!(both.is_both());
    /// ```
    pub fn from_both(both: BothOf<L, R>) -> Self {
        Self::new(Some(both.left), Some(both.right))
    }

    /// Creates a new `AnyOf` instance based on the presence of `left` and `right` values.
    ///
    /// See [Self::new].
    pub fn from_any(any: Any<L, R>) -> Self {
        Self::new(any.0, any.1)
    }

    /// Creates an `AnyOf` variant from an `Either` struct.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use any_of::{AnyOf, EitherOf, LeftOrRight};
    /// use any_of::either::EitherOf::{Right, Left};
    ///
    /// let either: EitherOf<i32, ()> = Left(42);
    /// let any_of: AnyOf<i32, ()> = AnyOf::from_either(either);
    /// assert!(any_of.is_left());
    /// ```
    pub fn from_either(either: EitherOf<L, R>) -> Self {
        Either(either)
    }

    /// Converts the `AnyOf` variant to a `Both` struct.
    ///
    /// # Panics
    ///
    /// This function will panic if `self` is not an `Both` variant.
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
    pub fn into_both(self) -> BothOf<L, R> {
        match self {
            Both(b) => b,
            _ => panic!("Can only convert Either::Both to Both"),
        }
    }

    /// Converts the `AnyOf` variant to an `Either` struct.
    ///
    /// # Panics
    ///
    /// This function will panic if `self` is not an `Either` variant.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use any_of::{AnyOf, EitherOf, Left};
    ///
    /// let either: EitherOf<i32, ()> = AnyOf::new_left(42).into_either();
    /// assert_eq!(either, Left(42));
    /// ```
    pub fn into_either(self) -> EitherOf<L, R> {
        if let Either(e) = self {
            e
        } else {
            panic!("Can only convert Either::Either to Either");
        }
    }

    /// Converts the current `AnyOf` variant into a pair of options containing `Either` variants.
    ///
    /// # Returns
    ///
    /// - `(Some(Left), None)` for `Either(Left)`.
    /// - `(None, Some(Right))` for `Either(Right)`.
    /// - `(Some(Left), Some(Right))` for `Both`.
    /// - `(None, None)` for `Neither`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use any_of::{AnyOf, Either, Right, Left};
    ///
    /// let any_of = AnyOf::new_both(42, "Hello");
    /// let (left, right) = any_of.to_either_pair();
    /// assert_eq!(left, Some(Left(42)));
    /// assert_eq!(right, Some(Right("Hello")));
    /// ```
    pub fn to_either_pair(&self) -> Pair<Option<EitherOf<L, R>>>
    where
        L: Clone,
        R: Clone,
    {
        let both = self.any();
        let left = both.0.map(|l| Left(l.clone()));
        let right = both.1.map(|r| Right(r.clone()));
        (left, right)
    }

    /// True if [Left] or [Both].
    pub fn has_left(&self) -> bool {
        matches!(self, Either(Left(_)) | Both(_))
    }

    /// True if [Right] or [Both].0
    pub fn has_right(&self) -> bool {
        matches!(self, Either(Right(_)) | Both(_))
    }

    /// True if not [Neither].
    pub fn is_any(&self) -> bool {
        matches!(self, Either(Left(_)) | Either(Right(_)) | Both(_))
    }

    /// True if [Either].
    pub fn is_either(&self) -> bool {
        matches!(self, Either(_))
    }

    /// True if [Both]
    pub fn is_both(&self) -> bool {
        matches!(self, Both(_))
    }

    /// True if [Neither]
    pub fn is_neither(&self) -> bool {
        matches!(self, Neither)
    }

    /// True if not [Either]
    pub fn is_neither_or_both(&self) -> bool {
        matches!(self, Neither | Both(_))
    }

    /// Returns `Some((&L, &R))` if `self.is_both()` is true, or `None`.
    pub fn both_or_none(&self) -> Option<Couple<&L, &R>> {
        Some((self.left()?, self.right()?))
    }

    /// Returns both values if present, or computes them with the provided function.
    pub fn both_or_else(self, f: impl FnOnce() -> BothOf<L, R>) -> BothOf<L, R> {
        match self {
            Neither => f(),
            Either(Left(l)) => BothOf::new(l, f().right),
            Either(Right(r)) => BothOf::new(f().left, r),
            Both(b) => b,
        }
    }

    /// Returns both values if present, or the provided default values.
    pub fn both_or(self, other: BothOf<L, R>) -> BothOf<L, R> {
        self.both_or_else(|| other)
    }

    /// Unwraps and returns both values, panicking if not available.
    pub fn unwrap_both(self) -> BothOf<L, R> {
        self.both_or_else(|| panic!("Can only unwrap both of Either::Both"))
    }

    /// Filters to keep only the left value if present, otherwise returns Neither.
    pub fn filter_left(self) -> Self {
        match self {
            Neither => Neither,
            Either(Left(l)) => Either(Left(l)),
            Either(Right(_)) => Neither,
            Both(BothOf { left: l, .. }) => Either(Left(l)),
        }
    }

    /// Filters to keep only the right value if present, otherwise returns Neither.
    pub fn filter_right(self) -> Self {
        match self {
            Neither => Neither,
            Either(Left(_)) => Neither,
            Either(Right(r)) => Either(Right(r)),
            Both(BothOf { right: r, .. }) => Either(Right(r)),
        }
    }

    /// Adds or replaces the right value, keeping or constructing the instance accordingly.
    pub fn with_right(self, right: R) -> Self {
        match self {
            Neither => Either(Right(right)),
            Either(Left(l)) => Both(BothOf { left: l, right }),
            Either(Right(_)) => Either(Right(right)),
            Both(BothOf { left: l, .. }) => Both(BothOf { left: l, right }),
        }
    }

    /// Adds or replaces the left value, keeping or constructing the instance accordingly.
    pub fn with_left(self, left: L) -> Self {
        match self {
            Neither => Either(Left(left)),
            Either(Left(_)) => Either(Left(left)),
            Either(Right(r)) => Both(BothOf { left, right: r }),
            Both(BothOf { right: r, .. }) => Both(BothOf { left, right: r }),
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
    ///     * `L & R` or `R & L` combines to an instance of `Both`,
    ///     * `L & l` or `r & R` selects the operand placed on the correct side of the operator :
    ///         * left**Left** + right**Left** = left**Left**
    ///         * left**Right** + right**Right** = right**Right**
    ///
    /// ## All cases
    ///
    /// * Neither cases :
    ///     * Neither + Neither = Neither
    ///     * Neither + **other** = other
    ///     * **self** + Neither = self
    /// * Trivial cases :
    ///     * **Left(x)** + Left(y) = Left(x)
    ///     * Right(x) + **Right(y)** = Right(y)
    ///     * **Both(x, y)** + other = Both(x, y)
    /// * Combined cases :
    ///     * Left(x) + Right(y) = Both(x, y)
    ///     * Right(x) + Left(y) = Both(y, x)
    ///     * Left(x) + Both(_, y) = Both(x, y)
    ///     * Right(x) + Both(y, _) = Both(y, x)
    pub fn combine(self, other: Self) -> Self {
        match self {
            Neither => other,
            Either(Left(l)) => match other {
                Neither => Either(Left(l)),
                Either(Left(_)) => Either(Left(l)),
                Either(Right(r)) => Both(BothOf { left: l, right: r }),
                Both(BothOf { right: r, .. }) => Both(BothOf { left: l, right: r }),
            },
            Either(Right(r)) => match other {
                Neither => Either(Right(r)),
                Either(Left(l)) => Both(BothOf { left: l, right: r }),
                Either(Right(r2)) => Either(Right(r2)),
                Both(BothOf { left: l, .. }) => Both(BothOf { left: l, right: r }),
            },
            Both(b) => Both(b),
        }
    }

    /// Filters (`-` operator) the current `AnyOf` instance using another `AnyOf` instance.
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
    ///     * `Neither - other = Neither`
    ///     * `Left(x) - Left(y) = Neither`
    ///     * `Right(x) - Right(y) = Neither`
    ///     * `other - Both(x, y) = Neither`
    /// - **Trivial case**:
    ///     * `other - Neither = other`
    /// - **Filtered cases**:
    ///     * `Left(x) - Right(y) = Left(x)`
    ///     * `Right(x) - Left(y) = Right(x)`
    ///     * `Both(x, y) - Right(z) = Left(x)`
    ///     * `Both(x, y) - Left(z) = Right(y)`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use any_of::{AnyOf, Both ,BothOf, Either, Left, Neither, Right};
    ///
    /// let both = Both(BothOf { left: 5, right: 10 });
    /// let left_only = Either(Left(5));
    /// let right_only = Either(Right(10));
    /// let neither: AnyOf<i32, i32> = Neither;
    ///
    /// // Filtering Both with Right results in Left
    /// assert_eq!(both - right_only, left_only);
    ///
    /// // Filtering Both with Left results in Right
    /// assert_eq!(both - left_only, right_only);
    ///
    /// // Filtering with Neither doesn't affect the original value
    /// assert_eq!(both - neither, both);
    /// assert_eq!(left_only - neither, left_only);
    /// assert_eq!(right_only - neither, right_only);
    ///
    /// // Filtering with Both always results in Neither
    /// assert_eq!(both - both, neither);
    /// assert_eq!(left_only - both, neither);
    /// assert_eq!(right_only - both, neither);
    /// ```
    pub fn filter(self, other: Self) -> Self {
        match other {
            Neither => self,
            Either(Left(_)) => match self {
                Either(Left(_)) => Neither,
                Either(Right(r)) => Either(Right(r)),
                Both(BothOf { right: r, .. }) => Either(Right(r)),
                _ => self,
            },
            Either(Right(_)) => match self {
                Either(Left(l)) => Either(Left(l)),
                Either(Right(_)) => Neither,
                Both(BothOf { left: l, .. }) => Either(Left(l)),
                _ => self,
            },
            Both(_) => Neither,
        }
    }
}

impl<L, R> Add for AnyOf<L, R> {
    type Output = Self;

    /// See : [Self::combine].
    fn add(self, rhs: Self) -> Self::Output {
        self.combine(rhs)
    }
}

impl<L, R> Sub for AnyOf<L, R> {
    type Output = Self;

    /// See : [Self::filter].
    fn sub(self, rhs: Self) -> Self::Output {
        self.filter(rhs)
    }
}

impl<L, R> Not for AnyOf<L, R> {
    type Output = AnyOf<R, L>;

    /// Swaps (`!` operator) the left and right components, creating a new `AnyOf` with reversed types.
    ///
    /// # Returns
    ///
    /// A new `AnyOf<R, L>` instance where the left and right components have been swapped.
    ///
    /// - If `self` is `Neither`, the result will also be `Neither`.
    /// - If `self` is an `Left`, the result will contain the value as an `Right`.
    /// - If `self` is an `Right`, the result will contain the value as an `Left`.
    /// - If `self` is a `Both`, the left and right values are swapped in the result.
    fn not(self) -> Self::Output {
        match self {
            Neither => AnyOf::<R, L>::Neither,
            Either(e) => AnyOf::<R, L>::Either(e.swap()),
            Both(b) => AnyOf::<R, L>::Both(b.swap()),
        }
    }
}
impl<L, R> Swap<L, R> for AnyOf<L, R> {
    type Output = <Self as Not>::Output;
}

impl<L, R> LeftOrRight<L, R> for AnyOf<L, R> {
    /// Returns `Some(&L)` if `self.has_left()` is true, or `None`.
    fn left(&self) -> Option<&L> {
        match self {
            Neither => None,
            Either(e) => e.left(),
            Both(b) => b.left(),
        }
    }

    /// Returns `Some(&R)` if `self.has_right()` is true, or `None`.
    fn right(&self) -> Option<&R> {
        match self {
            Neither => None,
            Either(e) => e.right(),
            Both(b) => b.right(),
        }
    }
}

impl<L, R> Map<L, R> for AnyOf<L, R> {
    type Output<L2, R2> = AnyOf<L2, R2>;

    fn map<FL, FR, L2, R2>(self, fl: FL, fr: FR) -> Self::Output<L2, R2>
    where
        FL: FnOnce(L) -> L2,
        FR: FnOnce(R) -> R2
    {
        self >> (fl, fr).into()
    }
}

impl<L, R> Unwrap<L, R> for AnyOf<L, R> {
    /// Returns the left value if present, or computes it with the provided function.
    fn left_or_else(self, f: impl FnOnce() -> L) -> L {
        match self {
            Neither => f(),
            Either(Left(l)) => l,
            Either(Right(_)) => f(),
            Both(BothOf { left: l, .. }) => l,
        }
    }

    /// Returns the right value if present, or computes it with the provided function.
    fn right_or_else(self, f: impl FnOnce() -> R) -> R {
        match self {
            Neither => f(),
            Either(Left(_)) => f(),
            Either(Right(r)) => r,
            Both(BothOf { right: r, .. }) => r,
        }
    }
}

impl<L, R, FL, FR, L2, R2> Shr<BothOf<FL, FR>> for AnyOf<L, R>
where
    FL: FnOnce(L) -> L2,
    FR: FnOnce(R) -> R2,
{
    type Output = AnyOf<L2, R2>;

    /// Maps two functions with the parts of this `AnyOf`
    ///
    /// # Examples
    ///
    /// ```
    /// use any_of::{AnyOf, Left, Unwrap};
    ///
    /// let left_fn = |x: i32| x * 2;
    /// let right_fn  = |y: i32| y + 3;
    /// let both_fn = (left_fn, right_fn).into();
    ///
    /// let both:AnyOf<i32, i32> = (5, 10).into();
    /// let result = both >> both_fn;
    /// assert_eq!(result, (10, 13).into());
    ///
    /// let left = AnyOf::Either(Left(25));
    /// let left_result = left >> both_fn;
    /// assert_eq!(left_result.unwrap_left(), 50);
    /// ```
    fn shr(self, rhs: BothOf<FL, FR>) -> Self::Output {
        match self {
            Neither => AnyOf::<L2, R2>::Neither,
            Either(e) => AnyOf::<L2, R2>::Either(e >> rhs),
            Both(b) => AnyOf::<L2, R2>::Both(b >> rhs),
        }
    }
}

pub mod conversions;

pub mod any_of_x;

#[cfg(test)]
mod tests;
