use std::collections::HashMap;

use super::value::{Table, Value};

pub trait Resolver {
    fn resolve_mut(&mut self, name: &str) -> Option<&mut Value>;
}

impl Resolver for Table {
    fn resolve_mut(&mut self, name: &str) -> Option<&mut Value> {
        self.get_mut(name.as_bytes())
    }
}

pub struct Scope<'a> {
    parent: &'a mut dyn Resolver,
    map: HashMap<String, Value>,
}

impl<'a> Scope<'a> {
    pub fn new(parent: &'a mut dyn Resolver) -> Self {
        Self {
            parent,
            map: HashMap::new(),
        }
    }

    pub fn assign(&mut self, name: String, value: Value) {
        self.map.insert(name, value);
    }

    pub fn push(&'a mut self) -> Scope<'a> {
        Scope::new(self)
    }
}

impl<'a> Resolver for Scope<'a> {
    fn resolve_mut(&mut self, name: &str) -> Option<&mut Value> {
        self.map
            .get_mut(name)
            .or_else(|| self.parent.resolve_mut(name))
    }
}
