use ast::*;
pub use error::Error;
use lexer::Token;
use logos::{Lexer, Logos};
use std::{mem, ops::Range};

mod ast;
mod error;
mod parser;

pub type Span = Range<usize>;
pub type ParseResult<T = AstNode> = Result<T, Error>;

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
    fn current(&self) -> ParseResult<&SpanToken> {
        self.current.as_ref().ok_or_else(Error::eof)
    }

    /// Returns one token worth of lookahead.
    fn lookahead(&self) -> ParseResult<&SpanToken> {
        self.lookahead.as_ref().ok_or_else(Error::eof)
    }

    /// Progresses the parser.
    fn advance(&mut self) {
        mem::swap(&mut self.lookahead, &mut self.current);
        self.lookahead = self.lexer.next();
        println!("Advanced! Token stack is now {:?}, {:?}", self.current, self.lookahead);
    }

    /// Produces an error if the current token is not of the expected type, and then progresses the parser.
    /// This can be used to "assert" that a token exists, such as a closing brace.
    fn expect(&mut self, expected: Token) -> ParseResult<()> {
        let result = self
            .current()
            .map_err(|_| Error::mismatch(expected, None))
            .and_then(|current_token| {
                if current_token.kind == expected {
                    Ok(())
                } else {
                    Err(Error::mismatch(expected, Some(current_token.kind)))
                }
            });

        self.advance();

        result
    }
}
