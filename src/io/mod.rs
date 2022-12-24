use crate::prelude::*;

/// `IO.map_`
pub mod map;

/// `IO.bind_`
pub mod bind;

/// `IO.apply_`
pub mod apply;

/// `IO::suspend`
pub mod suspend;

pub use bind::*;
pub use map::*;
pub use suspend::*;

/// IO HKT
pub mod hkt {
  use crate::prelude::*;

  /// IO<A>
  pub struct IO;

  impl HKT1 for IO {
    type T<A> = super::IO<A>;
  }
}

/// A lazy computation
pub trait IOLike<A>
  where Self: Sized + Equiv<To = IO<A>>
{
  /// Execute this lazy computation
  fn exec(self) -> A;
}

impl<I, A> FunctorSurrogate<hkt::IO, A> for I where I: Equiv<To = IO<A>> + IOLike<A>
{
  type Output<AB, B> = Map<AB, A, B, I>;

  fn map_<AB, B>(self, f: AB) -> Map<AB, A, B, I>
    where AB: F1<A, Ret = B>
  {
    Map::<AB, A, B, I>::new(f, self)
  }
}

impl<I, AB, TofA> ApplySurrogate<hkt::IO, AB, TofA> for I where I: Equiv<To = IO<AB>> + IOLike<AB>
{
  type ApplyOutput<A, B> = apply::Apply<A, B, AB, TofA, Self>;

  fn apply_<A, B>(self, a: TofA) -> apply::Apply<A, B, AB, TofA, Self>
    where AB: F1Once<A, Ret = B>
  {
    apply::Apply::new(self, a)
  }
}

impl<I, A> ApplicativeSurrogate<hkt::IO, A> for I where I: Equiv<To = IO<A>> + IOLike<A>
{
  fn pure(a: A) -> IO<A> {
    IO::pure(a)
  }
}

impl<I, A> MonadSurrogate<hkt::IO, A> for I where I: Equiv<To = IO<A>> + IOLike<A>
{
  type BindOutput<B, AMB> = Bind<AMB, A, B, Self>;

  fn bind_<B, AMB>(self, f: AMB) -> Self::BindOutput<B, AMB>
    where AMB: F1<A, Ret = <hkt::IO as HKT1>::T<B>>
  {
    Bind::new(f, self)
  }
}

/// Lazy managed I/O
///
/// This structure represents the concept of
/// any & all side-effecting computations,
/// wrapping lazy IO in a monad.
///
/// ## Value Proposition
/// Managed IO allows you to wrap procedural code in an ergonomic declarative
/// interface.
///
/// For some problems, it can be valuable to express a series of computations and
/// side-effects as a "pipe" that data will eventually flow through, rather than
/// a sequence of imperative statements.
///
/// Use of IO doesn't come with any inherent performance gains or costs (due to IO
/// not relying on heap allocations or dynamic dispatch) and is a mostly stylistic choice.
///
/// ## Initializing an `IO`
/// Values already in scope that need to be wrapped in `IO` can be lifted with [`IO::pure`].
///
/// Using `IO` to defer computations until some later time can be done with [`IO::suspend`].
///
/// ## Executing an `IO`
/// `IO<A>` (and the types returned by `map_`/`bind_`/`apply_`) implement the
/// [`IOLike`] trait, which provides a function `fn exec(self) -> A`
///
/// ## `IO` in parameter and return positions
/// Use `impl IOLike<A>` instead of concrete types like `IO<A>` or `Suspend<A>`.
///
/// ```
/// use core::ops::Add;
///
/// use naan::io;
/// use naan::prelude::*;
///
/// fn add12(n: usize) -> usize {
///   n + 12
/// }
///
/// // Prefer:
/// fn foo(io: impl IOLike<usize>) -> impl IOLike<usize> {
///   io.map_(add12)
/// }
///
/// // Over:
/// fn bar(io: IO<usize>) -> io::Map<fn(usize) -> usize, usize, usize, IO<usize>> {
///   io.map_(add12)
/// }
/// ```
///
/// Most of the types you'll interact with are "surrogate"
/// types that are still conceptually "an IO," but represent different transformations:
///  - [`Suspend`] (from [`IO::suspend`])
///  - [`Map`] (from [`FunctorSurrogate::map_`])
///  - [`Bind`] (from [`MonadSurrogate::bind_`])
///  - [`Apply`] (from [`ApplySurrogate::apply_`])
///
/// `IO` has been designed with no heap allocations or dynamic dispatch,
/// and accomplishes this by progressively building a stack of structs that
/// store data & work to do.
///
/// This stack is collapsed when you call [`IOLike.exec`].
///
/// Note, that `IOLike` provides [`map_`](FunctorSurrogate), [`apply_`](ApplySurrogate), and [`bind_`](MonadSurrogate),
/// so for most usecases you don't need to think or worry about concrete types.
///
/// ```
/// use core::ops::Add;
/// use std::cell::Cell;
///
/// use naan::prelude::*;
///
/// fn get_number_from_network() -> impl IOLike<usize> {
///   IO::suspend(|()| 1111)
/// }
///
/// let x = Cell::new(0usize);
/// let lazy = IO::suspend(|()| "123").map_(|s| usize::from_str_radix(s, 10).unwrap())
///                                   .map_(|n| {
///                                     x.set(x.get() + 1);
///                                     n
///                                   })
///                                   .map_((|a, b| a + b).curry()) // 123 + _
///                                   .apply_(get_number_from_network()); // 123 + 1111
///
/// assert_eq!(x.get(), 0);
/// assert_eq!(lazy.exec(), 1234);
/// assert_eq!(x.get(), 1);
/// ```
pub struct IO<T>(T);

impl<T> IO<T> {
  /// Lift an eager value of type `T` to `IO<T>`.
  pub fn pure(t: T) -> Self {
    Self(t)
  }

  /// Store a lazy computation
  pub fn suspend<F>(f: F) -> Suspend<F>
    where F: F1Once<(), Ret = T>
  {
    Suspend(f)
  }
}

impl<A> Equiv for IO<A> {
  type To = IO<A>;
}

impl<A> IOLike<A> for IO<A> {
  fn exec(self) -> A {
    self.0
  }
}
