use super::{machinery::marker::CompletedMarker, Parser};
use crate::T;

impl<'cache, 'source> Parser<'cache, 'source> {
    pub(super) fn r_func_call_args(&mut self) -> Option<CompletedMarker> {
        let marker = self.start();
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

        Some(marker.complete(self, T![func_args]))
    }

    pub(super) fn r_func(&mut self, expr: bool) -> Option<CompletedMarker> {
        let marker = self.start();
        self.expect(T![function]);

        if !expr {
            self.r_simple_expr(false);
        }

        self.r_func_def_args();
        self.r_block(|t| t == T![end]);
        self.expect(T![end]);
        let kind = if expr { T![func_expr] } else { T![func_stmt] };
        Some(marker.complete(self, kind))
    }

    fn r_func_def_args(&mut self) -> Option<CompletedMarker> {
        let marker = self.start();
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
                    self.expect(T![ident]);
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

        Some(marker.complete(self, T![func_args]))
    }
}
