use std::hint::unreachable_unchecked;

use crate::span::Span;
use lexer::Token;

pub struct Error<L> {
    location: Span,
    kind: ErrorKind<L>,
}

impl<L> Error<L> {
    pub fn new(location: Span, kind: ErrorKind<L>) -> Self {
        Self { location, kind }
    }

    pub fn location(&self) -> Span {
        self.location
    }

    pub fn kind(&self) -> &ErrorKind<L> {
        &self.kind
    }

    pub fn into_inner(self) -> (Span, ErrorKind<L>) {
        (self.location, self.kind)
    }
}

pub enum ErrorKind<L> {
    Unexpected(Option<Token>),
    Labelled(L),
}

// This is essentially just an error-reporting type that's like `Token` but can provide additional information for use
// in diagnostics.
pub enum DiagnosticToken {
    Word(String),
    Token(Token),
}

pub trait IntoDiagnosticExt {
    fn into_diagnostic(self, slice: &str) -> DiagnosticToken;
}

impl IntoDiagnosticExt for Token {
    fn into_diagnostic(self, slice: &str) -> DiagnosticToken {
        match self {
            Token::Identifier => DiagnosticToken::Word(slice.to_owned()),
            token => DiagnosticToken::Token(token),
        }
    }
}

pub enum Unlabelled {}

pub trait LabelExt {
    type Ok;

    fn label<L>(self, label: L) -> Result<Self::Ok, Error<L>>;
}

impl<T, L> LabelExt for Result<T, Error<L>> {
    type Ok = T;

    fn label<NewL>(self, label: NewL) -> Result<<Self as LabelExt>::Ok, Error<NewL>> {
        // We can't use struct update syntax here as the types differ. Even though the fields that are actually updated
        // have compatible types, rustc doesn't care.
        self.map_err(|error| Error::new(error.location(), ErrorKind::Labelled(label)))
    }
}

// It's not possible to use a blanket implementation of this without specialization.
impl From<ErrorKind<Unlabelled>> for ErrorKind<Diagnostic> {
    fn from(val: ErrorKind<Unlabelled>) -> Self {
        match val {
            ErrorKind::Unexpected(inner) => ErrorKind::Unexpected(inner),
            // SAFETY: It's not possible to construct an uninhabited type.
            ErrorKind::Labelled(_) => unsafe { unreachable_unchecked() },
        }
    }
}

// See above comment.
impl From<Error<Unlabelled>> for Error<Diagnostic> {
    fn from(val: Error<Unlabelled>) -> Self {
        let (location, kind) = val.into_inner();
        Error::new(location, kind.into())
    }
}

pub enum Diagnostic {
    Mismatch {
        expected: DiagnosticToken,
        found: DiagnosticToken,
    },
}
