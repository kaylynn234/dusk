use crate::{ast::*, Error, ParseResult, Parser, SpanToken};
use lexer::{token_category, Token};

macro_rules! bail_unexpected {
    ($error:expr) => {{
        return Err(Error::Unexpected(Some($error)));
    }};
}

#[repr(u8)]
enum Precedence {
    Start,
    Statement,
    Sequence,
    Pair,
    Conditional,
    Or,
    And,
    Comparison,
    Sum,
    Product,
    Prefix,
    Postfix,
    Call,
}

#[repr(u8)]
pub enum Associativity {
    Left,
    Right,
}

#[inline]
fn infix_precedence(token: &Token) -> u8 {
    println!("Getting precedence of {}", token);
    match token {
        Token::Equals => Precedence::Statement as u8,
        Token::Colon => Precedence::Pair as u8,
        Token::Or => Precedence::Or as u8,
        Token::And => Precedence::And as u8,
        token_category![ValueComparison] => Precedence::Comparison as u8,
        token_category![Sum] => Precedence::Sum as u8,
        token_category![Product] => Precedence::Product as u8,
        _ => 0,
    }
}

#[inline]
fn infix_associativity(token: &Token) -> Associativity {
    match token {
        _ => Associativity::Left,
    }
}

impl<'i> Parser<'i> {
    pub fn parse_prefix(&mut self, token: &SpanToken) -> ParseResult {
        match token.kind {
            token_category![UnaryOperator] => self.parse_unary(token),
            token_category![Atom] => self.parse_atom(token),
            Token::OpeningParen => self.parse_parenthesized_expression(),
            _ => bail_unexpected!(token.kind),
        }
    }

    pub fn parse_infix(&mut self, left: AstNode, token: &SpanToken) -> ParseResult {
        match token.kind {
            Token::Equals => self.parse_assignment(left),
            token_category![BinaryOperator] => self.parse_binary(left, token),
            Token::Colon => self.parse_pair(left),
            Token::OpeningParen => self.parse_call(left),
            _ => bail_unexpected!(token.kind),
        }
    }

    fn can_continue(&self, precedence: u8) -> bool {
        self.current
            .as_ref()
            .map_or(false, |token| precedence < infix_precedence(&token.kind))
    }

    pub fn parse_expression_with(&mut self, precedence: u8) -> ParseResult {
        let token = self.current()?.clone();
        self.advance();
        let mut left = self.parse_prefix(&token)?;

        while self.can_continue(precedence) {
            let token = self.current()?.clone();
            self.advance();
            left = self.parse_infix(left, &token)?;
        }

        Ok(left)
    }

    pub fn parse_expression(&mut self) -> ParseResult {
        let result = self.parse_expression_with(Precedence::Start as u8)?;
        self.expect(Token::Semicolon)?;

        Ok(result)
    }

    pub fn parse_block(&mut self) -> ParseResult<Vec<AstNode>> {
        self.expect(Token::OpeningBrace)?;
        let mut results = Vec::new();

        loop {
            if let Token::ClosingBrace = self.current()?.kind {
                self.advance();
                break;
            }

            results.push(self.parse_item()?);
        }

        Ok(results)
    }

    pub fn parse_item(&mut self) -> ParseResult {
        todo!()
    }

    pub fn parse_sequence_with(&mut self, buffer: &mut Vec<AstNode>) -> ParseResult<()> {
        // TODO: Clean this up if possible.
        loop {
            self.expect(Token::Comma)?;
            if let token_category![ClosingBracket] = self.current()?.kind {
                break;
            }

            let precedence = Precedence::Sequence as u8 - Associativity::Right as u8;
            buffer.push(self.parse_expression_with(precedence)?);

            if let token_category![ClosingBracket] = self.current()?.kind {
                break;
            }
        }

        Ok(())
    }

    pub fn parse_sequence(&mut self) -> ParseResult<Vec<AstNode>> {
        let precedence = Precedence::Sequence as u8 - Associativity::Right as u8;
        let mut items = vec![self.parse_expression_with(precedence)?];
        if let Token::Comma = self.current()?.kind {
            self.parse_sequence_with(&mut items)?;
        }

        Ok(items)
    }

    pub fn parse(&mut self) -> ParseResult<Vec<AstNode>> {
        let mut results = Vec::new();
        while self.current.is_some() {
            results.push(self.parse_item()?);
        }

        Ok(results)
    }

