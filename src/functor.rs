use crate::prelude::*;

/// [`Functor`], but specialized to know at compile-time
/// that the mapping function will only be called one time.
pub trait FunctorOnce<F, A>
  where F: HKT1<T<A> = Self>
{
  /// See [`FunctorOnce`]
  fn fmap1<AB, B>(self, f: AB) -> F::T<B>
    where AB: F1Once<A, B>;
}

/// Functor adds a mapping operation to generic types.
///
/// In essence, `map` allows one to lift a function of type
/// `fn(A) -> B` to some new Functor context, e.g. `fn(Option<A>) -> Option<B>`.
///
/// # Laws
/// - Invoking map with an identity function (e.g. `|a| a`) should do absolutely nothing.
///
/// ```
/// use naan::prelude::*;
///
/// #[derive(Debug, PartialEq, Eq)]
/// struct Container<T>(T);
/// struct ContainerHKT;
/// impl HKT1 for ContainerHKT {
///   type T<A> = Container<A>;
/// }
///
/// impl<A> Functor<ContainerHKT, A> for Container<A> {
///   fn fmap<AB, B>(self, f: AB) -> Container<B>
///     where AB: F1<A, B>
///   {
///     Container(f.call(self.0))
///   }
/// }
///
/// assert_eq!(Container(0u8).fmap(|n| n + 1).fmap(|n: u8| n.to_string()),
///            Container("1".to_string()))
/// ```
pub trait Functor<F, A>
  where F: HKT1<T<A> = Self>
{
  /// See [`Functor`]
  fn fmap<AB, B>(self, f: AB) -> F::T<B>
    where AB: F1<A, B>;
}
