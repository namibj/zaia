use std::{cmp, hash};
use super::gc::{Handle, Trace, Visitor};

#[cfg(not(target_endian = "little"))]
compile_error!("zaia currently only supports little-endian platforms");

// The NaN-boxing encoding used is loosely based off https://piotrduperas.com/posts/nan-boxing which is in turned based off SpiderMonkey.
// TODO: Investigate WebKit Strategy https://brionv.com/log/2018/05/17/javascript-engine-internals-nan-boxing/

const BOOL_MASK: u64 = 0x7FFE000000000002;
const INTEGER_MASK: u64 = 0x7FFC000000000000;
const FLOAT_MASK: u64 = 0xFFFF000000000000;
const TABLE_MASK: u64 = 0xFFFC000000000000;
const STRING_MASK: u64 = 0xFFFE000000000000;

const NIL_VALUE: u64 = 0x7FFE000000000000;
const TRUE_VALUE: u64 = BOOL_MASK | 3;
const FALSE_VALUE: u64 = BOOL_MASK | 2;

fn is_nil(x: u64) -> bool {
    x == NIL_VALUE
}

fn is_bool(x: u64) -> bool {
    (x & BOOL_MASK) == BOOL_MASK
}

fn is_int(x: u64) -> bool {
    (x & FLOAT_MASK) == INTEGER_MASK
}

fn is_float(x: u64) -> bool {
    (x & FLOAT_MASK) != FLOAT_MASK
}

fn is_table(x: u64) -> bool {
    (x & FLOAT_MASK) == TABLE_MASK
}

fn is_string(x: u64) -> bool {
    (x & FLOAT_MASK) == STRING_MASK
}

fn is_function(x: u64) -> bool {
    todo!()
}

fn is_userdata(x: u64) -> bool {
    todo!()
}

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

impl Value {
    pub fn integer(value: i32) -> Self {
        todo!()
    }

    pub fn float(value: f64) -> Self {
        todo!()
    }
}
