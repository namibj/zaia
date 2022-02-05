use std::collections::HashMap;

use super::{
    gc::Handle,
    value::{Table, Value},
};
use crate::error::{LuaError, LuaResult};

pub struct Scope {
    environment: Handle<Table>,
    stack: Vec<HashMap<String, Value>>,
}

impl Scope {
    pub fn new(environment: Handle<Table>) -> Self {
        Self {
            environment,
            stack: Vec::new(),
        }
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

    pub fn push(&mut self) {
        self.stack.push(HashMap::new());
    }

    pub fn pop(&mut self) {
        self.stack.pop();
    }
}
