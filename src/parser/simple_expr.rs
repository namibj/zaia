use std::process::Command;

use super::{machinery::marker::CompletedMarker, Parser};
use crate::T;

impl<'source> Parser<'source> {
    pub(super) fn r_simple_expr(&mut self) -> Option<CompletedMarker> {
        let mut lhs = self.r_ident()?;

        loop {
            let t = self.at();

            if t == T!['['] {
                let n = lhs.precede(self);
                self.expect(T!['[']);
                let _rhs = self.r_expr()?;
                self.expect(T![']']);
                lhs = n.complete(self, T![index]);
                continue;
            }

            if t == T![.] || t == T![:] {
                let n = lhs.precede(self);
                self.expect(t);
                let _rhs = self.r_simple_expr();
                lhs = n.complete(self, T![bin_op]);
                continue;
            }

            break;
        }

        Some(lhs)
    }
}
