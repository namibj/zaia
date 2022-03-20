use std::collections::hash_map::RandomState;

use hashbrown::HashMap;

use super::super::{
    gc::{Heap, Handle},
    value::{Table, Value, ByteString},
};

pub struct Ctx<'a> {
    global: &'a mut Table,
    scope: Vec<HashMap<Handle<ByteString>, Value, RandomState>>,
    heap: &'a Heap,
}

impl<'a> Ctx<'a> {
    pub fn new(global: &'a mut Table, heap: &'a Heap) -> Self {
        Ctx {
            global,
            scope: vec![HashMap::with_hasher(RandomState::new())],
            heap,
        }
    }

    pub fn heap(&self) -> &Heap {
        self.heap
    }

    pub fn scope_push(&mut self) {
        if !self.scope.last().unwrap().is_empty() {
            self.scope.push(HashMap::with_hasher(RandomState::new()));
        }
    }

    pub fn scope_pop(&mut self) {
        if !self.scope.last().unwrap().is_empty() {
            self.scope.pop();
        }
    }

    pub fn define_local(&mut self, key: Handle<ByteString>, value: Value) {
        self.scope.last_mut().unwrap().insert(key, value);
    }

    pub fn define_global(&mut self, key: Handle<ByteString>, value: Value) {
        let key = Value::from_string(key);
        self.global.insert(key, value);
    }

    pub fn resolve(&self, key: Handle<ByteString>) -> Value {
        for scope in self.scope.iter().rev() {
            if let Some(value) = scope.get(&key) {
                return value.clone();
            }
        }

        let key = Value::from_string(key);
        if let Some(value) = self.global.get(key) {
            return *value;
        }

        Value::from_nil()
    }
}
