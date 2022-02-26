use hashbrown::HashSet;

use super::handle::TaggedHandle;

pub trait Trace {
    fn visit(&self, visitor: &mut Visitor);
}

pub struct Visitor {
    marked: HashSet<TaggedHandle>,
    stale: Vec<TaggedHandle>,
}

impl Visitor {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            marked: HashSet::new(),
            stale: Vec::new(),
        }
    }

    pub fn mark(&mut self, handle: TaggedHandle) {
        self.marked.insert(handle);
    }

    pub fn run(&mut self, root: &dyn Trace) {
        root.visit(self);
    }

    pub fn unmarked<'a>(
        &'a mut self,
        objects: &HashSet<TaggedHandle>,
    ) -> impl Iterator<Item = TaggedHandle> + 'a {
        self.stale.extend(objects.difference(&self.marked).copied());
        self.stale.iter().copied()
    }

    pub fn reset(&mut self) {
        self.marked.clear();
        self.stale.clear();
    }
}
