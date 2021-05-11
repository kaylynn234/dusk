use crate::{
    errors::{ParseError, ParseResult},
    fragments::{ItemParser, Precedence},
    Parser,
};

use ast::AstNode;
use lexer::Token;

#[derive(Debug)]
pub struct FunctionParser;

impl<'i> ItemParser<'i> for FunctionParser {
    fn parse(&self, parser: &mut Parser<'i>) -> ParseResult<'i> {
        parser.expect(Token::KeywordFunction)?;
        let name = parser.expect(Token::Identifier)?.slice;
        parser.expect(Token::SymbolLeftParen)?;

        let parameters = match parser.peek()?.kind {
            Token::SymbolRightParen => None,
            _ => Some(Box::new(
                parser.parse_expression_with(Precedence::Statement as usize)?,
            )),
        };

        parser.expect(Token::SymbolRightParen)?;
        let return_type = parser
            .expect(Token::SymbolArrow)
            .and_then(|_| parser.parse_expression_with(Precedence::Sequence as usize))
            .ok()
            .map(Box::new);

        let next_token = parser.peek()?;
        let body = match next_token.kind {
            Token::SymbolLeftBrace => Some(parser.parse_block()?),
            Token::SymbolSemicolon => {
                parser.consume()?;
                None
            }
            _ => return Err(ParseError::unexpected_token(next_token.kind)),
        };

        let result = AstNode::Function {
            name,
            parameters,
            return_type,
            body,
        };

        Ok(result)
    }
}

#[derive(Debug)]
pub struct StructParser;

impl<'i> ItemParser<'i> for StructParser {
    fn parse(&self, parser: &mut Parser<'i>) -> ParseResult<'i> {
        parser.expect(Token::KeywordStruct)?;
        let name = parser.expect(Token::Identifier)?.slice;
        parser.expect(Token::SymbolLeftBrace)?;
        let fields = parser.parse_expression_with(Precedence::Statement as usize)?;
        parser.expect(Token::SymbolRightBrace)?;

        let result = AstNode::Struct {
            name,
            fields: Box::new(fields),
        };

        Ok(result)
    }
}

#[derive(Debug)]
pub struct MetadataParser;

impl<'i> ItemParser<'i> for MetadataParser {
    fn parse(&self, parser: &mut Parser<'i>) -> ParseResult<'i, AstNode<'i>> {
        parser.expect(Token::SymbolHash)?;

        let is_module_metadata = parser.expect(Token::SymbolExclamation).is_ok();

        parser.expect(Token::SymbolLeftBracket)?;
        let inner = parser.parse_expression_with(Precedence::Start as usize)?;
        parser.expect(Token::SymbolRightBracket)?;

        let result = {
            if is_module_metadata {
                AstNode::ModuleMetadata(Box::new(inner))
            } else {
                AstNode::ItemMetadata {
                    inner: Box::new(inner),
                    subject: Box::new(parser.parse_item()?),
                }
            }
        };

        Ok(result)
    }
}

#[derive(Debug)]
pub struct ModuleParser;

impl<'i> ItemParser<'i> for ModuleParser {
    fn parse(&self, parser: &mut Parser<'i>) -> ParseResult<'i, AstNode<'i>> {
        parser.expect(Token::KeywordModule)?;
        let module_name = parser.expect(Token::Identifier)?;
        parser.expect(Token::SymbolSemicolon)?;

        Ok(AstNode::Module(module_name.slice))
    }
}
