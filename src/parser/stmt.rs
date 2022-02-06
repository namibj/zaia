use super::{
    machinery::{kind::SyntaxKind, marker::CompletedMarker},
    Parser,
};
use crate::T;

const STATEMENT_RECOVERY: &[SyntaxKind] = &[];

impl<'source> Parser<'source> {
    pub(super) fn stmt(&mut self) -> Option<CompletedMarker> {
        match self.at() {
            T![do] => todo!(),
            T![while] => todo!(),
            T![repeat] => todo!(),
            T![if] => todo!(),
            T![for] => todo!(),
            T![return] => todo!(),
            T![break] => todo!(),
            T![function] => todo!(),
            T![local] => todo!(),
            T![ident] => todo!(),
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

                self.error(error);
                None
            },
        }
    }
}
