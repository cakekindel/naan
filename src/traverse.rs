use crate::prelude::*;

/// Sequence extends [`Traversable`] with a function that inverts collection
/// of Applicatives into an Applicative of a collection.
///
/// e.g. `Vec<Result<T, E>>` to `Result<Vec<T>, E>`
///
/// ```
/// use std::fs::{DirEntry, File, FileType};
/// use std::io;
/// use std::path::{Path, PathBuf};
///
/// use naan::prelude::*;
///
/// fn is_dir(ent: &DirEntry) -> bool {
///   ent.file_type()
///      .fmap(|ft: FileType| ft.is_dir())
///      .unwrap_or(false)
/// }
///
/// fn is_file(ent: &DirEntry) -> bool {
///   ent.file_type()
///      .fmap(|ft: FileType| ft.is_file())
///      .unwrap_or(false)
/// }
///
/// /// Recursively search directories and their children
/// /// for all files that match a predicate
/// fn find_all_rec<P: AsRef<Path>, F: Fn(PathBuf) -> io::Result<bool>>(
///   path: P,
///   f: F)
///   -> io::Result<Vec<PathBuf>> {
///   let find_matches = |matches: Vec<io::Result<Vec<PathBuf>>>, ent: DirEntry| {
///     let path_if_found = |found| {
///       if found {
///         vec![ent.path()]
///       } else {
///         vec![]
///       }
///     };
///
///     if is_file(&ent) {
///       matches.append_one(f(ent.path()).fmap(path_if_found))
///     } else if is_dir(&ent) {
///       matches.append_one(find_all_rec(ent.path(), &f))
///     } else {
///       matches
///     }
///   };
///
///   std::fs::read_dir(path).and_then(|dir| dir.into_iter().collect::<io::Result<Vec<DirEntry>>>())
///                          .and_then(|ents| {
///                            ents.foldl(find_matches, vec![])
///                                .sequence::<hkt::ResultOk<_>>()
///                          })
///                          .fmap(|vv: Vec<Vec<_>>| vv.concat())
/// }
///
/// /// ...or using try syntax:
/// fn find_all_rec2<P: AsRef<Path>, F: Fn(PathBuf) -> io::Result<bool>>(
///   path: P,
///   f: F)
///   -> io::Result<Vec<PathBuf>> {
///   let find_matches = |matches: Vec<io::Result<Vec<PathBuf>>>, ent: DirEntry| {
///     let path_if_found = |found| {
///       if found {
///         vec![ent.path()]
///       } else {
///         vec![]
///       }
///     };
///
///     if is_file(&ent) {
///       matches.append_one(f(ent.path()).fmap(path_if_found))
///     } else if is_dir(&ent) {
///       matches.append_one(find_all_rec(ent.path(), &f))
///     } else {
///       matches
///     }
///   };
///
///   let dir = std::fs::read_dir(path)?;
///   let ents = dir.into_iter().collect::<io::Result<Vec<DirEntry>>>()?;
///   let out: Vec<Vec<PathBuf>> = ents.foldl(find_matches, Vec::<io::Result<Vec<PathBuf>>>::new())
///                                    .sequence::<hkt::ResultOk<_>>()?;
///
///   Ok(out.concat())
/// }
/// ```
pub trait Sequence<F, A, TF> {
  /// See [`Sequence`]
  fn sequence<Ap>(self) -> Ap::T<F::T<A>>
    where Self: Sized + Traversable<F, Ap::T<A>, A, TF> + Foldable<F, Ap::T<A>>,
          Ap: HKT1,
          Ap::T<A>: Applicative<Ap, A> + ApplyOnce<Ap, A>,
          Ap::T<TF>: Applicative<Ap, TF> + ApplyOnce<Ap, TF>,
          Ap::T<F::T<A>>: Applicative<Ap, F::T<A>> + ApplyOnce<Ap, F::T<A>>,
          F: HKT1<T<Ap::T<A>> = Self>
  {
    self.traversem1::<Ap, _>(|a| a)
  }
}

