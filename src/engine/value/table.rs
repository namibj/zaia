use std::{borrow::Borrow, hash::Hash};

use fxhash::FxBuildHasher;
use hashbrown::HashMap;

use super::{
    super::{
        gc::{Trace, Visitor},
        Heap,
    },
    RefValue,
    Value,
};

pub struct Table {
    inner: HashMap<Value, Value, FxBuildHasher, Heap>,
}

impl Table {
    pub fn new(heap: Heap) -> Self {
        Table {
            inner: HashMap::with_hasher_in(FxBuildHasher::default(), heap),
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

impl Trace<RefValue> for Table {
    fn visit(&self, visitor: &mut Visitor<RefValue>) {
        self.inner.iter().for_each(|(key, value)| {
            key.visit(visitor);
            value.visit(visitor);
        });
    }
}
