//! This module provides type definitions representing combinations
//! of multiple possible types and their associated utility implementations.
//!
//! # Overview
//!
//! The `AnyOf` types are designed to express collections of various
//! possible types within a single type while preserving type safety
//! and making it easier to work with nested combinations of types.
//! Each `AnyOfX` type represents a combination containing up to
//! `X` potential variants.
//!
//! # Utility Implementations
//!
//! Each `AnyOfX` type provides utility methods to extract specific variants
//! if they exist. These methods are named based on their position in
//! the hierarchy and return an `Option` of the specified type.
//!
//! Example for `AnyOf8`:
//! - `lll()`: Returns the left-left-left value if it exists.
//! - `lrr()`: Returns the left-right-right value if it exists.
//! - `rrr()`: Returns the right-right-right value if it exists.
//!
//! Example usage:
//! ```rust
//! use any_of::AnyOf4;
//! use any_of::AnyOf;
//!
//! let value: AnyOf4<i32, &str, f64, char> = AnyOf4::new_left(AnyOf::new_left(42));
//! if let Some(inner) = value.ll() {
//!     println!("The left-left value is: {}", inner);
//! }
//! ```
//!
//! # Note
//! These `AnyOfX` types are highly compositional but increase in complexity as the `X` grows.
//! Use `AnyOf16` or higher with caution.

use crate::concepts::{Opt16, Opt4, Opt8};
use crate::{AnyOf, LeftOrRight};

/// A type representing a combination of four possible types.
pub type AnyOf4<LL, LR = LL, RL = LR, RR = RL> = AnyOf<AnyOf<LL, LR>, AnyOf<RL, RR>>;

impl<LL, LR, RL, RR> AnyOf4<LL, LR, RL, RR> {
    /// Creates a new `AnyOf4` instance from four optional values.
    ///
    /// The parameters correspond to the `ll`, `lr`, `rl`, and `rr` positions in
    /// the nested [`AnyOf`] structure. This mirrors [`AnyOf::new`].
    ///
    /// # Examples
    /// ```rust
    /// use any_of::{AnyOf4, AnyOf};
    ///
    /// let value = AnyOf4::new4(Some(1), None::<i32>, None::<i32>, None::<i32>);
    /// assert_eq!(value.ll(), Some(&1));
    /// ```
    pub fn new4(
        ll: Option<LL>,
        lr: Option<LR>,
        rl: Option<RL>,
        rr: Option<RR>,
    ) -> Self {
        Self::from_opt4((ll, lr, rl, rr))
    }

    /// Builds an `AnyOf4` from an [`Opt4`] tuple.
    ///
    /// `Opt4` is a convenient representation of four optional values.
    ///
    /// # Examples
    /// ```rust
    /// use any_of::{AnyOf4, Opt4};
    ///
    /// let tuple: Opt4<i32, i32, i32, i32> = (Some(1), None, None, None);
    /// let value = AnyOf4::from_opt4(tuple);
    /// assert!(value.ll().is_some());
    /// ```
    pub fn from_opt4(opt: Opt4<LL, LR, RL, RR>) -> Self {
        let (ll, lr, rl, rr) = opt;
        let left = if ll.is_none() && lr.is_none() {
            None
        } else {
            Some(AnyOf::new(ll, lr))
        };
        let right = if rl.is_none() && rr.is_none() {
            None
        } else {
            Some(AnyOf::new(rl, rr))
        };
        AnyOf::new(left, right)
    }

    /// Returns the left-left value if it exists.
    pub fn ll(&self) -> Option<&LL> {
        self.left()?.left()
    }

    /// Returns the left-right value if it exists.
    pub fn lr(&self) -> Option<&LR> {
        self.left()?.right()
    }

    /// Returns the right-left value if it exists.
    pub fn rl(&self) -> Option<&RL> {
        self.right()?.left()
    }

    /// Returns the right-right value if it exists.
    pub fn rr(&self) -> Option<&RR> {
        self.right()?.right()
    }

    /// Returns a tuple containing the four optional values represented by this `AnyOf4`.
    ///
    /// This is useful when you need a simple `(Option<LL>, Option<LR>, Option<RL>, Option<RR>)`
    /// representation instead of nested enums.
    ///
    /// # Examples
    /// ```rust
    /// use any_of::{AnyOf4, AnyOf, Opt4};
    ///
    /// let any: AnyOf4<i32, i32, i32, i32> = AnyOf4::new_left(AnyOf::new_left(1));
    /// assert_eq!(any.opt4(), (Some(&1), None, None, None));
    /// ```
    pub fn opt4(&self) -> Opt4<&LL, &LR, &RL, &RR> {
        (self.ll(), self.lr(), self.rl(), self.rr())
    }
}

