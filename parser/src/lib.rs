use error::{Error, ErrorKind, Unlabelled};
use lexer::Token;
use logos::Lexer;
use span::Span;

pub mod ast;
pub mod error;
pub mod parser;
pub mod span;
pub mod visitor;

pub mod prelude {
    pub use super::ast::*;
    pub use super::error::{
        Diagnostic, DiagnosticToken, Error, ErrorKind, IntoDiagnosticExt, LabelExt, Unlabelled,
    };
    pub use super::span::Span;
    pub use super::Parser;
    pub use super::Pattern;
}

#[inline]
fn _next_impl(lexer: &mut Lexer<'_, Token>) -> Result<(Span, Token), Error<Unlabelled>> {
    lexer
        .next()
        .map(|token| (lexer.span().into(), token))
        .ok_or_else(|| Error::new(lexer.span().into(), ErrorKind::Unexpected(None)))
}

pub struct Parser<'input> {
    lexer: Lexer<'input, Token>,
}

impl Parser<'_> {
    pub fn span(&self) -> Span {
        self.lexer.span().into()
    }

    pub fn source(&self) -> &str {
        self.lexer.source()
    }

    pub fn peek(&self) -> Result<(Span, Token), Error<Unlabelled>> {
        _next_impl(&mut self.lexer.clone())
    }

    pub fn next(&mut self) -> Result<(Span, Token), Error<Unlabelled>> {
        _next_impl(&mut self.lexer)
    }

    pub fn peek_span(&self) -> Result<Span, Error<Unlabelled>> {
        let (span, _) = _next_impl(&mut self.lexer.clone())?;
        Ok(span)
    }

    pub fn peek_token(&self) -> Result<Token, Error<Unlabelled>> {
        let (_, token) = _next_impl(&mut self.lexer.clone())?;
        Ok(token)
    }

    pub fn next_token(&mut self) -> Result<Token, Error<Unlabelled>> {
        let (_, token) = _next_impl(&mut self.lexer)?;
        Ok(token)
    }

    pub fn peek_matches<T>(&mut self, expected: T) -> Result<T::Output, Error<T::Label>>
    where
        T: Pattern,
        Error<T::Label>: From<Error<Unlabelled>>,
    {
        let (span, token) = self.peek()?;
        let slice = &self.source()[self.span()];
        Ok(expected.match_pattern(span, token, slice)?.1)
    }

    pub fn expect_matches<T>(&mut self, expected: T) -> Result<T::Output, Error<T::Label>>
    where
        T: Pattern,
        Error<T::Label>: From<Error<Unlabelled>>,
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

pub trait Pattern {
    type Output;
    type Label;

    fn match_pattern(
        self,
        span: Span,
        token: Token,
        slice: &str,
    ) -> Result<(Span, Self::Output), Error<Self::Label>>;
}

pub struct Cursor(usize);
