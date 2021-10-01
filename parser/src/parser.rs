use super::prelude::*;
use lexer::Token;

impl Pattern for Token {
    type Output = Token;
    type Label = Diagnostic;

    fn match_pattern(
        self,
        span: Span,
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

impl Pattern for &str {
    type Output = Token;
    type Label = Diagnostic;

    fn match_pattern(
        self,
        span: Span,
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

fn infix_precedence(token: Token) -> u32 {
    match token {
        _ => 0
    }
}

// With that machinery out of the way we can get on to the actual parser implementation. Here be dragons.
impl Parser<'_> {

}
