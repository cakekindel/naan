use std_alloc::vec;
use std_alloc::vec::Vec;

use crate::prelude::*;

/// Vec Kinds
pub mod hkt {
  use super::*;

  /// [`std::vec::Vec`] lifted to an HKT1
  ///
  /// (Kind `Type -> Type`)
  pub struct Vec;
  impl HKT1 for Vec {
    type T<A> = ::std_alloc::vec::Vec<A>;
  }
}

impl<A> Functor<hkt::Vec, A> for Vec<A> {
  fn fmap<AB, B>(self, f: AB) -> Vec<B>
    where AB: F1<A, Ret = B>
  {
    self.into_iter().map(|a| f.call(a)).collect()
  }
}

impl<AB> Apply<hkt::Vec, AB> for Vec<AB> {
  fn apply_with<A, B, Cloner>(self,
                              a: <hkt::Vec as HKT1>::T<A>,
                              cloner: Cloner)
                              -> <hkt::Vec as HKT1>::T<B>
    where AB: F1<A, Ret = B>,
          Cloner: for<'a> F1<&'a A, Ret = A>
  {
    self.into_iter()
        .flat_map(move |f| a.iter().map(|a| f.call(cloner.call(a))).collect::<Vec<B>>())
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

impl<A> FoldableIndexed<hkt::Vec, usize, A> for Vec<A> {
  fn foldl_idx<B, BAB>(self, f: BAB, b: B) -> B
    where BAB: F3<B, usize, A, Ret = B>
  {
    self.into_iter()
        .enumerate()
        .fold(b, |b, (ix, a)| f.call(b, ix, a))
  }

  /// CHECK: Enumerate yields _indexes_ (not the number of iterations) when
  /// going backwards in a double ended iterator
  ///
  /// ```
  /// use naan::prelude::*;
  ///
  /// vec![0, 1, 2].foldl_idx(|(), ix, val| assert_eq!(ix, val), ());
  /// vec![0, 1, 2].foldr_idx(|ix, val, ()| assert_eq!(ix, val), ());
  /// ```
  fn foldr_idx<B, ABB>(self, f: ABB, b: B) -> B
    where ABB: F3<usize, A, B, Ret = B>
  {
    self.into_iter()
        .enumerate()
        .rfold(b, |b, (ix, a)| f.call(ix, a, b))
  }

  fn foldl_idx_ref<'a, B, BAB>(&'a self, f: BAB, b: B) -> B
    where BAB: F3<B, usize, &'a A, Ret = B>,
          A: 'a
  {
    self.iter()
        .enumerate()
        .fold(b, |b, (ix, a)| f.call(b, ix, a))
  }

  fn foldr_idx_ref<'a, B, ABB>(&'a self, f: ABB, b: B) -> B
    where ABB: F3<usize, &'a A, B, Ret = B>,
          A: 'a
  {
    self.iter()
        .enumerate()
        .rfold(b, |b, (ix, a)| f.call(ix, a, b))
  }
}
deriving!(impl Foldable<hkt::Vec, A> for Vec<A> {..FoldableIndexed});

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
          AtoApOfB: F1<A, Ret = Ap::T<B>>,
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
          AtoApOfB: F1<A, Ret = Ap::T<B>>,
          hkt::Vec: HKT1<T<A> = Self>
  {
    self.foldl(|ap, a| f.call(a).fmap((append as append<B>).curry()).apply(ap),
               Ap::T::pure(vec![]))
  }
}

impl<A> Monad<hkt::Vec, A> for Vec<A> {
  fn bind<B, AMB>(self, f: AMB) -> Vec<B>
    where AMB: F1<A, Ret = Vec<B>>
  {
    let mut out = Vec::<B>::new();

    for i in self {
      Vec::append(&mut out, &mut f.call(i));
    }

    out
  }
}
