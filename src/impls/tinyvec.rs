use std::convert::identity as id;

use tinyvec::ArrayVec;

use crate::prelude::*;

/// Tinyvec HKTs
pub mod hkt {
  use super::*;

  /// [`tinyvec::ArrayVec`] lifted to an HKT1
  ///
  /// (Kind `Type -> Type`)
  pub struct ArrayVec<const N: usize>;
  impl<const N: usize> HKT1 for ArrayVec<N> {
    type T<A> = tinyvec::ArrayVec<[Option<A>; N]>;
  }
}

impl<const N: usize, A> Functor<hkt::ArrayVec<N>, A> for ArrayVec<[Option<A>; N]> {
  fn fmap<AB, B>(self, f: AB) -> ArrayVec<[Option<B>; N]>
    where AB: F1<A, Ret = B>
  {
    self.into_iter().map(|a| a.fmap(|a| f.call(a))).collect()
  }
}

impl<const N: usize, AB> Apply<hkt::ArrayVec<N>, AB> for ArrayVec<[Option<AB>; N]> {
  fn apply_with<A, B, Cloner>(self,
                              as_: ArrayVec<[Option<A>; N]>,
                              clone: Cloner)
                              -> ArrayVec<[Option<B>; N]>
    where AB: F1<A, Ret = B>,
          Cloner: for<'a> F1<&'a A, Ret = A>
  {
    self.into_iter()
        .filter_map(|atob| atob)
        .map(|atob| {
          as_.iter()
             .map(|a| {
               a.as_ref()
                .fmap(|a_ref| clone.call(a_ref))
                .fmap(|a| atob.call(a))
             })
             .collect::<ArrayVec<[Option<B>; N]>>()
        })
        .flatten()
        .collect()
  }
}

impl<A, const N: usize> Applicative<hkt::ArrayVec<N>, A> for ArrayVec<[Option<A>; N]> {
  fn pure(a: A) -> Self {
    tinyvec::array_vec!(_ => Some(a))
  }
}

impl<const N: usize, A> Alt<hkt::ArrayVec<N>, A> for ArrayVec<[Option<A>; N]> {
  fn alt(mut self, mut b: Self) -> Self {
    ArrayVec::append(&mut self, &mut b);
    self
  }
}

impl<A, const N: usize> Plus<hkt::ArrayVec<N>, A> for ArrayVec<[Option<A>; N]> {
  fn empty() -> Self {
    Default::default()
  }
}

impl<A, const N: usize> Semigroup for ArrayVec<[Option<A>; N]> {
  fn append(self, b: Self) -> Self {
    self.alt(b)
  }
}

impl<A, const N: usize> Monoid for ArrayVec<[Option<A>; N]> {
  fn identity() -> Self {
    Self::empty()
  }
}

impl<A, const N: usize> Foldable<hkt::ArrayVec<N>, A> for ArrayVec<[Option<A>; N]> {
  fn foldl<B, BAB>(self, f: BAB, b: B) -> B
    where BAB: F2<B, A, Ret = B>
  {
    self.into_iter().filter_map(id).fold(b, |b, a| f.call(b, a))
  }

  fn foldr<B, ABB>(self, f: ABB, b: B) -> B
    where ABB: F2<A, B, Ret = B>
  {
    self.into_iter()
        .filter_map(id)
        .rfold(b, |b, a| f.call(a, b))
  }

  fn foldl_ref<'a, B, BAB>(&'a self, f: BAB, b: B) -> B
    where BAB: F2<B, &'a A, Ret = B>,
          A: 'a
  {
    self.iter()
        .filter_map(Option::as_ref)
        .fold(b, |b, a| f.call(b, a))
  }

  fn foldr_ref<'a, B, ABB>(&'a self, f: ABB, b: B) -> B
    where ABB: F2<&'a A, B, Ret = B>,
          A: 'a
  {
    self.iter()
        .filter_map(Option::as_ref)
        .rfold(b, |b, a| f.call(a, b))
  }
}

#[allow(non_camel_case_types)]
type append<const N: usize, T> = fn(T, ArrayVec<[Option<T>; N]>) -> ArrayVec<[Option<T>; N]>;

/// curried [`fn@append`] waiting for both arguments
#[allow(non_camel_case_types)]
pub type append0<const N: usize, T> = curry2::Curry2<append<N, T>,
                                                     Nothing<T>,
                                                     Nothing<ArrayVec<[Option<T>; N]>>,
                                                     ArrayVec<[Option<T>; N]>>;

