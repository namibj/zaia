use std::rc::Rc;

use crate::syntax_tree::Function as FnInternal;

pub struct Function {
    internal: Rc<FnInternal>,
}

impl Function {
    pub fn new(internal: Rc<FnInternal>) -> Self {
        Self { internal }
    }
}
