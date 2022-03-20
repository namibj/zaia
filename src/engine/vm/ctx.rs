use std::{
    cell::{Ref, RefCell},
    collections::hash_map::RandomState,
};

use hashbrown::HashMap;

use super::super::{
    gc::{Handle, Heap},
    value::{ByteString, Table, Value},
};
use crate::parser::{machinery::cstree::interning::TokenInterner, syntax::Ident};

struct CtxInternal<'a> {
    global: &'a mut Table,
    scope: Vec<HashMap<Handle<ByteString>, Value, RandomState>>,
    heap: &'a Heap,
    interner: &'a TokenInterner,
    ident_cache: HashMap<String, Handle<ByteString>, RandomState>,
}

pub struct Ctx<'a> {
    internal: RefCell<CtxInternal<'a>>,
}

impl<'a> Ctx<'a> {
    pub fn new(global: &'a mut Table, heap: &'a Heap, interner: &'a TokenInterner) -> Self {
        Ctx {
            internal: RefCell::new(CtxInternal {
                global,
                scope: vec![HashMap::with_hasher(RandomState::new())],
                heap,
                interner,
                ident_cache: HashMap::with_hasher(RandomState::new()),
            }),
        }
    }

    pub fn heap(&self) -> Ref<Heap> {
        Ref::map(self.internal.borrow(), |internal| internal.heap)
    }

    pub fn scope(&self) -> ScopeKey<'a, '_> {
        let mut internal = self.internal.borrow_mut();

        if !internal.scope.last().unwrap().is_empty() {
            internal
                .scope
                .push(HashMap::with_hasher(RandomState::new()));
        }

        ScopeKey { ctx: self }
    }

    fn scope_destroy(&self) {
        let mut internal = self.internal.borrow_mut();

        if !internal.scope.last().unwrap().is_empty() {
            internal.scope.pop();
        }
    }

    pub fn local(&self, key: Handle<ByteString>) {
        self.internal
            .borrow_mut()
            .scope
            .last_mut()
            .unwrap()
            .insert(key, Value::from_nil());
    }

    pub fn assign(&self, key: Handle<ByteString>, value: Value) {
        let mut internal = self.internal.borrow_mut();

        for scope in internal.scope.iter_mut().rev() {
            if scope.contains_key(&key) {
                scope.insert(key, value);
                return;
            }
        }
    }

    pub fn resolve(&self, key: Handle<ByteString>) -> Value {
        let internal = self.internal.borrow();

        for scope in internal.scope.iter().rev() {
            if let Some(value) = scope.get(&key) {
                return *value;
            }
        }

        let key = Value::from_string(key);
        if let Some(value) = internal.global.get(key) {
            return *value;
        }

        Value::from_nil()
    }

    pub fn intern_ident(&self, ident: &Ident) -> Handle<ByteString> {
        let internal = self.internal.borrow_mut();
        let name = ident.name(internal.interner).unwrap();

        if let Some(handle) = internal.ident_cache.get(name) {
            return *handle;
        }

        let handle = todo!();
        internal.ident_cache.insert(name.to_owned(), handle);
        handle
    }
}

pub struct ScopeKey<'a, 'ctx> {
    ctx: &'ctx Ctx<'a>,
}

impl<'a, 'ctx> Drop for ScopeKey<'a, 'ctx> {
    fn drop(&mut self) {
        self.ctx.scope_destroy();
    }
}
