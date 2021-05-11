use ast::AstNode;
use lexer::Token;
use std::{
    error::Error,
    fmt::{Debug, Display},
};

fn token_or_eof(found: &Option<Token>) -> String {
    match found {
        Some(token) => token.to_string(),
        None => String::from("EOF"),
    }
}

#[derive(Debug)]
pub enum ParseError {
    Unexpected(Option<Token>),
    Expected {
        expected: Option<Token>,
        found: Option<Token>,
    },
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unexpected(item) => write!(f, "Unexpected {}", token_or_eof(item)),
            Self::Expected { expected, found } => {
                write!(
                    f,
                    "Expected {}, but found {}",
                    token_or_eof(expected),
                    token_or_eof(found)
                )
            }
        }
    }
}

impl Error for ParseError {}

impl ParseError {
    pub fn eof() -> Self {
        ParseError::Unexpected(None)
    }

    pub fn unexpected_token(token: Token) -> Self {
        ParseError::Unexpected(Some(token))
    }

    pub fn expected(expected: Token, found: Option<Token>) -> Self {
        ParseError::Expected {
            expected: Some(expected),
            found,
        }
    }
}

pub type ParseResult<'i, T = AstNode<'i>> = Result<T, ParseError>;
