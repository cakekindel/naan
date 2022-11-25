use crate::prelude::*;

pub mod hkt {
  use std::marker::PhantomData;

  use super::*;

  pub struct ResultOk<E>(PhantomData<E>);
  impl<E> HKT1 for ResultOk<E> {
    type T<A> = ::std::result::Result<A, E>;
  }

  pub struct Result;
  impl HKT2 for Result {
    type T<A, B> = ::std::result::Result<A, B>;
  }
}

impl<A, E> FunctorOnce<hkt::ResultOk<E>, A> for Result<A, E> {
  fn fmap1<B>(self, f: impl F1Once<A, B>) -> Result<B, E> {
    self.map(|a| f.call1(a))
  }
}
deriving!(impl<E> Functor<hkt::ResultOk<E>, A> for Result<A, E> {..FunctorOnce});

impl<AB, A, B, E> ApplyOnce<hkt::ResultOk<E>, AB, A, B> for Result<AB, E> where AB: F1Once<A, B>
{
  fn apply1(self, a: Result<A, E>) -> Result<B, E> {
    match self {
      | Ok(f) => a.map(|a| f.call1(a)),
      | Err(e) => Err(e),
    }
  }
}
deriving!(impl<E> Apply<hkt::ResultOk<E>, AB, A, B> for Result<AB, E> {..ApplyOnce});

impl<A, E> Alt<hkt::ResultOk<E>, A> for Result<A, E> {
  fn alt(self, b: Self) -> Self {
    self.or(b)
  }
}
