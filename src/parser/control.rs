use super::{machinery::marker::CompletedMarker, Parser};
use crate::T;

impl<'source> Parser<'source> {
    pub(super) fn r_do(&mut self) -> Option<CompletedMarker> {
        let marker = self.start();
        self.expect(T![do]);
        self.r_block();
        self.expect(T![end]);
        Some(marker.complete(self, T![block_stmt]))
    }

    pub(super) fn r_while(&mut self) -> Option<CompletedMarker> {
        let marker = self.start();
        self.expect(T![while]);
        self.expect(T!['(']);
        self.r_expr();
        self.expect(T![')']);
        self.r_do();
        Some(marker.complete(self, T![while_stmt]))
    }

    pub(super) fn r_repeat(&mut self) -> Option<CompletedMarker> {
        todo!()
    }

    pub(super) fn r_if(&mut self) -> Option<CompletedMarker> {
        todo!()
    }

    pub(super) fn r_for(&mut self) -> Option<CompletedMarker> {
        todo!()
    }

    pub(super) fn r_return(&mut self) -> Option<CompletedMarker> {
        let marker = self.start();
        self.expect(T![return]);
        self.r_expr_list();
        Some(marker.complete(self, T![return_stmt]))
    }

    pub(super) fn r_break(&mut self) -> Option<CompletedMarker> {
        let marker = self.start();
        self.expect(T![break]);
        Some(marker.complete(self, T![break_stmt]))
    }

    pub(super) fn r_block(&mut self) -> Option<CompletedMarker> {
        todo!()
    }
}
