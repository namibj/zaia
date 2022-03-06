use super::{event::Event, kind::SyntaxKind, state::State};
use crate::T;

pub struct Marker {
    position: usize,
    kind: SyntaxKind,
}

impl Marker {
    pub fn new(state: &mut State, position: usize, kind: SyntaxKind) -> Self {
        state.events().push(Event::Enter {
            kind,
            preceded_by: 0,
        });

        Self { position, kind }
    }

    pub fn complete(self, state: &mut State) -> CompletedMarker {
        state.events().push(Event::Exit);
        CompletedMarker {
            position: self.position,
            kind: self.kind,
        }
    }

    pub fn retype(self, state: &mut State, kind: SyntaxKind) -> Self {
        let event_at_pos = &mut state.events()[self.position];
        debug_assert_eq!(*event_at_pos, Event::tombstone());

        *event_at_pos = Event::Enter {
            kind,
            preceded_by: 0,
        };

        self
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
    pub fn precede(self, state: &mut State, kind: SyntaxKind) -> Marker {
        let marker = state.start(kind);

        if let Event::Enter { preceded_by, .. } = &mut state.events()[self.position] {
            *preceded_by = marker.position - self.position;
        } else {
            unreachable!();
        }

        marker
    }
}
