use std::collections::HashSet;

use super::handle::Handle;

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
        self.marked.insert(handle);
    }

    pub fn run(&mut self, root: &dyn Trace<T>) {
        root.visit(self);
    }

    pub fn unmarked<'a>(
        &'a self,
        objects: &'a HashSet<Handle<T>>,
    ) -> impl Iterator<Item = Handle<T>> + 'a {
        objects.difference(&self.marked).copied()
    }

    pub fn reset(&mut self) {
        self.marked.clear();
    }
}
