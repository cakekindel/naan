use core::marker::PhantomData;

use super::IOLike;
use crate::prelude::*;

/// A function from `A` to some `B` waiting to be applied to
/// a [lazy value](IOLike), transforming it from `IOLike<A>` to `IOLike<B>`.
///
/// This is the result of calling [`FunctorSurrogate.map_`] on an [`IOLike`].
#[must_use = "IO is not evaluated until `IOLike.exec` invoked"]
pub struct Map<F, X, A, IOX>(F, IOX, PhantomData<(X, A)>);

impl<F, X, A, IOX> core::fmt::Debug for Map<F, X, A, IOX>
  where F: core::fmt::Debug,
        IOX: core::fmt::Debug
{
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    f.debug_tuple("Map")
     .field(&self.0)
     .field(&self.1)
     .field(&"PhantomData")
     .finish()
  }
}

impl<F, X, A, IOX> Clone for Map<F, X, A, IOX>
  where F: Clone,
        IOX: Clone
{
  fn clone(&self) -> Self {
    Self(self.0.clone(), self.1.clone(), PhantomData)
  }
}

impl<F, X, A, IOA> Copy for Map<F, X, A, IOA>
  where F: Copy,
        IOA: Copy
{
}

impl<F, X, A, IOA> Map<F, X, A, IOA> {
  /// Create a new Map
  pub fn new(f: F, ioa: IOA) -> Self {
    Self(f, ioa, PhantomData)
  }
}

impl<F, X, A, IOX> Equiv for Map<F, X, A, IOX>
  where F: F1Once<X>,
        IOX: Equiv<To = IO<X>>
{
  /// `Map<F, A, IOA>` is conceptually equivalent to `IO<{return type of F}>`
  type To = IO<F::Ret>;
}

impl<F, X, A, IOX> IOLike<F::Ret> for Map<F, X, A, IOX>
  where F: F1Once<X, Ret = A>,
        IOX: IOLike<X>
{
  fn exec(self) -> F::Ret {
    self.0.call1(self.1.exec())
  }
}
