use std::ops::{Index, Range};

/// Represents a type that occupies an input span
pub trait Spanned {
    fn span(&self) -> Span;
}

/// A span, comprised of a start and end, representing a slice of some source input.
///
/// # Why is this not Range<usize>?
///
/// For some inane reason, `Range<usize>` is not `Copy`. This type is.
#[derive(Debug, Clone, Copy)]
pub struct Span {
    start: usize,
    end: usize,
}

impl Span {
    /// Creates a new span, referring to some portion of source input.
    ///
    /// # Panics
    ///
    /// This will panic if `end` < `start`, as that would result in an invalid span.
    pub fn new(start: usize, end: usize) -> Self {
        assert!(end >= start, "span end is higher than start");

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

impl Index<Span> for str {
    type Output = str;

    fn index(&self, index: Span) -> &Self::Output {
        &self[index.as_range()]
    }
}
