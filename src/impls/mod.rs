/// the Identity monad
pub mod identity;

/// Option trait impls
pub mod option;

/// Result trait impls
pub mod result;

/// String trait impls
pub mod string;

/// Unit trait impls
pub mod unit;

/// Vec trait impls
pub mod vec;

/// [`std::collections::HashMap`]
pub mod hash_map;

/// [`tinyvec`]
#[cfg(feature = "tinyvec")]
pub mod tinyvec;
