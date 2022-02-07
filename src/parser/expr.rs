use std::process::Command;

use super::{
    machinery::{kind::SyntaxKind, marker::CompletedMarker},
    Parser,
};
use crate::{
    parser::machinery::{
        binding_power::{
            infix_binding_power,
            prefix_binding_power,
            CALL_BINDING_POWER,
            INDEX_BINDING_POWER,
        },
        classifiers::{
            token_is_binary_op,
            token_is_expr_start,
            token_is_literal,
            token_is_unary_op,
        },
    },
    T,
};

impl<'source> Parser<'source> {
    pub(super) fn r_expr_list(&mut self) -> Option<CompletedMarker> {
        let marker = self.start();

        while token_is_expr_start(self.at()) {
            self.r_expr()?;
            if self.at() != T![,] {
                break;
            }

            self.expect(T![,]);
        }

        Some(marker.complete(self, T![expr_list]))
    }

    pub(super) fn r_expr(&mut self) -> Option<CompletedMarker> {
        self.r_expr_inner(0)
    }

    fn r_expr_inner(&mut self, min_bp: i32) -> Option<CompletedMarker> {
        let mut lhs = self.r_expr_lhs()?;

        loop {
            let t = self.at();

            if t == T!['('] && CALL_BINDING_POWER >= min_bp {
                let n = lhs.precede(self);
                self.expect(T!['(']);
                let _rhs = self.r_func_call_args()?;
                lhs = n.complete(self, T![func_call]);
                continue;
            }

            if t == T!['['] && INDEX_BINDING_POWER >= min_bp {
                let n = lhs.precede(self);
                self.expect(T!['[']);
                let _rhs = self.r_expr()?;
                self.expect(T![']']);
                lhs = n.complete(self, T![index]);
                continue;
            }

            if let Some((l_bp, r_bp)) = infix_binding_power(t) {
                if l_bp < min_bp {
                    break;
                }

                let n = lhs.precede(self);
                self.expect(t);
                let _rhs = self.r_expr_inner(r_bp);
                lhs = n.complete(self, T![bin_op]);
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
            T![function] => todo!(),
            t if token_is_unary_op(t) => todo!(),
            t if token_is_literal(t) => todo!(),
            _ => None,
        }
    }

    fn r_ident(&mut self) -> Option<CompletedMarker> {
        let marker = self.start();
        self.expect(T![ident]);
        Some(marker.complete(self, T![ident]))
    }

    fn r_vararg(&mut self) -> Option<CompletedMarker> {
        let marker = self.start();
        self.expect(T![...]);
        Some(marker.complete(self, T![vararg_expr]))
    }

    fn r_paren(&mut self) -> Option<CompletedMarker> {
        let marker = self.start();
        self.expect(T!['(']);
        let _rhs = self.r_expr()?;
        self.expect(T![')']);
        Some(marker.complete(self, T![expr]))
    }

    fn r_table(&mut self) -> Option<CompletedMarker> {
        todo!()
    }
}
