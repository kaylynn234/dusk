pub use ast::AstNode;
pub use error::Error;
pub use lexer::Token;
use logos::{Lexer, Logos};
use std::{mem, ops::Range};

pub mod ast;
pub mod error;
pub mod parser;
pub mod visitor;

pub type Span = Range<usize>;
pub type ParseResult<T = AstNode> = Result<T, Error>;

pub mod prelude {
    pub use super::ast::*;
    pub use super::error::Error;
    pub use super::Parser;
}

#[derive(Debug, Clone)]
pub struct SpanToken {
    kind: Token,
    span: Span,
}

struct SpanTokenIterator<'i>(Lexer<'i, Token>);

impl<'i> Iterator for SpanTokenIterator<'i> {
    type Item = SpanToken;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|token| SpanToken {
            kind: token,
            span: self.0.span(),
        })
    }
}

pub struct Parser<'i> {
    lexer: SpanTokenIterator<'i>,
    current: Option<SpanToken>,
    lookahead: Option<SpanToken>,
}

impl<'i> Parser<'i> {
    /// Creates a new parser from the given input.
    pub fn new(input: &'i str) -> Parser<'i> {
        let mut lexer = SpanTokenIterator(Token::lexer(input));
        let current = lexer.next();
        let lookahead = lexer.next();

        Parser {
            lexer,
            current,
            lookahead,
        }
    }

    /// Returns the current token.
    pub fn current(&self) -> ParseResult<&SpanToken> {
        self.current.as_ref().ok_or_else(Error::eof)
    }

    /// Returns one token worth of lookahead.
    pub fn lookahead(&self) -> ParseResult<&SpanToken> {
        self.lookahead.as_ref().ok_or_else(Error::eof)
    }

    /// Progresses the parser.
    pub fn advance(&mut self) {
        mem::swap(&mut self.lookahead, &mut self.current);
        self.lookahead = self.lexer.next();
    }

    // This is a bit of a cheat but I'm tired and don't feel like writing out a macro. oh well.
    fn _expect<F, O>(&mut self, expected: Token, f: F) -> ParseResult<O>
    where
        F: FnOnce(&SpanToken) -> O,
    {
        let result = self
            .current()
            .map_err(|_| Error::mismatch(expected, None))
            .and_then(|current_token| {
                if current_token.kind == expected {
                    Ok(f(current_token))
                } else {
                    Err(Error::mismatch(expected, Some(current_token.kind)))
                }
            });

        self.advance();

        result
    }

    /// Produces an error if the current token is not of the expected type, and then progresses the parser.
    /// This can be used to "assert" that a token exists, such as a closing brace.
    pub fn expect(&mut self, expected: Token) -> ParseResult<()> {
        self._expect(expected, |_| ())
    }

    /// Returns a clone of the current token or produces an error if the token is not of the expected type. This progresses the parser.
    /// It can be used to "assert" that a token exists, while also capturing its value.
    pub fn expect_cloned(&mut self, expected: Token) -> ParseResult<SpanToken> {
        self._expect(expected, |token| token.clone())
    }
}
