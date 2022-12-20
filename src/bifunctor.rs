use core::marker::PhantomData;

use crate::prelude::*;

/// [`Bifunctor`], but specialized to know at compile-time
/// that the functions will only be called at most once.
pub trait BifunctorOnce<F, A, B>
  where F: HKT2<T<A, B> = Self>
{
  /// See [`Bifunctor`]
  fn bimap1<A2, B2, FA, FB>(self, fa: FA, fb: FB) -> F::T<A2, B2>
    where FA: F1Once<A, Ret = A2>,
          FB: F1Once<B, Ret = B2>;

  /// Map the left type in the Bifunctor
  ///
  /// In Result, this maps the "Ok" type and is equivalent to `map`.
  fn lmap1<A2, FA>(self, fa: FA) -> F::T<A2, B>
    where Self: Sized,
          FA: F1Once<A, Ret = A2>
  {
    self.bimap1(fa, |b| b)
  }

  /// Map the right type in the Bifunctor
  ///
  /// In Result, this maps the "Error" type and is equivalent to `map_err`.
  fn rmap1<B2, FB>(self, fb: FB) -> F::T<A, B2>
    where Self: Sized,
          FB: F1Once<B, Ret = B2>
  {
    self.bimap1(|a| a, fb)
  }
}

/// A Bifunctor provides a `map` function for types with 2 parameters,
/// allowing you to act on both types at once.
///
/// Additionally, Bifunctor provides `lmap` and `rmap` which allow you to
/// unambiguously map only one of the types.
///
/// ```
/// use std::io;
/// use std::path::PathBuf;
///
/// use naan::prelude::*;
///
/// fn get_some_meaningful_filepath() -> io::Result<String> {
///   # Ok("".into())
/// }
///
/// // in one shot we turn `Result<String, io::Error>` into `Result<PathBuf, String>`.
/// get_some_meaningful_filepath().bimap(|ok| PathBuf::from(ok), |err| format!("{err:?}"));
/// ```
pub trait Bifunctor<F, A, B>
  where F: HKT2<T<A, B> = Self>
{
  /// See [`Bifunctor`]
  fn bimap<A2, B2, FA, FB>(self, fa: FA, fb: FB) -> F::T<A2, B2>
    where FA: F1<A, Ret = A2>,
          FB: F1<B, Ret = B2>;

  /// Map the left type in the Bifunctor
  ///
  /// In Result, this maps the "Ok" type and is equivalent to `map`.
  fn lmap<A2, FA>(self, fa: FA) -> F::T<A2, B>
    where Self: Sized,
          FA: F1<A, Ret = A2>
  {
    self.bimap(fa, |b| b)
  }

  /// Map the right type in the Bifunctor
  ///
  /// In Result, this maps the "Error" type and is equivalent to `map_err`.
  fn rmap<B2, FB>(self, fb: FB) -> F::T<A, B2>
    where Self: Sized,
          FB: F1<B, Ret = B2>
  {
    self.bimap(|a| a, fb)
  }

  /// Wrap this type in [`Join`]
  fn join(self) -> Join<F, Self, A>
    where Self: Sized,
          F: HKT2<T<A, A> = Self>
  {
    Join::join(self)
  }
}

/// Join is a newtype that provides a single [`fmap`](Functor::fmap) implementation
/// for [`Bifunctor`]s that have the same type for both parameters, e.g.
/// `Result<A, A>` or `(A, A)`.
///
/// ```
/// use naan::prelude::*;
///
/// let r = Result::<(), ()>::Ok(()) // Result<(), ()>
///                                 .join()
///                                 .fmap(|_| format!("hello!")) // Result<String, String>
///                                 .unjoin();
///
/// assert_eq!(r, Ok(format!("hello!")));
/// ```
pub struct Join<M, T, A>(T, PhantomData<(M, A)>);

impl<M, T, A> Join<M, T, A> {
  /// Wrap a bifunctor in [`Join`]
  pub fn join(t: T) -> Join<M, T, A>
    where M: HKT2<T<A, A> = T>
  {
    Join(t, PhantomData)
  }

  /// Destroy this [`Join`], yielding the inner type
  pub fn unjoin(self) -> T {
    self.0
  }
}

/// [`Join`] as kind `Type -> Type`
pub struct JoinHKT<M>(PhantomData<M>);

impl<M> JoinHKT<M> {
  // I have no idea why this works.
  fn join<A>(t: M::T<A, A>) -> <Self as HKT1>::T<A>
    where M: HKT2
  {
    Join::join(t)
  }
}

impl<M> HKT1 for JoinHKT<M> where M: HKT2
{
  type T<A> = Join<M, <M as HKT2>::T<A, A>, A>;
}

impl<M, T, A> Functor<JoinHKT<M>, A> for Join<M, T, A>
  where JoinHKT<M>: HKT1<T<A> = Self>,
        M: HKT2 + HKT2<T<A, A> = T>,
        T: Bifunctor<M, A, A>
{
  fn fmap<AB, B>(self, f: AB) -> <JoinHKT<M> as HKT1>::T<B>
    where AB: F1<A, Ret = B>
  {
    JoinHKT::<M>::join::<B>(self.0.bimap(|a| f.call(a), |b| f.call(b)))
  }
}
