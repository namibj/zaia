use std::process::Command;

use super::{machinery::marker::CompletedMarker, Parser};
use crate::T;

impl<'source> Parser<'source> {
    pub(super) fn r_simple_expr(&mut self) -> Option<CompletedMarker> {
        let mut marker = self.start();
        todo!();
        Some(marker.complete(self, T![simple_expr]))
    }
}
