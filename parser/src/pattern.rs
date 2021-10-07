use crate::span::{Spanned, SpannedToken};

use super::*;
use lexer::Token;

pub trait Pattern {
    fn match_pattern(self, parser: &Parser, token: SpannedToken) -> Result<SpannedToken, Error>;
}

impl Pattern for Token {
    fn match_pattern(self, parser: &Parser, token: SpannedToken) -> Result<SpannedToken, Error> {
        if self == token.kind() {
            return Ok(token);
        }

        let diagnostic = Diagnostic::Mismatch {
            expected: self.into_diagnostic(parser),
            found: token.into_diagnostic(parser),
        };

        Err(Error::new(token.span(), ErrorKind::Diagnostic(diagnostic)))
    }
}

impl Pattern for &str {
    fn match_pattern(self, parser: &Parser, token: SpannedToken) -> Result<SpannedToken, Error> {
        if token.kind() == Token::Identifier && self == &parser.source()[token.span()] {
            return Ok(token);
        }

        let diagnostic = Diagnostic::Mismatch {
            expected: DiagnosticTerm::Word(self.to_owned()),
            found: token.into_diagnostic(parser),
        };

        Err(Error::new(token.span(), ErrorKind::Diagnostic(diagnostic)))
    }
}
