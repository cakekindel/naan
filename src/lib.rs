//! ## deliciously succinct
//! [naan](https://en.wikipedia.org/wiki/Naan) is a functional programming prelude
//! for the Rust language that is:
//! * easy
//! * useful
//! * `std`- and `alloc`-optional
//! * _FAST_ - exclusively uses concrete types (no `dyn`amic dispatch) meaning near-zero perf cost
//!
//! ## new problem-solving tools
//! * higher-kinded types
//! * currying
//! * function composition
//! * new, general typeclasses
//! * lazy IO
//!
//! All of this is made possible with a trick using [Generic associated types](https://blog.rust-lang.org/2022/11/03/Rust-1.65.0.html#generic-associated-types-gats)
//! to emulate [**_Kinds_**](https://en.wikipedia.org/wiki/Kind_(type_theory))
//!
//! ### HKTs
//! #### What it is
//! In type theory, it can be useful to have language to differentiate between a concrete type (`u8`, `Vec<u8>`, `Result<File, io::Error>`)
//! and a generic type without its parameters supplied. (`Vec`, `Option`, `Result`)
//!
//! For example, `Vec` is a 1-argument (_unary_) type function, and `Vec<u8>` is a concrete type.
//!
//! Kind refers to how many (if any) parameters a type has.
//!
//! #### Why it's useful
//! In vanilla Rust, `Result::map` and `Option::map` have very similar shapes:
//! ```text
//! impl<A, E> Result<A, E> {
//!   fn map<B>(self, f: impl FnMut(A) -> B) -> Result<B, E>;
//! }
//!
//! impl<A> Option<A> {
//!   fn map<B>(self, f: impl FnMut(A) -> B) -> Option<B>;
//! }
//! ```
//! it would be useful (for reasons we'll expand on later) to have them
//! both implement a `Map` trait:
//! ```text
//! trait Map<A> {
//!   fn map<B>(self: Self<A>, f: impl FnMut(A) -> B) -> Self<B>;
//! }
//! ```
//! but this code snippet isn't legal Rust because `Self` needs to be generic (kind `* -> *`)
//! and in vanilla Rust `Self` must be a concrete type.
//!
//! #### How it's done
//! With the introduction of [Generic associated types](https://blog.rust-lang.org/2022/11/03/Rust-1.65.0.html#generic-associated-types-gats),
//! we can write a "type function of kind `* -> *`" trait (here called `HKT`).
//!
//! Using this we can implement `HKT` for `Option`, `Result`, or any `Self` _essentially_ generic by tying it to
//! and write the `Map` trait from above in legal Rust:
//!
//! ```rust
//! trait HKT {
//!   type Of<A>;
//! }
//!
//! struct OptionHKT;
//! impl HKT for OptionHKT {
//!   type Of<A> = Option<A>;
//! }
//!
//! trait Map<M, A>
//!   where M: HKT<Of<A> = Self>
//! {
//!   fn map<B, F>(self, f: F) -> M::Of<B>
//!     where F: FnMut(A) -> B;
//! }
//!
//! impl<A> Map<OptionHKT, A> for Option<A> {
//!   fn map<B, F>(self, f: F) -> Option<B>
//!     where F: FnMut(A) -> B
//!   {
//!     self.map(f)
//!   }
//! }
//! ```
//!
//! ### Currying
//! #### What it is
//! *Currying* is the technique where `naan` gets its name. Function currying is the strategy of splitting functions that
//! accept more than one argument into functions that return functions.
//!
//! Concrete example:
//! ```text
//! fn foo(String, usize) -> usize;
//! foo(format!("bar"), 12);
//! ```
//! would be curried into:
//! ```text
//! fn foo(String) -> impl Fn(usize) -> usize;
//! foo(format!("bar"))(12);
//! ```
//!
//! #### Why it's useful
//! Currying allows us to provide _some_ of a function's arguments and provide the rest of this
//! partially applied function's arguments at a later date.
//!
//! This allows us to use functions to store state, and lift functions that accept any number
//! of parameters to accept Results using [`Apply`](https://docs.rs/naan/latest/naan/apply/trait.Apply.html#example)
//!
//! **EXAMPLE: reusable function with a stored parameter**
//! ```rust,no_run
//! use std::fs::File;
//!
//! use naan::prelude::*;
//!
//! fn copy_file_to_dir(dir: String, file: File) -> std::io::Result<()> {
//!   // ...
//!   # Ok(())
//! }
//!
//! fn main() {
//!   let dir = std::env::var("DEST_DIR").unwrap();
//!   let copy = copy_file_to_dir.curry().call(dir);
//!
//!   File::open("a.txt").bind1(copy.clone())
//!                      .bind1(|_| File::open("b.txt"))
//!                      .bind1(copy.clone())
//!                      .bind1(|_| File::open("c.txt"))
//!                      .bind1(copy);
//! }
//!
//! /*
//!   equivalent to:
//!   fn main() {
//!     let dir = std::env::var("DEST_DIR").unwrap();
//!
//!     copy_file_to_dir(dir.clone(), File::open("a.txt")?)?;
//!     copy_file_to_dir(dir.clone(), File::open("b.txt")?)?;
//!     copy_file_to_dir(dir, File::open("c.txt")?)?;
//!   }
//! */
//! ```
//!
//! **EXAMPLE: lifting a function to accept Results (or Options)**
//! ```rust,no_run
//! use std::fs::File;
//!
//! use naan::prelude::*;
//!
//! fn append_contents(from: File, to: File) -> std::io::Result<()> {
//!   // ...
//!   # Ok(())
//! }
//!
//! fn main() -> std::io::Result<()> {
//!   Ok(append_contents.curry()).apply1(File::open("from.txt"))
//!                              .apply1(File::open("to.txt"))
//!                              .flatten()
//! }
//!
//! /*
//! equivalent to:
//! fn main() -> std::io::Result<()> {
//!   let from = File::open("from.txt")?;
//!   let to = File::open("to.txt")?;
//!   append_contents(from, to)
//! }
//! */
//! ```
//!
//! ### Function Composition
//!
//! ### Typeclasses
//!
//! ### Lazy IO

