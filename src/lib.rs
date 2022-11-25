pub mod alt;
pub mod apply;
pub mod fun;
pub mod functor;
pub mod impls;

#[macro_export]
macro_rules! deriving {
  (impl$(<$($vars:ident),+>)? Functor<$hkt:ty, $a:ident> for $t:ty {..FunctorOnce}) => {
    impl<$a, $($($vars),+)?> Functor<$hkt, $a> for $t {
      fn fmap<B>(self, f: impl F1<A, B>) -> <$hkt as HKT1>::T<B> {
        self.fmap1(f)
      }
    }
  };
  (impl$(<$($vars:ident),+>)? Apply<$hkt:ty, $ab:ident, $a:ident, $b:ident> for $t:ty {..ApplyOnce}) => {
    impl<$ab, $a, $b, $($($vars),+)?> Apply<$hkt, $ab, $a, $b> for $t where $ab: F1<$a, $b> {
      fn apply(self, f: <$hkt as HKT1>::T<$a>) -> <$hkt as HKT1>::T<$b> {
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
}

pub mod prelude {
  pub use crate::alt::*;
  pub use crate::apply::*;
  pub use crate::fun::*;
  pub use crate::functor::*;
  pub use crate::{deriving, HKT1, HKT2, HKT3};
}

pub trait HKT1 {
  type T<A>;
}

pub trait HKT2 {
  type T<A, B>;
}

pub trait HKT3 {
  type T<A, B, C>;
}

#[cfg(test)]
mod test {
  use super::prelude::*;

  #[test]
  fn fmap() {
    fn say_hello(s: &str) -> String {
      format!("Hello, {s}!")
    }
    assert_eq!(vec!["Sally", "Turd"].fmap(say_hello),
               vec!["Hello, Sally!", "Hello, Turd!"]);

    assert_eq!(Some("Fred").fmap(say_hello), Some("Hello, Fred!".into()));

    assert_eq!(Result::<&str, ()>::Ok("Fred").fmap(say_hello),
               Ok("Hello, Fred!".into()));
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

    assert_eq!(Some(say_hello!()).apply(Some("Fred")),
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
}
