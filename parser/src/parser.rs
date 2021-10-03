use super::*;
use lexer::Token;
use logos::{Lexer, Logos};

#[inline]
fn _next_impl(lexer: &mut Lexer<'_, Token>) -> Result<(Span, Token), Error> {
    lexer
        .next()
        .map(|token| (lexer.span().into(), token))
        .ok_or_else(|| Error::new(lexer.span().into(), ErrorKind::Unexpected(None)))
}

pub struct Parser<'input> {
    lexer: Lexer<'input, Token>,
    errors: Vec<Error>,
}

impl Parser<'_> {
    pub fn new(input: &str) -> Parser {
        Parser {
            lexer: Token::lexer(input),
            errors: Vec::new(),
        }
    }

    pub fn span(&self) -> Span {
        self.lexer.span().into()
    }

    pub fn source(&self) -> &str {
        self.lexer.source()
    }

    pub fn peek(&self) -> Result<(Span, Token), Error> {
        _next_impl(&mut self.lexer.clone())
    }

    pub fn next(&mut self) -> Result<(Span, Token), Error> {
        _next_impl(&mut self.lexer)
    }

    pub fn peek_span(&self) -> Result<Span, Error> {
        let (span, _) = _next_impl(&mut self.lexer.clone())?;
        Ok(span)
    }

    pub fn peek_token(&self) -> Result<Token, Error> {
        let (_, token) = _next_impl(&mut self.lexer.clone())?;
        Ok(token)
    }

    pub fn next_token(&mut self) -> Result<Token, Error> {
        let (_, token) = _next_impl(&mut self.lexer)?;
        Ok(token)
    }

    pub fn peek_matches<T>(&mut self, expected: T) -> Result<T::Output, Error>
    where
        T: Pattern,
    {
        let (span, token) = self.peek()?;
        let slice = &self.source()[self.span()];
        Ok(expected.match_pattern(span, token, slice)?.1)
    }

    pub fn expect_matches<T>(&mut self, expected: T) -> Result<T::Output, Error>
    where
        T: Pattern,
    {
        let (span, token) = self.peek()?;
        let slice = &self.source()[self.span()];
        let (_, result) = expected.match_pattern(span, token, slice)?;
        self.next()?;

        Ok(result)
    }

    pub fn cursor(&self) -> Cursor {
        Cursor(self.span().start())
    }

    pub fn measure(&self, Cursor(start): Cursor) -> Span {
        Span::new(start, self.span().end())
    }
}

pub struct Cursor(usize);
