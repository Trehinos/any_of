use crate::AnyOf;

pub type AnyOf4<LL, LR = LL, RL = LR, RR = RL> = AnyOf<AnyOf<LL, LR>, AnyOf<RL, RR>>;

impl<LL, LR, RL, RR> AnyOf4<LL, LR, RL, RR> {
    pub fn ll(&self) -> Option<&LL> {
        self.left()?.left()
    }
    pub fn lr(&self) -> Option<&LR> {
        self.left()?.right()
    }
    pub fn rl(&self) -> Option<&RL> {
        self.right()?.left()
    }
    pub fn rr(&self) -> Option<&RR> {
        self.right()?.right()
    }
}

pub type AnyOf8<LLL, LLR = LLL, LRL = LLR, LRR = LRL, RLL = LLL, RLR = LLR, RRL = LRL, RRR = LRR> =
AnyOf<AnyOf4<LLL, LLR, LRL, LRR>, AnyOf4<RLL, RLR, RRL, RRR>>;

impl<LLL, LLR, LRL, LRR, RLL, RLR, RRL, RRR> AnyOf8<LLL, LLR, LRL, LRR, RLL, RLR, RRL, RRR> {
    pub fn lll(&self) -> Option<&LLL> {
        self.left()?.ll()
    }
    pub fn llr(&self) -> Option<&LLR> {
        self.left()?.lr()
    }
    pub fn lrl(&self) -> Option<&LRL> {
        self.left()?.rl()
    }
    pub fn lrr(&self) -> Option<&LRR> {
        self.left()?.rr()
    }
    pub fn rll(&self) -> Option<&RLL> {
        self.right()?.ll()
    }
    pub fn rlr(&self) -> Option<&RLR> {
        self.right()?.lr()
    }
    pub fn rrl(&self) -> Option<&RRL> {
        self.right()?.rl()
    }
    pub fn rrr(&self) -> Option<&RRR> {
        self.right()?.rr()
    }
}

/// Never use this monstrosity.
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
    pub fn llll(&self) -> Option<&LLLL> {
        self.left()?.lll()
    }
    pub fn lllr(&self) -> Option<&LLLR> {
        self.left()?.llr()
    }
    pub fn llrl(&self) -> Option<&LLRL> {
        self.left()?.lrl()
    }
    pub fn llrr(&self) -> Option<&LLRR> {
        self.left()?.lrr()
    }
    pub fn lrll(&self) -> Option<&LRLL> {
        self.left()?.rll()
    }
    pub fn lrlr(&self) -> Option<&LRLR> {
        self.left()?.rlr()
    }
    pub fn lrrl(&self) -> Option<&LRRL> {
        self.left()?.rrl()
    }
    pub fn lrrr(&self) -> Option<&LRRR> {
        self.left()?.rrr()
    }
    pub fn rlll(&self) -> Option<&RLLL> {
        self.right()?.lll()
    }
    pub fn rllr(&self) -> Option<&RLLR> {
        self.right()?.llr()
    }
    pub fn rlrl(&self) -> Option<&RLRL> {
        self.right()?.lrl()
    }
    pub fn rlrr(&self) -> Option<&RLRR> {
        self.right()?.lrr()
    }
    pub fn rrll(&self) -> Option<&RRLL> {
        self.right()?.rll()
    }
    pub fn rrlr(&self) -> Option<&RRLR> {
        self.right()?.rlr()
    }
    pub fn rrrl(&self) -> Option<&RRRL> {
        self.right()?.rrl()
    }
    pub fn rrrr(&self) -> Option<&RRRR> {
        self.right()?.rrr()
    }
}
