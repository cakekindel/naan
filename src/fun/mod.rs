use core::marker::PhantomData;
use core::ops::Deref;

use curry2::Curry2;
use curry3::Curry3;

use self::compose::Compose;

/// Function composition
pub mod compose;

/// Currying functions with 2 arguments
pub mod curry2;

/// Currying functions with 3 arguments
pub mod curry3;

/// Create a tuple
///
/// ```
/// use naan::prelude::*;
///
/// let a = Some("a");
/// let b = Some(2);
///
/// let tup: Option<(&'static str, usize)> = a.and_then(|a| b.map(|b| (a, b)));
/// let tup: Option<(&'static str, usize)> = Some(tuple2.curry()).apply1(a).apply1(b);
/// ```
pub fn tuple2<A, B>(a: A, b: B) -> (A, B) {
  (a, b)
}

/// Create a 3-tuple
///
/// ```
/// use naan::prelude::*;
///
/// let a = Some("a");
/// let b = Some(2);
/// let c = Some([1, 2, 3]);
///
/// let tup: Option<(&'static str, usize, [usize; 3])> = a.and_then(|a| b.and_then(|b| c.map(|c| (a, b, c))));
/// let tup: Option<(&'static str, usize, [usize; 3])> = Some(tuple3.curry()).apply1(a).apply1(b).apply1(c);
/// ```
pub fn tuple3<A, B, C>(a: A, b: B, c: C) -> (A, B, C) {
  (a, b, c)
}

pub(self) mod arg {
  #[allow(unreachable_pub)]
  pub trait Arg {
    /// The type of the argument
    type T;
  }
}

/// Type-level marker indicating that the curried function has been applied with this argument
#[derive(Clone, Copy)]
pub struct Just<T>(/// The argument
                   pub T);

/// Type-level marker indicating that the curried function has **not** been applied with this argument
pub struct Nothing<T>(pub(self) PhantomData<T>);

impl<T> Nothing<T> {
  /// Create a Nothing
  pub fn new() -> Self {
    Self(PhantomData)
  }
}

impl<T> Default for Nothing<T> {
  fn default() -> Self {
    Self::new()
  }
}

impl<T> Clone for Nothing<T> {
  fn clone(&self) -> Self {
    Self::new()
  }
}

impl<T> arg::Arg for Just<T> {
  type T = T;
}

impl<T> arg::Arg for Nothing<T> {
  type T = T;
}

/// Type of [`fn@call_deref`]
#[allow(non_camel_case_types)]
pub type call_deref<F, A, B> = fn(f: F, a: A) -> B;

/// Lift a function from `&ADeref -> B` to `A -> B`
/// where `A` can [`Deref::deref`] as `ADeref`
///
/// Used by [`F1Once::chain_ref`].
pub fn call_deref<F, A, ADeref: ?Sized, B>(f: F, a: A) -> B
  where A: Deref<Target = ADeref>,
        F: for<'a> F1Once<&'a ADeref, Ret = B>
{
  f.call1(a.deref())
}

/// A function that accepts 1 argument
/// and can be called at most once.
pub trait F1Once<A> {
  /// The type returned by this function
  type Ret;

  /// Call the function
  fn call1(self, a: A) -> Self::Ret;

