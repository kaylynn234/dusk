use std::{borrow::Cow, fmt::Display, iter, ops::Range, sync::Arc};

#[derive(Clone)]
pub struct Position {
    line: usize,
    column: usize,
}

impl Default for Position {
    fn default() -> Self {
        Position { line: 1, column: 1 }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "line {}, column {}", self.line, self.column)
    }
}

impl Position {
    pub fn from_str(slice: &str) -> Self {
        let mut position = Position::default();

        for character in slice.chars() {
            if character == '\n' {
                position.line += 1;
                position.column = 1;
            } else {
                position.column += 1
            }
        }

        position
    }

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn column(&self) -> usize {
        self.column
    }
}

/// A struct representing a valid index into the input.
#[derive(Clone)]
pub struct Cursor {
    input: Arc<String>,
    // This /must/ be a valid codepoint boundary.
    index: usize,
}

impl Cursor {
    /// Tries to create a new cursor from the given input and index.
    /// This will fail if the input is not a valid codepoint boundary.
    pub fn try_new(input: &Arc<String>, index: usize) -> Option<Self> {
        input.get(..index).map(|_| Cursor {
            input: Arc::clone(input),
            index,
        })
    }

    /// Creates a new `Position` from this cursor. A `Position` represents line and column information,
    /// rather than an index into the input. Since all `Cursor`s are valid indexes in the input text,
    /// this function does not fail.
    pub fn to_position(&self) -> Position {
        Position::from_str(&self.input[..self.index])
    }
}

/// A smaller version of `Span` representing two indexes into a piece of input.
/// This exists for performance reasons and can be upgraded to a `Span` when required.
#[derive(Debug, Copy, Clone)]
pub struct PartialSpan {
    // These /must/ be valid codepoint boundaries.
    start: usize,
    end: usize,
}

impl PartialSpan {
    /// Creates a new `PartialSpan` from the `start` and `end` arguments.
    /// If you attempt to upgrade this to a `Span` later, these must be valid codepoint boundaries.
    pub fn new(start: usize, end: usize) -> Self {
        PartialSpan { start, end }
    }

    /// Returns this `PartialSpan` as a `Range<usize>`.
    pub fn to_range(&self) -> Range<usize> {
        self.start..self.end
    }

