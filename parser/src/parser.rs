use crate::{
    error::{ErrorBuilder, Unexpected},
    span::SpannedToken,
};

use super::*;
use lexer::Token;
use logos::{Lexer, Logos};

#[inline]
fn _next_impl(lexer: &mut Lexer<'_, Token>) -> Result<SpannedToken, Error> {
    lexer
        .next()
        .map(|token| SpannedToken(lexer.span().into(), token))
        .ok_or_else(|| Error::new(lexer.span().into(), ErrorKind::Simple(Unexpected::Eof)))
}

pub struct Parser<'source> {
    pub(crate) lexer: Lexer<'source, Token>,
    pub(crate) errors: Vec<Error>,
    pub(crate) unclosed_delimiters: Vec<SpannedToken>,
}

impl<'source> Parser<'source> {
    pub fn new(input: &str) -> Parser {
        Parser {
            lexer: Token::lexer(input),
            errors: Vec::new(),
            unclosed_delimiters: Vec::new(),
        }
    }

    pub fn span(&self) -> Span {
        self.lexer.span().into()
    }

    pub fn source(&self) -> &str {
        self.lexer.source()
    }

    pub fn peek(&self) -> Result<SpannedToken, Error> {
        _next_impl(&mut self.lexer.clone())
    }

    pub fn next(&mut self) -> Result<SpannedToken, Error> {
        _next_impl(&mut self.lexer)
    }

    pub fn peek_matches(&mut self, expected: impl Pattern) -> Result<SpannedToken, Error> {
        let token = self.peek()?;
        Ok(expected.match_pattern(self, token)?)
    }

    pub fn expect_matches(&mut self, expected: impl Pattern) -> Result<SpannedToken, Error> {
        let token = self.peek()?;
        let result = expected.match_pattern(self, token)?;
        self.next()?;

        Ok(result)
    }

    pub fn cursor(&self) -> Cursor {
        Cursor(self.span().start())
    }

    pub fn measure(&self, Cursor(start): Cursor) -> Span {
        Span::new(start, self.span().end())
    }

    pub fn error<'a>(&'a mut self) -> ErrorBuilder<'a, 'source> {
        ErrorBuilder::new(self)
    }

    /// Get a reference to the parser's errors.
    pub fn errors(&self) -> &[Error] {
        self.errors.as_slice()
    }
}

pub struct Cursor(usize);
