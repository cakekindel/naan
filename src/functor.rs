use crate::prelude::*;

/// [`Functor`], but specialized to know at compile-time
/// that the mapping function will only be called one time.
pub trait FunctorOnce<F, A>
  where F: HKT1<T<A> = Self>
{
  /// See [`FunctorOnce`]
  fn fmap1<AB, B>(self, f: AB) -> F::T<B>
    where AB: F1Once<A, Ret = B>;
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
///     where AB: F1<A, Ret = B>
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
  /// Use a function from `A -> B` to transform an
  /// `F<A>` to an `F<B>`.
  fn fmap<AB, B>(self, f: AB) -> F::T<B>
    where AB: F1<A, Ret = B>;
}

/// [`Functor`] but with looser type constraints,
/// allowing for blanket [`Functor`] implementations
/// on types [`Equiv`]alent to `F<A>`
pub trait FunctorSurrogate<F, A>
  where F: HKT1,
        Self: Equiv<To = F::T<A>>
{
  /// Type yielded by `fmap` that is akin to `F::T<B>`.
  ///
  /// The output type may use both type parameters, or only one.
  ///
  /// The reason we allow the output to be parameterized by `AB` (the function from `A -> B`)
  /// is so that the returning type can store **the function** and defer transformation.
  ///
  /// This allows implementing lazy functors with no heap dependency (ex. [`IO`])
  type Output<AB, B>;

  /// Use a function from `A -> B` to transform something
  /// akin to `F<A>` to something akin to `F<B>`.
  fn map_<AB, B>(self, f: AB) -> Self::Output<AB, B>
    where AB: F1<A, Ret = B>,
          Self::Output<AB, B>: Equiv<To = F::T<B>>;
}
