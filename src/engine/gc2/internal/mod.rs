mod arena;
mod deck;
mod eden;
mod optimizer;
mod pacer;

use std::{
    alloc::Layout,
    collections::{HashSet, LinkedList},
    time::Duration,
};

use arena::Arena;
use deck::Deck;
use eden::Eden;
use pacer::Pacer;

const LARGE_OBJECT_THRESHOLD: usize = 2 * 1024;
const MAX_PAUSE: Duration = Duration::from_millis(100);

pub struct InternalHeap {
    pacer: Pacer,
    eden: Eden,
    deck: Deck,
    regions: LinkedList<Arena>,
    large_objects: HashSet<(*mut u8, Layout)>,
}

impl InternalHeap {
    pub fn new() -> InternalHeap {
        InternalHeap {
            pacer: Pacer::new(MAX_PAUSE),
            eden: Eden::new(),
            deck: Deck::new(),
            regions: LinkedList::new(),
            large_objects: HashSet::new(),
        }
    }
}
