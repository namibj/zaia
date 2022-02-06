use std::{
    convert::TryFrom,
    fmt::{self, Debug, Display},
    ops::{self, Index},
};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Span {
    start: u32,
    end: u32,
}

impl Span {
    pub fn new(start: u32, end: u32) -> Self {
        Self { start, end }
    }

    pub fn start(self) -> u32 {
        self.start
    }

    pub fn end(self) -> u32 {
        self.end
    }

    pub fn from_range(range: ops::Range<usize>) -> Self {
        debug_assert_eq!(
            u32::try_from(range.start),
            Ok(range.start as u32),
            "range {} out of 32bit bounds (max is {})",
            range.start,
            u32::MAX,
        );
        debug_assert_eq!(
            u32::try_from(range.end),
            Ok(range.end as u32),
            "range {} out of 32bit bounds (max is {})",
            range.end,
            u32::MAX,
        );

        Self::new(range.start as u32, range.end as u32)
    }

    pub fn range(self) -> ops::Range<usize> {
        self.start() as usize..self.end() as usize
    }
}

impl ariadne::Span for Span {
    type SourceId = ();

    fn source(&self) -> &Self::SourceId {
        &()
    }

    fn start(&self) -> usize {
        self.start as usize
    }

    fn end(&self) -> usize {
        self.end as usize
    }
}

impl Index<Span> for str {
    type Output = str;

    fn index(&self, index: Span) -> &Self::Output {
        let range: ops::Range<usize> = index.range();
        &self[range]
    }
}

impl From<Span> for ops::Range<u32> {
    fn from(range: Span) -> Self {
        range.start()..range.end()
    }
}

impl Debug for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Debug::fmt(&self.start, f)?;
        f.write_str("..")?;
        Debug::fmt(&self.end, f)
    }
}

impl Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.start, f)?;
        f.write_str("..")?;
        Display::fmt(&self.end, f)
    }
}
