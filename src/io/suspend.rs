use crate::prelude::*;

/// A deferred computation
#[derive(Debug, Clone, Copy)]
#[must_use = "IO is not evaluated until `IOLike.exec` invoked"]
pub struct Suspend<F>(pub(super) F);

impl<F> Equiv for Suspend<F> where F: F1Once<()>
{
  /// `Suspend<F>` is conceptually equivalent to `IO<{return type of F}>`
  type To = IO<F::Ret>;
}

impl<F> IOLike<F::Ret> for Suspend<F> where F: F1Once<()>
{
  fn exec(self) -> F::Ret {
    self.0.call1(())
  }
}
