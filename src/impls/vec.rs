use crate::prelude::*;

/// Vec Kinds
pub mod hkt {
  use super::*;

  /// [`std::vec::Vec`] lifted to an HKT1
  ///
  /// (Kind `Type -> Type`)
  pub struct Vec;
  impl HKT1 for Vec {
    type T<A> = ::std::vec::Vec<A>;
  }
}

impl<A> Functor<hkt::Vec, A> for Vec<A> {
  fn fmap<B>(self, f: impl F1<A, B>) -> Vec<B> {
    self.into_iter().map(|a| f.call(a)).collect()
  }
}

impl<AB> Apply<hkt::Vec, AB> for Vec<AB> {
  fn apply_clone_with<A, B, Cloner>(self,
                                    a: <hkt::Vec as HKT1>::T<A>,
                                    cloner: Cloner)
                                    -> <hkt::Vec as HKT1>::T<B>
    where AB: F1<A, B>,
          Cloner: for<'a> F1<&'a A, A>
  {
    self.into_iter()
        .map(move |f| a.iter().map(|a| f.call(cloner.call(a))).collect::<Vec<B>>())
        .flatten()
        .collect()
  }
}

impl<A> Applicative<hkt::Vec, A> for Vec<A> {
  fn pure(a: A) -> Vec<A> {
    vec![a]
  }
}

impl<A> Alt<hkt::Vec, A> for Vec<A> {
  fn alt(mut self, mut b: Self) -> Self {
    Vec::append(&mut self, &mut b);
    self
  }
}
deriving!(impl Plus<hkt::Vec, A> for Vec<A> {..Default});

deriving!(impl<A> Semigroup for Vec<A> {..Alt});
deriving!(impl<A> Monoid for Vec<A> {..Default});

impl<A> Foldable<hkt::Vec, A> for Vec<A> {
  fn foldl<B, BAB>(self, f: BAB, b: B) -> B
    where BAB: F2<B, A, B>
  {
    self.into_iter().fold(b, |b, a| f.call(b, a))
  }

  fn foldr<B, ABB>(self, f: ABB, b: B) -> B
    where ABB: F2<A, B, B>
  {
    self.into_iter().rfold(b, |b, a| f.call(a, b))
  }

  fn foldl_ref<'a, B, BAB>(&'a self, f: BAB, b: B) -> B
    where BAB: F2<B, &'a A, B>,
          A: 'a
  {
    self.iter().fold(b, |b, a| f.call(b, a))
  }

  fn foldr_ref<'a, B, ABB>(&'a self, f: ABB, b: B) -> B
    where ABB: F2<&'a A, B, B>,
          A: 'a
  {
    self.iter().rfold(b, |b, a| f.call(a, b))
  }
}

#[allow(non_camel_case_types)]
type append<T> = fn(T, Vec<T>) -> Vec<T>;

/// curried [`fn@append`] waiting for both arguments
#[allow(non_camel_case_types)]
pub type append0<T> = curry2::Curry2<append<T>, Nothing<T>, Nothing<Vec<T>>, Vec<T>>;

/// curried [`fn@append`] that has a T and is waiting for the Vec to push it to
#[allow(non_camel_case_types)]
pub type append1<T> = curry2::Curry2<append<T>, Just<T>, Nothing<Vec<T>>, Vec<T>>;

/// Append an element to a vec
pub fn append<T>(t: T, mut v: Vec<T>) -> Vec<T> {
  v.push(t);
  v
}

impl<A, B> Traversable<hkt::Vec, A, B, append1<B>> for Vec<A> {
  fn traversem1<Ap, AtoApOfB>(self, f: AtoApOfB) -> Ap::T<Vec<B>>
    where Ap: HKT1,
          Self: Foldable<hkt::Vec, A>,
          Ap::T<B>: Applicative<Ap, B> + ApplyOnce<Ap, B>,
          Ap::T<append1<B>>: Applicative<Ap, append1<B>> + ApplyOnce<Ap, append1<B>>,
          Ap::T<Vec<B>>: Applicative<Ap, Vec<B>> + ApplyOnce<Ap, Vec<B>>,
          AtoApOfB: F1<A, Ap::T<B>>,
          hkt::Vec: HKT1<T<A> = Self>
  {
    self.foldl(|ap, a| f.call(a).fmap((append as append<B>).curry()).apply1(ap),
               Ap::T::pure(vec![]))
  }

  fn traversemm<Ap, AtoApOfB>(self, f: AtoApOfB) -> Ap::T<Vec<B>>
    where Ap: HKT1,
          Self: Foldable<hkt::Vec, A>,
          B: Clone,
          Ap::T<B>: Applicative<Ap, B>,
          Ap::T<append1<B>>: Applicative<Ap, append1<B>>,
          Ap::T<Vec<B>>: Applicative<Ap, Vec<B>>,
          AtoApOfB: F1<A, Ap::T<B>>,
          hkt::Vec: HKT1<T<A> = Self>
  {
    self.foldl(|ap, a| f.call(a).fmap((append as append<B>).curry()).apply(ap),
               Ap::T::pure(vec![]))
  }
}
