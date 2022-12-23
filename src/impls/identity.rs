#[allow(unused_imports)]
use core::fmt::Debug;
use core::ops::{Add, Div, Mul, Neg, Not};

use crate::prelude::*;

/// Id HKT
pub mod hkt {
  use crate::HKT1;

  /// Id HKT
  pub struct Id;

  impl HKT1 for Id {
    type T<A> = super::Id<A>;
  }
}

/// The Identity monad
///
/// This type does nothing more than wrap a `T`.
///
/// Typeclasses that Id impls for all T:
///  * [`FunctorOnce`]
///  * [`MonadOnce`]
///  * [`ApplyOnce`]
///  * [`Applicative`]
///  * [`Alt`] (ignores second Id)
///
/// Typeclasses that Id impls when T impls:
///  * `(Partial)Eq`
///  * `(Partial)Ord`
///  * `append` / `identity` [`Semigroup`] [`Monoid`]
///  * [`Debug`]
///  * [`Clone`], [`Copy`]
///  * [`Default`]
///  * Arithmetic / algebra [`Add`], [`Mul`], [`Div`], [`Neg`], [`Not`]
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Id<T>(pub T);

impl<T> Id<T> {
  /// Wrap a type in [`Id`]
  pub fn new(t: T) -> Self {
    Self(t)
  }

  /// Unwrap this [`Id`]
  pub fn get(self) -> T {
    self.0
  }
}

impl<T> Not for Id<T> where T: Not
{
  type Output = Id<<T as Not>::Output>;

  fn not(self) -> Self::Output {
    Id(!self.0)
  }
}

impl<T> Neg for Id<T> where T: Neg
{
  type Output = Id<<T as Neg>::Output>;

  fn neg(self) -> Self::Output {
    Id(-self.0)
  }
}

impl<T> Add for Id<T> where T: Add
{
  type Output = Id<<T as Add>::Output>;

  fn add(self, rhs: Self) -> Self::Output {
    Id(self.0 + rhs.0)
  }
}

impl<T> Mul for Id<T> where T: Mul
{
  type Output = Id<<T as Mul>::Output>;

  fn mul(self, rhs: Self) -> Self::Output {
    Id(self.0 * rhs.0)
  }
}

impl<T> Div for Id<T> where T: Div
{
  type Output = Id<<T as Div>::Output>;

  fn div(self, rhs: Self) -> Self::Output {
    Id(self.0 / rhs.0)
  }
}

impl<T> Semigroup for Id<T> where T: Semigroup
{
  fn append(self, b: Self) -> Self {
    Id(self.0.append(b.0))
  }
}

impl<T> Monoid for Id<T> where T: Monoid
{
  fn identity() -> Self {
    Id(T::identity())
  }
}

impl<T> FunctorOnce<hkt::Id, T> for Id<T> {
  fn fmap1<AB, B>(self, f: AB) -> Id<B>
    where AB: F1Once<T, Ret = B>
  {
    Id(f.call1(self.0))
  }
}
deriving!(impl Functor<hkt::Id, A> for Id<A> {..FunctorOnce});

impl<T> MonadOnce<hkt::Id, T> for Id<T> {
  fn bind1<B, AMB>(self, f: AMB) -> Id<B>
    where AMB: F1Once<T, Ret = Id<B>>
  {
    f.call1(self.0)
  }
}
deriving!(impl Monad<hkt::Id, A> for Id<A> {..MonadOnce});

impl<T> ApplyOnce<hkt::Id, T> for Id<T> {
  fn apply1<A, B>(self, a: Id<A>) -> Id<B>
    where T: F1Once<A, Ret = B>
  {
    Id(self.0.call1(a.0))
  }
}
deriving!(impl Apply<hkt::Id, AB> for Id<AB> {..ApplyOnce});

impl<T> Applicative<hkt::Id, T> for Id<T> {
  fn pure(a: T) -> Id<T> {
    Id(a)
  }
}

impl<T> Alt<hkt::Id, T> for Id<T> {
  fn alt(self, _: Self) -> Self {
    self
  }
}