    pub fn parse_atom(&mut self, token: &SpanToken) -> ParseResult {
        let result = match token.kind {
            Token::True => AstNode::Bool(LiteralBool(true)),
            Token::False => AstNode::Bool(LiteralBool(false)),
            Token::Integer => AstNode::Integer(LiteralInteger(token.span.clone())),
            Token::Float => AstNode::Float(LiteralFloat(token.span.clone())),
            Token::String => AstNode::String(LiteralString(token.span.clone())),
            Token::Identifier => AstNode::Identifier(Identifier(token.span.clone())),
            _ => bail_unexpected!(token.kind),
        };

        Ok(result)
    }

    pub fn parse_unary(&mut self, token: &SpanToken) -> ParseResult {
        // TODO: Refactor to another method. Maybe `TryInto`?
        let operator = match token.kind {
            Token::Not => UnaryOperator::Not,
            Token::Plus => UnaryOperator::Positive,
            Token::Minus => UnaryOperator::Negative,
            _ => bail_unexpected!(token.kind),
        };

        let result = UnaryExpression {
            operator,
            operand: Box::new(self.parse_expression_with(Precedence::Prefix as u8)?),
        };

        Ok(AstNode::Unary(result))
    }

    pub fn parse_binary(&mut self, left: AstNode, token: &SpanToken) -> ParseResult {
        // TODO: Refactor to another method. Maybe `TryInto`?
        let operator = match token.kind {
            Token::Plus => BinaryOperator::Add,
            Token::Minus => BinaryOperator::Subtract,
            Token::Asterisk => BinaryOperator::Multiply,
            Token::Slash => BinaryOperator::Divide,
            Token::Lesser => BinaryOperator::LessThan,
            Token::LesserEqual => BinaryOperator::LessThanOrEqual,
            Token::Greater => BinaryOperator::GreaterThan,
            Token::GreaterEqual => BinaryOperator::GreaterThanOrEqual,
            Token::EqualsEquals => BinaryOperator::Equal,
            Token::NotEqual => BinaryOperator::NotEqual,
            Token::And => BinaryOperator::And,
            Token::Or => BinaryOperator::Or,
            _ => bail_unexpected!(token.kind),
        };

        let precedence = infix_precedence(&token.kind);
        let associativity = infix_associativity(&token.kind);
        println!("Found token {:?} with precedence {}", token, precedence);
        let result = BinaryExpression {
            operator,
            left: Box::new(left),
            right: Box::new(self.parse_expression_with(precedence - associativity as u8)?),
        };

        Ok(AstNode::Binary(result))
    }

    pub fn parse_parenthesized_expression(&mut self) -> ParseResult {
        let first = self.parse_expression_with(Precedence::Statement as u8)?;
        let result = match self.current()?.kind {
            Token::Comma => {
                let mut items = vec![first];
                self.parse_sequence_with(&mut items)?;
                AstNode::Tuple(LiteralTuple(items))
            }
            _ => first,
        };

        self.expect(Token::ClosingParen)?;

        Ok(result)
    }

    pub fn parse_pair(&mut self, left: AstNode) -> ParseResult {
        let right = self.parse_expression_with(Precedence::Pair as u8)?;
        let result = Pair {
            left: Box::new(left),
            right: Box::new(right),
        };

        Ok(AstNode::Pair(result))
    }

    pub fn parse_call(&mut self, subject: AstNode) -> ParseResult {
        let arguments = match self.current()?.kind {
            Token::ClosingParen => Vec::new(),
            _ => self.parse_sequence()?,
        };

        self.expect(Token::ClosingParen)?;

        let result = Call {
            subject: Box::new(subject),
            arguments,
        };

        Ok(AstNode::Call(result))
    }

    pub fn parse_assignment(&mut self, left: AstNode) -> ParseResult {
        let right = self.parse_expression_with(Precedence::Statement as u8)?;
        let result = Assignment {
            ty: AssignmentType::Value,
            pattern: Box::new(left),
            value: Box::new(right),
        };

        Ok(AstNode::Assignment(result))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_matches {
        ($expression:expr, $($pattern:tt)+) => {
            match $expression {
                $($pattern)+ => (),
                ref e => panic!("assertion failed: `{:?}` does not match `{}`", e, stringify!($($pattern)+)),
            }
        }
    }

    #[test]
    fn test_invalid_tuple() {
        assert_matches!(
            Parser::new("(,);").parse_expression(),
            Err(Error::Unexpected(Some(Token::Comma)))
        );
    }

    #[test]
    fn test_invalid_trailing_comma() {
        assert_matches!(
            Parser::new("(1,,);").parse_expression(),
            Err(Error::Unexpected(Some(Token::Comma)))
        );
    }

    #[test]
    fn test_no_closing_paren() {
        assert_matches!(
            Parser::new("(1, 1; 1").parse_expression(),
            Err(Error::Expected {
                expected: Some(Token::Comma),
                found: Some(Token::Semicolon)
            })
        );
    }
}
