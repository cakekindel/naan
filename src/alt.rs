use crate::prelude::*;

/// An associative operation for a type with 1 generic parameter.
///
/// Similar to [`Semigroup`], but for generic types.
///
/// # Examples
/// - `Vec`'s Alt implementation concatenates the two Vecs.
/// - `Option`'s Alt implementation yields the first encountered Some.
/// - `Result`'s Alt implementation yields the first encountered Ok.
///
/// # Laws
/// * Associative (`a.alt(b.alt(c)) == a.alt(b).alt(c)`)
/// * Distributive (`a.alt(b).map(foo) == a.map(foo).alt(b.map(foo))`)
pub trait Alt<F, A>
  where Self: Functor<F, A>,
        F: HKT1<T<A> = Self>
{
  /// See [`Alt`]
  fn alt(self, b: Self) -> Self;
}

/// Plus adds an identity (empty) value to [`Alt`].
///
/// # Laws
/// The Plus identity value must be a no-op on either side
/// of an [`alt`](Alt.alt), e.g.
///
/// ```
/// use naan::prelude::*;
///
/// assert_eq!(Option::pure("hello").alt(Option::empty()),
///            Option::pure("hello"));
///
/// // Generalized:
/// fn assert_empty_is_identity<F, A, T>(t: T)
///   where T: PartialEq + core::fmt::Debug + Clone + Plus<F, A>,
///         F: HKT1<T<A> = T>
/// {
///   assert_eq!(t.clone().alt(T::empty()), t.clone());
///   assert_eq!(T::empty().alt(t.clone()), t);
/// }
///
/// assert_empty_is_identity(Some("hello"));
/// assert_empty_is_identity(vec!["hello"]);
/// ```
pub trait Plus<F, A>
  where Self: Alt<F, A>,
        F: HKT1<T<A> = Self>
{
  /// See [`Plus`]
  fn empty() -> F::T<A>;
}
