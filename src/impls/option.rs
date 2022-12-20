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
  fn fmap1<AB, B>(self, f: AB) -> Option<B>
    where AB: F1Once<A, Ret = B>
  {
    self.map(|a| f.call1(a))
  }
}
deriving!(impl Functor<hkt::Option, A> for Option<A> {..FunctorOnce});

impl<AB> ApplyOnce<hkt::Option, AB> for Option<AB> {
  fn apply1<A, B>(self, a: Option<A>) -> Option<B>
    where AB: F1Once<A, Ret = B>
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

impl<A> FoldableOnce<hkt::Option, A> for Option<A> {
  fn fold1<B, BAB>(self, f: BAB, b: B) -> B
    where BAB: F2Once<B, A, Ret = B>
  {
    match self {
      | Some(a) => f.call1(b, a),
      | None => b,
    }
  }

  fn fold1_ref<'a, B, BAB>(&'a self, f: BAB, b: B) -> B
    where BAB: F2Once<B, &'a A, Ret = B>,
          A: 'a
  {
    match self {
      | Some(a) => f.call1(b, a),
      | None => b,
    }
  }
}

deriving!(impl Foldable<hkt::Option, A> for Option<A> {..FoldableOnce});

impl<A, B> TraversableOnce<hkt::Option, A, B, ()> for Option<A> {
  fn traverse1m<Ap, AtoApOfB>(self, f: AtoApOfB) -> Ap::T<Option<B>>
    where Ap: HKT1,
          Self: FoldableOnce<hkt::Option, A>,
          Ap::T<B>: Applicative<Ap, B>,
          Ap::T<()>: Applicative<Ap, ()>,
          Ap::T<Option<B>>: Applicative<Ap, Option<B>>,
          AtoApOfB: F1Once<A, Ret = Ap::T<B>>,
          hkt::Option: HKT1<T<A> = Self>
  {
    match self {
      | Some(a) => f.call1(a).fmap(|b| Some(b)),
      | None => Ap::T::pure(None),
    }
  }

  fn traverse11<Ap, AtoApOfB>(self, f: AtoApOfB) -> Ap::T<Option<B>>
    where Ap: HKT1,
          Ap::T<B>: Applicative<Ap, B> + ApplyOnce<Ap, B>,
          Ap::T<()>: Applicative<Ap, ()> + ApplyOnce<Ap, ()>,
          Ap::T<Option<B>>: Applicative<Ap, Option<B>> + ApplyOnce<Ap, Option<B>>,
          AtoApOfB: F1Once<A, Ret = Ap::T<B>>
  {
    self.traverse1m::<Ap, AtoApOfB>(f)
  }
}
deriving!(impl Traversable<hkt::Option, A, B, ()> for Option<A> {..TraversableOnce});

impl<A> MonadOnce<hkt::Option, A> for Option<A> {
  fn bind1<B, AMB>(self, f: AMB) -> Option<B>
    where AMB: F1Once<A, Ret = Option<B>>
  {
    self.and_then(|a| f.call1(a))
  }
}
deriving!(impl Monad<hkt::Option, A> for Option<A> {..MonadOnce});
