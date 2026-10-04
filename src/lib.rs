#![no_std]
#![doc = include_str!("../README.md")]

extern crate alloc;

mod chunk;
mod deque;

pub use deque::Deque;
pub use deque::iter::*;
