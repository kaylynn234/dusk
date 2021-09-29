use std::ops::Range;

#[derive(Debug, Clone, Copy)]
pub struct Span {
    start: usize,
    end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub fn as_range(self) -> Range<usize> {
        self.into()
    }

    pub fn start(&self) -> usize {
        self.start
    }

    pub fn end(&self) -> usize {
        self.end
    }
}

impl From<Range<usize>> for Span {
    fn from(range: Range<usize>) -> Self {
        Self::new(range.start, range.end)
    }
}

impl Into<Range<usize>> for Span {
    fn into(self) -> Range<usize> {
        self.start..self.end
    }
}
