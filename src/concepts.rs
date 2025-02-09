//! This module defines utility types and traits for working with dual-variant data structures,
//! such as "product types" (tuples) and "sum types" (LeftOrRight) that can hold one of two possible variants.
//!
//! It includes definitions for `Couple`, `Pair`, and the `LeftOrRight` trait,
//! providing type-safe methods for ergonomic and efficient access to such data.

/// The `(T, U)` tuple.
pub type Couple<T, U> = (T, U);

/// A shortcut for `Couple<T, T>`.
pub type Pair<T> = Couple<T, T>;

/// A shortcut for `(Option<T>, Option<U>)`.
///
/// It is a valid representation of a [crate::AnyOf].
pub type Any<T, U> = Couple<Option<T>, Option<U>>;

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

    /// Checks if the `LeftOrRight<L, R>` value is the `L` variant.
    ///
    /// ## Returns
    ///
    /// `true` if the value is `L`, otherwise `false`.
    fn is_left(&self) -> bool {
        self.left().is_some()
    }

    /// Checks if the `LeftOrRight<L, R>` value is the `R` variant.
    ///
    /// ## Returns
    ///
    /// `true` if the value is `R`, otherwise `false`.
    fn is_right(&self) -> bool {
        self.right().is_some()
    }

    /// Returns references to the `L` or `R` values as a tuple of `Option`.
    ///
    /// ## Returns
    ///
    /// A tuple containing an `Option` reference to the left value and an `Option`
    /// reference to the right value.
    fn any(&self) -> Couple<Option<&L>, Option<&R>> {
        (self.left(), self.right())
    }
}

/// The `Swap` trait extends the `LeftOrRight` trait to include the ability to swap
/// the "left" and "right" variants of a type.
///
/// This can be particularly useful when working with dual-variant data structures,
/// where you need to reverse their roles in a type-safe manner.
///
/// ## Associated Type
/// - **`Output`**: The resulting type after swapping the `left` and `right` variants.
///
/// ## Provided Method
/// - **`swap`**: Produces a new instance with the `left` and `right` values swapped.
///
/// # Examples
/// ```rust
/// use any_of::BothOf;
/// use any_of::Swap;
///
/// let both = BothOf::new(42, "example");
/// let swapped = both.swap();
///
/// assert_eq!(swapped.left, "example");
/// assert_eq!(swapped.right, 42);
/// ```
pub trait Swap<L, R>: LeftOrRight<L, R> {
    type Output: LeftOrRight<R, L>;

    /// Swaps the `left` and `right` values of this `dyn LeftOrRight` instance.
    ///
    /// # Returns
    /// A new `dyn LeftOrRight` instance where the `left` and `right` values are swapped.
    ///
    /// # Examples
    /// ```rust
    /// use any_of::BothOf;
    /// use any_of::concepts::Swap;
    ///
    /// let both = BothOf::new(42, "example");
    /// let swapped = both.swap();
    ///
    /// assert_eq!(swapped.left, "example");
    /// assert_eq!(swapped.right, 42);
    /// ```
    fn swap(self) -> Self::Output;
}

/// The `Map` trait provides utilities for transforming the `left` or `right` variants
/// of a dual-variant type (`LeftOrRight`).
///
/// This allows for ergonomic and type-safe transformations of either side of the data structure.
///
/// ## Provided Methods
///
/// - **`map_left`**: Applies a transformation to the `left` value while leaving the
///   `right` value unchanged.
/// - **`map_right`**: Applies a transformation to the `right` value while leaving the
///   `left` value unchanged.
/// - **`map`**: Applies separate transformations to the `left` and `right` values
///   based on the variant.
pub trait Map<L, R>: LeftOrRight<L, R> {
    type Output<L2, R2>: LeftOrRight<L2, R2>;

    /// Transforms the `L` value using a provided function.
    ///
    /// ## Arguments
    ///
    /// * `f` - A function to transform the left value.
    ///
    /// ## Returns
    ///
    /// An `LeftOrRight` where the `L` value has been transformed to an `L2`. The `R` value,
    /// if present, remains unchanged.
    fn map_left<FL, L2>(self, f: FL) -> Self::Output<L2, R>
    where
        Self: Sized,
        FL: FnOnce(L) -> L2,
    {
        self.map(f, |r| r)
    }

