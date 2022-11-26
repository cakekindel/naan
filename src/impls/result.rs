use crate::prelude::*;

/// Result Kinds
pub mod hkt {
  use std::marker::PhantomData;

  use super::*;

  /// [`core::result::Result`] lifted to an HKT1
  /// with the error type pinned to some `E`.
  ///
  /// (Kind `Type -> Type`)
  pub struct ResultOk<E>(PhantomData<E>);
  impl<E> HKT1 for ResultOk<E> {
    type T<A> = ::std::result::Result<A, E>;
  }

  /// [`core::result::Result`] lifted to an HKT2
  ///
  /// (Kind `Type -> Type -> Type`)
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

impl<AB, E> ApplyOnce<hkt::ResultOk<E>, AB> for Result<AB, E> {
  fn apply1<A, B>(self, a: Result<A, E>) -> Result<B, E>
    where AB: F1Once<A, B>
  {
    match self {
      | Ok(f) => a.map(|a| f.call1(a)),
      | Err(e) => Err(e),
    }
  }
}
deriving!(impl<E> Apply<hkt::ResultOk<E>, AB> for Result<AB, E> {..ApplyOnce});

impl<A, E> Applicative<hkt::ResultOk<E>, A> for Result<A, E> {
  fn pure(a: A) -> Result<A, E> {
    Ok(a)
  }
}

impl<A, E> Alt<hkt::ResultOk<E>, A> for Result<A, E> {
  fn alt(self, b: Self) -> Self {
    self.or(b)
  }
}
