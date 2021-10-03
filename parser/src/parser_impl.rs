use crate::{Expression, Parser};
use lexer::Token;

// Something really bugs me about the actual parser implementation being in the same file as its definition, and I don't
// know why. I know it's a bit of a strange choice, but that's why this is a different module.

fn infix_precedence(token: Token) -> u32 {
    match token {
        _ => 0,
    }
}

impl Parser<'_> {
    pub fn parse_expression(&mut self) -> Expression {
        todo!()
    }
}