    /// Transforms the `R` value using a provided function.
    ///
    /// ## Arguments
    ///
    /// * `f` - A function to transform the right value.
    ///
    /// ## Returns
    ///
    /// An `LeftOrRight` where the `R` value has been transformed to an `R2`. The `L` value,
    /// if present, remains unchanged.
    fn map_right<FR, R2>(self, f: FR) -> Self::Output<L, R2>
    where
        Self: Sized,
        FR: FnOnce(R) -> R2,
    {
        self.map(|l| l, f)
    }

    /// Transforms the `L` or `R` value using separate functions, depending
    /// on the variant.
    ///
    /// ## Arguments
    ///
    /// * `fl` - A function to transform the left value.
    /// * `fr` - A function to transform the right value.
    ///
    /// ## Returns
    ///
    /// An `LeftOrRight` where the `Left` or `Right` value has been transformed
    /// by the corresponding function.
    fn map<FL, FR, L2, R2>(self, fl: FL, fr: FR) -> Self::Output<L2, R2>
    where
        FL: FnOnce(L) -> L2,
        FR: FnOnce(R) -> R2;
}

/// The `Unwrap` trait provides utilities for safely extracting values from 
/// a dual-variant type (`LeftOrRight`).
/// 
/// It allows ergonomic operations that retrieve either the left or right value, 
/// with fallbacks when the expected variant is not present.
///
/// ## Usage
///
/// The `Unwrap` trait simplifies the handling of `LeftOrRight` types by providing
/// convenient methods to extract values transparently or substitute a value when
/// the expected variant does not exist.
///
/// ## Examples
///
/// ```rust
/// use any_of::{EitherOf, Unwrap, Left, Right};
///
/// let left: EitherOf<i32, &str> = Left(42);
/// let right: EitherOf<i32, &str> = Right("example");
///
/// // Retrieve the left value, or default to 0
/// assert_eq!(left.left_or_default(), 42);
/// assert_eq!(right.left_or_default(), 0);
///
/// // Use closures to provide alternate values
/// assert_eq!(right.left_or_else(|| 100), 100);
/// assert_eq!(left.right_or_else(|| "default"), "default");
///
/// // Use custom default values
/// assert_eq!(left.left_or(99), 42);
/// assert_eq!(right.right_or("new default"), "example");
/// ```
///
/// This trait is especially useful in scenarios where fallbacks or default
/// values need to be derived dynamically or when working with optional or
/// fallible data structures.
pub trait Unwrap<L, R>: LeftOrRight<L, R> {
    /// Returns the left value or computes a default using the provided closure.
    ///
    /// ## Arguments
    ///
    /// * `f` - A closure that generates a default value if the variant is `R`.
    ///
    /// ## Returns
    ///
    /// The left value if the variant is `L`, otherwise the default value generated
    /// by the closure.
    fn left_or_else(self, f: impl FnOnce() -> L) -> L;

    /// Returns the right value or computes a default using the provided closure.
    ///
    /// ## Arguments
    ///
    /// * `f` - A closure that generates a default value if the variant is `L`.
    ///
    /// ## Returns
    ///
    /// The right value if the variant is `R`, otherwise the default value generated
    /// by the closure.
    fn right_or_else(self, f: impl FnOnce() -> R) -> R;

