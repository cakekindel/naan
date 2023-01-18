use std_alloc::collections::BTreeMap;

use crate::prelude::*;

/// HKTs
pub mod hkt {
  use core::marker::PhantomData;

  use super::*;

  /// [`std::collections::BTreeMap`] lifted to an HKT2
  ///
  /// (Kind `Type -> Type -> Type`)
  pub struct BTreeMap;

  impl HKT2 for BTreeMap {
    type T<K, V> = std_alloc::collections::BTreeMap<K, V>;
  }

  /// [`std::collections::BTreeMap`] lifted to an HKT1
  /// with a fixed key type
  ///
  /// (Kind `Type -> Type`)
  pub struct BTreeMapValues<K>(PhantomData<K>);

  impl<K> HKT1 for BTreeMapValues<K> where K: Ord
  {
    type T<A> = std_alloc::collections::BTreeMap<K, A>;
  }
}

impl<K, A> Functor<hkt::BTreeMapValues<K>, A> for BTreeMap<K, A> where K: Ord
{
  fn fmap<AB, B>(self, f: AB) -> BTreeMap<K, B>
    where AB: F1<A, Ret = B>
  {
    self.into_iter().map(|(k, a)| (k, f.call(a))).collect()
  }
}

impl<K, AB> Apply<hkt::BTreeMapValues<K>, AB> for BTreeMap<K, AB> where K: Ord
{
  fn apply_with<A, B, Cloner>(self, as_: BTreeMap<K, A>, cloner: Cloner) -> BTreeMap<K, B>
    where AB: F1<A, Ret = B>,
          Cloner: for<'a> F1<&'a A, Ret = A>
  {
    self.into_iter()
        .filter_map(move |(k, f)| as_.get(&k).map(|a| f.call(cloner.call(a))).map(|b| (k, b)))
        .collect()
  }
}

impl<K, A> Alt<hkt::BTreeMapValues<K>, A> for BTreeMap<K, A> where K: Ord
{
  /// Combine the two maps, preferring keys from `self` when self
  /// and `b` both have an entry for a given key.
  ///
  /// ```
  /// use std::collections::BTreeMap;
  ///
  /// use naan::prelude::*;
  ///
  /// let a_union_b = BTreeMap::from([("a", 1), ("b", 2)]).alt(BTreeMap::from([("b", 3), ("c", 3)]));
  /// assert_eq!(a_union_b, BTreeMap::from([("a", 1), ("b", 2), ("c", 3)]))
  /// ```
  fn alt(self, b: Self) -> Self {
    b.into_iter().chain(self.into_iter()).collect()
  }
}

impl<K, A> Plus<hkt::BTreeMapValues<K>, A> for BTreeMap<K, A> where K: Ord
{
  fn empty() -> <hkt::BTreeMapValues<K> as HKT1>::T<A> {
    Default::default()
  }
}

impl<K, A> Semigroup for BTreeMap<K, A> where K: Ord
{
  fn append(self, b: Self) -> Self {
    self.alt(b)
  }
}

impl<K, A> Monoid for BTreeMap<K, A> where K: Ord
{
  fn identity() -> Self {
    Self::empty()
  }
}

impl<A, K> Foldable<hkt::BTreeMapValues<K>, A> for BTreeMap<K, A> where K: Ord
{
  fn foldl<B, BAB>(self, f: BAB, b: B) -> B
    where BAB: F2<B, A, Ret = B>
  {
    self.into_iter().fold(b, |b, (_, a)| f.call(b, a))
  }

  fn foldr<B, ABB>(self, f: ABB, b: B) -> B
    where ABB: F2<A, B, Ret = B>
  {
    self.into_iter().fold(b, |b, (_, a)| f.call(a, b))
  }

  fn foldl_ref<'a, B, BAB>(&'a self, f: BAB, b: B) -> B
    where BAB: F2<B, &'a A, Ret = B>,
          A: 'a
  {
    self.iter().fold(b, |b, (_, a)| f.call(b, a))
  }

  fn foldr_ref<'a, B, ABB>(&'a self, f: ABB, b: B) -> B
    where ABB: F2<&'a A, B, Ret = B>,
          A: 'a
  {
    self.iter().fold(b, |b, (_, a)| f.call(a, b))
  }
}

