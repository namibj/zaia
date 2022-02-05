use std::hash::{BuildHasher, Hash, Hasher};

use fxhash::FxBuildHasher;
use hashbrown::{hash_map::RawEntryMut, HashMap};

use super::engine::{gc::Handle, value::RefValue, Heap};

pub struct Interner {
    map: HashMap<Handle<RefValue>, (), ()>,
    heap: Heap,
    hasher: FxBuildHasher,
}

impl Interner {
    pub fn new() -> Self {
        Self {
            map: HashMap::with_hasher(()),
            heap: Heap::new(),
            hasher: FxBuildHasher::default(),
        }
    }

    pub fn intern<T>(&mut self, item: &T) -> Handle<RefValue>
    where
        T: AsRef<[u8]>,
    {
        let item = item.as_ref();

        let hash = {
            let mut state = self.hasher.build_hasher();
            item.hash(&mut state);
            state.finish()
        };

        let entry = self.map.raw_entry_mut().from_hash(hash, |handle| {
            let key_string = unsafe { handle.get_unchecked() };
            item == key_string.cast_string()
        });

        match entry {
            RawEntryMut::Occupied(entry) => *entry.into_key(),
            RawEntryMut::Vacant(entry) => {
                let value = RefValue::String(item.to_vec());
                let handle = self.heap.insert(value);

                entry.insert_with_hasher(hash, handle, (), |key| {
                    let item = unsafe { key.get_unchecked() };

                    let mut state = self.hasher.build_hasher();
                    item.cast_string().hash(&mut state);
                    state.finish()
                });

                handle
            },
        }
    }

    pub fn remove(&mut self, handle: Handle<RefValue>) {
        let item = unsafe { handle.get_unchecked().cast_string() };

        let hash = {
            let mut state = self.hasher.build_hasher();
            item.hash(&mut state);
            state.finish()
        };

        let entry = self.map.raw_entry_mut().from_hash(hash, |handle| {
            let key_string = unsafe { handle.get_unchecked() };
            item == key_string.cast_string()
        });

        entry.and_replace_entry_with(|_, _| None);
    }
}
