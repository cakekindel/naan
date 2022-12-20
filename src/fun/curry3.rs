use std::marker::PhantomData;

use super::arg::Arg;
use super::{F1Once, F3Once, Just, Nothing, F1};

/// A curried function that accepts 3 arguments and has not been called with either.
pub type Applied0<F, A, B, C, D> = Curry3<F, Nothing<A>, Nothing<B>, Nothing<C>, D>;

/// A curried function that accepts 3 arguments and has been called with the first argument.
pub type Applied1<F, A, B, C, D> = Curry3<F, Just<A>, Nothing<B>, Nothing<C>, D>;

/// A curried function that accepts 3 arguments and has been called with the first & second arguments.
pub type Applied2<F, A, B, C, D> = Curry3<F, Just<A>, Just<B>, Nothing<C>, D>;

/// A curried function that accepts 2 arguments
pub struct Curry3<F, A, B, C, D> {
  f: F,
  a: A,
  b: B,
  _s: PhantomData<(C, D)>,
}

impl<F, A, B, C, D> Clone for Curry3<F, A, B, C, D>
  where F: Clone + FnOnce(A::T, B::T) -> C::T,
        A: Arg + Clone,
        B: Arg + Clone,
        C: Arg
{
  fn clone(&self) -> Self {
    Curry3 { f: self.f.clone(),
             a: self.a.clone(),
             b: self.b.clone(),
             _s: PhantomData }
  }
}

impl<F, A, B, C, D> Applied0<F, A, B, C, D> where F: F3Once<A, B, C, Ret = D>
{
  /// Curry a ternary function
  pub fn curry(f: F) -> Self {
    Self { f,
           a: Nothing::new(),
           b: Nothing::new(),
           _s: PhantomData }
  }

  /// Unwrap the `Curry2` wrapper, getting the inner function
  pub fn uncurry(self) -> F {
    self.f
  }
}

impl<F, A, B, C, D> F1<A> for Applied0<F, A, B, C, D> where F: Clone + Fn(A, B, C) -> D
{
  fn call(&self, a: A) -> Applied1<F, A, B, C, D> {
    Applied1::<F, A, B, C, D> { a: Just(a),
                                b: Nothing::new(),
                                f: self.f.clone(),
                                _s: PhantomData }
  }
}

impl<F, A, B, C, D> F1Once<A> for Applied0<F, A, B, C, D> where F: FnOnce(A, B, C) -> D
{
  type Ret = Applied1<F, A, B, C, D>;
  fn call1(self, a: A) -> Applied1<F, A, B, C, D> {
    Applied1::<F, A, B, C, D> { a: Just(a),
                                b: Nothing::new(),
                                f: self.f,
                                _s: PhantomData }
  }
}

impl<F, A, B, C, D> F1<B> for Applied1<F, A, B, C, D>
  where F: Fn(A, B, C) -> D + Clone,
        A: Clone
{
  fn call(&self, b: B) -> Applied2<F, A, B, C, D> {
    Applied2::<F, A, B, C, D> { a: self.a.clone(),
                                b: Just(b),
                                f: self.f.clone(),
                                _s: PhantomData }
  }
}

impl<F, A, B, C, D> F1Once<B> for Applied1<F, A, B, C, D> where F: FnOnce(A, B, C) -> D
{
  type Ret = Applied2<F, A, B, C, D>;
  fn call1(self, b: B) -> Applied2<F, A, B, C, D> {
    Applied2::<F, A, B, C, D> { a: self.a,
                                b: Just(b),
                                f: self.f,
                                _s: PhantomData }
  }
}

impl<F, A, B, C, D> F1<C> for Applied2<F, A, B, C, D>
  where F: Fn(A, B, C) -> D,
        A: Clone,
        B: Clone
{
  fn call(&self, c: C) -> D {
    (self.f)(self.a.clone().0, self.b.clone().0, c)
  }
}

impl<F, A, B, C, D> F1Once<C> for Applied2<F, A, B, C, D> where F: FnOnce(A, B, C) -> D
{
  type Ret = D;
  fn call1(self, c: C) -> D {
    (self.f)(self.a.0, self.b.0, c)
  }
}
