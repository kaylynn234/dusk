use colored::Colorize;
use std::{
    fmt::{Debug, Display},
    iter,
    sync::Arc,
};

use crate::{
    evil::log10,
    position::{LabelLines, Line, Span},
};

#[derive(Debug)]
pub enum OutputKind {
    Error,
    Warning,
    Hint,
}

impl Display for OutputKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            OutputKind::Error => "error".bright_red(),
            OutputKind::Warning => "warning".bright_yellow(),
            OutputKind::Hint => "hint".bright_blue(),
        };

        write!(f, "{}", message)
    }
}

#[derive(Debug)]
pub struct Codeblock {
    pub input: Arc<String>,
    pub span: Span,
    pub underline_span: Option<Span>,
    pub filename: String,
}

impl Display for Codeblock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // When highlighting, this will be the start of the highlighted span.
        // Otherwise it'll just be the start of the codeblock.
        let cursors = self
            .underline_span
            .as_ref()
            .map_or_else(|| self.span.to_cursors(), Span::to_cursors);

        let (start_position, end_position) = (cursors.0.to_position(), cursors.1.to_position());

        write!(f, "{} in {}", start_position, self.filename)?;

        let lines = iter::once(Line::empty()).chain({
            // This is a cheat. I wish rustfmt formatted it more nicely.
            let iterator: Box<dyn Iterator<Item = [Line; 2]>> = match &self.underline_span {
                Some(underline) => Box::new(self.span.highlight_with(underline).unwrap()),
                None => Box::new(self.span.annotated_lines().map(|(line_number, line)| {
                    [Line::new_labelled(line_number, line), Line::empty()]
                })),
            };

            iterator.flat_map(IntoIterator::into_iter)
        });

        let label_length = (log10(end_position.line() as u32) + 1) as usize;
        let labelled = LabelLines::new(label_length, lines);

        for line in labelled {
            write!(f, "\n{}", line.trim_end())?
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct Output {
    pub kind: OutputKind,
    pub message: String,
    pub context: Option<Codeblock>,
}

impl Display for Output {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(context) = &self.context {
            writeln!(f, "{}", context)?;
        }

        write!(f, "{}: {}", self.kind, self.message)?;

        Ok(())
    }
}

impl std::error::Error for Output {}
