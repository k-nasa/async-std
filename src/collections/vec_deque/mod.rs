//! The Rust double-ended queue, implemented with a growable ring buffer.

mod extend;
mod from_stream;

#[doc(inline)]
pub use core::collections::VecDeque;
