
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum Either<L, R> {
    Left(L),
    Right(R),
}

impl<L, R> Either<L, R> {
    pub fn new_left(left: L) -> Self {
        Self::Left(left)
    }
    pub fn new_right(right: R) -> Self {
        Self::Right(right)
    }

    pub fn is_left(&self) -> bool {
        matches!(self, Self::Left(_))
    }

    pub fn is_right(&self) -> bool {
        matches!(self, Self::Right(_))
    }

    pub fn any(&self) -> (Option<&L>, Option<&R>) {
        match self {
            Self::Left(l) => (Some(l), None),
            Self::Right(r) => (None, Some(r)),
        }
    }
    pub fn left(&self) -> Option<&L> {
        match self {
            Self::Left(l) => Some(l),
            Self::Right(_) => None,
        }
    }

    pub fn right(&self) -> Option<&R> {
        match self {
            Self::Left(_) => None,
            Self::Right(r) => Some(r),
        }
    }

    pub fn left_or_else(self, f: impl FnOnce() -> L) -> L {
        match self {
            Self::Left(l) => l,
            Self::Right(_) => f(),
        }
    }
    pub fn right_or_else(self, f: impl FnOnce() -> R) -> R {
        match self {
            Self::Left(_) => f(),
            Self::Right(r) => r,
        }
    }
    pub fn left_or(self, other: L) -> L {
        self.left_or_else(|| other)
    }
    pub fn right_or(self, other: R) -> R {
        self.right_or_else(|| other)
    }
    pub fn unwrap_left(self) -> L {
        self.left_or_else(|| panic!("Can only unwrap left of LeftOrRight::Left"))
    }
    pub fn unwrap_right(self) -> R {
        self.right_or_else(|| panic!("Can only unwrap right of LeftOrRight::Right"))
    }
    pub fn swap(self) -> Either<R, L> {
        match self {
            Self::Left(l) => Either::<R, L>::Right(l),
            Self::Right(r) => Either::<R, L>::Left(r),
        }
    }
    pub fn map_left<FL: FnOnce(L) -> L2, L2>(self, f: FL) -> Either<L2, R> {
        self.map(f, |r| r)
    }
    pub fn map_right<FR: FnOnce(R) -> R2, R2>(self, f: FR) -> Either<L, R2> {
        self.map(|l| l, f)
    }
    pub fn map<FL, FR, L2, R2>(self, f: FL, g: FR) -> Either<L2, R2>
    where
        FL: FnOnce(L) -> L2,
        FR: FnOnce(R) -> R2,
    {
        match self {
            Self::Left(l) => Either::<L2, R2>::Left(f(l)),
            Self::Right(r) => Either::<L2, R2>::Right(g(r)),
        }
    }
}
