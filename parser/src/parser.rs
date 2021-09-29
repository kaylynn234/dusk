use super::prelude::*;
use lexer::Token;

impl Morph for Token {
    type Output = Token;
    type Label = Diagnostic;

    fn morph(
        self,
        span: crate::span::Span,
        token: Token,
        slice: &str,
    ) -> Result<(Span, Self::Output), Error<Self::Label>> {
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

impl Morph for &str {
    type Output = Token;
    type Label = Diagnostic;

    fn morph(
        self,
        span: crate::span::Span,
        token: Token,
        slice: &str,
    ) -> Result<(Span, Self::Output), Error<Self::Label>> {
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

// With that machinery out of the way we can get on to the actual parser implementation. Here be dragons.
impl Parser<'_> {
    
}