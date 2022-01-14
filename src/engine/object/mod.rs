mod table;
mod function;

use broom::prelude::{Trace,Tracer};
use table::Table;
use function::Function;

// TODO: function instances
// TODO: table instances
// TODO: userdata instances
pub enum Object {
    Nil,
    Boolean(bool),
    Int(i32),
    Float(f32),
    String(String),
    Table(Table),
    Function(Function),
}

impl Trace<Self> for Object {
    fn trace(&self, tracer: &mut Tracer<Self>) {
        match self {
            Self::Table(table) => table.trace(tracer),
            Self::Function(function) => function.trace(tracer),
            _ => {}
        }
    }
}