    /// Tries to upgrade this to a `Span` using the provided `Arc<String>`.
    /// This returns `None` if either of this span's indexes are not valid codepoint boundaries for `input`.
    pub fn upgrade(&self, input: &Arc<String>) -> Option<Span> {
        input.get(self.start..self.end).map(|_| Span {
            input: Arc::clone(input),
            start: self.start,
            end: self.end,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Span {
    input: Arc<String>,
    // These /must/ be valid codepoint boundaries.
    start: usize,
    end: usize,
}

impl Span {
    /// Creates a new `Span` from two `Cursor`s.
    /// This panics if the cursors are from different inputs.
    pub fn from_cursors(first: &Cursor, end: &Cursor) -> Self {
        if !Arc::ptr_eq(&first.input, &end.input) {
            panic!("Cursors come from different inputs")
        }

        Span {
            input: Arc::clone(&first.input),
            start: first.index,
            end: end.index,
        }
    }

    /// Creates a pair of cursors from this span.
    pub fn to_cursors(&self) -> (Cursor, Cursor) {
        // We already know these spans are valid because of invariants.
        (
            Cursor::try_new(&self.input, self.start).unwrap(),
            Cursor::try_new(&self.input, self.end).unwrap(),
        )
    }

    /// Returns `true` if this span fits inside `second`.
    pub fn fits_in(&self, second: &Self) -> bool {
        second.start >= self.start && second.end <= self.end
    }

    /// Returns an iterator that highlights this span with another.
    /// If the argument passed to `second` does not fit within this one, this returns `None`.
    pub fn highlight_with(&self, second: &Self) -> Option<impl Iterator<Item = [Line; 2]> + '_> {
        self.fits_in(second).then(|| LinesHighlighted {
            seen: second.count_before(),
            start: second.start,
            end: second.end,
            lines: self.annotated_lines(),
        })
    }

    /// Returns a count of all characters in the input that occur before this span starts.
    pub fn count_before(&self) -> usize {
        let start = Position::from_str(&self.input[..self.start]);

        self.input
            .split_inclusive('\n')
            .take(start.line - 1)
            .map(str::len)
            .sum()
    }

    /// Returns an iterator yielding pairs of (line number, line) for each line this span covers.
    /// The yielded lines will include newlines.
    pub fn annotated_lines(&self) -> impl Iterator<Item = (usize, &str)> {
        let start = Position::from_str(&self.input[..self.start]);
        let end = Position::from_str(&self.input[..self.end]);

        self.input
            .split_inclusive('\n')
            .enumerate()
            .skip(start.line - 1)
            // TODO: Should this be a call to `min(1, ...)` instead?
            .take(end.line + 1 - start.line)
            .map(|(line_index, line)| (line_index + 1, line))
    }
}

#[inline]
fn repeat_n<T: Clone>(item: T, n: usize) -> impl Iterator<Item = T> {
    iter::repeat(item).take(n)
}

/// Represents a line of text that may or may not have a line number attached to it.
#[derive(Clone)]
pub enum Line<'i> {
    /// This line should be labelled with a line number
    Labelled(usize, Cow<'i, str>),
    /// This line should not be labelled with a line number.
    Unlabelled(Cow<'i, str>),
}

impl<'i> Line<'i> {
    pub fn is_labelled(&self) -> bool {
        match self {
            Line::Labelled(_, _) => true,
            Line::Unlabelled(_) => false,
        }
    }

    pub fn new_labelled<T: Into<Cow<'i, str>>>(index: usize, value: T) -> Self {
        Line::Labelled(index, value.into())
    }

    pub fn new_unlabelled<T: Into<Cow<'i, str>>>(value: T) -> Self {
        Line::Unlabelled(value.into())
    }

    pub fn empty() -> Self {
        Line::Unlabelled(Cow::Borrowed(""))
    }
}

pub struct LinesHighlighted<T> {
    seen: usize,
    start: usize,
    end: usize,
    lines: T,
}

fn difference(first: usize, second: usize) -> usize {
    (first as isize - second as isize).abs() as usize
}

impl<'i, T> Iterator for LinesHighlighted<T>
where
    T: Iterator<Item = (usize, &'i str)>,
{
    type Item = [Line<'i>; 2];

    fn next(&mut self) -> Option<Self::Item> {
        let (index, line) = self.lines.next()?;
        let next = self.seen + line.len();

        let has_start = (self.seen..=next).contains(&self.start);
        let has_end = (self.seen..=next).contains(&self.end);

        // I'm not the biggest fan of this.
        let highlighted = {
            if has_start && has_end {
                repeat_n(' ', difference(self.start, self.seen))
                    .chain(repeat_n('^', self.end - self.start))
                    .collect()
            } else if has_start {
                repeat_n(' ', difference(self.start, self.seen))
                    .chain(repeat_n('^', next - self.start))
                    .collect()
            } else if has_end {
                repeat_n('^', self.end - self.seen).collect()
            } else {
                String::new()
            }
        };

        self.seen = next;

        Some([
            Line::new_labelled(index, line),
            Line::new_unlabelled(highlighted),
        ])
    }
}

pub struct LabelLines<T> {
    padding: usize,
    lines: T,
}

impl<T> LabelLines<T> {
    pub fn new(padding: usize, lines: T) -> Self {
        Self { padding, lines }
    }
}

impl<'i, T> Iterator for LabelLines<T>
where
    T: Iterator<Item = Line<'i>>,
{
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.lines.next().map(|item| match item {
            Line::Labelled(number, text) => format!("{1:0$} | {2}", self.padding, number, text),
            Line::Unlabelled(text) => format!("{} | {}", " ".repeat(self.padding), text),
        })
    }
}
