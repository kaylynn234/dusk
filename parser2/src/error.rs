use lexer::Token;
use std::{
    error,
    fmt::{Debug, Display},
};

// None of this is pretty. It needs to change.

fn token_or_eof(found: &Option<Token>) -> String {
    match found {
        Some(token) => format!("`{}`", token),
        None => String::from("EOF"),
    }
}

#[derive(Debug)]
pub enum Error {
    Unexpected(Option<Token>),
    Expected {
        expected: Option<Token>,
        found: Option<Token>,
    },
}

impl Display for Error {
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

impl error::Error for Error {}

impl Error {
    pub fn eof() -> Self {
        Error::Unexpected(None)
    }

    pub fn unexpected(token: Token) -> Self {
        Error::Unexpected(Some(token))
    }

    pub fn mismatch(expected: Token, found: Option<Token>) -> Self {
        Error::Expected {
            expected: Some(expected),
            found,
        }
    }
}
