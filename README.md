[![crates.io](https://img.shields.io/crates/v/naan.svg)](https://crates.io/crates/naan)
[![docs.rs](https://docs.rs/naan/badge.svg)](https://docs.rs/naan/latest)
![Maintenance](https://img.shields.io/badge/maintenance-activly--developed-brightgreen.svg)

# naan

### deliciously succinct
[naan](https://en.wikipedia.org/wiki/Naan) is a functional programming prelude
for the Rust language that is:
* easy
* useful
* `std`- and `alloc`-optional
* _FAST_ - exclusively uses concrete types (no `dyn`amic dispatch) meaning near-zero perf cost

### Table of Contents
* [higher-kinded types](#higher-kinded-types)
  * [what?](#hkt---what-it-is)
  * [why?](#hkt---why-its-useful)
  * [how?](#hkt---how-its-done)
* [currying](#currying)
  * [what?](#currying---what-it-is)
  * [why?](#currying---why-its-useful)
  * [how?](#currying---how-its-done)
* [function composition](#function-composition)
* [typeclasses](#typeclasses)
  * [`append`, `identity`](#semigroup-and-monoid)
  * [`alt`, `empty`](#alt-and-plus)
  * [`fmap`, `map`](#functor)
  * [`bimap`, `lmap`, `rmap`](#bifunctor)
* lazy IO

### Higher-Kinded Types
#### HKT - What it is
In type theory, it can be useful to have language to differentiate between a concrete type (`u8`, `Vec<u8>`, `Result<File, io::Error>`)
and a generic type without its parameters supplied. (`Vec`, `Option`, `Result`)

For example, `Vec` is a 1-argument (_unary_) type function, and `Vec<u8>` is a concrete type.

Kind refers to how many (if any) parameters a type has.

#### HKT - Why it's useful
In vanilla Rust, `Result::map` and `Option::map` have very similar shapes:
```rust
impl<A, E> Result<A, E> {
  fn map<B>(self, f: impl FnMut(A) -> B) -> Result<B, E>;
}

impl<A> Option<A> {
  fn map<B>(self, f: impl FnMut(A) -> B) -> Option<B>;
}
```
it would be useful (for reasons we'll expand on later) to have them
both implement a `Map` trait:
```rust
trait Map<A> {
  fn map<B>(self: Self<A>, f: impl FnMut(A) -> B) -> Self<B>;
}

impl<A> Map<A> for Option<A> {
 fn map<B>(self, f: impl FnMut(A) -> B) -> Option<B> {
   Option::map(self, f)
 }
}
```
but this code snippet isn't legal Rust because `Self` needs to be generic and in vanilla Rust `Self` must be a concrete type.

#### HKT - How it's done
With the introduction of [Generic associated types](https://blog.rust-lang.org/2022/11/03/Rust-1.65.0.html#generic-associated-types-gats),
we can write a trait that can effectively replace a "generic self" feature.

Now we can actually write the trait above in legal, stable rust:
```rust
trait HKT {
  type Of<A>;
}

struct OptionHKT;
impl HKT for OptionHKT {
  type Of<A> = Option<A>;
}

trait Map<M, A>
  where M: HKT<Of<A> = Self>
{
  fn map<B, F>(self, f: F) -> M::Of<B>
    where F: FnMut(A) -> B;
}

impl<A> Map<OptionHKT, A> for Option<A> {
  fn map<B, F>(self, f: F) -> Option<B>
    where F: FnMut(A) -> B
  {
    Option::map(self, f)
  }
}
```

### Currying
#### Currying - What it is
*Currying* is the technique where `naan` gets its name. Function currying is the strategy of splitting functions that
accept more than one argument into multiple functions.

Example:
```rust
fn foo(String, usize) -> usize;
foo(format!("bar"), 12);
```
would be curried into:
```rust
fn foo(String) -> impl Fn(usize) -> usize;
foo(format!("bar"))(12);
```

#### Currying - Why it's useful
Currying allows us to provide _some_ of a function's arguments and provide the rest of this
partially applied function's arguments at a later date.

This allows us to use functions to store state, and lift functions that accept any number
of parameters to accept Results using [`Apply`](https://docs.rs/naan/latest/naan/apply/trait.Apply.html#example)

<details>
<summary>

**EXAMPLE: reusable function with a stored parameter**
</summary>

```rust
use std::fs::File;

use naan::prelude::*;

fn copy_file_to_dir(dir: String, file: File) -> std::io::Result<()> {
  // ...
  # Ok(())
}

fn main() {
  let dir = std::env::var("DEST_DIR").unwrap();
  let copy = copy_file_to_dir.curry().call(dir);

  File::open("a.txt").bind1(copy.clone())
                     .bind1(|_| File::open("b.txt"))
                     .bind1(copy.clone())
                     .bind1(|_| File::open("c.txt"))
                     .bind1(copy);
}

/*
  equivalent to:
  fn main() {
    let dir = std::env::var("DEST_DIR").unwrap();

    copy_file_to_dir(dir.clone(), File::open("a.txt")?)?;
    copy_file_to_dir(dir.clone(), File::open("b.txt")?)?;
    copy_file_to_dir(dir, File::open("c.txt")?)?;
  }
*/
```
</details>
<details>
<summary>

**EXAMPLE: lifting a function to accept Results (or Options)**
</summary>

```rust
use std::fs::File;

use naan::prelude::*;

fn append_contents(from: File, to: File) -> std::io::Result<()> {
  // ...
  # Ok(())
}

fn main() -> std::io::Result<()> {
  Ok(append_contents.curry()).apply1(File::open("from.txt"))
                             .apply1(File::open("to.txt"))
                             .flatten()
}

/*
equivalent to:
fn main() -> std::io::Result<()> {
  let from = File::open("from.txt")?;
  let to = File::open("to.txt")?;
  append_contents(from, to)
}
*/
```
</details>

##### Currying - How it's done
naan introduces a few new function traits that add
ergonomics around currying and function composition;
`F1`, `F2` and `F3`. These traits extend the builtin function
traits `Fn` and `FnOnce` with methods that allow currying and function
composition.

(note that each arity has a "callable multiple times"
version and a "callable at least once" version. The latter traits are
denoted with a suffix of `Once`)
<details>
<summary>

**`F2` and `F2Once` Definitions**
</summary>

```rust
pub trait F2Once<A, B, C>: Sized {
  /// The concrete type that `curry` returns.
  type Curried;

  /// Call the function
  fn call1(self, a: A, b: B) -> C;

  /// Curry this function, transforming it from
  ///
  /// `fn(A, B) -> C`
  /// to
  /// `fn(A) -> fn(B) -> C`
  fn curry(self) -> Self::Curried;
}

pub trait F2<A, B, C>: F2Once<A, B, C> {
  /// Call the function with all arguments
  fn call(&self, a: A, b: B) -> C;
}

impl<F, A, B, C> F2<A, B, C> for F where F: Fn(A, B) -> C { /* <snip> */ }
impl<F, A, B, C> F2Once<A, B, C> for F where F: FnOnce(A, B) -> C { /* <snip> */ }
```
</details>

#### Function Composition
##### Composition - What it is
Function composition is the strategy of chaining functions sequentially by
automatically passing the output of one function to the input of another.

This very powerful technique lets us concisely express programs in terms of
data that flows through pipes, rather than a sequence of time-bound statements:

```rust
use naan::prelude::*;

struct Apple;
struct Orange;
struct Grape;
#[derive(Debug, PartialEq)]
struct Banana;

fn apple_to_orange(a: Apple) -> Orange {
  Orange
}
fn orange_to_grape(o: Orange) -> Grape {
  Grape
}
fn grape_to_banana(g: Grape) -> Banana {
  Banana
}

fn main() {
  let apple_to_banana = apple_to_orange.chain(orange_to_grape)
                                       .chain(grape_to_banana);
  assert_eq!(apple_to_banana.call(Apple), Banana)
}
```

#### Typeclasses
Some of the most powerful & practical types in programming are locked behind
a feature that many languages choose not to implement in Higher-Kinded Types.

Utilities like `map`, `unwrap_or`, and `and_then` are enormously useful tools
in day-to-day rust that allow us to conveniently skip a lot of hand-written control flow.
<details>
<summary>

**Comparing `and_then` and `map` to their desugared equivalent**
</summary>

```rust
use std::io;

fn network_fetch_name() -> io::Result<String> {
  Ok("harry".into())
}
fn network_send_message(msg: String) -> io::Result<()> {
  Ok(())
}
fn global_state_store_name(name: &str) -> io::Result<()> {
  Ok(())
}

// Declarative
fn foo0() -> io::Result<()> {
  network_fetch_name().and_then(|name| {
                        global_state_store_name(&name)?;
                        Ok(name)
                      })
                      .map(|name| format!("hello, {name}!"))
                      .and_then(network_send_message)
}

// Idiomatic
fn foo1() -> io::Result<()> {
  let name = network_fetch_name()?;
  global_state_store_name(&name)?;
  network_send_message(format!("hello, {name}!"))
}

// Imperative
fn foo2() -> io::Result<()> {
  let name = match network_fetch_name() {
    | Ok(name) => name,
    | Err(e) => return Err(e),
  };

  match global_state_store_name(&name) {
    | Err(e) => return Err(e),
    | _ => (),
  };

  network_send_message(format!("hello, {name}!"))
}
```

A couple notes:
 - the "idiomatic" implementation is the most brief and scannable
 - the idiomatic and imperative implementations are more difficult to refactor due to scope sharing; imperative statements depend on the previous statements in order to be meaningful, while declarative expressions have little to no coupling to state or scope.
</details>

The value proposition of these typeclasses is that they allow us to think of types like Result, Option and Iterators as being abstract **containers**.

We don't need to know much about their internals to know how to use them effectively and productively.

This extremely simple but powerful metaphor allows us to solve some very complex problems with data structures that have
a shared set of interfaces.

#### `Functor`
##### using a function to transform values within a container
`Functor` is the name we give to types that allow us to take a function from `A -> B`
and effectively "penetrate" a type and apply it to some `F<A>`, yielding `F<B>` (`a.fmap(a_to_b)`).

_🔎 This is identical to `Result::map` and `Option::map`._

_🔎 There is a separate trait `FunctorOnce` which extends `Functor` to know that the mapping function will only be called once._

`Functor` is defined as:
```rust
// 🔎 `Self` must be generic over some type `A`
pub trait Functor<F, A> where F: HKT1<T<A> = Self>
{
  // 🔎 given a function `A -> B`,
  // apply it to the values of type `A` in `Self<A>` (if any),
  // yielding `Self<B>`
  fn fmap<AB, B>(self, f: AB) -> F::T<B> where AB: F1<A, B>;
}
```

#### `Bifunctor`
##### mapping types with 2 generic parameters
`Bifunctor` is the name we give to types that have 2 generic parameters,
both of which can be `map`ped.

`Bifunctor` requires:
* `bimap`
  * transforms `T<A, B>` to `T<C, D>`, given a function `A -> C` and another `B -> D`.

`Bifunctor` provides 2 methods:
* `lmap` (map left type)
  * `T<A, B> -> T<C, B>`
* `rmap` (map right type)
  * `T<A, B> -> T<A, D>`

_🔎 There is a separate trait `BifunctorOnce` which extends `Bifunctor` to know that the mapping functions will only be called once._

`Bifunctor` is defined as:
```rust
pub trait Bifunctor<F, A, B>
  where F: HKT2<T<A, B> = Self>
{
  /// 🔎 In Result, this combines `map` and `map_err` into one step.
  fn bimap<A2, B2, FA, FB>(self, fa: FA, fb: FB) -> F::T<A2, B2>
    where FA: F1<A, A2>,
          FB: F1<B, B2>;

  /// 🔎 In Result, this maps the "Ok" type and is equivalent to `map`.
  fn lmap<A2, FA>(self, fa: FA) -> F::T<A2, B>
    where Self: Sized,
          FA: F1<A, A2>
  {
    self.bimap(fa, |b| b)
  }

  /// 🔎 In Result, this maps the "Error" type and is equivalent to `map_err`.
  fn rmap<B2, FB>(self, fb: FB) -> F::T<A, B2>
    where Self: Sized,
          FB: F1<B, B2>
  {
    self.bimap(|a| a, fb)
  }
}
```

#### `Semigroup` and `Monoid`
##### Combining two values of a concrete type
`Semigroup` is the name we give types that support some associative combination
of two values (`a.append(b)`).

_🔎 Associative means `a.append( b.append(c) )` must equal `a.append(b).append(c)`._

Examples:
 * integer addition
   * `1 * (2 * 3) == (1 * 2) * 3`
 * integer multiplication
   * `1 + (2 + 3) == (1 + 2) + 3`
 * string concatenation
   * `"a".append("b".append("c")) == "a".append("b").append("c") == "abc"`
 * `Vec<T>` concatenation
   * `vec![1].append(vec![2].append(vec![3])) == vec![1, 2, 3]`
 * `Option<T>` (only when `T` implements `Semigroup`)
   * `Some("a").append(Some("b")) == Some("ab")`
 * `Result<T, _>` (only when `T` implements `Semigroup`)
   * `Ok("a").append(Ok("b")) == Ok("ab")`

`Monoid` extends `Semigroup` with an "identity" or "empty" value, that will do nothing when appended to another.

Examples:
 * 0 in integer addition
   * `0 + 1 == 1`
 * 1 in integer multiplication
   * `1 * 2 == 2`
 * empty string
   * `String::identity() == ""`
   * `"".append("a") == "a"`
 * `Vec<T>`
   * `Vec::<u32>::identity() == vec![]`
   * `vec![].append(vec![1, 2]) == vec![1, 2]`

These are defined as:
```rust
pub trait Semigroup {
  // 🔎 Note that this can be **any** combination of 2 selves,
  // not just concatenation.
  //
  // The only rule is that implementations have to be associative.
  fn append(self, b: Self) -> Self;
}

pub trait Monoid: Semigroup {
  fn identity() -> Self;
}
```

#### `Alt` and `Plus`
##### Combining two values of a generic type
`Alt` is the name we give to generic types that support an associative operation
on 2 values of the same type (`a.alt(b)`).

_🔎 `Alt` is identical to `Semigroup`, but the implementor is generic._

_🔎 `alt` is identical to `Result::or` and `Option::or`._

Examples:
 * `Vec<T>`
   * `vec![1].alt(vec![2]) == vec![1, 2]`
 * `Result<T, _>`
   * `Ok(1).alt(Err(_)) == Ok(1)`
 * `Option<T>`
   * `None.alt(Some(1)) == Some(1)`

`Plus` extends `Alt` with an "identity" or "empty" value, that will do nothing when `alt`ed to another.

_🔎 `Plus` is identical to `Monoid`, but the implementor is generic._

Examples:
 * `Vec<T>` (`Vec::empty() == vec![]`)
 * `Option<T>` (`Option::empty() == None`)

These are defined as:
```rust
// 🔎 `Self` must be generic over some type `A`.
pub trait Alt<F, A>
  where Self: Functor<F, A>,
        F: HKT1<T<A> = Self>
{
  fn alt(self, b: Self) -> Self;
}

pub trait Plus<F, A>
  where Self: Alt<F, A>,
        F: HKT1<T<A> = Self>
{
  fn empty() -> F::T<A>;
}
```

### Lazy IO

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
