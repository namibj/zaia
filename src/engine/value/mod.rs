use std::{cmp, hash};
use super::gc::{Handle, Trace, Visitor};
mod encoding;

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
#[derive(Clone)]
pub struct Value {
    data: u64,
}
