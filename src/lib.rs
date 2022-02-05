#![feature(allocator_api)]
#![feature(cell_update)]
#![feature(get_mut_unchecked)]
#![allow(dead_code)]

pub mod parser;
mod runtime;
pub mod syntax_tree;
mod utf8;
