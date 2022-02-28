use std::{collections::hash_map::RandomState, num::NonZeroUsize};

use super::super::interning::{
    Capacity,
    Interner,
    IntoReader,
    IntoReaderAndResolver,
    IntoResolver,
    Key,
    Reader,
    Resolver,
    Rodeo,
};

/// The default [`Interner`] used to deduplicate green token strings.
#[derive(Debug)]
pub struct TokenInterner {
    rodeo: Rodeo,
}

impl TokenInterner {
    pub(super) fn new() -> Self {
        Self {
            rodeo: Rodeo::with_capacity_and_hasher(
                // capacity values suggested by author of `lasso`
                Capacity::new(512, unsafe { NonZeroUsize::new_unchecked(4096) }),
                RandomState::default(),
            ),
        }
    }
}

impl Resolver for TokenInterner {
    fn resolve<'a>(&'a self, key: &Key) -> &'a str {
        self.rodeo.resolve(key)
    }

    fn try_resolve<'a>(&'a self, key: &Key) -> Option<&'a str> {
        self.rodeo.try_resolve(key)
    }

    unsafe fn resolve_unchecked<'a>(&'a self, key: &Key) -> &'a str {
        self.rodeo.resolve_unchecked(key)
    }

    fn contains_key(&self, key: &Key) -> bool {
        self.rodeo.contains_key(key)
    }

    fn len(&self) -> usize {
        self.rodeo.len()
    }
}

impl Reader for TokenInterner {
    fn get(&self, val: &str) -> Option<Key> {
        self.rodeo.get(val)
    }

    fn contains(&self, val: &str) -> bool {
        self.rodeo.contains(val)
    }
}

impl IntoResolver for TokenInterner {
    type Resolver = <Rodeo as IntoResolver>::Resolver;

    fn into_resolver(self) -> Self::Resolver
    where
        Self: 'static,
    {
        self.rodeo.into_resolver()
    }

    fn into_resolver_boxed(self: Box<Self>) -> Self::Resolver
    where
        Self: 'static,
    {
        Rodeo::into_resolver_boxed(Box::new(self.rodeo))
    }
}

impl Interner for TokenInterner {
    fn get_or_intern(&mut self, val: &str) -> Key {
        self.rodeo.get_or_intern(val)
    }

    fn try_get_or_intern(&mut self, val: &str) -> lasso::LassoResult<Key> {
        self.rodeo.try_get_or_intern(val)
    }

    fn get_or_intern_static(&mut self, val: &'static str) -> Key {
        self.rodeo.get_or_intern_static(val)
    }

    fn try_get_or_intern_static(&mut self, val: &'static str) -> lasso::LassoResult<Key> {
        self.rodeo.try_get_or_intern_static(val)
    }
}

impl IntoReader for TokenInterner {
    type Reader = <Rodeo as IntoReader>::Reader;

    fn into_reader(self) -> Self::Reader
    where
        Self: 'static,
    {
        self.rodeo.into_reader()
    }

    fn into_reader_boxed(self: Box<Self>) -> Self::Reader
    where
        Self: 'static,
    {
        Rodeo::into_reader_boxed(Box::new(self.rodeo))
    }
}

impl IntoReaderAndResolver for TokenInterner {}
