use crate::fragments::infix::CallParser;

pub use self::{
    infix::{BinaryParser, PairParser, PathParser, SequenceParser, ValueAssignmentParser},
    item::{FunctionParser, MetadataParser, ModuleParser, StructParser},
    prefix::{AtomParser, ParenthesizedExpressionParser, UnaryParser},
};

use super::{errors::ParseResult, Parser, SpanToken};
use ast::AstNode;
use lexer::{token_category, Token};
use std::fmt::Debug;

pub mod infix;
pub mod item;
pub mod prefix;

pub enum Precedence {
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

#[derive(Clone, Copy, Debug)]
pub enum Associativity {
    Left,
    Right,
}

static BOOLEAN_OR_PARSER: BinaryParser = BinaryParser::new_left(Precedence::Or as usize);
static BOOLEAN_AND_PARSER: BinaryParser = BinaryParser::new_left(Precedence::And as usize);
static COMPARISON_PARSER: BinaryParser = BinaryParser::new_left(Precedence::Comparison as usize);
static SUM_PARSER: BinaryParser = BinaryParser::new_left(Precedence::Sum as usize);
static PRODUCT_PARSER: BinaryParser = BinaryParser::new_left(Precedence::Product as usize);

pub trait PrefixParser<'i>: Debug {
    fn parse(&self, parser: &mut Parser<'i>, token: SpanToken<'i>) -> ParseResult<'i, AstNode<'i>>;
}

pub trait InfixParser<'i>: Debug {
    fn parse(
        &self,
        parser: &mut Parser<'i>,
        left: AstNode<'i>,
        token: SpanToken<'i>,
    ) -> ParseResult<'i, AstNode<'i>>;

    fn get_precedence(&self) -> usize;
}

pub trait ItemParser<'i>: Debug {
    fn parse(&self, parser: &mut Parser<'i>) -> ParseResult<'i, AstNode<'i>>;
}

pub fn get_prefix_parser<'i>(token: &SpanToken<'i>) -> Option<&'i dyn PrefixParser<'i>> {
    match token.kind {
        Token::SymbolLeftParen => Some(&ParenthesizedExpressionParser),
        token_category![UnaryOperator] => Some(&UnaryParser),
        token_category![Atom] => Some(&AtomParser),
        _ => None,
    }
}

pub fn get_infix_parser<'i>(token: &SpanToken<'i>) -> Option<&'i dyn InfixParser<'i>> {
    match token.kind {
        Token::SymbolLeftParen => Some(&CallParser),
        Token::SymbolEquals => Some(&ValueAssignmentParser),
        Token::SymbolComma => Some(&SequenceParser),
        Token::SymbolColon => Some(&PairParser),
        Token::SymbolDot => Some(&PathParser),
        Token::KeywordOr => Some(&BOOLEAN_OR_PARSER),
        Token::KeywordAnd => Some(&BOOLEAN_AND_PARSER),
        token_category![ValueComparison] => Some(&COMPARISON_PARSER),
        token_category![Sum] => Some(&SUM_PARSER),
        token_category![Product] => Some(&PRODUCT_PARSER),
        _ => None,
    }
}

pub fn get_item_parser<'i>(token: &SpanToken<'i>) -> Option<&'i dyn ItemParser<'i>> {
    match token.kind {
        Token::SymbolHash => Some(&MetadataParser),
        Token::KeywordModule => Some(&ModuleParser),
        Token::KeywordStruct => Some(&StructParser),
        Token::KeywordFunction => Some(&FunctionParser),
        _ => None,
    }
}
