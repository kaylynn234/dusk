use crate::{
    errors::ParseResult,
    fragments::{Associativity, InfixParser, Precedence},
    Parser, SpanToken,
};

use ast::{AssignmentType, AstNode};
use lexer::Token;
use std::convert::TryInto;

#[derive(Debug)]
pub struct BinaryParser {
    pub associativity: Associativity,
    pub precedence: usize,
}

impl BinaryParser {
    pub const fn new_left(precedence: usize) -> Self {
        BinaryParser {
            associativity: Associativity::Left,
            precedence,
        }
    }

    pub const fn new_right(precedence: usize) -> Self {
        BinaryParser {
            associativity: Associativity::Right,
            precedence,
        }
    }
}

impl<'i> InfixParser<'i> for BinaryParser {
    fn parse(
        &self,
        parser: &mut Parser<'i>,
        left: AstNode<'i>,
        token: SpanToken<'i>,
    ) -> ParseResult<'i> {
        // `self.associativity as usize` will be 0 if we're left-associative, or 1 otherwise.
        let right = parser.parse_expression_with(self.precedence - self.associativity as usize)?;

        // TODO: Proper error here.
        let result = AstNode::BinaryExpression {
            left: Box::new(left),
            operator: token.kind.try_into().unwrap(),
            right: Box::new(right),
        };

        Ok(result)
    }

    fn get_precedence(&self) -> usize {
        self.precedence
    }
}

#[derive(Debug)]
pub struct PathParser;

impl<'i> InfixParser<'i> for PathParser {
    fn parse(
        &self,
        parser: &mut Parser<'i>,
        left: AstNode<'i>,
        token: SpanToken<'i>,
    ) -> ParseResult<'i> {
        assert_eq!(token.kind, Token::SymbolDot);

        let right = parser
            .parse_expression_with(Precedence::Postfix as usize - Associativity::Right as usize)?;
        let result = AstNode::Path {
            left: Box::new(left),
            right: Box::new(right),
        };

        Ok(result)
    }

    fn get_precedence(&self) -> usize {
        Precedence::Postfix as usize
    }
}

#[derive(Debug)]
pub struct ValueAssignmentParser;

// Value assignment is assignment of the form `foo.bar = ...`
impl<'i> InfixParser<'i> for ValueAssignmentParser {
    fn parse(
        &self,
        parser: &mut Parser<'i>,
        left: AstNode<'i>,
        token: SpanToken<'i>,
    ) -> ParseResult<'i> {
        assert_eq!(token.kind, Token::SymbolEquals);

        let right = parser.parse_expression_with(Precedence::Statement as usize)?;
        let result = AstNode::Assignment {
            scope: AssignmentType::Value,
            subject: Box::new(left),
            value: Box::new(right),
        };

        Ok(result)
    }

    fn get_precedence(&self) -> usize {
        Precedence::Statement as usize
    }
}

#[derive(Debug)]
pub struct PairParser;

impl<'i> InfixParser<'i> for PairParser {
    fn parse(
        &self,
        parser: &mut Parser<'i>,
        left: AstNode<'i>,
        token: SpanToken<'i>,
    ) -> ParseResult<'i, AstNode<'i>> {
        assert_eq!(token.kind, Token::SymbolColon);

        let right = parser.parse_expression_with(Precedence::Pair as usize)?;
        let result = AstNode::Pair {
            left: Box::new(left),
            right: Box::new(right),
        };

        Ok(result)
    }

    fn get_precedence(&self) -> usize {
        Precedence::Pair as usize
    }
}

#[derive(Debug)]
pub struct SequenceParser;

impl<'i> InfixParser<'i> for SequenceParser {
    fn parse(
        &self,
        parser: &mut Parser<'i>,
        left: AstNode<'i>,
        token: SpanToken<'i>,
    ) -> ParseResult<'i, AstNode<'i>> {
        assert_eq!(token.kind, Token::SymbolComma);

        // If parsing another expression works, this acts like an infix parser. Otherwise it backtracks and acts like
        // a suffix parser to allow for a trailing comma.
        let right = parser
            .try_parse(|parser| {
                parser.parse_expression_with(
                    Precedence::Sequence as usize - Associativity::Right as usize,
                )
            })
            .ok()
            .map(Box::new);

        let result = AstNode::Sequence {
            left: Box::new(left),
            right,
        };

        Ok(result)
    }

    fn get_precedence(&self) -> usize {
        Precedence::Sequence as usize
    }
}

#[derive(Debug)]
pub struct CallParser;

impl<'i> InfixParser<'i> for CallParser {
    fn parse(
        &self,
        parser: &mut Parser<'i>,
        left: AstNode<'i>,
        token: SpanToken<'i>,
    ) -> ParseResult<'i, AstNode<'i>> {
        assert_eq!(token.kind, Token::SymbolLeftParen);

        let next_token = parser.peek()?.kind;
        let parameters = match next_token {
            Token::SymbolRightParen => Vec::new(),
            _ => vec![parser.parse_expression_with(Precedence::Statement as usize)?],
        };

        parser.expect(Token::SymbolRightParen)?;

        let result = AstNode::Call {
            subject: Box::new(left),
            parameters,
        };

        Ok(result)
    }

    fn get_precedence(&self) -> usize {
        Precedence::Call as usize
    }
}
