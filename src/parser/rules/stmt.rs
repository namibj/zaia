use super::super::State;
use crate::T;

pub fn parse_stmt(state: &mut State) {
    let marker = state.start();

    match state.at() {
        kind => state.error(
            state
                .new_error()
                .with_message("Unexpected token")
                .with_label(
                    state
                        .new_label()
                        .with_message(format!("Expecting start of statement but found {}", kind,)),
                )
                .finish(),
        ),
    }

    marker.complete(state, T![stmt]);
}
