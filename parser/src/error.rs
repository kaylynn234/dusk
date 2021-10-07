use std::borrow::Cow;

use crate::{
    span::{Span, Spanned, SpannedToken},
    Parser,
};
use derive_more::{Display, From};
use lexer::Token;

pub struct Error {
    location: Span,
    kind: ErrorKind,
}

impl Error {
    pub fn new(location: Span, kind: ErrorKind) -> Self {
        Self { location, kind }
    }

    pub fn details(&self) -> &ErrorKind {
        &self.kind
    }

    pub fn into_inner(self) -> (Span, ErrorKind) {
        (self.location, self.kind)
    }
}

impl Spanned for Error {
    fn span(&self) -> Span {
        self.location
    }
}

pub struct ErrorBuilder<'a, 'b> {
    parser: &'a mut Parser<'b>,
    location: Option<Span>,
    kind: Option<ErrorKind>,
}

impl<'a, 'b> ErrorBuilder<'a, 'b> {
    pub fn new(parser: &'a mut Parser<'b>) -> Self {
        Self {
            parser,
            location: None,
            kind: None,
        }
    }

    pub fn location(&mut self, span: impl Spanned) -> &mut Self {
        self.location = Some(span.span());
        self
    }

    pub fn message(&mut self, message: impl Into<Cow<'static, str>>) -> &mut Self {
        self.kind = Some(ErrorKind::Diagnostic(Diagnostic::Message {
            message: message.into(),
        }));
        self
    }

    pub fn unexpected(&mut self, unexpected: impl Into<Unexpected>) -> &mut Self {
        self.kind = Some(ErrorKind::Simple(unexpected.into()));
        self
    }

    pub fn mismatch(
        &mut self,
        expected: impl IntoDiagnostic,
        found: impl IntoDiagnostic,
    ) -> &mut Self {
        self.kind = Some(ErrorKind::Diagnostic(Diagnostic::Mismatch {
            expected: expected.into_diagnostic(self.parser),
            found: found.into_diagnostic(self.parser),
        }));
        self
    }

    pub fn build(&mut self) -> Option<Error> {
        Some(Error::new(self.location.take()?, self.kind.take()?))
    }

    pub fn finish<T: ErrorVariant>(&mut self) -> Option<T> {
        let span = self.location?;
        let finished = self.build()?;
        self.parser.errors.push(finished);

        Some(ErrorVariant::error(span))
    }
}

#[derive(Display, From)]
pub enum ErrorKind {
    Simple(Unexpected),
    Diagnostic(Diagnostic),
}

#[derive(Display)]
pub enum Unexpected {
    #[display(fmt = "unexpected {}", "_0")]
    Token(Token),
    #[display(fmt = "unexpected EOF")]
    Eof,
}

impl From<Option<Token>> for Unexpected {
    fn from(value: Option<Token>) -> Self {
        match value {
            Some(token) => Unexpected::Token(token),
            None => Unexpected::Eof,
        }
    }
}

// This is essentially just an error-reporting type that's like `Token` but can provide additional information for use
// in diagnostics.
#[derive(Debug, Display)]
pub enum DiagnosticTerm {
    Word(String),
    Token(Token),
}

#[derive(Debug, Display)]
pub enum Diagnostic {
    #[display(fmt = "expected {} but found {}", expected, found)]
    Mismatch {
        expected: DiagnosticTerm,
        found: DiagnosticTerm,
    },
    Message {
        message: Cow<'static, str>,
    },
}

/// A type that can represent an error itself, without using [Result]
pub trait ErrorVariant {
    fn error(span: Span) -> Self;
    fn is_error(&self) -> bool;
}

pub trait IntoDiagnostic {
    fn into_diagnostic(self, parser: &Parser) -> DiagnosticTerm;
}

impl IntoDiagnostic for SpannedToken {
    fn into_diagnostic(self, parser: &Parser) -> DiagnosticTerm {
        match self.kind() {
            Token::Identifier => DiagnosticTerm::Word(parser.source()[self.span()].to_owned()),
            token => DiagnosticTerm::Token(token),
        }
    }
}

impl IntoDiagnostic for Token {
    fn into_diagnostic(self, _: &Parser) -> DiagnosticTerm {
        DiagnosticTerm::Token(self)
    }
}

impl IntoDiagnostic for &str {
    fn into_diagnostic(self, _: &Parser) -> DiagnosticTerm {
        DiagnosticTerm::Word(self.to_owned())
    }
}

impl IntoDiagnostic for String {
    fn into_diagnostic(self, _: &Parser) -> DiagnosticTerm {
        DiagnosticTerm::Word(self)
    }
}

pub trait LabelExt {
    type Ok;

    fn label(self, label: impl Into<Diagnostic>) -> Result<Self::Ok, Error>;
}

impl<T> LabelExt for Result<T, Error> {
    type Ok = T;

    fn label(self, label: impl Into<Diagnostic>) -> Result<<Self as LabelExt>::Ok, Error> {
        self.map_err(|error| Error {
            kind: ErrorKind::Diagnostic(label.into()),
            ..error
        })
    }
}

pub trait SpannedTokenExt {
    fn token(self) -> Result<Token, Error>;
    fn span(self) -> Result<Span, Error>;
}

impl SpannedTokenExt for Result<SpannedToken, Error> {
    fn token(self) -> Result<Token, Error> {
        self.map(|token| token.kind())
    }

    fn span(self) -> Result<Span, Error> {
        self.map(|token| token.span())
    }
}

#[macro_export]
macro_rules! bail {
    ($expr:expr) => {{
        let expr = $expr;
        use $crate::span::Spanned;

        match expr {
            ::std::result::Result::Ok(ok) => ok,
            ::std::result::Result::Err(error) => {
                return $crate::error::ErrorVariant::error(error.span())
            }
        }
    }};
}