// docs
#![doc(html_root_url = "https://docs.rs/naan/0.1.13")]
#![cfg_attr(any(docsrs, feature = "docs"), feature(doc_cfg))]
// -
// deny
#![warn(missing_docs)]
#![cfg_attr(not(test), deny(unsafe_code))]
// -
// warnings
#![cfg_attr(not(test), warn(unreachable_pub))]
// -
// features
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc as std_alloc;

/// Alt, Plus
pub mod alt;

/// Apply, Applicative
pub mod apply;

/// Bifunctor
pub mod bifunctor;

/// Monad
pub mod monad;

/// Foldable
pub mod fold;

/// Functions
pub mod fun;

/// Functor
pub mod functor;

/// Implementors
pub mod impls;

/// Semigroup, Monoid
pub mod semigroup;

/// Traversable
pub mod traverse;

pub(crate) enum Never {}

/// Re-exports of HKT markers for types that have provided implementations
pub mod hkt {
  pub use crate::impls::option::hkt::Option;
  pub use crate::impls::result::hkt::{Result, ResultOk};
  pub use crate::impls::vec::hkt::Vec;

  /// `std::io`
  pub mod io {
    /// Result pinned to [`std::io::Error`]
    pub type Result = crate::impls::result::hkt::ResultOk<std::io::Error>;
  }
}

/// Glob import that provides all of the `naan` typeclasses
pub mod prelude {
  pub use crate::alt::*;
  pub use crate::apply::*;
  pub use crate::bifunctor::*;
  pub use crate::fold::*;
  pub use crate::fun::compose::*;
  pub use crate::fun::curry2::*;
  pub use crate::fun::curry3::*;
  pub use crate::fun::*;
  pub use crate::functor::*;
  pub use crate::monad::*;
  pub use crate::semigroup::*;
  pub use crate::traverse::*;
  pub use crate::{deriving, hkt, HKT1, HKT2};
}

/// A marker that points to a type with 1 generic
/// parameter.
///
/// ```
/// use naan::prelude::*;
///
/// enum Maybe<A> {
///   Just(A),
///   Nothing,
/// }
///
/// struct MaybeHKT;
///
/// impl HKT1 for MaybeHKT {
///   type T<A> = Maybe<A>;
/// }
/// ```
pub trait HKT1 {
  /// The generic type
  type T<A>;
}

/// A marker that points to a type with 2 generic
/// parameters.
///
/// ```
/// use naan::prelude::*;
///
/// enum Either<A, B> {
///   Left(A),
///   Right(B),
/// }
///
/// struct EitherHKT;
///
/// impl HKT2 for EitherHKT {
///   type T<A, B> = Either<A, B>;
/// }
/// ```
pub trait HKT2 {
  /// The generic type
  type T<A, B>;
}

