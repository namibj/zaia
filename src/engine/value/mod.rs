mod closure;
mod table;

use std::{cmp, hash};

pub use closure::Closure;
pub use table::Table;

use super::gc::{Handle, Trace, Visitor};

#[derive(Clone)]
pub enum Value {
    Boolean(bool),
    Integer(i32),
    Float(f32),
    Ref(Handle<RefValue>),
}

impl cmp::PartialEq for Value {
    fn eq(&self, other: &Value) -> bool {
        match (self, other) {
            (Value::Boolean(a), Value::Boolean(b)) => a == b,
            (Value::Integer(a), Value::Integer(b)) => a == b,
            (Value::Float(a), Value::Float(b)) => a == b,
            (Value::Ref(a), Value::Ref(b)) => a == b,
            _ => false,
        }
    }
}

impl cmp::Eq for Value {}

impl cmp::PartialOrd for Value {
    fn partial_cmp(&self, other: &Value) -> Option<cmp::Ordering> {
        match (self, other) {
            (Value::Boolean(a), Value::Boolean(b)) => a.partial_cmp(b),
            (Value::Integer(a), Value::Integer(b)) => a.partial_cmp(b),
            (Value::Float(a), Value::Float(b)) => a.partial_cmp(b),
            (Value::Ref(_), Value::Ref(_)) => None,
            _ => None,
        }
    }
}

impl cmp::Ord for Value {
    fn cmp(&self, other: &Value) -> cmp::Ordering {
        match (self, other) {
            (Value::Boolean(a), Value::Boolean(b)) => a.cmp(b),
            (Value::Integer(a), Value::Integer(b)) => a.cmp(b),
            (Value::Float(a), Value::Float(b)) => float_cmp(*a, *b),
            (Value::Ref(_), Value::Ref(_)) => cmp::Ordering::Equal,
            _ => cmp::Ordering::Equal,
        }
    }
}

impl hash::Hash for Value {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        match self {
            Value::Boolean(a) => a.hash(state),
            Value::Integer(a) => a.hash(state),
            Value::Float(a) => a.to_ne_bytes().hash(state),
            Value::Ref(ref a) => a.hash(state),
        }
    }
}

impl Trace<RefValue> for Value {
    fn visit(&self, visitor: &mut Visitor<RefValue>) {
        if let Value::Ref(value) = self {
            unsafe {
                value.get_unchecked().visit(visitor);
            }
        }
    }
}

pub enum RefValue {
    String(Vec<u8>),
    Closure(Closure),
    Table(Table),
}

impl RefValue {
    pub fn cast_string(&self) -> &[u8] {
        match self {
            RefValue::String(a) => a,
            _ => unreachable!(),
        }
    }
}

impl Trace<RefValue> for RefValue {
    fn visit(&self, visitor: &mut Visitor<RefValue>) {
        match self {
            RefValue::String(_a) => (),
            RefValue::Closure(_a) => (),
            RefValue::Table(a) => a.visit(visitor),
        }
    }
}

fn float_cmp(a: f32, b: f32) -> cmp::Ordering {
    let convert = |f: f32| {
        let i = f.to_bits();
        let bit = 1 << (32 - 1);
        if i & bit == 0 {
            i | bit
        } else {
            !i
        }
    };

    convert(a).cmp(&convert(b))
}
