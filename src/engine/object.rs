use std::collections::HashMap;
use broom::prelude::{Handle,Trace,Tracer};
use crate::syntax_tree::Ident;

pub struct Table {
    elements: HashMap<Handle<Object>, Handle<Object>>,
}

impl Table {
    pub fn new() -> Self {
        Table {
            elements: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: Handle<Object>, value: Handle<Object>) {
        self.elements.insert(key, value);
    }

    pub fn get(&self, key: Handle<Object>) -> Option<Handle<Object>> {
        self.elements.get(&key).cloned()
    }
}

impl Trace<Object> for Table {
    fn trace(&self, tracer: &mut Tracer<Object>) {
        for (key, value) in &self.elements {
            key.trace(tracer);
            value.trace(tracer);
        }
    }
}

pub struct Function {
    upvalues: HashMap<Ident, Handle<Object>>,
}

impl Function {
    pub fn new(captured: HashMap<Ident, Handle<Object>>) -> Self {
        Function {upvalues:captured}
    }
}

impl Trace<Object> for Function {
    fn trace(&self, tracer: &mut Tracer<Object>) {
        for value in self.upvalues.values() {
            value.trace(tracer);
        }
    }
}

// TODO(#19): userdata instances
pub enum Object {
    Nil,
    Boolean(bool),
    Int(i64),
    String(String),
    Table(Table),
    Function(Function),
}

impl Trace<Self> for Object {
    fn trace(&self, tracer: &mut Tracer<Self>) {
        match self {
            Self::Table(table) => table.trace(tracer),
            Self::Function(function) => function.trace(tracer),
            _ => {}
        }
    }
}
