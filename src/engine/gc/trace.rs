use hashbrown::HashSet;

use super::handle::Handle;

pub trait Trace<T> {
    fn visit(&self, visitor: &mut Visitor<T>);
}

pub struct Visitor<T> {
    marked: HashSet<Handle<T>>,
    stale: Vec<Handle<T>>,
}

impl<T> Visitor<T> {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            marked: HashSet::new(),
            stale: Vec::new(),
        }
    }

    pub fn mark(&mut self, handle: Handle<T>) {
        self.marked.insert(handle);
    }

    pub fn run(&mut self, root: &dyn Trace<T>) {
        root.visit(self);
    }

    pub fn unmarked<'a>(
        &'a mut self,
        objects: &HashSet<Handle<T>>,
    ) -> impl Iterator<Item = &Handle<T>> + 'a {
        self.stale.extend(objects.difference(&self.marked).copied());
        self.stale.iter()
    }

    pub fn reset(&mut self) {
        self.marked.clear();
        self.stale.clear();
    }
}
