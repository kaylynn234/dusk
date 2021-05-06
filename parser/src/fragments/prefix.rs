use crate::{
    errors::ParseResult,
    fragments::{Precedence, PrefixParser},
    Parser, SpanToken,
};

use ast::AstNode;
use lexer::Token;
use std::convert::TryInto;

#[derive(Debug)]
pub struct AtomParser;

impl<'i> PrefixParser<'i> for AtomParser {
    fn parse(&self, _parser: &mut Parser<'i>, token: SpanToken<'i>) -> ParseResult<'i> {
        let result = match token.kind {
            Token::KeywordTrue => AstNode::Bool(true),
            Token::KeywordFalse => AstNode::Bool(false),
            Token::LiteralInteger => AstNode::Integer(token.slice),
            Token::LiteralFloat => AstNode::Float(token.slice),
            Token::Identifier => AstNode::Identifier(token.slice),
            Token::LiteralString => AstNode::String(token.slice),
            _ => unreachable!(),
        };

        Ok(result)
    }
}

#[derive(Debug)]
pub struct ParenthesizedExpressionParser;

impl<'i> PrefixParser<'i> for ParenthesizedExpressionParser {
    fn parse(&self, parser: &mut Parser<'i>, token: SpanToken<'i>) -> ParseResult<'i> {
        assert_eq!(token.kind, Token::SymbolLeftParen);
        let result = parser.parse_expression_with(Precedence::Statement as usize)?;
        parser.expect(Token::SymbolRightParen)?;

        Ok(result)
    }
}

#[derive(Debug)]
pub struct UnaryParser;

impl<'i> PrefixParser<'i> for UnaryParser {
    fn parse(&self, parser: &mut Parser<'i>, token: SpanToken<'i>) -> ParseResult<'i> {
        // TODO: Proper error here.
        let result = AstNode::UnaryExpression {
            operator: token.kind.try_into().unwrap(),
            subject: Box::new(parser.parse_expression_with(Precedence::Prefix as usize)?),
        };

        Ok(result)
    }
}
