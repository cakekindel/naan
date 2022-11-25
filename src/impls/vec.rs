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

impl<AB, A, B> Apply<hkt::Vec, AB, A, B> for Vec<AB>
  where A: Clone,
        AB: F1<A, B>
{
  fn apply(self, a: Vec<A>) -> Vec<B> {
    self.into_iter()
        .map(|f| a.iter().cloned().map(|a| f.call(a)).collect::<Vec<B>>())
        .flatten()
        .collect()
  }
}

impl<A> Alt<hkt::Vec, A> for Vec<A> {
  fn alt(mut self, mut b: Self) -> Self {
    self.append(&mut b);
    self
  }
}
deriving!(impl Plus<hkt::Vec, A> for Vec<A> {..Default});