/// A type representing a combination of eight possible types.
pub type AnyOf8<LLL, LLR = LLL, LRL = LLR, LRR = LRL, RLL = LLL, RLR = LLR, RRL = LRL, RRR = LRR> =
    AnyOf<AnyOf4<LLL, LLR, LRL, LRR>, AnyOf4<RLL, RLR, RRL, RRR>>;

impl<LLL, LLR, LRL, LRR, RLL, RLR, RRL, RRR> AnyOf8<LLL, LLR, LRL, LRR, RLL, RLR, RRL, RRR> {
    /// Creates a new `AnyOf8` from eight optional values.
    ///
    /// # Examples
    /// ```rust
    /// use any_of::AnyOf8;
    ///
    /// let value = AnyOf8::new8(
    ///     Some(1),
    ///     None::<i32>, None::<i32>, None::<i32>,
    ///     None::<i32>, None::<i32>, None::<i32>, None::<i32>,
    /// );
    /// assert!(value.lll().is_some());
    /// ```
    pub fn new8(
        lll: Option<LLL>,
        llr: Option<LLR>,
        lrl: Option<LRL>,
        lrr: Option<LRR>,
        rll: Option<RLL>,
        rlr: Option<RLR>,
        rrl: Option<RRL>,
        rrr: Option<RRR>,
    ) -> Self {
        Self::from_opt8((lll, llr, lrl, lrr, rll, rlr, rrl, rrr))
    }

    /// Builds an `AnyOf8` from an [`Opt8`] tuple.
    ///
    /// # Examples
    /// ```rust
    /// use any_of::{AnyOf8, Opt8};
    ///
    /// let tuple: Opt8<i32, i32, i32, i32, i32, i32, i32, i32> =
    ///     (Some(1), None, None, None, None, None, None, None);
    /// let value = AnyOf8::from_opt8(tuple);
    /// assert_eq!(value.lll(), Some(&1));
    /// ```
    pub fn from_opt8(opt: Opt8<LLL, LLR, LRL, LRR, RLL, RLR, RRL, RRR>) -> Self {
        let (lll, llr, lrl, lrr, rll, rlr, rrl, rrr) = opt;
        let left = if [lll.is_none(), llr.is_none(), lrl.is_none(), lrr.is_none()].iter().all(|&x| x) {
            None
        } else {
            Some(AnyOf4::new4(lll, llr, lrl, lrr))
        };
        let right = if [rll.is_none(), rlr.is_none(), rrl.is_none(), rrr.is_none()].iter().all(|&x| x) {
            None
        } else {
            Some(AnyOf4::new4(rll, rlr, rrl, rrr))
        };
        AnyOf::new(left, right)
    }

    /// Returns the left-left-left value if it exists.
    pub fn lll(&self) -> Option<&LLL> {
        self.left()?.ll()
    }

    /// Returns the left-left-right value if it exists.
    pub fn llr(&self) -> Option<&LLR> {
        self.left()?.lr()
    }

    /// Returns the left-right-left value if it exists.
    pub fn lrl(&self) -> Option<&LRL> {
        self.left()?.rl()
    }

    /// Returns the left-right-right value if it exists.
    pub fn lrr(&self) -> Option<&LRR> {
        self.left()?.rr()
    }

    /// Returns the right-left-left value if it exists.
    pub fn rll(&self) -> Option<&RLL> {
        self.right()?.ll()
    }

    /// Returns the right-left-right value if it exists.
    pub fn rlr(&self) -> Option<&RLR> {
        self.right()?.lr()
    }

    /// Returns the right-right-left value if it exists.
    pub fn rrl(&self) -> Option<&RRL> {
        self.right()?.rl()
    }

    /// Returns the right-right-right value if it exists.
    pub fn rrr(&self) -> Option<&RRR> {
        self.right()?.rr()
    }

    /// Returns the eight optional values represented by this `AnyOf8` as a tuple.
    ///
    /// The returned [`Opt8`] allows easier pattern matching and conversion into
    /// other structures.
    ///
    /// # Examples
    /// ```rust
    /// use any_of::{AnyOf, AnyOf4, AnyOf8, Opt8};
    ///
    /// let any: AnyOf8<i32, i32, i32, i32, i32, i32, i32, i32> =
    ///     AnyOf8::new_left(AnyOf4::new_left(AnyOf::new_left(1)));
    /// let tuple = any.opt8();
    /// assert_eq!(tuple.0, Some(&1));
    /// ```
    pub fn opt8(&self) -> Opt8<&LLL, &LLR, &LRL, &LRR, &RLL, &RLR, &RRL, &RRR> {
        (
            self.lll(),
            self.llr(),
            self.lrl(),
            self.lrr(),
            self.rll(),
            self.rlr(),
            self.rrl(),
            self.rrr(),
        )
    }
}

