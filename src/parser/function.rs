use super::{machinery::marker::CompletedMarker, Parser};
use crate::T;

impl<'cache, 'source> Parser<'cache, 'source> {
    pub(super) fn r_func_call_args(&mut self) -> Option<CompletedMarker> {
        let marker = self.start(T![func_args]);
        self.expect(T!['(']);

        loop {
            match self.at() {
                T![')'] => {
                    self.expect(T![')']);
                    break;
                },
                _ => {
                    self.r_expr();
                },
            }

            if self.at() == T![,] {
                self.expect(T![,]);
            } else {
                self.expect(T![')']);
                break;
            }
        }

        Some(marker.complete(self))
    }

    pub(super) fn r_func(&mut self, expr: bool) -> Option<CompletedMarker> {
        let kind = if expr { T![func_expr] } else { T![func_stmt] };
        let marker = self.start(kind);
        self.expect(T![function]);

        if !expr {
            self.r_simple_expr(false);
        }

        self.r_func_def_args();
        self.r_block(|t| t == T![end]);
        self.expect(T![end]);
        Some(marker.complete(self))
    }

    fn r_func_def_args(&mut self) -> Option<CompletedMarker> {
        let marker = self.start(T![func_args]);
        self.expect(T!['(']);

        loop {
            match self.at() {
                T![')'] => {
                    self.expect(T![')']);
                    break;
                },
                T![...] => {
                    self.expect(T![...]);
                },
                T![ident] => {
                    self.r_ident();
                },
                _ => unreachable!(),
            }

            if self.at() == T![,] {
                self.expect(T![,]);
            } else {
                self.expect(T![')']);
                break;
            }
        }

        Some(marker.complete(self))
    }
}
