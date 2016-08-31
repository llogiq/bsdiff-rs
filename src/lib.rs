#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;
extern crate bzip2;
extern crate brotli;

pub mod errors;
//mod bsdiff;
pub mod bspatch;
