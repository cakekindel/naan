use crate::prelude::*;

/// [`Monad`], but specialized to know at compile-time
/// that the function contained in `AMB` will only be called one time.
pub trait MonadOnce<M, A>: Monad<M, A>
  where Self: Applicative<M, A>,
        M: HKT1<T<A> = Self>
{
  /// See [`MonadOnce`]
  fn bind1<B, AMB>(self, f: AMB) -> M::T<B>
    where AMB: F1Once<A, Ret = M::T<B>>;

  /// [`MonadOnce::bind1`] that [`Discard`]s the output of the function.
  ///
  /// For [`Result`], this allows you to easily perform side effects
  /// that may fail without dropping the data in the Result.
  ///
  /// ```
  /// use naan::prelude::*;
  ///
  /// fn log(s: &str) {
  ///   println!("{s}")
  /// }
  ///
  /// let name: Result<String, String> = Ok("hello".into());
  /// name.discard(|n: &String| {
  ///       log(n.as_str());
  ///       Ok(())
  ///     });
  /// ```
  fn discard<AMB, B>(self, f: AMB) -> M::T<A>
    where Self: Sized,
          B: Discard,
          AMB: for<'a> F1Once<&'a A, Ret = M::T<B>>,
          M::T<B>: MonadOnce<M, B>
  {
    self.bind1::<A, _>(|a| f.call1(&a).bind1::<A, _>(|_| M::T::<A>::pure(a)))
  }

  /// [`MonadOnce::discard`] with mutable access to the data
  ///
  /// ```
  /// use naan::prelude::*;
  ///
  /// fn log(s: &str) {
  ///   println!("{s}")
  /// }
  ///
  /// let name: Result<String, String> = Ok("hello, ".into());
  /// name.discard_mut(|hi: &mut String| {
  ///       log(hi.as_str());
  ///       *hi = format!("{hi}, world!");
  ///       Ok(())
  ///     })
  ///     .discard(|n: &String| {
  ///       log(n.as_str());
  ///       Ok(())
  ///     });
  /// ```
  fn discard_mut<AMB, B>(self, f: AMB) -> M::T<A>
    where Self: Sized,
          B: Discard,
          AMB: for<'a> F1Once<&'a mut A, Ret = M::T<B>>,
          M::T<B>: MonadOnce<M, B>
  {
    self.bind1::<A, _>(|mut a| f.call1(&mut a).bind1::<A, _>(|_| M::T::<A>::pure(a)))
  }
}

/// `Monad` generalizes the concept of a *sequence of computations*,
/// using the return value of one to determine the next.
///
/// For [`Result`](Result::and_then) and [`Option`](Option::and_then), `bind` is an alias for their
/// `and_then` inherent methods.
///
/// ```no_run
/// use std::fs::{File, OpenOptions};
/// use std::io::{Read, Write};
///
/// use naan::prelude::*;
///
/// OpenOptions::new().append(true)
///                   .open("./README.txt")
///                   .bind(|mut file: File| {
///                     write!(file, "\n## New Heading\n\nHello from example!").fmap1(|_| file)
///                   })
///                   .bind(|mut file: File| {
///                     let mut s = String::new();
///                     file.read_to_string(&mut s).fmap1(|_| s)
///                   });
/// ```
pub trait Monad<M, A>
  where Self: Applicative<M, A>,
        M: HKT1<T<A> = Self>
{
  /// See [`Monad`]
  fn bind<B, AMB>(self, f: AMB) -> M::T<B>
    where AMB: F1<A, Ret = M::T<B>>;

  /// Flatten a nested `Monad`
  /// ```
  /// use naan::prelude::*;
  ///
  /// assert_eq!(Some(Some("hello")).flatten(), Some("hello"));
  /// ```
  fn flatten<AA>(self) -> M::T<AA>
    where Self: Sized,
          M: HKT1<T<AA> = A> + HKT1<T<<M as HKT1>::T<AA>> = Self>
  {
    self.bind::<AA, _>(|s| s)
  }
}

/// [`Monad`] but with looser type constraints,
/// allowing for blanket [`Monad`] implementations
/// on types [`Equiv`]alent to `M<A>`
pub trait MonadSurrogate<M, A>
  where Self: Equiv<To = M::T<A>> + ApplicativeSurrogate<M, A>,
        M: HKT1
{
  /// Type yielded by `bind_` that isn't _exactly_ `M::T<B>`, but
  /// is conceptually [`Equiv`]alent.
  ///
  /// The output type may use both type parameters, or only one.
  ///
  /// The reason we allow the output to be parameterized by `AMB` (the function from `A -> Self<B>`)
  /// is so that the returning type can store **the function** and defer transformation.
  ///
  /// This allows implementing lazy monads with no heap dependency (ex. [`IO`])
  type BindOutput<B, AMB>;

  /// Use a function from `A -> M<B>` to transform something
  /// akin to `M<A>` to something akin to `M<B>`.
  fn bind_<B, AMB>(self, f: AMB) -> Self::BindOutput<B, AMB>
    where AMB: F1<A, Ret = M::T<B>>,
          Self::BindOutput<B, AMB>: Equiv<To = M::T<B>>;
}
