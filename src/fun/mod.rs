use core::marker::PhantomData;

use curry2::Curry2;
use curry3::Curry3;

/// Currying functions with 2 arguments
pub mod curry2;

/// Currying functions with 3 arguments
pub mod curry3;

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

/// A function that accepts 1 argument
/// and can be called at most once.
pub trait F1Once<A, B> {
  /// Call the function
  fn call1(self, a: A) -> B;
}

/// A function that accepts 2 arguments
/// and can be called at most once.
pub trait F2Once<A, B, C>: Sized {
  /// The concrete type that `curry` returns.
  type Curried;

  /// Call the function
  fn call1(self, a: A, b: B) -> C;

  /// Curry this function, transforming it from
  /// `fn(A, B) -> C` to `fn(A) -> fn(B) -> C`
  fn curry(self) -> Self::Curried;
}

/// A function that accepts 3 arguments
/// and can be called at most once.
pub trait F3Once<A, B, C, D>: Sized {
  /// The concrete type that `curry` returns.
  type Curried;

  /// Call the function
  fn call1(self, a: A, b: B, c: C) -> D;

  /// Curry this function, transforming it from
  /// `fn(A, B, C) -> D` to `fn(A) -> fn(B) -> fn(C) -> D`
  fn curry(self) -> Self::Curried;
}

/// A function that accepts 1 argument
/// and can be called any number of times.
pub trait F1<A, B>: F1Once<A, B> {
  /// Call the function
  fn call(&self, a: A) -> B;
}

/// A function that accepts 2 arguments
/// and can be called any number of times.
pub trait F2<A, B, C>: F2Once<A, B, C> {
  /// Call the function with all arguments
  fn call(&self, a: A, b: B) -> C;
}

/// A function that accepts 3 arguments
/// and can be called any number of times.
pub trait F3<A, B, C, D>: F3Once<A, B, C, D> {
  /// Call the function with all arguments
  fn call(&self, a: A, b: B, c: C) -> D;
}

impl<F, A, B> F1<A, B> for F where F: Fn(A) -> B
{
  fn call(&self, a: A) -> B {
    self(a)
  }
}

impl<F, A, B> F1Once<A, B> for F where F: FnOnce(A) -> B
{
  fn call1(self, a: A) -> B {
    self(a)
  }
}

impl<F, A, B, C> F2<A, B, C> for F where F: Fn(A, B) -> C
{
  fn call(&self, a: A, b: B) -> C {
    self(a, b)
  }
}

impl<F, A, B, C> F2Once<A, B, C> for F where F: FnOnce(A, B) -> C
{
  type Curried = curry2::Applied0<Self, A, B, C>;
  fn call1(self, a: A, b: B) -> C {
    self(a, b)
  }

  fn curry(self) -> Self::Curried {
    Curry2::curry(self)
  }
}

impl<F, A, B, C, D> F3<A, B, C, D> for F where F: Fn(A, B, C) -> D
{
  fn call(&self, a: A, b: B, c: C) -> D {
    self(a, b, c)
  }
}

impl<F, A, B, C, D> F3Once<A, B, C, D> for F where F: FnOnce(A, B, C) -> D
{
  type Curried = curry3::Applied0<Self, A, B, C, D>;

  fn call1(self, a: A, b: B, c: C) -> D {
    self(a, b, c)
  }

  fn curry(self) -> Self::Curried {
    Curry3::curry(self)
  }
}