impl<F, A, TF, T> Sequence<F, A, TF> for T {}

/// [`Traversable`]'s signatures that guarantee the traversal
/// function will only ever be called once.
pub trait TraversableOnce<F, A, B, TF>: Traversable<F, A, B, TF> {
  /// Traverse a structure with 0 or 1 elements, collecting into
  /// an Applicative of 0 or 1 elements.
  ///
  /// ```
  /// use std::fs::File;
  ///
  /// use naan::prelude::*;
  ///
  /// fn maybe_path() -> Option<&'static str> {
  ///   None
  /// }
  ///
  /// let tried_read: std::io::Result<Option<File>> =
  ///   maybe_path().traverse11::<hkt::io::Result, _>(File::open);
  ///
  /// assert!(matches!(tried_read, Ok(None)))
  /// ```
  fn traverse11<Ap, AtoApOfB>(self, f: AtoApOfB) -> Ap::T<F::T<B>>
    where Ap: HKT1,
          Self: FoldableOnce<F, A>,
          Ap::T<B>: Applicative<Ap, B> + ApplyOnce<Ap, B>,
          Ap::T<TF>: Applicative<Ap, TF> + ApplyOnce<Ap, TF>,
          Ap::T<F::T<B>>: Applicative<Ap, F::T<B>> + ApplyOnce<Ap, F::T<B>>,
          AtoApOfB: F1Once<A, Ap::T<B>>,
          F: HKT1<T<A> = Self>;

  /// Traverse a structure with 0 or 1 elements, collecting into
  /// an Applicative of 0 or more elements.
  fn traverse1m<Ap, AtoApOfB>(self, f: AtoApOfB) -> Ap::T<F::T<B>>
    where Self: FoldableOnce<F, A>,
          Ap: HKT1,
          Ap::T<B>: Applicative<Ap, B>,
          Ap::T<TF>: Applicative<Ap, TF>,
          Ap::T<F::T<B>>: Applicative<Ap, F::T<B>>,
          AtoApOfB: F1Once<A, Ap::T<B>>,
          F: HKT1<T<A> = Self>;

  /// Aliases [`TraversableOnce::traverse11`]
  fn traverse_swap<Ap, AtoApOfB>(self, f: AtoApOfB) -> Ap::T<F::T<B>>
    where Ap: HKT1,
          Self: Sized + FoldableOnce<F, A>,
          Ap::T<B>: Applicative<Ap, B> + ApplyOnce<Ap, B>,
          Ap::T<TF>: Applicative<Ap, TF> + ApplyOnce<Ap, TF>,
          Ap::T<F::T<B>>: Applicative<Ap, F::T<B>> + ApplyOnce<Ap, F::T<B>>,
          AtoApOfB: F1Once<A, Ap::T<B>>,
          F: HKT1<T<A> = Self>
  {
    self.traverse11::<Ap, AtoApOfB>(f)
  }

  /// Aliases [`TraversableOnce::traverse1m`]
  fn traverse_replicate<Ap, AtoApOfB>(self, f: AtoApOfB) -> Ap::T<F::T<B>>
    where Ap: HKT1,
          Self: Sized + FoldableOnce<F, A>,
          Ap::T<B>: Applicative<Ap, B>,
          Ap::T<TF>: Applicative<Ap, TF>,
          Ap::T<F::T<B>>: Applicative<Ap, F::T<B>>,
          AtoApOfB: F1Once<A, Ap::T<B>>,
          F: HKT1<T<A> = Self>
  {
    self.traverse1m::<Ap, AtoApOfB>(f)
  }
}

