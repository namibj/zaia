use std::{cmp, collections::HashMap, hash};

use broom::Handle;

#[derive(Clone)]
pub enum Value {
    Boolean(bool),
    Integer(i32),
    Float(f32),
    String(Vec<u8>),
    Ref(Handle<RefValue>),
}

impl cmp::PartialEq for Value {
    fn eq(&self, other: &Value) -> bool {
        match (self, other) {
            (Value::Boolean(a), Value::Boolean(b)) => a == b,
            (Value::Integer(a), Value::Integer(b)) => a == b,
            (Value::Float(a), Value::Float(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
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
            (Value::String(a), Value::String(b)) => a.partial_cmp(b),
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
            (Value::String(a), Value::String(b)) => a.cmp(b),
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
            Value::String(ref a) => a.hash(state),
            Value::Ref(ref a) => a.hash(state),
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

pub enum RefValue {
    Function(Function),
    Table(Table),
}

pub struct Function {
    scope: HashMap<String, Value>,
}

pub struct Table {
    inner: HashMap<Value, Value>,
}
