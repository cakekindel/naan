use core::hash::Hash;
use std::collections::HashMap;

use crate::prelude::*;

/// HKTs
pub mod hkt {
  use core::marker::PhantomData;

  use super::*;

  /// [`std::collections::HashMap`] lifted to an HKT2
  ///
  /// (Kind `Type -> Type -> Type`)
  pub struct HashMap;

  impl HKT2 for HashMap {
    type T<K, A> = std::collections::HashMap<K, A>;
  }

  /// [`std::collections::HashMap`] lifted to an HKT1
  /// with a fixed key type
  ///
  /// (Kind `Type -> Type`)
  pub struct HashMapValues<K>(PhantomData<K>);

  impl<K> HKT1 for HashMapValues<K> where K: Hash + Eq
  {
    type T<A> = std::collections::HashMap<K, A>;
  }
}

impl<K, A> Functor<hkt::HashMapValues<K>, A> for HashMap<K, A> where K: Hash + Eq
{
  fn fmap<AB, B>(self, f: AB) -> HashMap<K, B>
    where AB: F1<A, Ret = B>
  {
    self.into_iter().map(|(k, a)| (k, f.call(a))).collect()
  }
}

impl<K, AB> Apply<hkt::HashMapValues<K>, AB> for HashMap<K, AB> where K: Eq + Hash
{
  fn apply_with<A, B, Cloner>(self, as_: HashMap<K, A>, cloner: Cloner) -> HashMap<K, B>
    where AB: F1<A, Ret = B>,
          Cloner: for<'a> F1<&'a A, Ret = A>
  {
    self.into_iter()
        .filter_map(move |(k, f)| as_.get(&k).map(|a| f.call(cloner.call(a))).map(|b| (k, b)))
        .collect()
  }
}

impl<K, A> Alt<hkt::HashMapValues<K>, A> for HashMap<K, A> where K: Eq + Hash
{
  /// Combine the two maps, preferring keys from `self` when self
  /// and `b` both have an entry for a given key.
  ///
  /// ```
  /// use std::collections::HashMap;
  ///
  /// use naan::prelude::*;
  ///
  /// let a_union_b = HashMap::from([("a", 1), ("b", 2)]).alt(HashMap::from([("b", 3), ("c", 3)]));
  /// assert_eq!(a_union_b, HashMap::from([("a", 1), ("b", 2), ("c", 3)]))
  /// ```
  fn alt(self, b: Self) -> Self {
    b.into_iter().chain(self.into_iter()).collect()
  }
}

impl<K, A> Plus<hkt::HashMapValues<K>, A> for HashMap<K, A> where K: Eq + Hash
{
  fn empty() -> <hkt::HashMapValues<K> as HKT1>::T<A> {
    Default::default()
  }
}

impl<K, A> Semigroup for HashMap<K, A> where K: Eq + Hash
{
  fn append(self, b: Self) -> Self {
    self.alt(b)
  }
}

impl<K, A> Monoid for HashMap<K, A> where K: Eq + Hash
{
  fn identity() -> Self {
    Self::empty()
  }
}

impl<A, K> Foldable<hkt::HashMapValues<K>, A> for HashMap<K, A> where K: Eq + Hash
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

impl<A, K> FoldableIndexed<hkt::HashMapValues<K>, K, A> for HashMap<K, A> where K: Eq + Hash + Clone
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
type insert<K, A> = fn(K, A, HashMap<K, A>) -> HashMap<K, A>;

/// curried [`fn@insert`] waiting for all arguments
#[allow(non_camel_case_types)]
pub type insert0<K, A> = curry3::Applied0<insert<K, A>, K, A, HashMap<K, A>, HashMap<K, A>>;

/// curried [`fn@insert`] partially applied with 1 argument
#[allow(non_camel_case_types)]
pub type insert1<K, A> = curry3::Applied1<insert<K, A>, K, A, HashMap<K, A>, HashMap<K, A>>;

/// curried [`fn@insert`] partially applied with 2 arguments
#[allow(non_camel_case_types)]
pub type insert2<K, A> = curry3::Applied2<insert<K, A>, K, A, HashMap<K, A>, HashMap<K, A>>;

/// Insert an item into a hash map
pub fn insert<K, A>(k: K, a: A, mut map: HashMap<K, A>) -> HashMap<K, A>
  where K: Eq + Hash
{
  map.insert(k, a);
  map
}

impl<K, A, B> Traversable<hkt::HashMapValues<K>, A, B, insert2<K, B>> for HashMap<K, A>
  where K: Clone + Eq + Hash,
        hkt::HashMapValues<K>: HKT1<T<B> = HashMap<K, B>>
{
  fn traversem1<Ap, AtoApOfB>(self, f: AtoApOfB) -> Ap::T<HashMap<K, B>>
    where Ap: HKT1,
          Self: Foldable<hkt::HashMapValues<K>, A>,
          Ap::T<B>: Applicative<Ap, B> + ApplyOnce<Ap, B>,
          Ap::T<insert2<K, B>>: Applicative<Ap, insert2<K, B>> + ApplyOnce<Ap, insert2<K, B>>,
          Ap::T<HashMap<K, B>>: Applicative<Ap, HashMap<K, B>> + ApplyOnce<Ap, HashMap<K, B>>,
          AtoApOfB: F1<A, Ret = Ap::T<B>>,
          hkt::HashMapValues<K>: HKT1<T<A> = Self>
  {
    self.foldl_idx(|ap, k, a| {
                     let insert = (insert as insert<K, B>).curry().call(k);
                     f.call(a).fmap(insert).apply1(ap)
                   },
                   Ap::T::pure(HashMap::<K, B>::empty()))
  }

  fn traversemm<Ap, AtoApOfB>(self, f: AtoApOfB) -> Ap::T<HashMap<K, B>>
    where Ap: HKT1,
          Self: Foldable<hkt::HashMapValues<K>, A>,
          B: Clone,
          Ap::T<B>: Applicative<Ap, B>,
          Ap::T<insert2<K, B>>: Applicative<Ap, insert2<K, B>>,
          Ap::T<HashMap<K, B>>: Applicative<Ap, HashMap<K, B>>,
          AtoApOfB: F1<A, Ret = Ap::T<B>>,
          hkt::HashMapValues<K>: HKT1<T<A> = Self>
  {
    self.foldl_idx(|ap, k, a| {
                     let insert = (insert as insert<K, B>).curry().call(k);
                     f.call(a).fmap(insert).apply(ap)
                   },
                   Ap::T::pure(HashMap::<K, B>::empty()))
  }
}
