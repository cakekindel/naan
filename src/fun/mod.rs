use curry2::{Applied0, Curry2};

pub mod curry2;

pub trait F1Once<A, B> {
  fn call1(self, a: A) -> B;
}

pub trait F2Once<A, B, C>: Sized {
  type Curried;
  fn call1(self, a: A, b: B) -> C;
  fn curry(self) -> Self::Curried;
}

pub trait F1<A, B>: F1Once<A, B> {
  fn call(&self, a: A) -> B;
}

pub trait F2<A, B, C>: F2Once<A, B, C> {
  fn call(&self, a: A, b: B) -> C;
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
  type Curried = Applied0<Self, A, B, C>;
  fn call1(self, a: A, b: B) -> C {
    self(a, b)
  }

  fn curry(self) -> Self::Curried {
    Curry2::curry(self)
  }
}
