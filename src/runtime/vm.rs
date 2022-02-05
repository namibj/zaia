use std::alloc;

use super::{gc::Handle, value::Table};

pub struct VM {
    environment: Handle<Table<alloc::Global>>,
}

impl VM {
    pub fn new() -> Self {
        Self {
            environment: Handle::unmanaged(Table::new(alloc::Global)),
        }
    }
}
