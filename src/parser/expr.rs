use super::{machinery::marker::CompletedMarker, Parser};
use crate::{
    parser::machinery::{
        binding_power::{
            infix_binding_power,
            prefix_binding_power,
            CALL_BINDING_POWER,
            INDEX_BINDING_POWER,
        },
        classifiers::{token_is_expr_start, token_is_literal, token_is_unary_op},
    },
    T,
};

impl<'cache, 'source> Parser<'cache, 'source> {
    pub(super) fn r_expr_list(&mut self) {
        let marker = self.start(T![expr_list]);

        while token_is_expr_start(self.at()) {
            self.r_expr();
            if self.at() != T![,] {
                break;
            }

            self.expect(T![,]);
        }

        marker.complete(self);
    }

    pub(super) fn r_expr(&mut self) -> Option<CompletedMarker> {
        self.r_expr_inner(0)
    }

    fn r_expr_inner(&mut self, min_bp: i32) -> Option<CompletedMarker> {
        let mut lhs = self.r_expr_lhs()?;

        loop {
            let t = self.at();

            if t == T!['('] && CALL_BINDING_POWER >= min_bp {
                let n = lhs.precede(self, T![func_call]);
                let _rhs = self.r_func_call_args()?;
                lhs = n.complete(self);
                continue;
            }

            if t == T!['['] && INDEX_BINDING_POWER >= min_bp {
                let n = lhs.precede(self, T![index]);
                self.expect(T!['[']);
                let _rhs = self.r_expr()?;
                self.expect(T![']']);
                lhs = n.complete(self);
                continue;
            }

            if let Some((l_bp, r_bp)) = infix_binding_power(t) {
                if l_bp < min_bp {
                    break;
                }

                let n = lhs.precede(self, T![bin_op]);
                self.expect(t);
                let _rhs = self.r_expr_inner(r_bp);
                lhs = n.complete(self);
                continue;
            }

            break;
        }

        Some(lhs)
    }

    fn r_expr_lhs(&mut self) -> Option<CompletedMarker> {
        match self.at() {
            T![ident] => self.r_ident(),
            T![...] => self.r_vararg(),
            T!['{'] => self.r_table(),
            T!['('] => self.r_paren(),
            T![function] => self.r_func(true),
            t if token_is_unary_op(t) => self.r_expr_unary(),
            t if token_is_literal(t) => self.r_literal(),
            _ => None,
        }
    }

    fn r_expr_unary(&mut self) -> Option<CompletedMarker> {
        let n = self.start(T![prefix_op]);
        let op = self.at();
        self.expect(op);
        let ((), r_bp) = prefix_binding_power(op);
        let _rhs = self.r_expr_inner(r_bp);
        Some(n.complete(self))
    }

    pub(super) fn r_ident(&mut self) -> Option<CompletedMarker> {
        let marker = self.start(T![ident]);
        self.expect(T![ident]);
        Some(marker.complete(self))
    }

    pub(super) fn r_vararg(&mut self) -> Option<CompletedMarker> {
        let marker = self.start(T![vararg_expr]);
        self.expect(T![...]);
        Some(marker.complete(self))
    }

    fn r_paren(&mut self) -> Option<CompletedMarker> {
        let marker = self.start(T![expr]);
        self.expect(T!['(']);
        let _rhs = self.r_expr()?;
        self.expect(T![')']);
        Some(marker.complete(self))
    }

    fn r_literal(&mut self) -> Option<CompletedMarker> {
        let marker = self.start(T![literal_expr]);
        let kind = self.at();
        self.expect(kind);
        Some(marker.complete(self))
    }
}
