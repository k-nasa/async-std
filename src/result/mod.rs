//! The Rust core error handling type
//!
//! This module provides the `Result<T, E>` type for returning and
//! propagating errors.

mod from_stream;

#[doc(inline)]
pub use core::result::Result;

cfg_unstable! {
    mod product;
    mod sum;
}
