pub use ast::AstNode;
use common::{
    output::{Codeblock, Output, OutputKind},
    position::{PartialSpan, Span},
};
pub use lexer::Token;
use logos::{Lexer, Logos};
use std::{fmt::Display, mem, sync::Arc};

pub mod ast;
pub mod parser;
pub mod visitor;

pub type Result<T = AstNode> = std::result::Result<T, Output>;

#[derive(Debug, Clone)]
pub struct SpanToken {
    kind: Token,
    span: PartialSpan,
}

struct SpannedLexer<'i> {
    lexer: Lexer<'i, Token>,
}

impl<'i> Iterator for SpannedLexer<'i> {
    type Item = SpanToken;

    fn next(&mut self) -> Option<Self::Item> {
        let kind = self.lexer.next()?;
        let span = self.lexer.span();

        // Logos gives us spans as ranges, but we use our own type.
        let span = PartialSpan::new(span.start, span.end);

        Some(SpanToken { kind, span })
    }
}

pub struct Parser<'i> {
    /// The lexer that this parser uses.
    lexer: SpannedLexer<'i>,
    /// The previous token. This is used for error reporting.
    previous: Option<SpanToken>,
    /// The current token. This is `None` at EOF.
    current: Option<SpanToken>,
    /// The filename this parser is reading from. This is used for error reporting.
    filename: String,
    /// The input this parser is reading from.
    input: Arc<String>,
}

// TODO: Make errors consistent
impl<'i> Parser<'i> {
    /// Creates a new parser from the given input.
    pub fn new(input: &'i Arc<String>, filename: String) -> Parser<'i> {
        let mut lexer = SpannedLexer {
            lexer: Token::lexer(input),
        };

        let current = lexer.next();

        Parser {
            lexer,
            previous: None,
            current,
            filename,
            input: Arc::clone(input),
        }
    }

    /// Returns a `Span` that highlights either the current token, or the previous token if at EOF.
    /// This will panic if the input is empty.
    fn highlight_current(&self) -> Span {
        self.current
            .as_ref()
            .or_else(|| self.previous.as_ref())
            .map(|token| token.span)
            .expect("tried to highlight current token of empty parser")
            .upgrade(&self.input)
            .expect("span is an invalid character boundary")
    }

    fn expected<T: Display>(&self, expected: T, found: Option<&SpanToken>) -> Output {
        let message = match found {
            Some(found) => format!("Expected this to be {}, but found {} instead.", expected, found.kind),
            None => format!("Expected {} after this, but reached EOF.", expected),
        };

        let span = self.highlight_current();
        let codeblock = Codeblock {
            // It might be better to use an `Arc` for the filename but I guess I'm not really fussed.
            filename: self.filename.clone(),
            input: Arc::clone(&self.input),
            span: span.clone(),
            underline_span: Some(span),
        };

        Output {
            kind: OutputKind::Error,
            message,
            context: Some(codeblock),
        }
    }

    /// Returns the current token wrapped in `Ok`, or an error at EOF.
    pub fn current_or<T: Display>(&self, message: T) -> Result<&SpanToken> {
        self.current
            .as_ref()
            .ok_or_else(|| self.expected(message, None))
    }

    /// Progresses the parser.
    pub fn advance(&mut self) {
        mem::swap(&mut self.current, &mut self.previous);
        self.current = self.lexer.next();
    }

    // This is a bit of a cheat but I'm tired and don't feel like writing out a macro. oh well.
    fn _assert<T, F, O>(&mut self, message: T, expected: Token, f: F) -> Result<O>
    where
        T: Display,
        F: FnOnce(&SpanToken) -> O,
    {
        let result = self
            .current
            .as_ref()
            .ok_or_else(|| self.expected(&message, None))
            .and_then(|current_token| {
                if current_token.kind == expected {
                    Ok(f(current_token))
                } else {
                    Err(self.expected(&message, Some(&current_token)))
                }
            });

        self.advance();

        result
    }

    /// Produces an error if the current token is not of the expected type, and then progresses the parser.
    /// This can be used to "assert" that a token exists, such as a closing brace.
    pub fn assert(&mut self, expected: Token) -> Result<()> {
        self._assert(expected, expected, |_| ())
    }

    /// Returns a clone of the current token or produces an error if the token is not of the expected type. This progresses the parser.
    /// It can be used to "assert" that a token exists, while also capturing its value.
    pub fn assert_cloned(&mut self, expected: Token) -> Result<SpanToken> {
        self._assert(expected, expected, |token| token.clone())
    }
}
