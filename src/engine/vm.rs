use super::{scope::Scope, Heap};

pub struct VM {
    scope: Scope,
}

impl VM {
    pub fn new(heap: Heap) -> Self {
        Self {
            scope: Scope::new(heap),
        }
    }
}
