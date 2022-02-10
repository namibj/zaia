use super::{event::Event, kind::SyntaxKind, state::State};
use crate::T;

pub struct Marker {
    position: usize,
}

impl Marker {
    pub fn new(position: usize) -> Self {
        Self { position }
    }

    pub fn complete(self, state: &mut State, kind: SyntaxKind) -> CompletedMarker {
        let event_at_pos = &mut state.events()[self.position];
        debug_assert_eq!(*event_at_pos, Event::tombstone());

        *event_at_pos = Event::Enter {
            kind,
            preceded_by: 0,
        };

        state.events().push(Event::Exit);
        CompletedMarker {
            position: self.position,
            kind,
        }
    }

    pub fn abandon(self, state: &mut State) {
        match &mut state.events()[self.position] {
            Event::Enter {
                kind,
                preceded_by: 0,
            } => {
                *kind = T![tombstone];
            },

            _ => unreachable!(),
        }

        debug_assert_eq!(state.events()[self.position], Event::tombstone());
        if self.position == state.events().len() - 1 {
            state.events().pop();
        }
    }
}

#[derive(Debug)]
pub struct CompletedMarker {
    position: usize,
    kind: SyntaxKind,
}

impl CompletedMarker {
    pub fn precede(self, state: &mut State) -> Marker {
        let marker = state.start();

        if let Event::Enter { preceded_by, .. } = &mut state.events()[self.position] {
            *preceded_by = marker.position - self.position;
        } else {
            unreachable!();
        }

        marker
    }
}