impl<A, K> FoldableIndexed<hkt::BTreeMapValues<K>, K, A> for BTreeMap<K, A> where K: Ord + Clone
{
  fn foldl_idx<B, BAB>(self, f: BAB, b: B) -> B
    where BAB: F3<B, K, A, Ret = B>
  {
    self.into_iter().fold(b, |b, (k, a)| f.call(b, k, a))
  }

  fn foldr_idx<B, ABB>(self, f: ABB, b: B) -> B
    where ABB: F3<K, A, B, Ret = B>
  {
    self.into_iter().fold(b, |b, (k, a)| f.call(k, a, b))
  }

  fn foldl_idx_ref<'a, B, BAB>(&'a self, f: BAB, b: B) -> B
    where BAB: F3<B, K, &'a A, Ret = B>,
          A: 'a
  {
    self.iter().fold(b, |b, (k, a)| f.call(b, k.clone(), a))
  }

  fn foldr_idx_ref<'a, B, ABB>(&'a self, f: ABB, b: B) -> B
    where ABB: F3<K, &'a A, B, Ret = B>,
          A: 'a
  {
    self.iter().fold(b, |b, (k, a)| f.call(k.clone(), a, b))
  }
}

#[allow(non_camel_case_types)]
type insert<K, A> = fn(K, A, BTreeMap<K, A>) -> BTreeMap<K, A>;

/// curried [`fn@insert`] waiting for all arguments
#[allow(non_camel_case_types)]
pub type insert0<K, A> = curry3::Applied0<insert<K, A>, K, A, BTreeMap<K, A>, BTreeMap<K, A>>;

/// curried [`fn@insert`] partially applied with 1 argument
#[allow(non_camel_case_types)]
pub type insert1<K, A> = curry3::Applied1<insert<K, A>, K, A, BTreeMap<K, A>, BTreeMap<K, A>>;

/// curried [`fn@insert`] partially applied with 2 arguments
#[allow(non_camel_case_types)]
pub type insert2<K, A> = curry3::Applied2<insert<K, A>, K, A, BTreeMap<K, A>, BTreeMap<K, A>>;

/// Insert an item into a hash map
pub fn insert<K, A>(k: K, a: A, mut map: BTreeMap<K, A>) -> BTreeMap<K, A>
  where K: Ord
{
  map.insert(k, a);
  map
}

impl<K, A, B> Traversable<hkt::BTreeMapValues<K>, A, B, insert2<K, B>> for BTreeMap<K, A>
  where K: Clone + Ord,
        hkt::BTreeMapValues<K>: HKT1<T<B> = BTreeMap<K, B>>
{
  fn traversem1<Ap, AtoApOfB>(self, f: AtoApOfB) -> Ap::T<BTreeMap<K, B>>
    where Ap: HKT1,
          Self: Foldable<hkt::BTreeMapValues<K>, A>,
          Ap::T<B>: Applicative<Ap, B> + ApplyOnce<Ap, B>,
          Ap::T<insert2<K, B>>: Applicative<Ap, insert2<K, B>> + ApplyOnce<Ap, insert2<K, B>>,
          Ap::T<BTreeMap<K, B>>: Applicative<Ap, BTreeMap<K, B>> + ApplyOnce<Ap, BTreeMap<K, B>>,
          AtoApOfB: F1<A, Ret = Ap::T<B>>,
          hkt::BTreeMapValues<K>: HKT1<T<A> = Self>
  {
    self.foldl_idx(|ap, k, a| {
                     let insert = (insert as insert<K, B>).curry().call(k);
                     f.call(a).fmap(insert).apply1(ap)
                   },
                   Ap::T::pure(BTreeMap::<K, B>::empty()))
  }

  fn traversemm<Ap, AtoApOfB>(self, f: AtoApOfB) -> Ap::T<BTreeMap<K, B>>
    where Ap: HKT1,
          Self: Foldable<hkt::BTreeMapValues<K>, A>,
          B: Clone,
          Ap::T<B>: Applicative<Ap, B>,
          Ap::T<insert2<K, B>>: Applicative<Ap, insert2<K, B>>,
          Ap::T<BTreeMap<K, B>>: Applicative<Ap, BTreeMap<K, B>>,
          AtoApOfB: F1<A, Ret = Ap::T<B>>,
          hkt::BTreeMapValues<K>: HKT1<T<A> = Self>
  {
    self.foldl_idx(|ap, k, a| {
                     let insert = (insert as insert<K, B>).curry().call(k);
                     f.call(a).fmap(insert).apply(ap)
                   },
                   Ap::T::pure(BTreeMap::<K, B>::empty()))
  }
}
