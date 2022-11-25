use std::marker::PhantomData;

use arg_state::ArgumentState;

use super::{F1Once, F2Once, F1};
mod arg_state {
    pub trait ArgumentState<T> {}
}

pub struct Invkd<T>(PhantomData<T>);
pub struct Waiting<T>(PhantomData<T>);

impl<T> ArgumentState<T> for Invkd<T> {}
impl<T> ArgumentState<T> for Waiting<T> {}

pub type Applied0<F, A, B, C> = Curry2<F, A, B, C, Waiting<A>, Waiting<B>>;
pub type Applied1<F, A, B, C> = Curry2<F, A, B, C, Invkd<A>, Waiting<B>>;

pub struct Curry2<F, A, B, C, SA, SB>
where
    F: F2Once<A, B, C>,
    SA: ArgumentState<A>,
    SB: ArgumentState<B>,
{
    f: F,
    a: Option<A>,
    _s: PhantomData<(SA, SB, B, C)>,
}

impl<A, B, C, F, SA, SB> Clone for Curry2<F, A, B, C, SA, SB>
where
    A: Clone,
    B: Clone,
    C: Clone,
    F: Clone + FnOnce(A, B) -> C,
    SA: ArgumentState<A>,
    SB: ArgumentState<B>,
{
    fn clone(&self) -> Self {
        Curry2 {
            f: self.f.clone(),
            a: self.a.clone(),
            _s: PhantomData,
        }
    }
}

impl<F, A, B, C> Applied0<F, A, B, C>
where
    F: F2Once<A, B, C>,
{
    pub fn curry(f: F) -> Self {
        Self {
            f,
            a: None,
            _s: PhantomData,
        }
    }

    pub fn uncurry(self) -> F {
        self.f
    }
}

impl<F, A, B, C> F1<A, Applied1<F, A, B, C>> for Applied0<F, A, B, C>
where
    F: Clone + Fn(A, B) -> C,
{
    fn call(&self, a: A) -> Applied1<F, A, B, C> {
        Applied1::<F, A, B, C> {
            a: Some(a),
            f: self.f.clone(),
            _s: PhantomData,
        }
    }
}

impl<F, A, B, C> F1Once<A, Applied1<F, A, B, C>> for Applied0<F, A, B, C>
where
    F: FnOnce(A, B) -> C,
{
    fn call1(self, a: A) -> Applied1<F, A, B, C> {
        Applied1::<F, A, B, C> {
            a: Some(a),
            f: self.f,
            _s: PhantomData,
        }
    }
}

impl<F, A, B, C> F1<B, C> for Applied1<F, A, B, C>
where
    F: Fn(A, B) -> C,
    A: Clone,
{
    fn call(&self, b: B) -> C {
        (self.f)(self.a.as_ref().cloned().unwrap(), b)
    }
}

impl<F, A, B, C> F1Once<B, C> for Applied1<F, A, B, C>
where
    F: FnOnce(A, B) -> C,
{
    fn call1(self, b: B) -> C {
        (self.f)(self.a.unwrap(), b)
    }
}
