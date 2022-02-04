use super::handle::Handle;
use std::collections::HashSet;

pub trait Trace<T> {
    fn visit(&self, visitor: &mut Visitor<T>);
}

pub struct Visitor<T> {
    marked: HashSet<Handle<T>>, 
}

impl<T> Visitor<T> {
    pub fn new() -> Self {
        Self {
            marked: HashSet::new(),
        }
    }

    pub fn mark(&mut self, handle: Handle<T>) {
        self.marked.insert(handle.clone());
    }

    pub fn run(mut self, root: &dyn Trace<T>) -> HashSet<Handle<T>> {
        root.visit(&mut self);
        self.marked
    }
}

