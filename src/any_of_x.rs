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
//! use any_of::any_of_x::AnyOf8;
//!
//! let value: AnyOf8<i32, &str, f64, char, bool, u8, u16, i64> = todo!();
//! if let Some(inner) = value.lll() {
//!     println!("The left-left-left value is: {}", inner);
//! }
//! ```
//!
//! # Note
//! These `AnyOfX` types are highly compositional but increase in complexity as the `X` grows.
//! Use `AnyOf16` or higher with caution.

use crate::AnyOf;

/// A type representing a combination of four possible types.
pub type AnyOf4<LL, LR = LL, RL = LR, RR = RL> = AnyOf<AnyOf<LL, LR>, AnyOf<RL, RR>>;

impl<LL, LR, RL, RR> AnyOf4<LL, LR, RL, RR> {
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
}

/// A type representing a combination of eight possible types.
pub type AnyOf8<LLL, LLR = LLL, LRL = LLR, LRR = LRL, RLL = LLL, RLR = LLR, RRL = LRL, RRR = LRR> =
AnyOf<AnyOf4<LLL, LLR, LRL, LRR>, AnyOf4<RLL, RLR, RRL, RRR>>;

impl<LLL, LLR, LRL, LRR, RLL, RLR, RRL, RRR> AnyOf8<LLL, LLR, LRL, LRR, RLL, RLR, RRL, RRR> {
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
}

/// A type representing a combination of sixteen possible types.
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
> AnyOf16<
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
> {
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
}