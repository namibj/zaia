use super::{handle::TaggedHandle, set::ObjectSet};

pub trait Trace {
    fn visit(&self, visitor: &mut Visitor);
}

pub struct Visitor {
    marked: ObjectSet,
    stale: Vec<TaggedHandle>,
}

impl Visitor {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            marked: ObjectSet::new(),
            stale: Vec::new(),
        }
    }

    pub fn mark(&mut self, handle: TaggedHandle) {
        self.marked.insert(handle);
    }

    pub fn unmarked<'a>(
        &'a mut self,
        objects: &ObjectSet,
    ) -> impl Iterator<Item = TaggedHandle> + 'a {
        self.stale.extend(objects.difference(&self.marked));
        self.stale.iter().copied()
    }

    pub fn reset(&mut self) {
        self.marked.clear();
        self.stale.clear();
    }
}
