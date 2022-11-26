use crate::prelude::*;

/// Option Kinds
pub mod hkt {
  use crate::prelude::*;
  use crate::Never;

  /// [`core::option::Option`] lifted to an HKT1
  ///
  /// (Kind `Type -> Type`)
  pub struct Option(Never);
  impl HKT1 for Option {
    type T<A> = ::std::option::Option<A>;
  }
}

impl<A> FunctorOnce<hkt::Option, A> for Option<A> {
  fn fmap1<B>(self, f: impl F1Once<A, B>) -> Option<B> {
    self.map(|a| f.call1(a))
  }
}
deriving!(impl Functor<hkt::Option, A> for Option<A> {..FunctorOnce});

impl<AB> ApplyOnce<hkt::Option, AB> for Option<AB> {
  fn apply1<A, B>(self, a: Option<A>) -> Option<B>
    where AB: F1Once<A, B>
  {
    match self {
      | Some(f) => a.map(|a| f.call1(a)),
      | None => None,
    }
  }
}
deriving!(impl Apply<hkt::Option, AB> for Option<AB> {..ApplyOnce});

impl<A> Applicative<hkt::Option, A> for Option<A> {
  fn pure(a: A) -> Option<A> {
    Some(a)
  }
}

impl<A> Alt<hkt::Option, A> for Option<A> {
  fn alt(self, b: Self) -> Self {
    self.or(b)
  }
}
deriving!(impl Plus<hkt::Option, A> for Option<A> {..Default});

impl<A> Semigroup for Option<A> where A: Semigroup
{
  fn append(self, b: Self) -> Self {
    match (self, b) {
      | (Some(a), Some(b)) => Some(a.append(b)),
      | (Some(a), _) => Some(a),
      | (_, Some(b)) => Some(b),
      | _ => None,
    }
  }
}

impl<A> Monoid for Option<A> where A: Semigroup
{
  fn identity() -> Self {
    None
  }
}
