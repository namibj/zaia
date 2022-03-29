mod arena;
mod eden;
mod deck;
mod pacer;

use arena::Arena;
use eden::Eden;
use std::collections::LinkedList;
use deck::Deck;
use pacer::Pacer;
use std::time::Duration;

const MAX_PAUSE: Duration = Duration::from_millis(100);

pub struct InternalHeap {
    pacer: Pacer,
    eden: Eden,
    deck: Deck,
    regions: LinkedList<Arena>,
}

impl InternalHeap {
    pub fn new() -> InternalHeap {
        InternalHeap {
            pacer: Pacer::new(MAX_PAUSE),
            eden: Eden::new(),
            deck: Deck::new(),
            regions: LinkedList::new(),
        }
    }
}
