use core::marker::PhantomData;

use super::IOLike;
use crate::prelude::*;

/// An [`IOLike`] that, when executed, will apply some `IOLike<A>` to an `IOLike<A -> B>` to get an `IOLike<B>`.
///
/// This is the result of calling [`ApplySurrogate.apply_`] on an [`IOLike`].
#[must_use = "IO is not evaluated until `IOLike.exec` invoked"]
pub struct Apply<A, B, AB, IOA, IOAB>(IOAB, IOA, PhantomData<(A, B, AB)>);

impl<A, B, AB, IOA, IOAB> Apply<A, B, AB, IOA, IOAB> {
  /// Create a new Apply
  pub fn new(ioab: IOAB, ioa: IOA) -> Self {
    Self(ioab, ioa, PhantomData)
  }
}

impl<A, B, AB, IOA, IOAB> Equiv for Apply<A, B, AB, IOA, IOAB> {
  type To = IO<B>;
}

impl<A, B, AB, IOA, IOAB> IOLike<B> for Apply<A, B, AB, IOA, IOAB>
  where AB: F1Once<A, Ret = B>,
        IOA: IOLike<A>,
        IOAB: IOLike<AB>
{
  fn exec(self) -> B {
    self.0.exec().call1(self.1.exec())
  }
}
