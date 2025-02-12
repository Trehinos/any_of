//! This module defines the `BothOf` struct, a utility for pairing two values together.
//!
//! The `BothOf` struct is a generic struct that holds a pair of values, `left` and `right`, of potentially different types.
//! It provides helper methods for constructing, converting, and destructuring these pairs of data.
//!
//! # Struct Overview
//!
//! ## `BothOf` Struct
//!
//! The `BothOf` struct is generic over two types `L` and `R`, allowing users to pair any two types together.
//! It derives a variety of useful traits such as `Copy`, `Clone`, `Eq`, `PartialEq`, `Debug`, and `Hash`.
//!
//! ### Fields
//! - `left`: The left value of type `L`.
//! - `right`: The right value of type `R`.
//!
//! # Methods
//!
//! - `new(left, right) -> Self`: Creates a new `BothOf` instance with the given left and right values.
//! - `from_couple(couple) -> Self`: Constructs a `BothOf` instance from a `Couple`, which is a tuple `(L, R)`.
//! - `into_couple() -> Couple<L, R>`: Converts this struct into a `Couple`, returning it as a tuple `(L, R)`.
//! - `into_left() -> Either<L, R>`: Converts this struct into a `Left` variant of the `Either` enum, using the `left` value.
//! - `into_right() -> Either<L, R>`: Converts this struct into a `Right` variant of the `Either` enum, using the `right` value.
//!
//! # Usage Examples
//!
//! ```rust
//! use any_of::{BothOf, Couple, Either, Left, Right};
//!
//! let both = BothOf::new(10, "right");
//! assert_eq!(both.left, 10);
//! assert_eq!(both.right, "right");
//!
//! let couple: Couple<i32, &str> = both.into_couple();
//! assert_eq!(couple, (10, "right"));
//!
//! let left = both.into_left();
//! match left {
//!     Left(value) => assert_eq!(value, 10),
//!     _ => panic!("Expected Left"),
//! }
//!
//! let right = both.into_right();
//! match right {
//!     Right(value) => assert_eq!(value, "right"),
//!     _ => panic!("Expected Right"),
//! }
//! ```

use crate::concepts::Swap;
use crate::either::EitherOf;
use crate::{Couple, LeftOrRight, Map, Unwrap};
use core::ops::{Not, Shr};

/// `BothOf` is a generic struct that allows pairing two values of potentially different types.
///
/// The `BothOf` struct is a utility for combining two values together,
/// making it easier to manipulate pairs of values with helper methods for construction,
/// transformation, and conversion.
///
/// # Examples
/// ```rust
/// use any_of::BothOf;
///
/// let both = BothOf::new(1, "example");
/// assert_eq!(both.left, 1);
/// assert_eq!(both.right, "example");
/// ```
///
/// For more examples, see the documentation of the individual methods below.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct BothOf<L, R = L> {
    pub left: L,
    pub right: R,
}

impl<L, R> BothOf<L, R> {
    /// Creates a new instance of the `BothOf` struct.
    ///
    /// # Arguments
    /// - `left` - The left-hand value of type `L`.
    /// - `right` - The right-hand value of type `R`.
    ///
    /// # Returns
    /// A new `BothOf` instance containing `left` and `right`.
    ///
    /// # Examples
    /// ```rust
    /// use any_of::BothOf;
    ///
    /// let both = BothOf::new(10, "right");
    /// assert_eq!(both.left, 10);
    /// assert_eq!(both.right, "right");
    /// ```
    pub fn new(left: L, right: R) -> Self {
        Self { left, right }
    }

    /// Constructs a `BothOf` instance from a `Couple`, which is a tuple `(L, R)`.
    ///
    /// # Arguments
    /// - `couple` - A tuple containing the left and right values.
    ///
    /// # Returns
    /// A new `BothOf` instance containing the values of the tuple.
    ///
    /// # Examples
    /// ```rust
    /// use any_of::{BothOf, Couple};
    ///
    /// let couple: Couple<i32, &str> = (42, "answer");
    /// let both = BothOf::from_couple(couple);
    /// assert_eq!(both.left, 42);
    /// assert_eq!(both.right, "answer");
    /// ```
    pub fn from_couple(couple: Couple<L, R>) -> Self {
        Self {
            left: couple.0,
            right: couple.1,
        }
    }

    /// Converts this `BothOf` instance into a `Couple` (a tuple `(L, R)`).
    ///
    /// # Returns
    /// A tuple containing the left and right values of the `BothOf` instance.
    ///
    /// # Examples
    /// ```rust
    /// use any_of::{BothOf, Couple};
    ///
    /// let both = BothOf::new(50, "hello");
    /// let couple: Couple<i32, &str> = both.into_couple();
    /// assert_eq!(couple, (50, "hello"));
    /// ```
    pub fn into_couple(self) -> Couple<L, R> {
        (self.left, self.right)
    }

