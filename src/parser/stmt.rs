use super::{
    machinery::{kind::SyntaxKind, marker::CompletedMarker},
    Parser,
};
use crate::T;

const STATEMENT_RECOVERY: &[SyntaxKind] = &[
    T![do],
    T![while],
    T![repeat],
    T![if],
    T![for],
    T![return],
    T![break],
    T![function],
    T![local],
];

impl<'cache, 'source> Parser<'cache, 'source> {
    pub(super) fn r_stmt(&mut self) -> Option<CompletedMarker> {
        match self.at() {
            T![do] => self.r_do(),
            T![while] => self.r_while(),
            T![repeat] => self.r_repeat(),
            T![if] => self.r_if(T![if]),
            T![for] => self.r_for(),
            T![return] => self.r_return(),
            T![break] => self.r_break(),
            T![function] => self.r_func(false),
            T![local] => self.r_decl(),
            T![ident] | T!['('] => self.r_maybe_assign(),
            T![;] => self.r_semicolon(),
            T![eof] => None,
            _ => {
                let span = self.error_eat_until(STATEMENT_RECOVERY);
                let source = self.source(span);
                let error = self
                    .new_error()
                    .with_message("expected a statement")
                    .with_label(
                        self.new_label()
                            .with_message(format!("expected a statement but got \"{}\"", source,)),
                    )
                    .finish();

                self.report(error);
                None
            },
        }
    }

    fn r_semicolon(&mut self) -> Option<CompletedMarker> {
        let marker = self.start();
        self.expect(T![;]);
        Some(marker.complete(self, T![;]))
    }
}
