use std::marker::PhantomData;

use super::arg::Arg;
use super::{F1Once, F2Once, Just, Nothing, F1};

/// A curried function that accepts 2 arguments and has not been called with either.
pub type Applied0<F, A, B, C> = Curry2<F, Nothing<A>, Nothing<B>, C>;

/// A curried function that accepts 2 arguments and has been called with the first argument.
pub type Applied1<F, A, B, C> = Curry2<F, Just<A>, Nothing<B>, C>;

/// A curried function that accepts 2 arguments
pub struct Curry2<F, A, B, C> {
  f: F,
  a: A,
  _s: PhantomData<(B, C)>,
}

impl<A, B, C, F> Clone for Curry2<F, A, B, C>
  where A: Arg + Clone,
        B: Arg,
        F: Clone + Fn(A::T, B::T) -> C
{
  fn clone(&self) -> Self {
    Curry2 { f: self.f.clone(),
             a: self.a.clone(),
             _s: PhantomData }
  }
}

impl<A, B, C, F> Copy for Curry2<F, A, B, C>
  where A: Arg + Copy,
        B: Arg,
        F: Copy + Fn(A::T, B::T) -> C
{
}

impl<F, A, B, C> Applied0<F, A, B, C> where F: F2Once<A, B, C>
{
  /// Curry a binary function
  pub fn curry(f: F) -> Self {
    Self { f,
           a: Nothing::new(),
           _s: PhantomData }
  }

  /// Unwrap the `Curry2` wrapper, getting the inner function
  pub fn uncurry(self) -> F {
    self.f
  }
}

impl<F, A, B, C> F1<A, Applied1<F, A, B, C>> for Applied0<F, A, B, C> where F: Clone + Fn(A, B) -> C
{
  fn call(&self, a: A) -> Applied1<F, A, B, C> {
    Applied1::<F, A, B, C> { a: Just(a),
                             f: self.f.clone(),
                             _s: PhantomData }
  }
}

impl<F, A, B, C> F1Once<A, Applied1<F, A, B, C>> for Applied0<F, A, B, C> where F: FnOnce(A, B) -> C
{
  fn call1(self, a: A) -> Applied1<F, A, B, C> {
    Applied1::<F, A, B, C> { a: Just(a),
                             f: self.f,
                             _s: PhantomData }
  }
}

impl<F, A, B, C> F1<B, C> for Applied1<F, A, B, C>
  where F: Fn(A, B) -> C,
        A: Clone
{
  fn call(&self, b: B) -> C {
    (self.f)(self.a.0.clone(), b)
  }
}

impl<F, A, B, C> F1Once<B, C> for Applied1<F, A, B, C> where F: FnOnce(A, B) -> C
{
  fn call1(self, b: B) -> C {
    (self.f)(self.a.0, b)
  }
}
