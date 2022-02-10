use super::{
    machinery::{classifiers::token_is_expr_start, marker::CompletedMarker},
    Parser,
};
use crate::T;

impl<'cache, 'source> Parser<'cache, 'source> {
    pub(super) fn r_table(&mut self) -> Option<CompletedMarker> {
        let marker = self.start();
        self.expect(T!['{']);

        loop {
            match self.at() {
                T!['}'] => {
                    self.expect(T!['}']);
                    break;
                },
                _ => {
                    self.r_table_elem();
                },
            }

            let t = self.at();
            if t == T![,] || t == T![;] {
                self.expect(t);
            } else {
                self.expect(T!['}']);
                break;
            }
        }

        Some(marker.complete(self, T![table_expr]))
    }

    fn r_table_elem(&mut self) -> Option<CompletedMarker> {
        match self.at() {
            T![ident] if self.peek() == T![=] => self.r_table_elem_map(),
            T!['['] => self.r_table_elem_generic(),
            t if token_is_expr_start(t) => self.r_table_elem_array(),
            _ => unreachable!(),
        }
    }

    fn r_table_elem_array(&mut self) -> Option<CompletedMarker> {
        let marker = self.start();
        self.r_expr();
        Some(marker.complete(self, T![table_array_elem]))
    }

    fn r_table_elem_map(&mut self) -> Option<CompletedMarker> {
        let marker = self.start();
        self.expect(T![ident]);
        self.expect(T![=]);
        self.r_expr();
        Some(marker.complete(self, T![table_map_elem]))
    }

    fn r_table_elem_generic(&mut self) -> Option<CompletedMarker> {
        let marker = self.start();
        self.expect(T!['[']);
        self.r_expr();
        self.expect(T![']']);
        self.expect(T![=]);
        self.r_expr();
        Some(marker.complete(self, T![table_generic_elem]))
    }
}
