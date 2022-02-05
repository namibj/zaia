use std::ptr;

use super::{
    gc::{Trace, Visitor},
    value::RefValue,
    vm::VM,
};

pub struct Marker {
    data: *const VM,
}

impl Marker {
    pub fn new() -> Self {
        Self {
            data: ptr::null_mut(),
        }
    }

    pub fn initialize(&mut self, vm: &VM) {
        self.data = vm;
    }
}

impl Trace<RefValue> for Marker {
    fn visit(&self, _visitor: &mut Visitor<RefValue>) {}
}
