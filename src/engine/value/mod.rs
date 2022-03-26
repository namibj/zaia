pub mod encoding;
mod function;
mod string;
mod table;
mod userdata;

use std::cmp::PartialEq;

use encoding::*;
pub use function::Function;
pub use string::ByteString;
pub use table::Table;
pub use userdata::Userdata;

use super::{
    gc::{Handle, TaggedHandle, Trace, Visitor},
    util::mix_u64,
    vm::ctx::Ctx,
};

#[derive(Debug, PartialEq)]
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

fn int_op(iop: fn(i32, i32) -> i32, a: Value, b: Value) -> Value {
    if a.ty() != ValueType::Int || b.ty() != ValueType::Int {
        panic!("int_op: invalid types");
    }

    Value::from_int(iop(get_int(a.data), get_int(b.data)))
}

fn arith_op(iop: fn(i32, i32) -> Value, fop: fn(f64, f64) -> Value, a: Value, b: Value) -> Value {
    if a.ty() == ValueType::Float || b.ty() == ValueType::Float {
        fop(a.convert_float(), b.convert_float())
    } else {
        iop(a.cast_int(), b.cast_int())
    }
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
#[derive(Clone, Copy, PartialEq)]
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

    fn cast_string_unchecked<'a>(self) -> &'a ByteString {
        unsafe { &*(get_string(self.data) as *const ByteString) }
    }

    pub fn cast_bool_unchecked(&self) -> bool {
        get_bool(self.data)
    }

    fn cast_table_unchecked<'a>(self) -> &'a Table {
        unsafe { &*(get_table(self.data) as *const Table) }
    }

    pub fn is_truthy(self) -> bool {
        match self.ty() {
            ValueType::Nil => false,
            ValueType::Bool => get_bool(self.data),
            _ => true,
        }
    }

    pub fn convert_float(self) -> f64 {
        match self.ty() {
            ValueType::Float => get_float(self.data),
            ValueType::Int => get_int(self.data) as f64,
            _ => panic!("cannot convert to float"),
        }
    }

    pub fn cast_int(self) -> i32 {
        match self.ty() {
            ValueType::Int => get_int(self.data),
            _ => panic!("value is not int"),
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

    pub fn op_eq(self, other: Self) -> Value {
        Value::from_bool(self.data == other.data)
    }

    pub fn op_gt(self, other: Self) -> Value {
        let ty_1 = self.ty();
        let ty_2 = other.ty();

        if ty_1 != ty_2 {
            return Value::from_bool(false);
        }

        Value::from_bool(match ty_1 {
            ValueType::Int => get_int(self.data) > get_int(other.data),
            ValueType::Float => get_float(self.data) > get_float(other.data),
            ValueType::String => **self.cast_string_unchecked() > **other.cast_string_unchecked(),
            _ => panic!("attempted op_gt on unsupported type: {:?}", ty_1),
        })
    }

    pub fn op_lt(self, other: Self) -> Value {
        let ty_1 = self.ty();
        let ty_2 = other.ty();

        if ty_1 != ty_2 {
            return Value::from_bool(false);
        }

        Value::from_bool(match ty_1 {
            ValueType::Int => get_int(self.data) < get_int(other.data),
            ValueType::Float => get_float(self.data) < get_float(other.data),
            ValueType::String => **self.cast_string_unchecked() < **other.cast_string_unchecked(),
            _ => panic!("attempted op_lt on unsupported type: {:?}", ty_1),
        })
    }

    pub fn op_and(self, other: Self) -> Self {
        Value::from_bool(self.is_truthy() && other.is_truthy())
    }

    pub fn op_or(self, other: Self) -> Self {
        Value::from_bool(self.is_truthy() || other.is_truthy())
    }

    pub fn op_add(self, other: Self) -> Self {
        arith_op(
            |a, b| Value::from_int(a + b),
            |a, b| Value::from_float(a + b),
            self,
            other,
        )
    }

    pub fn op_sub(self, other: Self) -> Self {
        arith_op(
            |a, b| Value::from_int(a - b),
            |a, b| Value::from_float(a - b),
            self,
            other,
        )
    }

    pub fn op_mul(self, other: Self) -> Self {
        arith_op(
            |a, b| Value::from_int(a * b),
            |a, b| Value::from_float(a * b),
            self,
            other,
        )
    }

    pub fn op_div(self, other: Self) -> Self {
        arith_op(
            |a, b| Value::from_float(a as f64 / b as f64),
            |a, b| Value::from_float(a / b),
            self,
            other,
        )
    }

    pub fn op_int_div(self, other: Self) -> Self {
        arith_op(
            |a, b| Value::from_int(a / b),
            |a, b| Value::from_int((a / b).floor() as i32),
            self,
            other,
        )
    }

    pub fn op_exp(self, other: Self) -> Self {
        arith_op(
            |a, b| Value::from_int(a.pow(b as u32)),
            |a, b| Value::from_float(a.powf(b)),
            self,
            other,
        )
    }

    pub fn op_mod(self, other: Self) -> Self {
        arith_op(
            |a, b| Value::from_int(a % b),
            |a, b| Value::from_float(a % b),
            self,
            other,
        )
    }

    pub fn op_bit_and(self, other: Self) -> Self {
        int_op(|a, b| a & b, self, other)
    }

    pub fn op_bit_or(self, other: Self) -> Self {
        int_op(|a, b| a | b, self, other)
    }

    pub fn op_lshift(self, other: Self) -> Self {
        int_op(|a, b| a << b, self, other)
    }

    pub fn op_rshift(self, other: Self) -> Self {
        int_op(|a, b| a >> b, self, other)
    }

    pub fn op_bit_xor(self, other: Self) -> Self {
        int_op(|a, b| a ^ b, self, other)
    }

    pub fn op_neq(self, other: Self) -> Self {
        Value::from_bool(!get_bool(self.op_eq(other).data))
    }

    pub fn op_leq(self, other: Self) -> Self {
        let ty_1 = self.ty();
        let ty_2 = other.ty();

        if ty_1 != ty_2 {
            return Value::from_bool(false);
        }

        Value::from_bool(match ty_1 {
            ValueType::Int => get_int(self.data) <= get_int(other.data),
            ValueType::Float => get_float(self.data) <= get_float(other.data),
            ValueType::String => **self.cast_string_unchecked() <= **other.cast_string_unchecked(),
            _ => panic!("attempted op_leq on unsupported type: {:?}", ty_1),
        })
    }

    pub fn op_geq(self, other: Self) -> Self {
        let ty_1 = self.ty();
        let ty_2 = other.ty();

        if ty_1 != ty_2 {
            return Value::from_bool(false);
        }

        Value::from_bool(match ty_1 {
            ValueType::Int => get_int(self.data) >= get_int(other.data),
            ValueType::Float => get_float(self.data) >= get_float(other.data),
            ValueType::String => **self.cast_string_unchecked() >= **other.cast_string_unchecked(),
            _ => panic!("attempted op_geq on unsupported type: {:?}", ty_1),
        })
    }

    pub fn op_property(self, other: Self) -> Self {
        if self.ty() != ValueType::Table {
            panic!()
        }

        let table = self.cast_table_unchecked();
        table.get(other)
    }

    pub fn op_method(self, other: Self) -> Self {
        if self.ty() != ValueType::Table {
            panic!()
        }

        let table = self.cast_table_unchecked();
        let value = table.get(other);

        if value.ty() != ValueType::Function {
            panic!()
        }

        value
    }

    pub fn op_concat(self, other: Self, ctx: &Ctx) -> Self {
        let ty_1 = self.ty();
        let ty_2 = other.ty();

        if ty_1 != ValueType::String && ty_2 != ValueType::String {
            panic!()
        }

        let str_1 = self.cast_string_unchecked();
        let str_2 = other.cast_string_unchecked();
        let mut buf = Vec::new();
        buf.extend_from_slice(str_1);
        buf.extend_from_slice(str_2);
        let new_str = ctx.intern(&buf);
        Value::from_string(new_str)
    }

    pub fn op_neg(self) -> Self {
        match self.ty() {
            ValueType::Int => Value::from_int(-get_int(self.data)),
            ValueType::Float => Value::from_float(-get_float(self.data)),
            _ => panic!(),
        }
    }

    pub fn op_not(self) -> Self {
        Value::from_bool(!self.is_truthy())
    }

    pub fn op_len(self) -> Self {
        match self.ty() {
            ValueType::Table => {
                let len = self.cast_table_unchecked().len();
                Value::from_int(len as i32)
            },
            ValueType::String => {
                let len = self.cast_string_unchecked().len();
                Value::from_int(len as i32)
            },
            _ => panic!(),
        }
    }

    pub fn op_bit_not(self) -> Self {
        if let ValueType::Int = self.ty() {
            let x = get_int(self.data);
            return Value::from_int(!x);
        }

        panic!()
    }

    pub fn op_hash(self) -> u64 {
        mix_u64(self.data)
    }
}

impl Trace for Value {
    fn visit(&self, visitor: &mut Visitor) {
        if is_ptr(self.data) {
            let handle = TaggedHandle::new(self.data);
            visitor.mark(handle);

            if is_table(self.data) {
                let table = unsafe { &mut *(get_table(self.data) as *mut Table) };
                table.visit(visitor);
            }
        }
    }
}
