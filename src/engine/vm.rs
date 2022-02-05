use std::alloc;

use super::{gc::Handle, scope::Scope, value::Table};

pub struct VM {
    scope: Scope,
}

impl VM {
    pub fn new() -> Self {
        Self {
            scope: Scope::new(),
        }
    }
}