  /// Create a new function that passes this one's output to `g`'s input
  ///
  /// (Left-to-right function composition)
  ///
  /// ```no_run
  /// use std::path::{Path, PathBuf};
  ///
  /// use naan::prelude::*;
  ///
  /// fn ensure_trailing_slash(s: &str) -> String {
  ///   if !s.ends_with("/") {
  ///     format!("{s}/")
  ///   } else {
  ///     s.into()
  ///   }
  /// }
  ///
  /// fn main() {
  ///   let dir_contains_readme = ensure_trailing_slash.chain(|path| format!("{path}README.md"))
  ///                                                  .chain(PathBuf::from)
  ///                                                  .chain_ref(Path::exists);
  ///
  ///   assert!(dir_contains_readme.call("toad-lib/toad/toad"));
  ///   assert!(!dir_contains_readme.call("toad-lib/toad"));
  /// }
  /// ```
  ///
  /// ## A Note on Type Errors
  /// TLDR: try `chain_ref` if the function is a receiver of `&self` or a similar shape,
  /// and if that fails wrap the composed function with `Box::new(...) as Box<dyn F1<.., ..>>`
  /// for more valuable compiler errors.
  ///
  /// <details>
  ///
  /// The type errors from chained composition type errors tend to be very complex.
  ///
  /// While debugging type errors like this, it may help to use `Box<dyn F1>` to
  /// provide more type information to the compiler, e.g.
  ///
  /// ```compile_fail
  /// use std::path::{Path, PathBuf};
  ///
  /// use naan::prelude::*;
  ///
  /// fn ensure_trailing_slash(s: &str) -> String {
  ///   if !s.ends_with("/") {
  ///     format!("{s}/")
  ///   } else {
  ///     s.into()
  ///   }
  /// }
  ///
  /// fn main() {
  ///   let dir_contains_readme =
  ///     ensure_trailing_slash.chain(|path| format!("{path}README.md")).chain(PathBuf::from)
  ///                                                                       .chain(Path::exists);
  ///
  ///   assert!(dir_contains_readme.call("toad-lib/toad/toad"));
  /// }
  /// ```
  /// produces type error:
  /// ```text
  /// error[E0599]: the method `call1` exists for struct `Compose<Compose<Compose<..>, ..>, .., ..>`, but its trait bounds were not satisfied
  ///   --> src/fun/mod.rs:37:31
  ///      |
  ///   19 |   assert!(dir_contains_readme.call1("toad-lib/toad/toad"));
  ///      |                               ^^^^^ method cannot be called on `Compose<Compose<Compose<..>, ..>, .., ..>` due to unsatisfied trait bounds
  /// ```
  ///
  /// when we box it, the compiler much more helpfully tells us that the issue is that `Path::exists` is `&Path -> bool`, rather than `PathBuf -> bool`:
  ///
  /// ```compile_fail
  /// # use std::path::{Path, PathBuf};
  /// # use naan::prelude::*;
  /// # fn ensure_trailing_slash(s: &str) -> String {
  /// #   if !s.ends_with("/") {
  /// #     format!("{s}/")
  /// #   } else {
  /// #     s.into()
  /// #   }
  /// # }
  /// fn main() {
  ///   let dir_contains_readme =
  ///     ensure_trailing_slash.chain(|path| format!("{path}README.md")).chain(PathBuf::from)
  ///                                                                   .chain(Path::exists);
  ///
  ///   let dir_contains_readme_boxed: Box<dyn F1<&str, Ret = bool>> = Box::new(dir_contains_readme) as _;
  ///
  ///   assert!(dir_contains_readme_boxed.call("toad-lib/toad/toad"));
  /// }
  /// ```
  /// yields
  /// ```text
  /// error[E0631]: type mismatch in function arguments
  ///   --> src/fun/compose.rs:91:75
  ///    |
  /// 17 | ...                   .chain(Path::exists);
  ///    |                        ----- ^^^^^^^^^^^^
  ///    |                        |     |
  ///    |                        |     expected due to this
  ///    |                        |     found signature defined here
  ///    |                        required by a bound introduced by this call
  ///    |
  ///    = note: expected function signature `fn(PathBuf) -> _`
  ///               found function signature `for<'r> fn(&'r Path) -> _`
  ///    = note: required for `for<'r> fn(&'r Path) -> bool {Path::exists}` to implement `F1Once<PathBuf, _>`
  /// ```
  /// Once all type errors are resolved, the Box debugging can be undone and you can use the concrete
  /// nested `Compose` types.
  /// </details>
  fn chain<G, C>(self, g: G) -> Compose<Self, G, Self::Ret>
    where Self: Sized,
          G: F1Once<Self::Ret, Ret = C>
  {
    Compose::compose(self, g)
  }