/// A Traversable structure is one that can be folded and collected into some [`Applicative`].
///
/// Practically, this allows you to swap nested types, e.g.
///  * apply `FnOnce(T) -> Option<R>` to `Result<T, E>`, yielding `Option<Result<R, E>>`
///  * apply `Fn(T) -> Result<R, E>` to `Vec<T>`, yielding `Result<Vec<R>, E>`
///
/// # Use-cases
/// ## Traverse a structure of length 1 into an Ap of length 1
/// In effect, this swaps the order of types after applying the function.
///
/// e.g. traversing `Option<A>` with a function `Fn(A) -> Result<B, E>` will result in
/// `Result<Option<B>, E>`.
///
/// * `None` -> `Ok(None)` -> `Ok(None)`
/// * `Some(a)` -> `Ok(b)` -> `Ok(Some(b))`
/// * `Some(a)` -> `Err(e)` -> `Err(e)`
///
/// ## Traverse a structure of length 1 into an Ap of length > 1
/// In effect, this will perform a sort of "replication" of the variant wrapping `A`:
///
/// e.g. traversing `Option<A>` with a function `Fn(A) -> Vec<B>` will result in
/// `Vec<Option<B>>`.
///
/// * `None` -> `vec![1, 2, 3]` -> `vec![None, None, None]`
/// * `Some(a)` -> `vec![1, 2, 3]` -> `vec![Some(1), Some(2), Some(3)]`
///
/// ## Traverse a structure of length > 1 into an Ap of length 1
/// By far the most practically useful `traverse` usecase,
/// this allows you to perform an action that returns `Result`, `Option`, etc. and
/// fail as soon as the first `Err`, `None` is encountered.
///
/// e.g. traversing `Vec<A>` with a function `Fn(A) -> Result<B, E>` will result in
/// `Result<Vec<B>>`.
///
/// * `vec![1, 2, 3]` -> `vec![Ok(1), Err(e), ..]` -> `Err(e)`
/// * `vec![a, b, c]` -> `vec![Ok(a), Ok(b), Ok(c)]` -> `Ok(vec![a, b, c])`
///
/// **Note**: the traversal function must be callable multiple times. (must implement [`F1`])
///
/// ## Traverse a structure of length > 1 into an Ap of length > 1
/// Arguably the least useful `traverse` usecase, this performs a cross-product between
/// the two collections.
///
/// <details>
///
/// e.g. traversing `Vec<A>` with a function `Fn(A) -> Vec<B>` will result in
/// a cross-product `Vec<Vec<B>>`.
///
/// 1. Input `vec![2, 4]`
/// 2. Traverse function `f` of `|n| vec![n * 10, n * 20]`
/// 3. Initial output of `Vec::<Vec<B>>::new(Vec::<B>::new())`
/// 4. Intermediate value of `vec![20, 40]` after applying `f` to `2`
/// 5. Map the intermediate to be a vec of _appending actions_ to run against each element of output `vec![append(20), append(40)]`
/// 6. Invoke each append fn on each element in output (currently `vec![vec![]]`) -> `vec![vec![20], vec![40]]`
/// 7. Repeat for each element
/// 8. Observed output is `vec![vec![20, 40], vec![20, 80], vec![40, 40], vec![40, 80]]`
/// </details>
///
/// **Note**: the traversal function must be callable multiple times. (must implement [`F1`])
///
/// **Note**: each value of type `B` appears many times in the return value, so it must be [`Clone`].
///
/// # `traverse` `m1`/`mm`/`1m`/`11`
/// In other languages and environments, `traverse` makes no distinction between the
/// lengths of the Traversable or the [`Applicative`] output, and uses the final usecase mentioned above
/// (_Traverse a structure of length > 1 into an Ap of length > 1_) as the general `traverse`
/// implementation.
///
/// A major consideration is that in Rust is that we must tell the compiler:
///  * when we need to deep-clone values
///  * when we need a function to be callable more than once
///
/// Using a single general traverse implementation would require you to clone
/// values captured in closures (because they must be callable many times) and
/// prevent you from returning many useful types that are not [`Clone`], e.g. [`std::fs::File`].
///
/// In order to provide the best UX around FnOnce-ness and Clone-ness, naan provides
/// specific signatures for each usecase:
///
/// |                         | Traversable length == 1 | Traversable length > 1 |
/// | - | - | - |
/// | Applicative length == 1 | [`traverse11`](TraversableOnce::traverse11)  | [`traversem1`](Traversable::traversem1)|
/// | Applicative length > 1  | [`traverse1m`](TraversableOnce::traverse1m)  | [`traversemm`](Traversable::traversemm)|
///
/// Which translates to extra trait requirements:
///
/// |                         | Traversable length == 1 | Traversable length > 1 |
/// | - | - | - |
/// | Applicative length == 1 | None                    | function must be [`F1`] |
/// | Applicative length > 1  | None                    | function must be [`F1`], `B` must be [`Clone`] |
///
/// Additionally, we provide aliases to better describe the problem each signature solves:
///  * `traverse11` -> `traverse_swap`
///  * `traverse1m` -> `traverse_replicate`
///  * `traversem1` -> `traverse`
pub trait Traversable<F, A, B, TF> {
  /// Traverse a structure with 0 or more elements, collecting into
  /// an Applicative of 0 or 1 elements.
  ///
  /// ```
  /// use naan::prelude::*;
  ///
  /// // Mock std::fs::File
  /// pub struct File;
  /// impl File {
  ///   pub fn open(path: &str) -> std::io::Result<Self> {
  ///     if path == "doesnt-exist.txt" {
  ///       Err(std::io::Error::new(std::io::ErrorKind::NotFound, ""))
  ///     } else {
  ///       Ok(Self)
  ///     }
  ///   }
  /// }
  ///
  /// let paths = vec!["a.txt", "b.txt", "c.txt"];
  /// let files: std::io::Result<Vec<File>> = paths.traversem1::<hkt::io::Result, _>(File::open);
  /// assert!(matches!(files.as_ref().map(|v| v.as_slice()), Ok(&[_, _, _])));
  ///
  /// let paths = vec!["a.txt", "doesnt-exist.txt", "c.txt"];
  /// let files: std::io::Result<Vec<File>> = paths.traversem1::<hkt::io::Result, _>(File::open);
  /// assert!(matches!(files.map_err(|e| e.kind()),
  ///                  Err(std::io::ErrorKind::NotFound)));
  /// ```
  fn traversem1<Ap, AtoApOfB>(self, f: AtoApOfB) -> Ap::T<F::T<B>>
    where Ap: HKT1,
          Self: Foldable<F, A>,
          Ap::T<B>: Applicative<Ap, B> + ApplyOnce<Ap, B>,
          Ap::T<TF>: Applicative<Ap, TF> + ApplyOnce<Ap, TF>,
          Ap::T<F::T<B>>: Applicative<Ap, F::T<B>> + ApplyOnce<Ap, F::T<B>>,
          AtoApOfB: F1<A, Ap::T<B>>,
          F: HKT1<T<A> = Self>;

