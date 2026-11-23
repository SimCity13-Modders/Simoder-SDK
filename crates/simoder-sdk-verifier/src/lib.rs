#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! Offline SDK declaration verifier, Beta contract matrix validator (`#1`), and
//! declaration-vs-implementation conformance checker (`#2`) for `Simoder-SDK`.

pub mod conformance;
pub mod manifest;
pub mod matrix;

pub use {conformance::*, manifest::*, matrix::*};