  /// [`chain`](F1Once::chain) that passes a reference to this function's return type
  /// to function `g`.
  ///
  /// Also performs [`Deref::deref`], allowing you to do things like pipe a `Vec`
  /// to a function expecting a slice or `String` to a function expecting `&str`.
  fn chain_ref<G, BDeref: ?Sized, C>(
    self,
    g: G)
    -> Compose<Self, curry2::Applied1<call_deref<G, Self::Ret, C>, G, Self::Ret, C>, Self::Ret>
    where Self: Sized,
          G: for<'any> F1Once<&'any BDeref, Ret = C>,
          Self::Ret: Deref<Target = BDeref>
  {
    Compose::compose(self,
                     (call_deref as call_deref<G, Self::Ret, C>).curry().call(g))
  }
}

/// A function that accepts 2 arguments
/// and can be called at most once.
pub trait F2Once<A, B>: Sized {
  /// The type returned by this function
  type Ret;

  /// The concrete type that `curry` returns.
  type Curried;

  /// Call the function
  fn call1(self, a: A, b: B) -> Self::Ret;

  /// Curry this function, transforming it from
  /// `fn(A, B) -> C` to `fn(A) -> fn(B) -> C`
  fn curry(self) -> Self::Curried;
}

/// A function that accepts 3 arguments
/// and can be called at most once.
pub trait F3Once<A, B, C>: Sized {
  /// The type returned by this function
  type Ret;

  /// The concrete type that `curry` returns.
  type Curried;

  /// Call the function
  fn call1(self, a: A, b: B, c: C) -> Self::Ret;

  /// Curry this function, transforming it from
  /// `fn(A, B, C) -> D` to `fn(A) -> fn(B) -> fn(C) -> Self::Ret`
  fn curry(self) -> Self::Curried;
}

/// A function that accepts 1 argument
/// and can be called any number of times.
pub trait F1<A>: F1Once<A> {
  /// Call the function
  fn call(&self, a: A) -> Self::Ret;
}

/// A function that accepts 2 arguments
/// and can be called any number of times.
pub trait F2<A, B>: F2Once<A, B> {
  /// Call the function with all arguments
  fn call(&self, a: A, b: B) -> Self::Ret;
}

/// A function that accepts 3 arguments
/// and can be called any number of times.
pub trait F3<A, B, C>: F3Once<A, B, C> {
  /// Call the function with all arguments
  fn call(&self, a: A, b: B, c: C) -> Self::Ret;
}

impl<F, A, B> F1<A> for F where F: Fn(A) -> B
{
  fn call(&self, a: A) -> B {
    self(a)
  }
}

impl<F, A, B> F1Once<A> for F where F: FnOnce(A) -> B
{
  type Ret = B;

  fn call1(self, a: A) -> B {
    self(a)
  }
}

impl<F, A, B, C> F2<A, B> for F where F: Fn(A, B) -> C
{
  fn call(&self, a: A, b: B) -> Self::Ret {
    self(a, b)
  }
}

impl<F, A, B, C> F2Once<A, B> for F where F: FnOnce(A, B) -> C
{
  type Ret = C;
  type Curried = curry2::Applied0<Self, A, B, C>;

  fn call1(self, a: A, b: B) -> C {
    self(a, b)
  }

  fn curry(self) -> Self::Curried {
    Curry2::curry(self)
  }
}

impl<F, A, B, C, D> F3<A, B, C> for F where F: Fn(A, B, C) -> D
{
  fn call(&self, a: A, b: B, c: C) -> D {
    self(a, b, c)
  }
}

impl<F, A, B, C, D> F3Once<A, B, C> for F where F: FnOnce(A, B, C) -> D
{
  type Ret = D;
  type Curried = curry3::Applied0<Self, A, B, C, D>;

  fn call1(self, a: A, b: B, c: C) -> D {
    self(a, b, c)
  }

  fn curry(self) -> Self::Curried {
    Curry3::curry(self)
  }
}