  /// Traverse a structure with 0 or more elements, collecting into
  /// an Applicative of 0 or more elements.
  fn traversemm<Ap, AtoApOfB>(self, f: AtoApOfB) -> Ap::T<F::T<B>>
    where Ap: HKT1,
          B: Clone,
          Self: Foldable<F, A>,
          Ap::T<B>: Applicative<Ap, B>,
          Ap::T<TF>: Applicative<Ap, TF>,
          Ap::T<F::T<B>>: Applicative<Ap, F::T<B>>,
          AtoApOfB: F1<A, Ap::T<B>>,
          F: HKT1<T<A> = Self>;

  /// Aliases [`Traversable::traversem1`]
  fn traverse<Ap, AtoApOfB>(self, f: AtoApOfB) -> Ap::T<F::T<B>>
    where Ap: HKT1,
          Self: Sized + Foldable<F, A>,
          Ap::T<B>: Applicative<Ap, B> + ApplyOnce<Ap, B>,
          Ap::T<TF>: Applicative<Ap, TF> + ApplyOnce<Ap, TF>,
          Ap::T<F::T<B>>: Applicative<Ap, F::T<B>> + ApplyOnce<Ap, F::T<B>>,
          AtoApOfB: F1<A, Ap::T<B>>,
          F: HKT1<T<A> = Self>
  {
    self.traversem1::<Ap, AtoApOfB>(f)
  }
}
