pub trait Semigroup {
    fn append(self, b: Self) -> Self;
}

pub trait Monoid: Semigroup {
    fn identity() -> Self;
}
