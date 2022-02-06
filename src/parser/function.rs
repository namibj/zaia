use std::process::Command;

use super::{machinery::marker::CompletedMarker, Parser};
use crate::T;

impl<'source> Parser<'source> {
    pub(super) fn r_func_call_args(&mut self) -> Option<CompletedMarker> {
        let marker = self.start();
        self.expect(T!['(']);

        while self.at() != T![')'] {
            self.r_expr();

            if self.at() == T![,] {
                self.expect(T![,]);
            } else {
                break;
            }
        }

        self.expect(T![')']);
        Some(marker.complete(self, T![func_args]))
    }

    pub(super) fn r_func(&mut self) -> Option<CompletedMarker> {
        let marker = self.start();
        self.expect(T![function]);
        self.r_simple_expr();
        self.r_func_def_args();
        self.r_block(|t| t == T![end]);
        Some(marker.complete(self, T![func_stmt]))
    }

    fn r_func_def_args(&mut self) -> Option<CompletedMarker> {
        let marker = self.start();
        self.expect(T!['(']);

        while self.at() != T![')'] {
            self.expect(T![ident]);

            if self.at() == T![,] {
                self.expect(T![,]);
            } else {
                break;
            }
        }

        self.expect(T![')']);
        Some(marker.complete(self, T![func_args]))
    }
}
