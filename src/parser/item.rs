use super::{machinery::marker::CompletedMarker, Parser};
use crate::T;

impl<'source> Parser<'source> {
    pub(super) fn items(&mut self) {
        while self.at() != T![eof] {
            if self.stmt().is_none() {
                break;
            }
        }
    }
}
