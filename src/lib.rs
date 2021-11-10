//! This crate implements a 256-bit unsigned integer type.
//!
//! The implementation tries to follow as closely as possible to primitive
//! integer types, and should implement all the common methods and traits as the
//! primitive integer types.

#![deny(missing_docs)]
#![no_std]

#[cfg(test)]
extern crate alloc;

mod cmp;
mod int;
mod fmt;
pub mod intrinsics;
mod iter;
mod ops;
mod uint;

pub use self::{int::*, uint::*};
