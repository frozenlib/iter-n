#![no_std]

//! A utility for functions returning `impl Iterator` to return one of several distinct types.
//!
//! # Examples
//!
//! `use iter_n::iter2::*` must be placed in function scope, not in module scope.
//!
//! Since `iter_n::iter2`, `iter_n::iter3`, etc. define methods of the same name, if multiple `use iter_n::iter{N}::*;` are placed in the module scope, there will be a conflict with the methods.
//!
//! ```
//! fn f(x: i32) -> impl Iterator<Item = i32> {
//!     use iter_n::iter2::*;
//!     if x % 2 == 0 {
//!         [0, 1].iter().map(|y| y + 1).into_iter0()
//!     } else {
//!         [0, 1].iter().map(|y| y + 2).into_iter1()
//!     }
//! }
//! ```
//!
//! ```
//! fn g(x: i32) -> impl Iterator<Item = i32> {
//!     use iter_n::iter3::*;
//!     if x % 3 == 0 {
//!         [0, 1].iter().map(|y| y + 1).into_iter0()
//!     } else if x % 3 == 0 {
//!         [0, 1].iter().map(|y| y + 2).into_iter1()
//!     } else {
//!         [0, 1].iter().map(|y| y + 3).into_iter2()
//!     }
//! }
//! ```
mod generate;
pub use generate::*;
