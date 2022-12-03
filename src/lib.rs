//! TODO

// docs
#![doc(html_root_url = "https://docs.rs/naan/0.1.8")]
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
  pub use crate::impls::result::hkt::ResultOk;
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
      fn fmap<B>(self, f: impl F1<A, B>) -> <$hkt as HKT1>::T<B> {
        self.fmap1(f)
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
