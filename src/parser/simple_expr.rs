use super::{machinery::marker::CompletedMarker, Parser};
use crate::T;

impl<'cache, 'source> Parser<'cache, 'source> {
    pub(super) fn r_simple_expr(&mut self, allow_call: bool) -> Option<CompletedMarker> {
        if self.at() == T!['('] {
            let marker = self.start();
            self.r_expr();
            return Some(marker.complete(self, T![simple_expr]));
        }

        let mut lhs = self.r_ident()?;

        loop {
            let t = self.at();

            if t == T!['('] && allow_call {
                let n = lhs.precede(self);
                let _rhs = self.r_func_call_args()?;
                lhs = n.complete(self, T![func_call]);
                continue;
            }

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
                let _rhs = self.r_ident();
                lhs = n.complete(self, T![bin_op]);
                continue;
            }

            break;
        }

        Some(lhs)
    }
}
