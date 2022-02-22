use std::{cmp, hash};
use super::gc::{Handle, Trace, Visitor};

#[cfg(target_endian = "big")]
compile_error!("zaia does not yet support big-endian platforms");

fn is_smi(x: usize) -> bool {
    return false
}

// Value represents runtime values such as integers and strings.
// This uses a complex format loosely based off NaN-boxing.
//
// We define the following types:
// Value
//   - Integer: a signed 31-bit integer
//   - Float: a 32-bit IEEE-754 floating point number
//   - Object
//     - String: a heap-allocated UTF-8 string
//     - Table: a Lua table
//     - Function: a Lua function, possibly with captured upvalues
//     - Userdata: a custom type defined outside of Lua
#[derive(Clone)]
pub struct Value {
    data: usize,
}

impl Value {
    pub fn integer(value: i32) -> Self {
        todo!()
    }

    pub fn float(value: f32) -> Self {
        todo!()
    }
}