use crate::span::Span;
use lexer::Token;

pub struct Error {
    location: Span,
    kind: ErrorKind,
}

pub enum ErrorKind {
    Unexpected(Option<Token>),
    Labelled(Diagnostic),
}

impl Error {
    pub fn new(location: Span, kind: ErrorKind) -> Self {
        Self { location, kind }
    }

    pub fn location(&self) -> Span {
        self.location
    }

    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }

    pub fn into_inner(self) -> (Span, ErrorKind) {
        (self.location, self.kind)
    }
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

pub trait LabelExt {
    type Ok;

    fn label(self, label: Diagnostic) -> Result<Self::Ok, Error>;
}

impl<T> LabelExt for Result<T, Error> {
    type Ok = T;

    fn label(self, label: Diagnostic) -> Result<<Self as LabelExt>::Ok, Error> {
        // We can't use struct update syntax here as the types differ. Even though the fields that are actually updated
        // have compatible types, rustc doesn't care.
        self.map_err(|error| Error::new(error.location(), ErrorKind::Labelled(label)))
    }
}

pub enum Diagnostic {
    Mismatch {
        expected: DiagnosticToken,
        found: DiagnosticToken,
    },
}
