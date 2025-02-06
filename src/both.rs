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
//! use any_of::Both;
//! use any_of::Couple;
//! use any_of::either::Either;
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

use crate::Couple;
use crate::either::Either;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct Both<L, R> {
    pub left: L,
    pub right: R,
}

impl<L, R> Both<L, R> {
    pub fn new(left: L, right: R) -> Self {
        Self { left, right }
    }
    pub fn from_couple(couple: Couple<L, R>) -> Self {
        Self {
            left: couple.0,
            right: couple.1,
        }
    }
    pub fn into_couple(self) -> Couple<L, R> {
        (self.left, self.right)
    }
    pub fn into_left(self) -> Either<L, R> {
        Either::<L, R>::Left(self.left)
    }
    pub fn into_right(self) -> Either<L, R> {
        Either::<L, R>::Right(self.right)
    }
}