use std::{borrow::Borrow, hash::Hash};
use std::alloc;

use hashbrown::{hash_map::DefaultHashBuilder, HashMap};

use super::{super::Heap, Value};

pub struct Table<A = Heap> where A:alloc::Allocator+Clone {
    inner: HashMap<Value, Value, DefaultHashBuilder, A>,
}

impl<A> Table<A> where A:alloc::Allocator+Clone {
    pub fn new(alloc: A) -> Self {
        Table {
            inner: HashMap::with_capacity_in(0, alloc),
        }
    }

    pub fn get<Q>(&self, key: &Q) -> Option<&Value>
    where
        Value: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.inner.get(key)
    }

    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut Value>
    where
        Value: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.inner.get_mut(key)
    }

    pub fn insert(&mut self, key: Value, value: Value) {
        self.inner.insert(key, value);
    }

    pub fn remove(&mut self, key: &Value) -> Option<Value> {
        self.inner.remove(key)
    }
}
