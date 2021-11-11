//! This crate implements a 256-bit unsigned integer type.
//!
//! The implementation tries to follow as closely as possible to primitive
//! integer types, and should implement all the common methods and traits as the
//! primitive integer types.

#![deny(missing_docs)]
#![no_std]

#[cfg(test)]
extern crate alloc;

#[macro_use]
mod macros {
    #[macro_use]
    pub mod cmp;
    #[macro_use]
    pub mod ops;
}

mod error;
mod fmt;
mod int;
pub mod intrinsics;
mod iter;
mod ops;
mod ops2;
mod uint;

pub use self::{int::*, uint::*};