    /// Converts this `BothOf` instance into a `Left` variant of the `Either` enum,
    /// using the `left` value of this struct.
    ///
    /// # Returns
    /// A `Left` variant containing the left value.
    ///
    /// # Examples
    /// ```rust
    /// use any_of::{BothOf, Either, Left};
    ///
    /// let both = BothOf::new(100, "unused");
    /// let left = both.into_left();
    /// match left {
    ///     Left(value) => assert_eq!(value, 100),
    ///     _ => panic!("Expected Left"),
    /// }
    /// ```
    pub fn into_left(self) -> EitherOf<L, R> {
        EitherOf::<L, R>::Left(self.left)
    }

    /// Converts this `BothOf` instance into a `Right` variant of the `Either` enum,
    /// using the `right` value of this struct.
    ///
    /// # Returns
    /// A `Right` variant containing the right value.
    ///
    /// # Examples
    /// ```rust
    /// use any_of::{BothOf, Either, Right};
    ///
    /// let both = BothOf::new("unused", 2023);
    /// let right = both.into_right();
    /// match right {
    ///     Right(value) => assert_eq!(value, 2023),
    ///     _ => panic!("Expected Right"),
    /// }
    /// ```
    pub fn into_right(self) -> EitherOf<L, R> {
        EitherOf::<L, R>::Right(self.right)
    }
}

impl<L, R> LeftOrRight<L, R> for BothOf<L, R> {
    /// Returns a reference to the left value.
    ///
    /// ## Returns
    ///
    /// Always Some(L).
    fn left(&self) -> Option<&L> {
        Some(&self.left)
    }

    /// Returns a reference to the right value.
    ///
    /// ## Returns
    ///
    /// Always Some(R).
    fn right(&self) -> Option<&R> {
        Some(&self.right)
    }
}

impl<L, R> Not for BothOf<L, R> {
    type Output = BothOf<R, L>;

    /// Swaps the `left` and `right` values of this `BothOf` instance.
    ///
    /// # Returns
    /// A new `BothOf` instance where the `left` and `right` values are swapped.
    ///
    /// # Examples
    /// ```rust
    /// use any_of::BothOf;
    /// use any_of::Swap;
    ///
    /// let both = BothOf::new(42, "example");
    /// let swapped = !both;
    /// let swapped = both.swap();
    ///
    /// assert_eq!(swapped.left, "example");
    /// assert_eq!(swapped.right, 42);
    /// ```
    fn not(self) -> Self::Output {
        BothOf {
            left: self.right,
            right: self.left,
        }
    }
}
impl<L, R> Swap<L, R> for BothOf<L, R> {
    type Output = <Self as Not>::Output;
}

impl<L, R> Map<L, R> for BothOf<L, R> {
    type Output<L2, R2> = BothOf<L2, R2>;

    /// Applies the provided transformation functions to the `left` and `right` values of this `BothOf` instance.
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
    /// # Returns
    /// A new `BothOf` instance with transformed `left` and `right` values.
    ///
    /// # Examples
    /// ```rust
    /// use any_of::{BothOf, Map};
    ///
    /// let both = BothOf::new(2, "example");
    /// let transformed = both.map(|left| left * 2, |right| format!("{}!", right));
    ///
    /// assert_eq!(transformed.left, 4);
    /// assert_eq!(transformed.right, "example!");
    /// ```
    fn map<FL, FR, L2, R2>(self, fl: FL, fr: FR) -> Self::Output<L2, R2>
    where
        FL: FnOnce(L) -> L2,
        FR: FnOnce(R) -> R2,
    {
        self >> (fl, fr).into()
    }
}

impl<L, R> Unwrap<L, R> for BothOf<L, R> {
    fn left_or_else(self, _: impl FnOnce() -> L) -> L {
        self.left
    }

    fn right_or_else(self, _: impl FnOnce() -> R) -> R {
        self.right
    }
}

impl<L, R, FL, FR, L2, R2> Shr<BothOf<FL, FR>> for BothOf<L, R>
where
    FL: FnOnce(L) -> L2,
    FR: FnOnce(R) -> R2,
{
    type Output = BothOf<L2, R2>;

    fn shr(self, rhs: BothOf<FL, FR>) -> Self::Output {
        BothOf {
            left: (rhs.left)(self.left),
            right: (rhs.right)(self.right),
        }
    }
}
