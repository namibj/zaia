use super::Parser;
use crate::T;

impl<'cache, 'source> Parser<'cache, 'source> {
    pub(super) fn r_items(&mut self) {
        while self.at() != T![eof] {
            if self.r_stmt().is_none() {
                break;
            }
        }
    }
}
