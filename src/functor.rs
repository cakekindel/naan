use crate::prelude::*;

pub trait FunctorOnce<F, A>
  where F: HKT1<T<A> = Self>
{
  fn fmap1<B>(self, f: impl F1Once<A, B>) -> F::T<B>;
}

pub trait Functor<F, A>
  where F: HKT1<T<A> = Self>
{
  fn fmap<B>(self, f: impl F1<A, B>) -> F::T<B>;
}
