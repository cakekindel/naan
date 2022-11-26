use curry2::{Applied0, Curry2};

/// Currying functions with 2 arguments
pub mod curry2;

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

/// A function that accepts 1 argument
/// and can be called any number of times.
pub trait F1<A, B>: F1Once<A, B> {
    /// Call the function
    fn call(&self, a: A) -> B;
}

/// A function that accepts 2 arguments
/// and can be called any number of times.
pub trait F2<A, B, C>: F2Once<A, B, C> {
    /// Call the function
    fn call(&self, a: A, b: B) -> C;
}

impl<F, A, B> F1<A, B> for F
where
    F: Fn(A) -> B,
{
    fn call(&self, a: A) -> B {
        self(a)
    }
}

impl<F, A, B> F1Once<A, B> for F
where
    F: FnOnce(A) -> B,
{
    fn call1(self, a: A) -> B {
        self(a)
    }
}
impl<F, A, B, C> F2<A, B, C> for F
where
    F: Fn(A, B) -> C,
{
    fn call(&self, a: A, b: B) -> C {
        self(a, b)
    }
}

impl<F, A, B, C> F2Once<A, B, C> for F
where
    F: FnOnce(A, B) -> C,
{
    type Curried = Applied0<Self, A, B, C>;
    fn call1(self, a: A, b: B) -> C {
        self(a, b)
    }

    fn curry(self) -> Self::Curried {
        Curry2::curry(self)
    }
}
