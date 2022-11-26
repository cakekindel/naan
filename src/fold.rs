use crate::prelude::*;

/// [`Foldable`], but specialized to know at compile-time
/// that the reducing function will only be called one time.
pub trait FoldableOnce<F, A>
  where F: HKT1<T<A> = Self>
{
  /// Fold the data structure
  fn fold1<B, BAB>(self, f: BAB, b: B) -> B
    where BAB: F2Once<B, A, B>;

  /// Fold the data structure
  fn fold1_ref<'a, B, BAB>(&'a self, f: BAB, b: B) -> B
    where BAB: F2Once<B, &'a A, B>,
          A: 'a;

  /// Unwrap the data structure, using a provided default value if empty
  ///
  /// ```
  /// use naan::prelude::*;
  ///
  /// assert_eq!(Some("a").get_or("b"), "a");
  /// assert_eq!(None.get_or("b"), "b");
  /// ```
  fn get_or(self, a: A) -> A
    where Self: Sized
  {
    self.fold1(|_, a| a, a)
  }

  /// Unwrap the data structure, using [`Default::default`] if empty
  ///
  /// ```
  /// use naan::prelude::*;
  ///
  /// assert_eq!(Some(format!("a")).get_or_default(), format!("a"));
  /// assert_eq!(Option::<String>::None.get_or_default(), String::new());
  /// ```
  fn get_or_default(self) -> A
    where Self: Sized,
          A: Default
  {
    self.get_or(Default::default())
  }
}

