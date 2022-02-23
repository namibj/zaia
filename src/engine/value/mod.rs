use std::{cmp, hash};
use super::gc::{Handle, Trace, Visitor};
mod encoding;

use encoding::*;

// Customized match using NaN-boxing type guards.
//
// For optimal code generation the dispatch order should be:
//   - int
//   - bool
//   - nil
//   - table
//   - string
//   - float
//   - function
//   - userdata
macro_rules! dispatch {
    ($x:expr, $($guard:ident => $arm:expr),*) => {{
        match $x {
            $(v if $guard(v) => $arm),*,
            _ => unsafe {
                #[cfg(debug_assertions)]
                unreachable!();

                #[cfg(not(debug_assertions))]
                std::hint::unreachable_unchecked();
            },
        }
    }}
}

// Value represents runtime values such as integers and strings.
// This uses a complex format loosely based off NaN-boxing.
//
// We define the following types:
// Value
//   - Nil
//   - True
//   - False
//   - Integer: a signed 32-bit integer
//   - Float: a 64-bit IEEE-754 floating point number
//   - Object
//     - Table: a Lua table
//     - String: a heap-allocated UTF-8 string
//     - Function: a Lua function, possibly with captured upvalues
//     - Userdata: a custom type defined outside of Lua
#[derive(Clone, Copy)]
pub struct Value {
    data: u64,
}

impl Value {
    pub fn from_nil() -> Self {
        Value { data: make_nil() }
    }

    pub fn from_bool(x: bool) -> Self {
        Value { data: make_bool(x) }
    }

    pub fn from_int(x: i32) -> Self {
        Value { data: make_int(x) }
    }

    pub fn from_float(x: f64) -> Self {
        Value { data: make_float(x) }
    }

    pub fn from_table(x: *mut u8) -> Self {
        Value { data: make_table(x) }
    }

    pub fn from_string(x: *mut u8) -> Self {
        Value { data: make_string(x) }
    }

    pub fn from_function(x: *mut u8) -> Self {
        Value { data: make_function(x) }
    }

    pub fn from_userdata(x: *mut u8) -> Self {
        Value { data: make_userdata(x) }
    }
}