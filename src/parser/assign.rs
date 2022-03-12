use super::{
    machinery::marker::{CompletedMarker, Marker},
    Parser,
};
use crate::T;

impl<'cache, 'source> Parser<'cache, 'source> {
    pub(super) fn r_maybe_assign(&mut self) -> Option<CompletedMarker> {
        let assign_marker = self.start(T![assign_stmt]);
        let assign_list_marker = self.start(T![assign_list]);
        let expr_marker = self.r_simple_expr(true);
        if matches!(self.at(), T![=] | T![,]) {
            self.r_assign(assign_marker, assign_list_marker)
        } else {
            assign_list_marker.abandon(self);
            assign_marker.abandon(self);
            expr_marker
        }
    }

    pub(super) fn r_assign(
        &mut self,
        assign_marker: Marker,
        list_marker: Marker,
    ) -> Option<CompletedMarker> {
        while self.at() == T![,] {
            self.expect(T![,]);
            self.r_simple_expr(true);
        }

        list_marker.complete(self);
        self.expect(T![=]);
        self.r_expr_list();
        Some(assign_marker.complete(self))
    }

    pub(super) fn r_decl(&mut self) -> Option<CompletedMarker> {
        let marker = self.start(T![decl_stmt]);
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

        Some(marker.complete(self))
    }

    fn r_decl_target(&mut self) -> Option<CompletedMarker> {
        let marker = self.start(T![decl_target]);
        self.r_ident();
        self.r_attrib();
        Some(marker.complete(self))
    }

    fn r_attrib(&mut self) {
        let t = self.at();

        if matches!(t, T![const] | T![close]) {
            self.expect(t);
        }
    }
}
