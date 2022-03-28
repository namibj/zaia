use hashbrown::{hash_map, HashMap};

use super::TaggedHandle;

pub struct ObjectSet {
    set: HashMap<TaggedHandle, (), ()>,
}

impl ObjectSet {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            set: HashMap::with_hasher(()),
        }
    }

    fn entry_mut(
        &mut self,
        handle: TaggedHandle,
    ) -> hash_map::RawEntryMut<'_, TaggedHandle, (), ()> {
        let hash = handle.hash();

        self.set
            .raw_entry_mut()
            .from_hash(hash, |other| handle.value() == other.value())
    }

    pub fn insert(&mut self, handle: TaggedHandle) {
        if let hash_map::RawEntryMut::Vacant(entry) = self.entry_mut(handle) {
            let hash = handle.hash();
            entry.insert_with_hasher(hash, handle, (), |handle| handle.value() as u64);
        } else {
            unreachable!()
        }
    }

    pub fn remove(&mut self, handle: TaggedHandle) {
        if let hash_map::RawEntryMut::Occupied(entry) = self.entry_mut(handle) {
            entry.remove();
            return;
        }

        unreachable!()
    }

    fn contains(&self, handle: TaggedHandle) -> bool {
        let hash = handle.hash();

        self.set
            .raw_entry()
            .from_hash(hash, |other| handle.value() == other.value())
            .is_some()
    }

    pub fn iter(&self) -> impl Iterator<Item = TaggedHandle> + '_ {
        self.set.keys().copied()
    }

    pub fn difference<'a>(&'a self, other: &'a Self) -> impl Iterator<Item = TaggedHandle> + 'a {
        self.set
            .keys()
            .filter(|handle| !other.contains(**handle))
            .copied()
    }

    pub fn clear(&mut self) {
        self.set.clear();
    }
}