/// Helper macro that allows deriving various typeclass instances from
/// other traits or typeclasses.
///
/// e.g. Functor can use the implementation for FunctorOnce, Plus can use Default.
///
/// ```
/// use naan::prelude::*;
///
/// #[derive(Default)]
/// pub struct Foo(String);
///
/// impl Semigroup for Foo {
///   fn append(self, other: Self) -> Self {
///     Foo(self.0.append(other.0))
///   }
/// }
///
/// deriving!(impl Monoid for Foo {..Default});
/// ```
#[macro_export]
macro_rules! deriving {
  (impl$(<$($vars:ident),+>)? Functor<$hkt:ty, $a:ident> for $t:ty {..FunctorOnce}) => {
    impl<$a, $($($vars),+)?> Functor<$hkt, $a> for $t {
      fn fmap<AB, B>(self, f: AB) -> <$hkt as HKT1>::T<B> where AB: F1<A, B> {
        self.fmap1(f)
      }
    }
  };
  (impl$(<$($vars:ident),+>)? Bifunctor<$hkt:ty, $a:ident, $b:ident> for $t:ty {..BifunctorOnce}) => {
    impl<$a, $b, $($($vars),+)?> Bifunctor<$hkt, $a, $b> for $t {
      fn bimap<AB, BB, FA, FB>(self, fa: FA, fb: FB) -> <$hkt as HKT2>::T<AB, BB> where FA: F1<$a, AB>, FB: F1<$b, BB> {
        self.bimap1(fa, fb)
      }
    }
  };
  (impl$(<$($vars:ident),+>)? Apply<$hkt:ty, $ab:ident> for $t:ty {..ApplyOnce}) => {
    impl<$ab, $($($vars),+)?> Apply<$hkt, $ab> for $t {
  fn apply_clone_with<A, B, Cloner>(self, a: <$hkt as HKT1>::T<A>, _: Cloner) -> <$hkt as HKT1>::T<B>
      where AB: F1<A, B>,
            Cloner: for<'a> F1<&'a A, A>
            {
        self.apply1(a)
      }
    }
  };
  (impl$(<$($vars:ident),+>)? Plus<$hkt:ty, $a:ident> for $t:ty {..Default}) => {
    impl<$a, $($($vars),+)?> Plus<$hkt, $a> for $t {
      fn empty() -> <$hkt as HKT1>::T<$a> {
        Default::default()
      }
    }
  };
  (impl$(<$($vars:ident),+>)? Semigroup for $t:ty {..Alt}) => {
    impl$(<$($vars),+>)? Semigroup for $t {
      fn append(self, other: Self) -> Self {
        self.alt(other)
      }
    }
  };
  (impl$(<$($vars:ident),+>)? Monoid for $t:ty {..Default}) => {
    impl$(<$($vars),+>)? Monoid for $t {
      fn identity() -> Self {
        Default::default()
      }
    }
  };
  (impl$(<$($vars:ident),+>)? Foldable<$hkt:ty, $a:ident> for $t:ty {..FoldableOnce}) => {
    impl<$a, $($($vars),+)?> Foldable<$hkt, $a> for $t {
      fn foldl<B, BAB>(self, f: BAB, b: B) -> B
      where BAB: F2<B, A, B> {
        self.fold1(f, b)
      }

      fn foldr<B, ABB>(self, f: ABB, b: B) -> B
      where ABB: F2<A, B, B> {
        self.fold1(|a, b| f.call(b, a), b)
      }


  /// Fold the data structure from left -> right
  fn foldl_ref<'a, B, BAB>(&'a self, f: BAB, b: B) -> B
  where BAB: F2<B, &'a A, B>, A: 'a {
    self.fold1_ref(f, b)
  }

  /// Fold the data structure from right -> left
  fn foldr_ref<'a, B, ABB>(&'a self, f: ABB, b: B) -> B
    where ABB: F2<&'a A, B, B>, A: 'a {
    self.fold1_ref(|a, b| f.call(b, a), b)
    }}
  };
  (impl$(<$($vars:ident),+>)? Traversable<$hkt:ty, $a:ident, $b:ident, $tf:ty> for $t:ty {..TraversableOnce}) => {
    impl<$a, $b, $($($vars),+)?> Traversable<$hkt, $a, $b, $tf> for $t {
  fn traversem1<Ap, AtoApOfB>(self, f: AtoApOfB) -> Ap::T<<$hkt as HKT1>::T<B>>
    where Ap: HKT1,
      Ap::T<B>: Applicative<Ap, B> + ApplyOnce<Ap, B>,
      Ap::T<$tf>: Applicative<Ap, $tf> + ApplyOnce<Ap, $tf>,
      Ap::T<<$hkt as HKT1>::T<B>>: Applicative<Ap, <$hkt as HKT1>::T<B>> + ApplyOnce<Ap, <$hkt as HKT1>::T<B>>,
      AtoApOfB: F1<A, Ap::T<B>> {
        self.traverse11::<Ap, AtoApOfB>(f)
      }

  fn traversemm<Ap, AtoApOfB>(self, f: AtoApOfB) -> Ap::T<<$hkt as HKT1>::T<B>>
    where Ap: HKT1,
      Ap::T<B>: Applicative<Ap, B>,
      Ap::T<$tf>: Applicative<Ap, $tf>,
      Ap::T<<$hkt as HKT1>::T<B>>: Applicative<Ap, <$hkt as HKT1>::T<B>>,
      AtoApOfB: F1<A, Ap::T<B>>
       {
        self.traverse1m::<Ap, AtoApOfB>(f)
      }
    }
  };
  (impl$(<$($vars:ident),+>)? Monad<$hkt:ty, $a:ident> for $t:ty {..MonadOnce}) => {
    impl<$a, $($($vars),+)?> Monad<$hkt, $a> for $t {
      fn bind<B, AMB>(self, f: AMB) -> <$hkt as HKT1>::T<B> where AMB: F1<$a, <$hkt as HKT1>::T<B>> {
        self.bind1(f)
      }
    }
  };
}
