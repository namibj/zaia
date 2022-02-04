use std::ptr::NonNull;

use super::{
    gc::{Trace, Visitor},
    value::RefValue,
    vm::VM,
};

pub struct Marker {
    data: NonNull<VM>,
}

impl Marker {
    pub fn new(data: NonNull<VM>) -> Self {
        Self { data }
    }
}

impl Trace<RefValue> for Marker {
    fn visit(&self, _visitor: &mut Visitor<RefValue>) {}
}
