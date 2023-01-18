use std_alloc::format;
use std_alloc::string::String;

use crate::prelude::*;

impl Semigroup for String {
  fn append(self, b: Self) -> Self {
    format!("{self}{b}")
  }
}
deriving!(impl Monoid for String {..Default});
