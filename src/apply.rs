use crate::prelude::*;

pub trait ApplyOnce<F, AB>
where
    Self: Functor<F, AB>,
    F: HKT1<T<AB> = Self>,
{
    fn apply1<A, B>(self, a: F::T<A>) -> F::T<B>
    where
        AB: F1Once<A, B>;
}

pub trait Apply<F, AB>
where
    Self: Functor<F, AB>,
    F: HKT1<T<AB> = Self>,
{
    fn apply<A, B>(self, a: F::T<A>) -> F::T<B>
    where
        AB: F1<A, B>,
        A: Clone;
}

pub trait Applicative<F, A>
where
    Self: Apply<F, A>,
    F: HKT1<T<A> = Self>,
{
    fn pure(a: A) -> F::T<A>;
}
