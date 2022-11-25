use crate::prelude::*;

pub trait Alt<F, A>
where
    Self: Functor<F, A>,
    F: HKT1<T<A> = Self>,
{
    fn alt(self, b: Self) -> Self;
}

pub trait Plus<F, A>
where
    Self: Alt<F, A>,
    F: HKT1<T<A> = Self>,
{
    fn empty() -> F::T<A>;
}
