use std::collections::HashMap;

use super::{
    gc::Handle,
    value::{Table, Value},
    Heap,
};
use crate::error::{LuaError, LuaResult};

pub struct Scope {
    environment: Handle<Table>,
    stack: Vec<HashMap<String, Value>>,
}

impl Scope {
    pub fn new(heap: Heap) -> Self {
        Self {
            environment: Handle::unmanaged(Table::new(heap)),
            stack: Vec::new(),
        }
    }

    pub fn environment(&mut self) -> &mut Table {
        unsafe { self.environment.get_unchecked_mut() }
    }

    pub fn declare(&mut self, key: String, value: Value) -> LuaResult<()> {
        if self.stack.last_mut().unwrap().insert(key, value).is_some() {
            return Err(LuaError::VariableAlreadyDeclared);
        }

        Ok(())
    }

    pub fn assign(&mut self, key: &str, value: Value) {
        for scope in self.stack.iter_mut().rev() {
            if let Some(v) = scope.get_mut(key) {
                *v = value;
                return;
            }
        }

        unsafe {
            let key = Value::String(key.as_bytes().to_vec());
            self.environment.get_unchecked_mut().insert(key, value);
        }
    }

    pub fn resolve(&self, key: &str) -> Option<&Value> {
        for scope in self.stack.iter().rev() {
            if let Some(v) = scope.get(key) {
                return Some(v);
            }
        }

        unsafe { self.environment.get_unchecked().get(key.as_bytes()) }
    }

    pub fn push(&mut self) {
        self.stack.push(HashMap::new());
    }

    pub fn pop(&mut self) {
        self.stack.pop();
    }
}
