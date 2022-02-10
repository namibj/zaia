use super::{
    machinery::marker::{CompletedMarker, Marker},
    Parser,
};
use crate::T;

impl<'cache, 'source> Parser<'cache, 'source> {
    pub(super) fn r_maybe_assign(&mut self) -> Option<CompletedMarker> {
        let assign_marker = self.start();
        let expr_marker = self.r_simple_expr(true);
        if matches!(self.at(), T![=] | T![,]) {
            self.r_assign(assign_marker)
        } else {
            assign_marker.abandon(self);
            expr_marker
        }
    }

    pub(super) fn r_assign(&mut self, marker: Marker) -> Option<CompletedMarker> {
        while self.at() == T![,] {
            self.expect(T![,]);
            self.r_simple_expr(true);
        }

        self.expect(T![=]);
        self.r_expr_list();
        Some(marker.complete(self, T![assign_stmt]))
    }

    pub(super) fn r_decl(&mut self) -> Option<CompletedMarker> {
        let marker = self.start();
        self.expect(T![local]);

        if self.at() == T![function] {
            self.r_func(false);
        } else {
            self.r_decl_target();

            while self.at() == T![,] {
                self.expect(T![,]);
                self.r_decl_target();
            }

            if self.at() == T![=] {
                self.expect(T![=]);
                self.r_expr_list();
            }
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
