use core::marker::PhantomData;

use super::{F1Once, F1};

/// Struct capturing [`Sized`] function composition.
///
/// Implements [`F1`]/[`F1Once`] when `F` and `G`
/// implement [`F1`]/[`F1Once`].
///
/// `X` is the "erased" type returned by `F` and passed
/// to `G`, stored as [`PhantomData`] to make trait implementation
/// simpler.
///
/// For examples, see [`F1Once::chain`].
pub struct Compose<F, G, X> {
  f: F,
  g: G,
  hidden_type: PhantomData<X>,
}

impl<F, G, X> Compose<F, G, X> {
  /// See [`Compose`]
  pub fn compose<A, B>(f: F, g: G) -> Self
    where F: F1Once<A, Ret = X>,
          G: F1Once<X, Ret = B>
  {
    Self { f,
           g,
           hidden_type: PhantomData }
  }

  /// See [`F1Once::chain`]
  pub fn chain<A, B, X2, G2>(self, g2: G2) -> Compose<Compose<F, G, X>, G2, X2>
    where F: F1Once<A, Ret = X>,
          G: F1Once<X, Ret = X2>,
          G2: F1Once<X2, Ret = B>
  {
    Compose { f: self,
              g: g2,
              hidden_type: PhantomData }
  }
}

impl<F, G, A, X, C> F1Once<A> for Compose<F, G, X>
  where F: F1Once<A, Ret = X>,
        G: F1Once<X, Ret = C>
{
  type Ret = C;
  fn call1(self, a: A) -> C {
    self.g.call1(self.f.call1(a))
  }
}

impl<F, G, A, X, C> F1<A> for Compose<F, G, X>
  where F: F1<A, Ret = X>,
        G: F1<X, Ret = C>
{
  fn call(&self, a: A) -> C {
    self.g.call(self.f.call(a))
  }
}
