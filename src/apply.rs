use crate::prelude::*;

/// [`Apply`], but specialized to know at compile-time
/// that the function contained in `F` will only be called one time.
pub trait ApplyOnce<F, AB>
  where Self: Functor<F, AB>,
        F: HKT1<T<AB> = Self>
{
  /// See [`ApplyOnce`]
  fn apply1<A, B>(self, a: F::T<A>) -> F::T<B>
    where AB: F1Once<A, B>;
}

/// `Apply` generalizes [`Functor`] to any arity.
///
/// In other words, you can lift a function
///
/// `A -> B -> C -> D`
///
/// to
///
/// `F<A> -> F<B> -> F<C> -> F<D>`.
///
/// # Example
/// ```
/// use naan::prelude::*;
///
/// fn maybe_n() -> Option<usize> {
///   Some(1)
/// }
///
/// fn maybe_string() -> Option<String> {
///   None
/// }
///
/// fn maybe_bytes() -> Option<Vec<u8>> {
///   None
/// }
///
/// fn combine_many_things(n: usize, string: String, bytes: Vec<u8>) {
///   // ...
/// }
///
/// # fn a() {
/// Some(combine_many_things.curry()).apply(maybe_n())
///                                  .apply(maybe_string())
///                                  .apply(maybe_bytes());
///
/// // with `if let`:
/// if let Some(n) = maybe_n() {
///   if let Some(s) = maybe_string() {
///     if let Some(bs) = maybe_bytes() {
///       combine_many_things(n, s, bs);
///     }
///   }
/// }
///
/// // with `let else`:
/// let Some(n) = maybe_n() else {return};
/// let Some(s) = maybe_string() else {return};
/// let Some(bs) = maybe_bytes() else {return};
/// combine_many_things(n, s, bs);
///
/// // with `match`:
/// match (maybe_n(), maybe_string(), maybe_bytes()) {
///   | (Some(n), Some(s), Some(bs)) => combine_many_things(n, s, bs),
///   | _ => (),
/// }
/// # }
/// ```
pub trait Apply<F, AB>
  where Self: Functor<F, AB>,
        F: HKT1<T<AB> = Self>
{
  /// See [`Apply`]
  fn apply<A, B>(self, a: F::T<A>) -> F::T<B>
    where Self: Sized,
          AB: F1<A, B>,
          A: Clone
  {
    self.apply_clone_with(a, Clone::clone)
  }

  /// See [`Apply`]
  fn apply_clone_with<A, B, Cloner>(self, a: F::T<A>, cloner: Cloner) -> F::T<B>
    where AB: F1<A, B>,
          Cloner: for<'a> F1<&'a A, A>;
}

/// Adds onto [`Apply`] the ability to lift a _value_
/// to the `F` context.
pub trait Applicative<F, A>
  where Self: Apply<F, A>,
        F: HKT1<T<A> = Self>
{
  /// Lift `A` to `F<A>`
  fn pure(a: A) -> F::T<A>;

  /// Append a single `A` to `F<A>`
  fn append_one(self, a: A) -> Self
    where Self: Sized + Semigroup
  {
    self.append(Self::pure(a))
  }
}
