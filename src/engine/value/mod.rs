mod table;
mod string;

use table::Table;

use super::gc::{Handle, Trace, Tracer};

// TODO: string instances
// TODO: table instances
// TODO: function instances
// TODO: userdata instances
pub enum Value {
    Nil,
    Boolean(bool),
    Int(i32),
    Float(f32),
    String(Handle<String>),
    Table(Handle<Table>),
    Shared(Handle<Value>),
}

impl Trace<Self> for Value {
    fn trace(&self, tracer: &mut Tracer<Self>) {
        unsafe {
            match self {
                Self::Table(table) => table.get_unchecked().trace(tracer),
                Self::Shared(shared) => shared.get_unchecked().trace(tracer),
                _ => {},
            }
        }
    }
}
