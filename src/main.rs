#![deny(missing_docs)]
#![feature(proc_macro)]
//! Example crate showing doc issues.

extern crate proc_macro;

#[cfg(feature = "case-one")]
pub mod load_one;

#[cfg(feature = "case-two")]
pub mod load_two;

#[cfg(feature = "case-three")]
pub mod load_three;

#[cfg(feature = "case-four")]
pub mod load_four;


fn main() {
    println!("Hello, world!");
}
