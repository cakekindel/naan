use crate::prelude::*;

/// [`Monad`], but specialized to know at compile-time
/// that the function contained in `AMB` will only be called one time.
pub trait MonadOnce<M, A>: Monad<M, A>
  where Self: Applicative<M, A>,
        M: HKT1<T<A> = Self>
{
  /// See [`MonadOnce`]
  fn bind1<B, AMB>(self, f: AMB) -> M::T<B>
    where AMB: F1Once<A, M::T<B>>;
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
    where AMB: F1<A, M::T<B>>;

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
