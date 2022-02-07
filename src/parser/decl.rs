use std::process::Command;

use super::{machinery::marker::CompletedMarker, Parser};
use crate::T;

impl<'source> Parser<'source> {
    pub(super) fn r_decl(&mut self) -> Option<CompletedMarker> {
        let marker = self.start();
        self.expect(T![local]);
        self.r_decl_target();

        while self.at() == T![,] {
            self.expect(T![,]);
            self.r_decl_target();
        }

        if self.at() == T![=] {
            self.expect(T![=]);
            self.r_expr_list();
        }

        Some(marker.complete(self, T![decl_stmt]))
    }

    fn r_decl_target(&mut self) -> Option<CompletedMarker> {
        let marker = self.start();
        self.expect(T![ident]);
        self.r_attrib();
        Some(marker.complete(self, T![decl_target]))
    }

    fn r_attrib(&mut self) {
        let t = self.at();

        if matches!(t, T![const] | T![close]) {
            self.expect(t);
        }
    }
}
