use super::{
    gc::{Trace, Visitor},
    value::RefValue,
    vm::VM,
};

pub struct Marker {
    data: *const VM,
}

impl Marker {
    pub fn new(vm: &VM) -> Self {
        Self {
            data: vm,
        }
    }
}

impl Trace<RefValue> for Marker {
    fn visit(&self, _visitor: &mut Visitor<RefValue>) {}
}
