mod encoding;
mod string;
mod table;
mod userdata;

use std::{
    cmp,
    cmp::{Eq, PartialEq},
    hash::{self, Hash},
};

use encoding::*;
pub use string::ByteString;

use super::gc::{Handle, TaggedHandle, Trace, Visitor};

#[derive(PartialEq)]
enum ValueType {
    Nil,
    Bool,
    Int,
    Float,
    Table,
    String,
    Function,
    Userdata,
}

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
            #[allow(unused_unsafe)]
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
        Value {
            data: make_float(x),
        }
    }

    pub fn from_table(x: *mut u8) -> Self {
        Value {
            data: make_table(x),
        }
    }

    pub fn from_string(x: Handle<ByteString>) -> Self {
        Value {
            data: make_string(x.as_ptr() as *mut u8),
        }
    }

    pub fn from_function(x: *mut u8) -> Self {
        Value {
            data: make_function(x),
        }
    }

    pub fn from_userdata(x: *mut u8) -> Self {
        Value {
            data: make_userdata(x),
        }
    }

    fn ty(self) -> ValueType {
        dispatch!(self.data,
            is_int => ValueType::Int,
            is_bool => ValueType::Bool,
            is_nil => ValueType::Nil,
            is_table => ValueType::Table,
            is_string => ValueType::String,
            is_float => ValueType::Float,
            is_function => ValueType::Function,
            is_userdata => ValueType::Userdata
        )
    }

    pub fn op_eq(self, other: Self) -> bool {
        let ty_1 = self.ty();
        let ty_2 = other.ty();

        if ty_1 != ty_2 {
            return false;
        }

        match ty_1 {
            _ => todo!()
        }
    }

    pub fn op_hash(self) -> u64 {
        (self.data >> 3).wrapping_add(1099511628211)
    }
}

impl Trace for Value {
    fn visit(&self, visitor: &mut Visitor) {
        todo!()
    }
}
