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