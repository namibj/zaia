use super::Object;
use broom::prelude::{Handle,Trace,Tracer};
use std::collections::HashMap;
use crate::syntax_tree::Ident;

pub struct Function {
    upvalues: HashMap<Ident, Handle<Object>>,
}

impl Function {
    pub fn new(captured: HashMap<Ident, Handle<Object>>) -> Self {
        Function {upvalues:captured}
    }
}

impl Trace<Object> for Function {
    fn trace(&self, tracer: &mut Tracer<Object>) {
        for value in self.upvalues.values() {
            value.trace(tracer);
        }
    }
}
