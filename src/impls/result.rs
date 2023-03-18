use crate::prelude::*;

/// Result Kinds
pub mod hkt {
  use core::marker::PhantomData;

  use super::*;

  /// [`core::result::Result`] lifted to an HKT1
  /// with the error type pinned to some `E`.
  ///
  /// (Kind `Type -> Type`)
  pub struct ResultOk<E>(PhantomData<E>);
  impl<E> HKT1 for ResultOk<E> {
    type T<A> = ::core::result::Result<A, E>;
  }

  /// [`core::result::Result`] lifted to an HKT2
  ///
  /// (Kind `Type -> Type -> Type`)
  pub struct Result;
  impl HKT2 for Result {
    type T<A, B> = ::core::result::Result<A, B>;
  }
}

pub trait ResultExt<T, E>
  where Self: Sized
{
  /// Swap the Ok and Err variants
  fn swap(self) -> Result<E, T>;

  /// Allows turning an Err back into Ok by binding on the Err variant
  fn recover<R, F>(self, f: F) -> Result<T, R>
    where F: F1Once<E, Ret = Result<T, R>>;

  /// Perform some IO when this Result is Err
  fn discard_err<F, B>(self, f: F) -> Result<T, E>
    where F: for<'a> F1Once<&'a E, Ret = B>,
          B: Discard;

  /// Test the data in Ok and turn it into an Err if it doesn't pass a predicate
  fn filter<P, F>(self, pred: P, on_fail: F) -> Result<T, E>
    where P: for<'a> F1Once<&'a T, Ret = bool>,
          F: for<'a> F1Once<&'a T, Ret = E>;

  /// Apply a function from `&T -> Result<R, E>` to this result,
  /// short-circuiting the first error encountered, and combining
  /// `T` and `R` if the input Result and return value of the function is Ok
  fn zip<R, F>(self, f: F) -> Result<(T, R), E>
    where F: for<'a> F1Once<&'a T, Ret = Result<R, E>>;
}

impl<T, E> ResultExt<T, E> for Result<T, E> {
  fn swap(self) -> Result<E, T> {
    match self {
      | Ok(t) => Err(t),
      | Err(e) => Ok(e),
    }
  }

  fn recover<R, F>(self, f: F) -> Result<T, R>
    where F: F1Once<E, Ret = Result<T, R>>
  {
    match self {
      | Ok(t) => Ok(t),
      | Err(e) => f.call1(e),
    }
  }

  fn discard_err<F, B>(self, f: F) -> Result<T, E>
    where F: for<'a> F1Once<&'a E, Ret = B>,
          B: Discard
  {
    match self {
      | Ok(t) => Ok(t),
      | Err(e) => {
        f.call1(&e);
        Err(e)
      },
    }
  }

  fn filter<P, F>(self, pred: P, on_fail: F) -> Result<T, E>
    where P: for<'a> F1Once<&'a T, Ret = bool>,
          F: for<'a> F1Once<&'a T, Ret = E>
  {
    match self {
      | Ok(t) if pred.call1(&t) => Ok(t),
      | Ok(t) => Err(on_fail.call1(&t)),
      | Err(e) => Err(e),
    }
  }

  fn zip<R, F>(self, f: F) -> Result<(T, R), E>
    where F: for<'a> F1Once<&'a T, Ret = Result<R, E>>
  {
    self.bind1(|t| f.call1(&t).map(|r| (t, r)))
  }
}

impl<A, E> FunctorOnce<hkt::ResultOk<E>, A> for Result<A, E> {
  fn fmap1<AB, B>(self, f: AB) -> Result<B, E>
    where AB: F1Once<A, Ret = B>
  {
    self.map(|a| f.call1(a))
  }
}
deriving!(impl<E> Functor<hkt::ResultOk<E>, A> for Result<A, E> {..FunctorOnce});

impl<AB, E> ApplyOnce<hkt::ResultOk<E>, AB> for Result<AB, E> {
  fn apply1<A, B>(self, a: Result<A, E>) -> Result<B, E>
    where AB: F1Once<A, Ret = B>
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

impl<A, E> FoldableOnce<hkt::ResultOk<E>, A> for Result<A, E> {
  fn fold1<B, BAB>(self, f: BAB, b: B) -> B
    where BAB: F2Once<B, A, Ret = B>
  {
    self.ok().fold1(f, b)
  }

  fn fold1_ref<'a, B, BAB>(&'a self, f: BAB, b: B) -> B
    where BAB: F2Once<B, &'a A, Ret = B>,
          A: 'a
  {
    match self {
      | Ok(a) => f.call1(b, a),
      | Err(_) => b,
    }
  }
}

deriving!(impl<E> Foldable<hkt::ResultOk<E>, A> for Result<A, E> {..FoldableOnce});

impl<A, B, E> TraversableOnce<hkt::ResultOk<E>, A, B, ()> for Result<A, E>
  where hkt::ResultOk<E>: HKT1<T<B> = Result<B, E>> + HKT1<T<A> = Result<A, E>>
{
  fn traverse1m<Ap, AtoApOfB>(self, f: AtoApOfB) -> Ap::T<Result<B, E>>
    where Ap: HKT1,
          Ap::T<B>: Applicative<Ap, B>,
          Ap::T<Result<B, E>>: Applicative<Ap, Result<B, E>>,
          AtoApOfB: F1Once<A, Ret = Ap::T<B>>
  {
    match self {
      | Ok(a) => f.call1(a).fmap(Ok),
      | Err(e) => Ap::T::pure(Err(e)),
    }
  }

  fn traverse11<Ap, AtoApOfB>(self, f: AtoApOfB) -> Ap::T<Result<B, E>>
    where Ap: HKT1,
          Ap::T<B>: Applicative<Ap, B> + ApplyOnce<Ap, B>,
          Ap::T<()>: Applicative<Ap, ()> + ApplyOnce<Ap, ()>,
          Ap::T<Result<B, E>>: Applicative<Ap, Result<B, E>> + ApplyOnce<Ap, Result<B, E>>,
          AtoApOfB: F1Once<A, Ret = Ap::T<B>>
  {
    self.traverse1m::<Ap, AtoApOfB>(f)
  }
}
deriving!(impl<E> Traversable<hkt::ResultOk<E>, A, B, ()> for Result<A, E> {..TraversableOnce});

impl<A, E> MonadOnce<hkt::ResultOk<E>, A> for Result<A, E> {
  fn bind1<B, AMB>(self, f: AMB) -> Result<B, E>
    where AMB: F1Once<A, Ret = Result<B, E>>
  {
    self.and_then(|a| f.call1(a))
  }
}
deriving!(impl<E> Monad<hkt::ResultOk<E>, A> for Result<A, E> {..MonadOnce});

impl<A, E> BifunctorOnce<hkt::Result, A, E> for Result<A, E> {
  fn bimap1<AB, BB, FA, FB>(self, fa: FA, fb: FB) -> <hkt::Result as HKT2>::T<AB, BB>
    where FA: F1Once<A, Ret = AB>,
          FB: F1Once<E, Ret = BB>
  {
    match self {
      | Ok(a) => Ok(fa.call1(a)),
      | Err(e) => Err(fb.call1(e)),
    }
  }
}
deriving!(impl Bifunctor<hkt::Result, A, E> for Result<A, E> {..BifunctorOnce});
