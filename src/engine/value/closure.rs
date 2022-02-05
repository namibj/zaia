use std::rc::Rc;

use crate::syntax_tree::Function;

pub struct Closure {
    internal: Rc<Function>,
}

impl Closure {
    pub fn new(internal: Rc<Function>) -> Self {
        Self { internal }
    }
}