/// A type representing a combination of sixteen possible types.
///
/// This type is highly complex—use cautiously.
pub type AnyOf16<
    LLLL,
    LLLR = LLLL,
    LLRL = LLLR,
    LLRR = LLRL,
    LRLL = LLLL,
    LRLR = LLLR,
    LRRL = LLRL,
    LRRR = LLRR,
    RLLL = LLLL,
    RLLR = LLLR,
    RLRL = LLRL,
    RLRR = LLRR,
    RRLL = LRLL,
    RRLR = LRLR,
    RRRL = LRRL,
    RRRR = LRRR,
> = AnyOf<
    AnyOf8<LLLL, LLLR, LLRL, LLRR, LRLL, LRLR, LRRL, LRRR>,
    AnyOf8<RLLL, RLLR, RLRL, RLRR, RRLL, RRLR, RRRL, RRRR>,
>;

impl<
        LLLL,
        LLLR,
        LLRL,
        LLRR,
        LRLL,
        LRLR,
        LRRL,
        LRRR,
        RLLL,
        RLLR,
        RLRL,
        RLRR,
        RRLL,
        RRLR,
        RRRL,
        RRRR,
    >
    AnyOf16<
        LLLL,
        LLLR,
        LLRL,
        LLRR,
        LRLL,
        LRLR,
        LRRL,
        LRRR,
        RLLL,
        RLLR,
        RLRL,
        RLRR,
        RRLL,
        RRLR,
        RRRL,
        RRRR,
    >
{
    /// Creates a new `AnyOf16` from sixteen optional values.
    ///
    /// # Examples
    /// ```rust
    /// use any_of::AnyOf16;
    ///
    /// let value = AnyOf16::new16(
    ///     Some(1), None::<i32>, None::<i32>, None::<i32>,
    ///     None::<i32>, None::<i32>, None::<i32>, None::<i32>,
    ///     None::<i32>, None::<i32>, None::<i32>, None::<i32>,
    ///     None::<i32>, None::<i32>, None::<i32>, None::<i32>,
    /// );
    /// assert!(value.llll().is_some());
    /// ```
    #[allow(clippy::too_many_arguments)]
    pub fn new16(
        llll: Option<LLLL>,
        lllr: Option<LLLR>,
        llrl: Option<LLRL>,
        llrr: Option<LLRR>,
        lrll: Option<LRLL>,
        lrlr: Option<LRLR>,
        lrrl: Option<LRRL>,
        lrrr: Option<LRRR>,
        rlll: Option<RLLL>,
        rllr: Option<RLLR>,
        rlrl: Option<RLRL>,
        rlrr: Option<RLRR>,
        rrll: Option<RRLL>,
        rrlr: Option<RRLR>,
        rrrl: Option<RRRL>,
        rrrr: Option<RRRR>,
    ) -> Self {
        Self::from_opt16((
            llll, lllr, llrl, llrr, lrll, lrlr, lrrl, lrrr,
            rlll, rllr, rlrl, rlrr, rrll, rrlr, rrrl, rrrr,
        ))
    }

    /// Builds an `AnyOf16` from an [`Opt16`] tuple.
    ///
    /// # Examples
    /// ```rust
    /// use any_of::{AnyOf16, Opt16};
    ///
    /// let tuple: Opt16<i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32,
    ///     i32, i32, i32, i32, i32> =
    ///     (Some(1), None, None, None, None, None, None, None,
    ///      None, None, None, None, None, None, None, None);
    /// let value = AnyOf16::from_opt16(tuple);
    /// assert_eq!(value.llll(), Some(&1));
    /// ```
    #[allow(clippy::too_many_arguments)]
    pub fn from_opt16(
        opt: Opt16<
            LLLL,
            LLLR,
            LLRL,
            LLRR,
            LRLL,
            LRLR,
            LRRL,
            LRRR,
            RLLL,
            RLLR,
            RLRL,
            RLRR,
            RRLL,
            RRLR,
            RRRL,
            RRRR,
        >,
    ) -> Self {
        let (
            llll, lllr, llrl, llrr, lrll, lrlr, lrrl, lrrr,
            rlll, rllr, rlrl, rlrr, rrll, rrlr, rrrl, rrrr,
        ) = opt;
        let left_empty = [
            llll.is_none(),
            lllr.is_none(),
            llrl.is_none(),
            llrr.is_none(),
            lrll.is_none(),
            lrlr.is_none(),
            lrrl.is_none(),
            lrrr.is_none(),
        ]
        .iter()
        .all(|&x| x);
        let left = if left_empty {
            None
        } else {
            Some(AnyOf8::new8(
                llll, lllr, llrl, llrr, lrll, lrlr, lrrl, lrrr,
            ))
        };
        let right_empty = [
            rlll.is_none(),
            rllr.is_none(),
            rlrl.is_none(),
            rlrr.is_none(),
            rrll.is_none(),
            rrlr.is_none(),
            rrrl.is_none(),
            rrrr.is_none(),
        ]
        .iter()
        .all(|&x| x);
        let right = if right_empty {
            None
        } else {
            Some(AnyOf8::new8(
                rlll, rllr, rlrl, rlrr, rrll, rrlr, rrrl, rrrr,
            ))
        };
        AnyOf::new(left, right)
    }

    /// Returns the left-left-left-left value if it exists.
    pub fn llll(&self) -> Option<&LLLL> {
        self.left()?.lll()
    }

    /// Returns the left-left-left-right value if it exists.
    pub fn lllr(&self) -> Option<&LLLR> {
        self.left()?.llr()
    }

    /// Returns the left-left-right-left value if it exists.
    pub fn llrl(&self) -> Option<&LLRL> {
        self.left()?.lrl()
    }

    /// Returns the left-left-right-right value if it exists.
    pub fn llrr(&self) -> Option<&LLRR> {
        self.left()?.lrr()
    }

    /// Returns the left-right-left-left value if it exists.
    pub fn lrll(&self) -> Option<&LRLL> {
        self.left()?.rll()
    }

    /// Returns the left-right-left-right value if it exists.
    pub fn lrlr(&self) -> Option<&LRLR> {
        self.left()?.rlr()
    }

    /// Returns the left-right-right-left value if it exists.
    pub fn lrrl(&self) -> Option<&LRRL> {
        self.left()?.rrl()
    }

    /// Returns the left-right-right-right value if it exists.
    pub fn lrrr(&self) -> Option<&LRRR> {
        self.left()?.rrr()
    }

    /// Returns the right-left-left-left value if it exists.
    pub fn rlll(&self) -> Option<&RLLL> {
        self.right()?.lll()
    }

    /// Returns the right-left-left-right value if it exists.
    pub fn rllr(&self) -> Option<&RLLR> {
        self.right()?.llr()
    }

    /// Returns the right-left-right-left value if it exists.
    pub fn rlrl(&self) -> Option<&RLRL> {
        self.right()?.lrl()
    }

    /// Returns the right-left-right-right value if it exists.
    pub fn rlrr(&self) -> Option<&RLRR> {
        self.right()?.lrr()
    }

    /// Returns the right-right-left-left value if it exists.
    pub fn rrll(&self) -> Option<&RRLL> {
        self.right()?.rll()
    }

    /// Returns the right-right-left-right value if it exists.
    pub fn rrlr(&self) -> Option<&RRLR> {
        self.right()?.rlr()
    }

    /// Returns the right-right-right-left value if it exists.
    pub fn rrrl(&self) -> Option<&RRRL> {
        self.right()?.rrl()
    }

    /// Returns the right-right-right-right value if it exists.
    pub fn rrrr(&self) -> Option<&RRRR> {
        self.right()?.rrr()
    }

    #[allow(clippy::type_complexity)]
    /// Returns all sixteen optional values represented by this `AnyOf16`.
    ///
    /// This method provides a direct mapping to the [`Opt16`] type for easier
    /// inspection or conversion.
    ///
    /// # Examples
    /// ```rust
    /// use any_of::{AnyOf, AnyOf4, AnyOf8, AnyOf16, Opt16};
    ///
    /// let any: AnyOf16<i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32> =
    ///     AnyOf16::new_left(AnyOf8::new_left(AnyOf4::new_left(AnyOf::new_left(1))));
    /// let tuple = any.opt16();
    /// assert_eq!(tuple.0, Some(&1));
    /// ```
    pub fn opt16(
        &self,
    ) -> Opt16<
        &LLLL,
        &LLLR,
        &LLRL,
        &LLRR,
        &LRLL,
        &LRLR,
        &LRRL,
        &LRRR,
        &RLLL,
        &RLLR,
        &RLRL,
        &RLRR,
        &RRLL,
        &RRLR,
        &RRRL,
        &RRRR,
    > {
        (
            self.llll(),
            self.lllr(),
            self.llrl(),
            self.llrr(),
            self.lrll(),
            self.lrlr(),
            self.lrrl(),
            self.lrrr(),
            self.rlll(),
            self.rllr(),
            self.rlrl(),
            self.rlrr(),
            self.rrll(),
            self.rrlr(),
            self.rrrl(),
            self.rrrr(),
        )
    }
}