/// Foldable represents data structures which can be collapsed by
/// starting with an initial value `B`, then a function that accumulates
/// `A`s into `B`.
///
/// # Examples
/// ## `fold` instead of `map` followed by `unwrap_or_else`
/// ```
/// use naan::prelude::*;
///
/// assert_eq!(Some(10).map(|a| i32::max(a, 20)).unwrap_or(20), 20);
///
/// assert_eq!(Some(10).foldl(i32::max, 20), 20);
/// assert_eq!(Some(10).foldl(i32::max, 1), 10);
/// assert_eq!(None.foldl(i32::max, 1), 1);
/// ```
///
/// ## Collapsing a collection to a single value
/// ```
/// use naan::prelude::*;
///
/// let ns = vec![4usize, 8, 10, 12];
///
/// let sum = ns.clone().foldl(|sum, cur| sum + cur, 0);
/// let product = ns.clone().foldl(|sum, cur| sum * cur, 1);
///
/// assert_eq!(sum, 34);
/// assert_eq!(product, 3840);
/// ```
///
/// # Picking a winning element from a collection
/// ```
/// use naan::prelude::*;
///
/// fn is_prime(n: usize) -> bool {
///   // <snip>
///   # false
/// }
///
/// let ns = vec![4usize, 8, 10, 12];
///
/// let smallest = ns.clone()
///                  .foldl(|largest: Option<usize>, cur| Some(largest.foldl(usize::min, cur)),
///                         None);
///
/// let largest = ns.clone()
///                 .foldl(|largest: Option<usize>, cur| Some(largest.foldl(usize::max, cur)),
///                        None);
///
/// let largest_prime =
///   ns.clone().foldl(|largest_p: Option<usize>, cur| {
///                      largest_p.foldl(|a: Option<usize>, p: usize| a.fmap(|a| usize::max(a, p)),
///                                      Some(cur).filter(|n| is_prime(*n)))
///                    },
///                    None);
///
/// assert_eq!(largest, Some(12));
/// assert_eq!(smallest, Some(4));
/// assert_eq!(largest_prime, None);
/// ```
pub trait Foldable<F, A>
  where F: HKT1<T<A> = Self>
{
  /// Fold the data structure from left -> right
  fn foldl<B, BAB>(self, f: BAB, b: B) -> B
    where BAB: F2<B, A, B>;

  /// Fold the data structure from right -> left
  fn foldr<B, ABB>(self, f: ABB, b: B) -> B
    where ABB: F2<A, B, B>;

  /// Fold the data structure from left -> right
  fn foldl_ref<'a, B, BAB>(&'a self, f: BAB, b: B) -> B
    where BAB: F2<B, &'a A, B>,
          A: 'a;

  /// Fold the data structure from right -> left
  fn foldr_ref<'a, B, ABB>(&'a self, f: ABB, b: B) -> B
    where ABB: F2<&'a A, B, B>,
          A: 'a;

  /// Fold the data structure, accumulating the values into a [`Monoid`].
  ///
  /// ```
  /// use naan::prelude::*;
  ///
  /// let nums = vec![0u8, 1, 2];
  ///
  /// assert_eq!(nums.fold_map(|n: u8| format!("{n}")), format!("012"));
  /// ```
  fn fold_map<AB, B>(self, f: AB) -> B
    where Self: Sized,
          AB: F1<A, B>,
          B: Monoid
  {
    self.foldl(|b, a| B::append(b, f.call(a)), B::identity())
  }

  /// Accumulate the values in the data structure into a [`Monoid`].
  ///
  /// ```
  /// use naan::prelude::*;
  ///
  /// let strings = vec![format!("a"), format!("b"), format!("c")];
  ///
  /// assert_eq!(strings.fold(), format!("abc"));
  /// ```
  fn fold(self) -> A
    where Self: Sized,
          A: Monoid
  {
    #[inline(always)]
    fn identity<T>(t: T) -> T {
      t
    }
    self.fold_map(identity)
  }

  /// [`fold`](Foldable::fold) the elements into `A` when `A` is [`Monoid`],
  /// inserting a separator between adjacent elements.
  ///
  /// ```
  /// use naan::prelude::*;
  ///
  /// let strings = vec![format!("a"), format!("b"), format!("c")];
  ///
  /// assert_eq!(strings.intercalate(format!(", ")), format!("a, b, c"));
  /// ```
  fn intercalate(self, sep: A) -> A
    where Self: Sized,
          A: Monoid + Clone
  {
    struct Acc<A> {
      is_first: bool,
      acc: A,
    }

    self.foldl(|Acc { is_first, acc }: Acc<A>, a| {
                 if is_first {
                   Acc { is_first: false,
                         acc: a }
                 } else {
                   Acc { is_first: false,
                         acc: acc.append(sep.clone()).append(a) }
                 }
               },
               Acc::<A> { is_first: true,
                          acc: A::identity() })
        .acc
  }

  /// Test if the structure contains a value `a`
  ///
  /// ```
  /// use naan::prelude::*;
  ///
  /// let strings = vec![format!("a"), format!("b"), format!("c")];
  ///
  /// assert_eq!(strings.contains(&format!("a")), true);
  /// assert_eq!(strings.contains(&format!("d")), false);
  /// ```
  fn contains<'a>(&'a self, a: &'a A) -> bool
    where &'a A: PartialEq,
          A: 'a
  {
    self.any(|cur| cur == a)
  }

  /// Test if the structure does not contain a value `a`
  ///
  /// ```
  /// use naan::prelude::*;
  ///
  /// let strings = vec![format!("a"), format!("b"), format!("c")];
  ///
  /// assert_eq!(strings.not_contains(&format!("a")), false);
  /// assert_eq!(strings.not_contains(&format!("d")), true);
  /// ```
  fn not_contains<'a>(&'a self, a: &'a A) -> bool
    where &'a A: PartialEq,
          A: 'a
  {
    !self.contains(a)
  }

  /// Test if any element in the structure satisfies a predicate `f`
  ///
  /// ```
  /// use naan::prelude::*;
  ///
  /// let strings = vec![format!("ab"), format!("cde")];
  ///
  /// assert_eq!(strings.any(|s: &String| s.len() > 2), true);
  /// ```
  fn any<'a, P>(&'a self, f: P) -> bool
    where P: F1<&'a A, bool>,
          A: 'a
  {
    self.foldl_ref(|pass: bool, cur| if !pass { f.call(cur) } else { true },
                   false)
  }

  /// Test if every element in the structure satisfies a predicate `f`
  ///
  /// ```
  /// use naan::prelude::*;
  ///
  /// let strings = vec![format!("ab"), format!("cde")];
  ///
  /// assert_eq!(strings.all(|s: &String| s.len() < 4), true);
  /// ```
  fn all<'a, P>(&'a self, f: P) -> bool
    where P: F1<&'a A, bool>,
          A: 'a
  {
    self.foldl_ref(|pass: bool, cur| if pass && f.call(cur) { true } else { false },
                   true)
  }

  /// Get the number of elements contained within the structure
  ///
  /// ```
  /// use naan::prelude::*;
  ///
  /// assert_eq!(Vec::<()>::new().length(), 0);
  /// assert_eq!(vec![(), (), ()].length(), 3);
  /// ```
  fn length(&self) -> usize {
    self.foldl_ref(|n: usize, _| n + 1, 0usize)
  }

  /// Test if the structure is empty
  ///
  /// ```
  /// use naan::prelude::*;
  ///
  /// assert_eq!(Vec::<()>::new().is_empty(), true);
  /// assert_eq!(vec![()].is_empty(), false);
  /// ```
  fn is_empty(&self) -> bool {
    self.length() == 0
  }
}
