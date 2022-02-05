use super::{gc::Handle, value::Table, Heap};

pub struct VM {
    environment: Handle<Table>,
}

impl VM {
    pub fn new(heap: Heap) -> Self {
        Self {
            environment: Handle::unmanaged(Table::new(heap)),
        }
    }
}
