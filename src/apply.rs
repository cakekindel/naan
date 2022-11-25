use crate::prelude::*;

pub trait ApplyOnce<F, AB, A, B>
  where Self: Functor<F, AB>,
        F: HKT1<T<AB> = Self>,
        AB: F1Once<A, B>
{
  fn apply1(self, a: F::T<A>) -> F::T<B>;
}

pub trait Apply<F, AB, A, B>
  where Self: Functor<F, AB>,
        F: HKT1<T<AB> = Self>,
        AB: F1<A, B>
{
  fn apply(self, a: F::T<A>) -> F::T<B>;
}
