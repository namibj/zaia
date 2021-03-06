//! TODO(#29): Replace this with a butterfly-like structure.

use hashbrown::{hash_map, HashMap};

use super::{
    super::gc::{Heap, PtrTag, Trace, Visitor},
    encoding,
    Value,
};

pub struct Table {
    map: HashMap<Value, Value, (), Heap>,
}

impl Table {
    pub fn new(heap: Heap) -> Self {
        Table {
            map: HashMap::with_hasher_in((), heap),
        }
    }

    fn entry_mut(&mut self, key: Value) -> hash_map::RawEntryMut<'_, Value, Value, (), Heap> {
        let hash = key.op_hash();

        self.map
            .raw_entry_mut()
            .from_hash(hash, |other| key.op_eq(*other).cast_bool_unchecked())
    }

    pub fn get(&self, key: Value) -> Value {
        let hash = key.op_hash();

        self.map
            .raw_entry()
            .from_hash(hash, |other| key.op_eq(*other).cast_bool_unchecked())
            .map(|(_, v)| v)
            .copied()
            .unwrap_or_else(Value::from_nil)
    }

    pub fn insert(&mut self, key: Value, value: Value) {
        match self.entry_mut(key) {
            hash_map::RawEntryMut::Vacant(entry) => {
                let hash = key.op_hash();
                entry.insert_with_hasher(hash, key, value, |key| key.op_hash());
            },

            hash_map::RawEntryMut::Occupied(mut entry) => {
                *entry.get_mut() = value;
            },
        }
    }

    pub fn remove(&mut self, key: Value) {
        if let hash_map::RawEntryMut::Occupied(entry) = self.entry_mut(key) {
            entry.remove();
        }
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
}

impl Trace for Table {
    fn visit(&self, visitor: &mut Visitor) {
        self.map.iter().for_each(|(key, value)| {
            key.visit(visitor);
            value.visit(visitor);
        });
    }
}

unsafe impl PtrTag for Table {
    fn is(x: u64) -> bool {
        encoding::is_table(x)
    }

    fn tag(x: usize) -> u64 {
        encoding::make_table(x as *mut u8)
    }
}
