use crate::{ast::*, Parser, Result, SpanToken};
use lexer::{token_category, Token};

macro_rules! bail {
    ($error:expr) => {
        return Err($error)
    };
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
    match token {
        Token::Colon => Precedence::Pair as u8,
        Token::Or => Precedence::Or as u8,
        Token::And => Precedence::And as u8,
        token_category![ValueComparison] => Precedence::Comparison as u8,
        token_category![Sum] => Precedence::Sum as u8,
        token_category![Product] => Precedence::Product as u8,
        Token::OpeningParen => Precedence::Call as u8,
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
    pub fn parse_prefix(&mut self, token: &SpanToken) -> Result {
        match token.kind {
            token_category![UnaryOperator] => self.parse_unary(token),
            token_category![Atom] => self.parse_atom(token),
            Token::OpeningParen => self.parse_parenthesized_expression(),
            _ => bail!(self.expected("an expression", Some(token))),
        }
    }

    pub fn parse_infix(&mut self, left: AstNode, token: &SpanToken) -> Result {
        match token.kind {
            token_category![BinaryOperator] => self.parse_binary(left, token),
            // TODO: Make pair not an operator like this.
            Token::Colon => self.parse_pair(left),
            Token::OpeningParen => self.parse_call(left),
            _ => bail!(self.expected(
                "either a type annotation, a function/method call or an operator",
                Some(token)
            )),
        }
    }

    fn can_continue(&self, precedence: u8) -> bool {
        self.current
            .as_ref()
            .map_or(false, |token| precedence < infix_precedence(&token.kind))
    }

    pub fn parse_expression_with(&mut self, precedence: u8) -> Result {
        let token = self.current_or("either an operator or expression")?.clone();
        self.advance();
        let mut left = self.parse_prefix(&token)?;

        while self.can_continue(precedence) {
            let token = self.current_or("either an operator or expression")?.clone();
            self.advance();
            left = self.parse_infix(left, &token)?;
        }

        Ok(left)
    }

    pub fn parse_expression(&mut self) -> Result {
        let mut result = self.parse_expression_with(Precedence::Start as u8)?;
        if let Token::Equals = self.current_or(Token::Semicolon)?.kind {
            self.advance();
            result = self.parse_assignment(result)?;
        }

        self.assert(Token::Semicolon)?;

        Ok(result)
    }

    pub fn parse_block(&mut self) -> Result<Vec<AstNode>> {
        self.assert(Token::OpeningBrace)?;
        let mut results = Vec::new();

        loop {
            if let Token::ClosingBrace = self.current_or(Token::ClosingBrace)?.kind {
                self.advance();
                break;
            }

            results.push(self.parse_item()?);
        }

        Ok(results)
    }

    pub fn parse_item(&mut self) -> Result {
        match self.current_or("an item")?.kind {
            Token::Function => self.parse_functiom(),
            Token::Struct => self.parse_struct(),
            _ => self.parse_expression(),
        }
    }

    // TODO: Make this more composable. Maybe a macro?
    pub fn parse_sequence_with(&mut self, buffer: &mut Vec<AstNode>, stop_at: Token) -> Result<()> {
        // TODO: Clean this up if possible.
        // TODO: "Expected a _"
        loop {
            self.assert(Token::Comma)?;
            if self.current_or(stop_at)?.kind == stop_at {
                break;
            }

            let precedence = Precedence::Sequence as u8 - Associativity::Right as u8;
            buffer.push(self.parse_expression_with(precedence)?);

            if self.current_or(stop_at)?.kind == stop_at {
                break;
            }
        }

        Ok(())
    }

    pub fn parse_sequence(&mut self, stop_at: Token) -> Result<Vec<AstNode>> {
        let precedence = Precedence::Sequence as u8 - Associativity::Right as u8;
        let mut items = vec![self.parse_expression_with(precedence)?];
        if let Token::Comma = self.current_or(format!("comma or {}", stop_at))?.kind {
            self.parse_sequence_with(&mut items, stop_at)?;
        }

        Ok(items)
    }

    pub fn parse(&mut self) -> Result<Vec<AstNode>> {
        let mut results = Vec::new();
        while self.current.is_some() {
            results.push(self.parse_item()?);
        }

        Ok(results)
    }

    pub fn parse_atom(&mut self, token: &SpanToken) -> Result {
        let span = token.span.clone();
        let result = match token.kind {
            Token::True => AstNode::Bool(LiteralBool { value: true }),
            Token::False => AstNode::Bool(LiteralBool { value: false }),
            Token::Integer => AstNode::Integer(LiteralInteger { span }),
            Token::Float => AstNode::Float(LiteralFloat { span }),
            Token::String => AstNode::String(LiteralString { span }),
            Token::Identifier => AstNode::Identifier(Identifier { span }),
            token => panic!("token {} should not be reachable here", token),
        };

        Ok(result)
    }

    pub fn parse_unary(&mut self, token: &SpanToken) -> Result {
        let operator = match token.kind {
            Token::Not => UnaryOperator::Not,
            Token::Plus => UnaryOperator::Positive,
            Token::Minus => UnaryOperator::Negative,
            token => panic!("token {} should not be reachable here", token),
        };

        let result = UnaryExpression {
            operator,
            operand: Box::new(self.parse_expression_with(Precedence::Prefix as u8)?),
        };

        Ok(AstNode::Unary(result))
    }

    pub fn parse_binary(&mut self, left: AstNode, token: &SpanToken) -> Result {
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
            token => panic!("token {} should not be reachable here", token),
        };

        let precedence = infix_precedence(&token.kind);
        let associativity = infix_associativity(&token.kind);
        let result = BinaryExpression {
            operator,
            left: Box::new(left),
            right: Box::new(self.parse_expression_with(precedence - associativity as u8)?),
        };

        Ok(AstNode::Binary(result))
    }

    pub fn parse_parenthesized_expression(&mut self) -> Result {
        let first = self.parse_expression_with(Precedence::Statement as u8)?;
        let result = match self.current_or(Token::ClosingParen)?.kind {
            Token::Comma => {
                let mut elements = vec![first];
                self.parse_sequence_with(&mut elements, Token::ClosingParen)?;
                AstNode::Tuple(LiteralTuple { elements })
            }
            _ => first,
        };

        self.assert(Token::ClosingParen)?;

        Ok(result)
    }

    pub fn parse_pair(&mut self, left: AstNode) -> Result {
        let right = self.parse_expression_with(Precedence::Pair as u8)?;
        let result = Pair {
            left: Box::new(left),
            right: Box::new(right),
        };

        Ok(AstNode::Pair(result))
    }

    // TODO: `blah()` fails to parse
    pub fn parse_call(&mut self, subject: AstNode) -> Result {
        let arguments = match self.current_or(Token::ClosingParen)?.kind {
            Token::ClosingParen => Vec::new(),
            _ => self.parse_sequence(Token::ClosingParen)?
        };

        self.assert(Token::ClosingParen)?;

        let result = Call {
            subject: Box::new(subject),
            arguments,
        };

        Ok(AstNode::Call(result))
    }

    pub fn parse_assignment(&mut self, left: AstNode) -> Result {
        let right = self.parse_expression_with(Precedence::Statement as u8)?;
        let result = Assignment {
            ty: AssignmentType::Value,
            pattern: Box::new(left),
            value: Box::new(right),
        };

        Ok(AstNode::Assignment(result))
    }

    pub fn parse_functiom(&mut self) -> Result {
        self.assert(Token::Function)?;
        let name = Identifier {
            span: self.assert_cloned(Token::Identifier)?.span,
        };

        self.assert(Token::OpeningParen)?;
        let parameters = match self.current_or(Token::ClosingParen)?.kind {
            Token::ClosingParen => Vec::new(),
            _ => self.parse_sequence(Token::ClosingParen)?,
        };

        self.assert(Token::ClosingParen)?;
        let return_type = match self.current_or("a return type or block")?.kind {
            Token::Arrow => {
                self.advance();
                let result = Box::new(self.parse_expression_with(Precedence::Statement as u8)?);
                Some(result)
            }
            _ => None,
        };

        let current = self.current_or("block")?;
        let body = match current.kind {
            Token::Semicolon => {
                self.advance();
                None
            }
            Token::OpeningBrace => Some(self.parse_block()?),
            _ => bail!(self.expected(format!("{} or a block", Token::Semicolon), Some(current))),
        };

        let result = Function {
            name,
            parameters,
            return_type,
            body,
        };

        Ok(AstNode::Function(result))
    }

    pub fn parse_struct(&mut self) -> Result {
        self.assert(Token::Struct)?;
        let name = Identifier {
            span: self.assert_cloned(Token::Identifier)?.span,
        };

        self.assert(Token::OpeningParen)?;
        let fields = match self.current_or(Token::ClosingParen)?.kind {
            Token::ClosingParen => Vec::new(),
            _ => self.parse_sequence(Token::ClosingParen)?,
        };

        self.assert(Token::ClosingParen)?;
        let result = Struct { name, fields };
        Ok(AstNode::Struct(result))
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
}
