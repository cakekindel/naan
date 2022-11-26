//! TODO

// docs
#![doc(html_root_url = "https://docs.rs/naan/0.1.6")]
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

pub(crate) enum Never {}

/// Glob import that provides all of the `naan` typeclasses
pub mod prelude {
  pub use crate::alt::*;
  pub use crate::apply::*;
  pub use crate::fold::*;
  pub use crate::fun::*;
  pub use crate::functor::*;
  pub use crate::semigroup::*;
  pub use crate::{deriving, HKT1, HKT2};
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
      fn apply<_A, _B>(self, f: <$hkt as HKT1>::T<_A>) -> <$hkt as HKT1>::T<_B> where $ab: F1<_A, _B> {
        self.apply1(f)
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
}

#[cfg(test)]
mod test {
  use ::std::string::String;

  use super::prelude::*;
  use crate::impls;
  use crate::prelude::curry2::{Applied0, Applied1, Curry2};

  #[test]
  fn fmap() {
    fn say_hello(s: &str) -> String {
      format!("Hello, {s}!")
    }
    assert_eq!(vec!["Sally", "Turd"].fmap(say_hello),
               vec!["Hello, Sally!", "Hello, Turd!"]);

    assert_eq!(Option::pure("Fred").fmap(say_hello),
               Option::pure("Hello, Fred!".into()));

    assert_eq!(Result::<&str, ()>::pure("Fred").fmap(say_hello),
               Ok("Hello, Fred!".into()));
  }

  #[test]
  fn apply_curry() {
    #![allow(non_camel_case_types)]

    type addfn = fn(usize, usize) -> usize;
    type add0 = Applied0<addfn, usize, usize, usize>;
    type add1 = Applied1<addfn, usize, usize, usize>;
    fn add(a: usize, b: usize) -> usize {
      a + b
    }

    fn test_add_with<Tusize: Clone + core::fmt::Debug + Eq + Applicative<F, usize>,
                         Tadd0: Applicative<F, add0>,
                         Tadd1: Applicative<F, add1>,
                         F: HKT1<T<usize> = Tusize> + HKT1<T<add0> = Tadd0> + HKT1<T<add1> = Tadd1>>(
      empty: Tusize) {
      assert_eq!(Tadd0::pure((add as addfn).curry()).apply(Tusize::pure(1))
                                                    .apply(Tusize::pure(2)),
                 Tusize::pure(3));
      assert_eq!(Tadd0::pure((add as addfn).curry()).apply(Tusize::pure(1))
                                                    .apply(empty.clone()),
                 empty.clone());
      assert_eq!(Tadd0::pure((add as addfn).curry()).apply(empty.clone())
                                                    .apply(Tusize::pure(2)),
                 empty);
    }

    test_add_with(Err(()));
    test_add_with(None);
    test_add_with(vec![]);
  }

  #[test]
  fn apply() {
    type StrToString = Box<dyn Fn(&str) -> String>;

    macro_rules! say_hello {
      () => {
        Box::new(|name: &str| format!("Hello, {name}!")) as StrToString
      };
    }

    macro_rules! say_goodbye {
      () => {
        Box::new(|name: &str| format!("Bye bye, {name}!")) as StrToString
      };
    }

    assert_eq!(vec![say_hello!(), say_goodbye!()].apply(vec!["Fred", "Harry"]),
               vec!["Hello, Fred!",
                    "Hello, Harry!",
                    "Bye bye, Fred!",
                    "Bye bye, Harry!"]);

    assert_eq!(Option::pure(say_hello!()).apply(Some("Fred")),
               Some("Hello, Fred!".to_string()));

    assert_eq!(Ok(say_hello!()).apply(Result::<&str, ()>::Ok("Fred")),
               Ok("Hello, Fred!".to_string()));
  }

  #[test]
  fn alt() {
    assert_eq!(vec![1u8, 2, 3].alt(vec![4u8]), vec![1u8, 2, 3, 4]);

    assert_eq!(Some(1u8).alt(None), Some(1u8));
    assert_eq!(None.alt(Some(1u8)), Some(1u8));

    assert_eq!(Result::<usize, ()>::Ok(1).alt(Ok(2)), Ok(1));
    assert_eq!(Result::<usize, ()>::Err(()).alt(Ok(2)), Ok(2));
  }

  #[test]
  fn plus() {
    assert_eq!(Option::<u8>::empty(), None);
    assert_eq!(Vec::<u8>::empty(), vec![]);
  }

  #[test]
  fn semigroup() {
    assert_eq!(Some("foo".to_string()).append(Some("bar".to_string())),
               Some("foobar".into()));
    assert_eq!(Vec::<u8>::identity().append(vec![0]), vec![0]);
    assert_eq!("foo".to_string().append("bar".into()), "foobar".to_string());
  }
}
