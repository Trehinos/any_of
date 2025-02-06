//! This module defines the `Both` struct, a utility for pairing two values together.
//!
//! The `Both` struct is a generic struct that holds a pair of values, `left` and `right`, of potentially different types.
//! It provides helper methods for constructing, converting, and destructuring these pairs of data.
//!
//! # Struct Overview
//!
//! ## `Both` Struct
//!
//! The `Both` struct is generic over two types `L` and `R`, allowing users to pair any two types together.
//! It derives a variety of useful traits such as `Copy`, `Clone`, `Eq`, `PartialEq`, `Debug`, and `Hash`.
//!
//! ### Fields
//! - `left`: The left value of type `L`.
//! - `right`: The right value of type `R`.
//!
//! # Methods
//!
//! - `new(left, right) -> Self`: Creates a new `Both` instance with the given left and right values.
//! - `from_couple(couple) -> Self`: Constructs a `Both` instance from a `Couple`, which is a tuple `(L, R)`.
//! - `into_couple() -> Couple<L, R>`: Converts this struct into a `Couple`, returning it as a tuple `(L, R)`.
//! - `into_left() -> Either<L, R>`: Converts this struct into a `Left` variant of the `Either` enum, using the `left` value.
//! - `into_right() -> Either<L, R>`: Converts this struct into a `Right` variant of the `Either` enum, using the `right` value.
//!
//! # Usage Examples
//!
//! ```rust
//! use any_of::{Both, Couple, Either};
//!
//! let both = Both::new(10, "right");
//! assert_eq!(both.left, 10);
//! assert_eq!(both.right, "right");
//!
//! let couple: Couple<i32, &str> = both.into_couple();
//! assert_eq!(couple, (10, "right"));
//!
//! let left = both.into_left();
//! match left {
//!     Either::Left(value) => assert_eq!(value, 10),
//!     _ => panic!("Expected Left"),
//! }
//!
//! let right = both.into_right();
//! match right {
//!     Either::Right(value) => assert_eq!(value, "right"),
//!     _ => panic!("Expected Right"),
//! }
//! ```

use core::ops::Not;
use crate::either::Either;
use crate::Couple;

/// `Both` is a generic struct that allows pairing two values of potentially different types.
///
/// The `Both` struct is a utility for combining two values together,
/// making it easier to manipulate pairs of values with helper methods for construction,
/// transformation, and conversion.
///
/// # Examples
/// ```rust
/// use any_of::Both;
///
/// let both = Both::new(1, "example");
/// assert_eq!(both.left, 1);
/// assert_eq!(both.right, "example");
/// ```
///
/// For more examples, see the documentation of the individual methods below.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct Both<L, R> {
    pub left: L,
    pub right: R,
}

impl<L, R> Both<L, R> {
    /// Creates a new instance of the `Both` struct.
    ///
    /// # Arguments
    /// - `left` - The left-hand value of type `L`.
    /// - `right` - The right-hand value of type `R`.
    ///
    /// # Returns
    /// A new `Both` instance containing `left` and `right`.
    ///
    /// # Examples
    /// ```rust
    /// use any_of::Both;
    ///
    /// let both = Both::new(10, "right");
    /// assert_eq!(both.left, 10);
    /// assert_eq!(both.right, "right");
    /// ```
    pub fn new(left: L, right: R) -> Self {
        Self { left, right }
    }

    /// Constructs a `Both` instance from a `Couple`, which is a tuple `(L, R)`.
    ///
    /// # Arguments
    /// - `couple` - A tuple containing the left and right values.
    ///
    /// # Returns
    /// A new `Both` instance containing the values of the tuple.
    ///
    /// # Examples
    /// ```rust
    /// use any_of::{Both, Couple};
    ///
    /// let couple: Couple<i32, &str> = (42, "answer");
    /// let both = Both::from_couple(couple);
    /// assert_eq!(both.left, 42);
    /// assert_eq!(both.right, "answer");
    /// ```
    pub fn from_couple(couple: Couple<L, R>) -> Self {
        Self {
            left: couple.0,
            right: couple.1,
        }
    }

    /// Converts this `Both` instance into a `Couple` (a tuple `(L, R)`).
    ///
    /// # Returns
    /// A tuple containing the left and right values of the `Both` instance.
    ///
    /// # Examples
    /// ```rust
    /// use any_of::{Both, Couple};
    ///
    /// let both = Both::new(50, "hello");
    /// let couple: Couple<i32, &str> = both.into_couple();
    /// assert_eq!(couple, (50, "hello"));
    /// ```
    pub fn into_couple(self) -> Couple<L, R> {
        (self.left, self.right)
    }

    /// Converts this `Both` instance into a `Left` variant of the `Either` enum,
    /// using the `left` value of this struct.
    ///
    /// # Returns
    /// An `Either::Left` variant containing the left value.
    ///
    /// # Examples
    /// ```rust
    /// use any_of::{Both, Either};
    ///
    /// let both = Both::new(100, "unused");
    /// let left = both.into_left();
    /// match left {
    ///     Either::Left(value) => assert_eq!(value, 100),
    ///     _ => panic!("Expected Left"),
    /// }
    /// ```
    pub fn into_left(self) -> Either<L, R> {
        Either::<L, R>::Left(self.left)
    }

    /// Converts this `Both` instance into a `Right` variant of the `Either` enum,
    /// using the `right` value of this struct.
    ///
    /// # Returns
    /// An `Either::Right` variant containing the right value.
    ///
    /// # Examples
    /// ```rust
    /// use any_of::{Both, Either};
    ///
    /// let both = Both::new("unused", 2023);
    /// let right = both.into_right();
    /// match right {
    ///     Either::Right(value) => assert_eq!(value, 2023),
    ///     _ => panic!("Expected Right"),
    /// }
    /// ```
    pub fn into_right(self) -> Either<L, R> {
        Either::<L, R>::Right(self.right)
    }
    
    /// Applies the provided transformation functions to the `left` and `right` values of this `Both` instance.
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
    /// A new `Both` instance with transformed `left` and `right` values.
    ///
    /// # Examples
    /// ```rust
    /// use any_of::Both;
    ///
    /// let both = Both::new(2, "example");
    /// let transformed = both.map(|left| left * 2, |right| format!("{}!", right));
    ///
    /// assert_eq!(transformed.left, 4);
    /// assert_eq!(transformed.right, "example!");
    /// ```
    pub fn map<L2, R2, FL, FR>(self, fl: FL, fr: FR) -> Both<L2, R2>
    where
        FL: FnOnce(L) -> L2,
        FR: FnOnce(R) -> R2,
    {
        Both {
            left: fl(self.left),
            right: fr(self.right),
        }
    }
    

    /// Swaps the `left` and `right` values of this `Both` instance.
    ///
    /// # Returns
    /// A new `Both` instance where the `left` and `right` values are swapped.
    ///
    /// # Examples
    /// ```rust
    /// use any_of::Both;
    ///
    /// let both = Both::new(42, "example");
    /// let swapped = both.swap();
    ///
    /// assert_eq!(swapped.left, "example");
    /// assert_eq!(swapped.right, 42);
    /// ```
    pub fn swap(self) -> Both<R, L> {
        Both {
            left: self.right,
            right: self.left,
        }
    }
}

impl<L, R> Not for Both<L, R> {
    type Output = Both<R, L>;

    /// See : [Self::swap].
    fn not(self) -> Self::Output {
        self.swap()
    }
}
