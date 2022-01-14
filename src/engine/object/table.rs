use super::Object;
use broom::prelude::{Trace,Tracer};

pub struct Table {}

impl Trace<Object> for Table {
    fn trace(&self, tracer: &mut Tracer<Object>) {
        todo!()
    }
}
