use crate::prelude::*;

pub mod hkt {
  use crate::prelude::*;

  pub struct Option;
  impl HKT1 for Option {
    type T<A> = ::std::option::Option<A>;
  }
}

impl<A> FunctorOnce<hkt::Option, A> for Option<A> {
  fn fmap1<B>(self, f: impl F1Once<A, B>) -> Option<B> {
    self.map(|a| f.call1(a))
  }
}
deriving!(impl Functor<hkt::Option, A> for Option<A> {..FunctorOnce});

impl<AB, A, B> ApplyOnce<hkt::Option, AB, A, B> for Option<AB> where AB: F1Once<A, B>
{
  fn apply1(self, a: Option<A>) -> Option<B> {
    match self {
      | Some(f) => a.map(|a| f.call1(a)),
      | None => None,
    }
  }
}
deriving!(impl Apply<hkt::Option, AB, A, B> for Option<AB> {..ApplyOnce});

impl<A> Alt<hkt::Option, A> for Option<A> {
  fn alt(self, b: Self) -> Self {
    self.or(b)
  }
}
deriving!(impl Plus<hkt::Option, A> for Option<A> {..Default});
