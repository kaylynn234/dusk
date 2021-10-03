use super::*;
use lexer::Token;

pub trait Pattern {
    type Output;

    fn match_pattern(
        self,
        span: Span,
        token: Token,
        slice: &str,
    ) -> Result<(Span, Self::Output), Error>;
}

impl Pattern for Token {
    type Output = Token;

    fn match_pattern(
        self,
        span: Span,
        token: Token,
        slice: &str,
    ) -> Result<(Span, Self::Output), Error> {
        if self == token {
            return Ok((span, token));
        }

        let diagnostic = Diagnostic::Mismatch {
            expected: self.into_diagnostic(slice),
            found: token.into_diagnostic(slice),
        };

        Err(Error::new(span, ErrorKind::Labelled(diagnostic)))
    }
}

impl Pattern for &str {
    type Output = Token;

    fn match_pattern(
        self,
        span: Span,
        token: Token,
        slice: &str,
    ) -> Result<(Span, Self::Output), Error> {
        if token == Token::Identifier && self == slice {
            return Ok((span, token));
        }

        let diagnostic = Diagnostic::Mismatch {
            expected: DiagnosticToken::Word(self.to_owned()),
            found: token.into_diagnostic(slice),
        };

        Err(Error::new(span, ErrorKind::Labelled(diagnostic)))
    }
}
