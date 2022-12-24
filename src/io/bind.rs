use core::marker::PhantomData;

use super::IOLike;
use crate::prelude::*;

/// A function from `A` to an `IOLike<B>` waiting to be applied to
/// a [lazy value](IOLike), transforming it from `IOLike<A>` to `IOLike<B>`.
///
/// This is the result of calling [`MonadSurrogate.bind_`] on an [`IOLike`].
#[must_use = "IO is not evaluated until `IOLike.exec` invoked"]
pub struct Bind<F, A, B, IOA>(F, IOA, PhantomData<(A, B)>);

impl<F, A, B, IOA> Bind<F, A, B, IOA> {
  /// Create a new Bind
  pub fn new(f: F, ioa: IOA) -> Self {
    Self(f, ioa, PhantomData)
  }
}

impl<F, A, B, IOA> core::fmt::Debug for Bind<F, A, B, IOA>
  where F: core::fmt::Debug,
        IOA: core::fmt::Debug
{
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    f.debug_tuple("Bind")
     .field(&self.0)
     .field(&self.1)
     .field(&"PhantomData")
     .finish()
  }
}

impl<F, A, B, IOA> Clone for Bind<F, A, B, IOA>
  where F: Clone,
        IOA: Clone
{
  fn clone(&self) -> Self {
    Self(self.0.clone(), self.1.clone(), PhantomData)
  }
}

impl<F, A, B, IOA> Copy for Bind<F, A, B, IOA>
  where F: Copy,
        IOA: Copy
{
}

impl<F, A, B, IOA> Equiv for Bind<F, A, B, IOA>
  where F: F1Once<A>,
        F::Ret: Equiv<To = IO<B>>,
        IOA: Equiv<To = IO<A>>
{
  /// `Bind<F, A, B, IOA>` is conceptually equivalent to `IO<{return type of F}>`
  type To = IO<B>;
}

impl<F, A, B, IOA> IOLike<B> for Bind<F, A, B, IOA>
  where F: F1Once<A>,
        F::Ret: Equiv<To = IO<B>> + IOLike<B>,
        IOA: IOLike<A>
{
  fn exec(self) -> B {
    self.0.call1(self.1.exec()).exec()
  }
}
