use crate::prelude::*;

pub mod hkt {
    use super::*;

    pub struct Vec;
    impl HKT1 for Vec {
        type T<A> = ::std::vec::Vec<A>;
    }
}

impl<A> Functor<hkt::Vec, A> for Vec<A> {
    fn fmap<B>(self, f: impl F1<A, B>) -> Vec<B> {
        self.into_iter().map(|a| f.call(a)).collect()
    }
}

impl<AB> Apply<hkt::Vec, AB> for Vec<AB> {
    fn apply<A, B>(self, a: Vec<A>) -> Vec<B>
    where
        AB: F1<A, B>,
        A: Clone,
    {
        self.into_iter()
            .map(|f| a.iter().cloned().map(|a| f.call(a)).collect::<Vec<B>>())
            .flatten()
            .collect()
    }
}

impl<A> Applicative<hkt::Vec, A> for Vec<A> {
    fn pure(a: A) -> Vec<A> {
        vec![a]
    }
}

impl<A> Alt<hkt::Vec, A> for Vec<A> {
    fn alt(mut self, mut b: Self) -> Self {
        Vec::append(&mut self, &mut b);
        self
    }
}
deriving!(impl Plus<hkt::Vec, A> for Vec<A> {..Default});

deriving!(impl<A> Semigroup for Vec<A> {..Alt});
deriving!(impl<A> Monoid for Vec<A> {..Default});
