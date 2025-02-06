//! This module defines utility types and traits for working with dual-variant data structures,
//! such as "product types" (tuples) and "sum types" (LeftOrRight) that can hold one of two possible variants.
//!
//! It includes definitions for `Couple`, `Pair`, and the `LeftOrRight` trait,
//! providing type-safe methods for ergonomic and efficient access to such data.

/// The `(T, U)` tuple.
pub type Couple<T, U> = (T, U);

/// A shortcut for `Couple<T, T>`.
pub type Pair<T> = Couple<T, T>;

/// The `LeftOrRight` trait provides utility methods for working with types that 
/// can represent one of two possible variants: a "left" variant (`L`) or a 
/// "right" variant (`R`) (or possibly both).
///
/// ## Provided Methods
///
/// - **`is_left`**: Checks if the value is the `L` (left) variant.
/// - **`is_right`**: Checks if the value is the `R` (right) variant.
/// - **`any`**: Returns both left and right values wrapped in `Option`s as a tuple.
/// - **`left`**: Returns a reference to the left variant if present.
/// - **`right`**: Returns a reference to the right variant if present.
///
/// This trait is useful for types implementing a "sum type"-like behavior, where
/// values may contain one of two possible forms, and you need utilities for
/// type-safe and ergonomic access to their internals.
pub trait LeftOrRight<L, R> {
    /// Checks if the `LeftOrRight<L, R>` value is the `L` variant.
    ///
    /// ## Returns
    ///
    /// `true` if the value is `L`, otherwise `false`.
    fn is_left(&self) -> bool;

    /// Checks if the `LeftOrRight<L, R>` value is the `R` variant.
    ///
    /// ## Returns
    ///
    /// `true` if the value is `R`, otherwise `false`.
    fn is_right(&self) -> bool;

    /// Returns references to the `L` or `R` values as a tuple of `Option`.
    ///
    /// ## Returns
    ///
    /// A tuple containing an `Option` reference to the left value and an `Option`
    /// reference to the right value.
    fn any(&self) -> Couple<Option<&L>, Option<&R>>;

    /// Returns a reference to the left value if it exists.
    ///
    /// ## Returns
    ///
    /// An `Option` containing a reference to the left value if the variant is `L`,
    /// otherwise `None`.
    fn left(&self) -> Option<&L>;

    /// Returns a reference to the right value if it exists.
    ///
    /// ## Returns
    ///
    /// An `Option` containing a reference to the right value if the variant is `R`,
    /// otherwise `None`.
    fn right(&self) -> Option<&R>;
    
}