/// curried [`fn@append`] that has a T and is waiting for the Vec to push it to
#[allow(non_camel_case_types)]
pub type append1<const N: usize, T> = curry2::Curry2<append<N, T>,
                                                     Just<T>,
                                                     Nothing<ArrayVec<[Option<T>; N]>>,
                                                     ArrayVec<[Option<T>; N]>>;

/// Append an element to a vec
pub fn append<const N: usize, T>(t: T,
                                 mut v: ArrayVec<[Option<T>; N]>)
                                 -> ArrayVec<[Option<T>; N]> {
  v.push(Some(t));
  v
}

impl<A, B, const N: usize> Traversable<hkt::ArrayVec<N>, A, B, append1<N, B>>
  for ArrayVec<[Option<A>; N]> where hkt::ArrayVec<N>: HKT1<T<B> = ArrayVec<[Option<B>; N]>>
{
  fn traversem1<Ap, AtoApOfB>(self, f: AtoApOfB) -> Ap::T<ArrayVec<[Option<B>; N]>>
    where Ap: HKT1,
          Self: Foldable<hkt::ArrayVec<N>, A>,
          Ap::T<B>: Applicative<Ap, B> + ApplyOnce<Ap, B>,
          Ap::T<append1<N, B>>: Applicative<Ap, append1<N, B>> + ApplyOnce<Ap, append1<N, B>>,
          Ap::T<ArrayVec<[Option<B>; N]>>:
            Applicative<Ap, ArrayVec<[Option<B>; N]>> + ApplyOnce<Ap, ArrayVec<[Option<B>; N]>>,
          AtoApOfB: F1<A, Ret = Ap::T<B>>,
          hkt::ArrayVec<N>: HKT1<T<A> = Self>
  {
    self.foldl(|ap, a| f.call(a).fmap((append as append<N, B>).curry()).apply1(ap),
               Ap::T::pure(ArrayVec::<[Option<B>; N]>::identity()))
  }

  fn traversemm<Ap, AtoApOfB>(self, f: AtoApOfB) -> Ap::T<ArrayVec<[Option<B>; N]>>
    where Ap: HKT1,
          Self: Foldable<hkt::ArrayVec<N>, A>,
          B: Clone,
          Ap::T<B>: Applicative<Ap, B>,
          Ap::T<append1<N, B>>: Applicative<Ap, append1<N, B>>,
          Ap::T<ArrayVec<[Option<B>; N]>>: Applicative<Ap, ArrayVec<[Option<B>; N]>>,
          AtoApOfB: F1<A, Ret = Ap::T<B>>,
          hkt::ArrayVec<N>: HKT1<T<A> = Self>
  {
    self.foldl(|ap, a| f.call(a).fmap((append as append<N, B>).curry()).apply(ap),
               Ap::T::pure(ArrayVec::<[Option<B>; N]>::identity()))
  }
}

impl<A, const N: usize> Monad<hkt::ArrayVec<N>, A> for ArrayVec<[Option<A>; N]> {
  fn bind<B, AMB>(self, f: AMB) -> ArrayVec<[Option<B>; N]>
    where AMB: F1<A, Ret = ArrayVec<[Option<B>; N]>>
  {
    let mut out = ArrayVec::empty();

    for i in self {
      if let Some(i) = i {
        ArrayVec::append(&mut out, &mut f.call(i));
      }
    }

    out
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn tinyvec() {
    type V = tinyvec::ArrayVec<[Option<u32>; 32]>;

    let v = V::empty();
    assert_eq!(v, V::default());

    let v = v.append(V::pure(1)).append(V::pure(2)).append(V::pure(3));
    assert_eq!(v, tinyvec::array_vec!(_ => Some(1), Some(2), Some(3)));

    let sum = v.clone().foldl(|sum, n| sum + n, 0);
    assert_eq!(sum, 6);

    type R = Result<u32, ()>;
    type RV = tinyvec::ArrayVec<[Option<R>; 32]>;

    let rv = RV::empty().append(RV::pure(R::Ok(1)))
                        .append(RV::pure(R::Ok(2)));
    assert_eq!(rv.sequence::<crate::hkt::ResultOk<()>>(),
               Result::<V, ()>::Ok(tinyvec::array_vec!(_ => Some(1), Some(2))));
  }
}
