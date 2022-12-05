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

### new problem-solving tools
* higher-kinded types
* currying
* function composition
* new, general typeclasses
* lazy IO

All of this is made possible with a trick using [Generic associated types](https://blog.rust-lang.org/2022/11/03/Rust-1.65.0.html#generic-associated-types-gats)
to emulate [**_Kinds_**](https://en.wikipedia.org/wiki/Kind_(type_theory))

#### HKTs
##### What it is
In type theory, it can be useful to have language to differentiate between a concrete type (`u8`, `Vec<u8>`, `Result<File, io::Error>`)
and a generic type without its parameters supplied. (`Vec`, `Option`, `Result`)

For example, `Vec` is a 1-argument (_unary_) type function, and `Vec<u8>` is a concrete type.

Kind refers to how many (if any) parameters a type has.

##### Why it's useful
In vanilla Rust, `Result::map` and `Option::map` have very similar shapes:
```
impl<A, E> Result<A, E> {
  fn map<B>(self, f: impl FnMut(A) -> B) -> Result<B, E>;
}

impl<A> Option<A> {
  fn map<B>(self, f: impl FnMut(A) -> B) -> Option<B>;
}
```
it would be useful (for reasons we'll expand on later) to have them
both implement a `Map` trait:
```
trait Map<A> {
  fn map<B>(self: Self<A>, f: impl FnMut(A) -> B) -> Self<B>;
}
```
but this code snippet isn't legal Rust because `Self` needs to be generic (kind `* -> *`)
and in vanilla Rust `Self` must be a concrete type.

##### How it's done
With the introduction of [Generic associated types](https://blog.rust-lang.org/2022/11/03/Rust-1.65.0.html#generic-associated-types-gats),
we can write a "type function of kind `* -> *`" trait (here called `HKT`).

Using this we can implement `HKT` for `Option`, `Result`, or any `Self` _essentially_ generic by tying it to
and write the `Map` trait from above in legal Rust:

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
    self.map(f)
  }
}
```

#### Currying
##### What it is
*Currying* is the technique where `naan` gets its name. Function currying is the strategy of splitting functions that
accept more than one argument into functions that return functions.

Concrete example:
```
fn foo(String, usize) -> usize;
foo(format!("bar"), 12);
```
would be curried into:
```
fn foo(String) -> impl Fn(usize) -> usize;
foo(format!("bar"))(12);
```

##### Why it's useful
Currying allows us to provide _some_ of a function's arguments and provide the rest of this
partially applied function's arguments at a later date.

This allows us to use functions to store state, and lift functions that accept any number
of parameters to accept Results using [`Apply`](https://docs.rs/naan/latest/naan/apply/trait.Apply.html#example)

**EXAMPLE: reusable function with a stored parameter**
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

**EXAMPLE: lifting a function to accept Results (or Options)**
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

#### Function Composition

#### Typeclasses

#### Lazy IO

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
