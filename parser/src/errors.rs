use crate::SpanToken;
use ast::AstNode;
use std::fmt::Debug;

#[derive(Debug)]
pub enum ParseErrorKind {
    NoMatch,
    MissingTag,
    AllParsersFailed,
    NotEnoughSucceeded,
}

#[derive(Debug)]
pub struct ParseError {
    pub context: String,
}

impl ParseError {
    pub fn new() -> Self {
        ParseError {
            context: String::from("..."),
        }
    }

    pub fn with(context: String) -> Self {
        ParseError { context }
    }

    pub fn end_of_input() -> Self {
        ParseError {
            context: String::from("Unexpected end of input"),
        }
    }

    pub fn unexpected_token<'i>(token: &SpanToken<'i>) -> Self {
        ParseError {
            context: format!("Unexpected {}", token),
        }
    }
}

pub type ParseResult<'i, T = AstNode<'i>> = Result<T, ParseError>;
