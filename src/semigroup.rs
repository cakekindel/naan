/// Semigroup defines some associative operation that can be done
/// to 2 instances of a type.
///
/// # Examples
/// - `String`'s semigroup implementation concatenates the two strings.
/// - `Vec`'s semigroup implementation concatenates the two Vecs.
///
/// # Laws
/// Implementations of `Semigroup` must be associative, e.g.
///
/// ```
/// use naan::prelude::*;
///
/// let a = || vec![1u8, 2];
/// let b = || vec![3u8, 4];
/// let c = || vec![5u8, 6];
///
/// assert_eq!(a().append(b().append(c())), a().append(b()).append(c()));
/// ```
pub trait Semigroup {
  /// See [`Semigroup`]
  fn append(self, b: Self) -> Self;
}

/// Monoid extends [`Semigroup`] with the an "identity" value
/// that [`append`](Semigroup.append)ing to a Semigroup will
/// result in the same value.
///
/// # Laws
/// The Monoid identity value must be a no-op on either side
/// of an [`append`](Semigroup.append), e.g.
///
/// ```
/// use naan::prelude::*;
///
/// assert_eq!(String::from("hello").append(String::identity()),
///            String::identity().append(String::from("hello")));
///
/// // Generalized:
/// fn assert_monoid_identity<T>(t: T)
///   where T: PartialEq + core::fmt::Debug + Clone + Monoid
/// {
///   assert_eq!(t.clone().append(T::identity()), t.clone());
///   assert_eq!(T::identity().append(t.clone()), t);
/// }
///
/// assert_monoid_identity(String::from("hello"));
/// assert_monoid_identity(vec!["hello"]);
/// ```
pub trait Monoid: Semigroup {
  /// See [`Monoid`]
  fn identity() -> Self;
}
