use super::{
    machinery::{
        kind::SyntaxKind,
        marker::{CompletedMarker, Marker},
    },
    Parser,
};
use crate::T;

impl<'cache, 'source> Parser<'cache, 'source> {
    pub(super) fn r_do(&mut self) -> Option<CompletedMarker> {
        let marker = self.start(T![do_stmt]);
        self.expect(T![do]);
        self.r_block(|t| t == T![end]);
        self.expect(T![end]);
        Some(marker.complete(self))
    }

    pub(super) fn r_while(&mut self) -> Option<CompletedMarker> {
        let marker = self.start(T![while_stmt]);
        self.expect(T![while]);
        self.r_expr();
        self.r_do();
        Some(marker.complete(self))
    }

    pub(super) fn r_repeat(&mut self) -> Option<CompletedMarker> {
        let marker = self.start(T![repeat_stmt]);
        self.expect(T![repeat]);
        self.r_block(|t| t == T![until]);
        self.expect(T![until]);
        self.r_expr();
        Some(marker.complete(self))
    }

    pub(super) fn r_if(&mut self, if_kind: SyntaxKind) -> Option<CompletedMarker> {
        let marker = self.start(T![if_stmt]);
        self.expect(if_kind);
        self.r_expr();
        self.expect(T![then]);
        self.r_block(|t| matches!(t, T![end] | T![elseif] | T![else]));

        match self.at() {
            T![end] => {
                self.expect(T![end]);
            },
            T![elseif] | T![else] => {
                self.r_else();
            },
            _ => unreachable!(),
        }

        Some(marker.complete(self))
    }

    pub(super) fn r_else(&mut self) -> Option<CompletedMarker> {
        let marker = self.start(T![else_chain]);

        match self.at() {
            T![else] => {
                self.expect(T![else]);
                self.r_block(|t| t == T![end]);
                self.expect(T![end]);
            },
            T![elseif] => {
                self.r_if(T![elseif]);
            },
            _ => unreachable!(),
        }

        Some(marker.complete(self))
    }

    pub(super) fn r_for(&mut self) -> Option<CompletedMarker> {
        let marker = self.start(T![tombstone]);
        self.expect(T![for]);
        let assign_list_marker = self.start(T![assign_list]);
        self.r_ident();

        if self.at() == T![=] {
            assign_list_marker.abandon(self);
            self.r_num_for(marker)
        } else {
            self.r_gen_for(marker, assign_list_marker)
        }
    }

    pub(super) fn r_num_for(&mut self, marker: Marker) -> Option<CompletedMarker> {
        self.expect(T![=]);
        self.r_expr();
        self.expect(T![,]);
        self.r_expr();
        if self.at() == T![,] {
            self.expect(T![,]);
            self.r_expr();
        }

        self.r_do();
        Some(marker.retype(self, T![for_num_stmt]).complete(self))
    }

    pub(super) fn r_gen_for(
        &mut self,
        marker: Marker,
        list_marker: Marker,
    ) -> Option<CompletedMarker> {
        while self.at() == T![,] {
            self.expect(T![,]);
            self.r_ident();
        }

        list_marker.complete(self);
        self.expect(T![in]);
        self.r_expr_list();
        self.r_do();
        Some(marker.retype(self, T![for_gen_stmt]).complete(self))
    }

    pub(super) fn r_return(&mut self) -> Option<CompletedMarker> {
        let marker = self.start(T![return_stmt]);
        self.expect(T![return]);
        self.r_expr_list();
        Some(marker.complete(self))
    }

    pub(super) fn r_break(&mut self) -> Option<CompletedMarker> {
        let marker = self.start(T![break_stmt]);
        self.expect(T![break]);
        Some(marker.complete(self))
    }

    pub(super) fn r_block<F>(&mut self, stop: F) -> Option<CompletedMarker>
    where
        F: Fn(SyntaxKind) -> bool,
    {
        let marker = self.start(T![stmt_list]);
        while !stop(self.at()) {
            self.r_stmt();
        }

        Some(marker.complete(self))
    }
}
