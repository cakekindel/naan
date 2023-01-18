/// the Identity monad
pub mod identity;

/// Option trait impls
pub mod option;

/// Result trait impls
pub mod result;

/// String trait impls
#[cfg(feature = "alloc")]
pub mod string;

/// Unit trait impls
pub mod unit;

/// Vec trait impls
#[cfg(feature = "alloc")]
pub mod vec;

/// [`std::collections::HashMap`]
#[cfg(feature = "std")]
pub mod hash_map;

/// [`std::collections::BTreeMap`]
#[cfg(feature = "alloc")]
pub mod btree_map;

/// [`tinyvec`]
#[cfg(feature = "tinyvec")]
pub mod tinyvec;
