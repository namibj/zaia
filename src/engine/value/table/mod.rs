// This module implements the table structure which forms the core of how Lua
// represents data. Inspiration for the implementation has been taken from the
// LuaJIT sources and the V8 Javacript engine. The specific V8 blogpost about this can be found here: https://v8.dev/blog/fast-properties.

mod raw_table;

use super::{
    super::gc::{Handle, Heap, Trace, Tracer},
    Value,
};

pub struct Table {}

impl Table {
    pub fn new() -> Self {
        todo!()
    }
}

impl Trace<Value> for Table {
    fn trace(&self, tracer: &mut Tracer<Value>) {
        todo!();
    }
}
