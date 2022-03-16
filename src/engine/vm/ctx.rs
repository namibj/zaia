use super::super::value::{Value, Table};
use hashbrown::HashMap;
use std::collections::hash_map::RandomState;

pub struct Ctx<'a> {
    global: &'a mut Table,
    scope: Vec<HashMap<String, Value, RandomState>>,
}


impl<'a> Ctx<'a> {
    pub fn new(global: &'a mut Table) -> Self {
        Ctx {
            global,
            scope: vec![HashMap::with_hasher(RandomState::new())],
        }
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

    pub fn define_local(&mut self, key: String, value: Value) {
        todo!()
    }

    pub fn define_global(&mut self, key: String, value: Value) {
        todo!()
    }

    pub fn resolve(&self, key: &str) -> Value {
        todo!()
    }
}
