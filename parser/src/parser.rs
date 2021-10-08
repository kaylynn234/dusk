use crate::{
    error::{ErrorBuilder, Unexpected},
    span::{Spanned, SpannedToken},
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

// See https://github.com/rust-lang/rust/issues/34511#issuecomment-373423999 for information on why we do this.
trait Captures<'a> {}
impl<'a, T: ?Sized> Captures<'a> for T {}

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

    /// Peek the next token in the stream, to see if it matches a pattern. This will not progress the parser - it is
    /// purely lookahead.
    pub fn peek_matches(&mut self, expected: impl Pattern) -> Result<SpannedToken, Error> {
        let token = self.peek()?;
        Ok(expected.match_pattern(self, token)?)
    }

    /// Consume the next token in the stream, and return an error if it does not match a pattern. This will **always**
    /// progress the parser, as it attempts to recover if the pattern does not match.
    pub fn expect_matches(
        &mut self,
        expected: impl Pattern + IntoDiagnostic,
    ) -> Result<SpannedToken, Error> {
        let token = self.next()?;
        expected
            .match_pattern(self, token)
            .or_else(self.recover_with_token_deletion(token, expected))
    }

    pub fn expect_matches_or_recover_with(
        &mut self,
        expected: impl Pattern + IntoDiagnostic + Copy,
        recover: impl Pattern + IntoDiagnostic,
    ) -> Result<SpannedToken, Error> {
        let token = self.next()?;
        expected
            .match_pattern(self, token)
            .or_else(self.recover_with_token_insertion(token, expected, recover))
            .or_else(self.recover_with_token_deletion(token, expected))
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

    fn recover_with_token_deletion<'a>(
        &'a mut self,
        mut token: SpannedToken,
        expected: impl Pattern + IntoDiagnostic + 'a,
    ) -> impl FnOnce(Error) -> Result<SpannedToken, Error> + 'a + Captures<'source> {
        move |original| {
            // Here we use "token deletion" as a recovery strategy. If we expect a token X, but find any number of
            // tokens Y followed immediately by a token X, then we can "delete" those Y tokens, and go about our day.
            // That being said, we still need to report the error.
            let first = token;

            let mut result = loop {
                if let Ok(token) = expected.match_pattern(self, token) {
                    let error = self
                        .error()
                        .location(first.span().union(token.span()))
                        .mismatch(expected, first)
                        .build()
                        .unwrap();

                    self.errors.push(error);
                    break Ok(token);
                }

                token = match self.next() {
                    Ok(token) => token,
                    Err(_) => break Err(original),
                };
            };

            if let Err(ref mut error) = result {
                error.location = first.span().union(error.location)
            }

            result
        }
    }

    fn recover_with_token_insertion<'a>(
        &'a mut self,
        token: SpannedToken,
        expected: impl Pattern + IntoDiagnostic + 'a,
        recover: impl Pattern + IntoDiagnostic + 'a,
    ) -> impl FnOnce(Error) -> Result<SpannedToken, Error> + 'a + Captures<'source> {
        move |original| {
            // Here we use "token insertion" as a recovery strategy. If we expect a token X, but find a token Y, and Y
            // would be valid after X, we pretend that we "inserted" X. That being said, we still need to report the
            // error.
            let found = recover.match_pattern(self, token).or(Err(original))?;
            let error = self
                .error()
                .location(token.span())
                .mismatch(expected, found)
                .build()
                .unwrap();

            self.errors.push(error);
            Ok(token)
        }
    }
}

pub struct Cursor(usize);