    /// Returns the left value or a provided default.
    ///
    /// ## Arguments
    ///
    /// * `other` - The default value to use if the variant is `Right`.
    ///
    /// ## Returns
    ///
    /// The left value if the variant is `Left`, otherwise the provided default value.
    fn left_or(self, l: L) -> L
    where
        Self: Sized,
    {
        self.left_or_else(|| l)
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
    fn right_or(self, r: R) -> R
    where
        Self: Sized,
    {
        self.right_or_else(|| r)
    }

    /// Returns the left value or a default value.
    ///
    /// If the variant is `Left`, this method returns the `left` value.
    /// Otherwise, it returns the default value of the `L` type.
    ///
    /// ## Returns
    ///
    /// * `L` - The left value if it exists, or the default value of `L` otherwise.
    ///
    /// ## Requirements
    ///
    /// * The `L` type must implement the `Default` trait.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use any_of::{EitherOf, Unwrap, Left, Right};
    ///
    /// let left: EitherOf<i32, &str> = Left(42);
    /// let right: EitherOf<i32, &str> = Right("example");
    ///
    /// // Returns the left value as it exists.
    /// assert_eq!(left.left_or_default(), 42);
    ///
    /// // Returns the default value for i32 (0) because the variant is `Right`.
    /// assert_eq!(right.left_or_default(), 0);
    /// ```
    fn left_or_default(self) -> L
    where
        Self: Sized,
        L: Default,
    {
        self.left_or(L::default())
    }

    /// Returns the right value or a default value.
    ///
    /// If the variant is `Right`, this method returns the `right` value.
    /// Otherwise, it returns the default value of the `R` type.
    ///
    /// ## Returns
    ///
    /// * `R` - The right value if it exists, or the default value of `R` otherwise.
    ///
    /// ## Requirements
    ///
    /// * The `R` type must implement the `Default` trait.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use any_of::{EitherOf, Unwrap, Left, Right};
    ///
    /// let left: EitherOf<i32, &str> = Left(42);
    /// let right: EitherOf<i32, &str> = Right("example");
    ///
    /// // Returns the default value for &str ("") because the variant is `Left`.
    /// assert_eq!(left.right_or_default(), "");
    ///
    /// // Returns the right value as it exists.
    /// assert_eq!(right.right_or_default(), "example");
    /// ```
    fn right_or_default(self) -> R
    where
        Self: Sized,
        R: Default,
    {
        self.right_or(R::default())
    }

    /// Extracts the left value, panicking with the provided message if the variant is `Right`.
    ///
    /// ## Arguments
    ///
    /// * `msg` - The panic message to display if the variant is `Right`.
    ///
    /// ## Panics
    ///
    /// Panics if the variant is `Right`.
    ///
    /// ## Returns
    ///
    /// The left value if the variant is `Left`.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use any_of::{EitherOf, Unwrap, Left, Right};
    ///
    /// let left: EitherOf<i32, &str> = Left(42);
    /// let right: EitherOf<i32, &str> = Right("example");
    ///
    /// // Successfully extracts the left value.
    /// assert_eq!(left.expect_left("Expected left value"), 42);
    ///
    /// // Panics with the provided message.
    /// // right.expect_left("Expected left value");
    /// ```
    fn expect_left(self, msg: &str) -> L
    where
        Self: Sized,
    {
        self.left_or_else(|| panic!("{}", msg))
    }

    /// Extracts the right value, panicking with the provided message if the variant is `Left`.
    ///
    /// ## Arguments
    ///
    /// * `msg` - The panic message to display if the variant is `Left`.
    ///
    /// ## Panics
    ///
    /// Panics if the variant is `Left`.
    ///
    /// ## Returns
    ///
    /// The right value if the variant is `Right`.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use any_of::{EitherOf, Unwrap, Left, Right};
    ///
    /// let left: EitherOf<i32, &str> = Left(42);
    /// let right: EitherOf<i32, &str> = Right("example");
    ///
    /// // Successfully extracts the right value.
    /// assert_eq!(right.expect_right("Expected right value"), "example");
    ///
    /// // Panics with the provided message.
    /// // left.expect_right("Expected right value");
    /// ```
    fn expect_right(self, msg: &str) -> R
    where
        Self: Sized,
    {
        self.right_or_else(|| panic!("{}", msg))
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
    fn unwrap_left(self) -> L
    where
        Self: Sized,
    {
        self.expect_left("called `unwrap_left` on `LeftOrRight` value that is `Right`")
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
    fn unwrap_right(self) -> R
    where
        Self: Sized,
    {
        self.expect_right("called `unwrap_right` on `LeftOrRight` value that is `Left`")
    }
}
