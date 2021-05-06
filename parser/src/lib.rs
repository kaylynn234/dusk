use self::{
    errors::{ParseError, ParseResult},
    fragments::{get_infix_parser, get_item_parser, get_prefix_parser, Precedence},
};

use ast::AstNode;
use lexer::Token;
use logos::Logos;
use std::{fmt::Display, ops::Range};

pub mod errors;
pub mod fragments;

#[derive(Debug, Clone)]
pub struct SpanToken<'i> {
    kind: Token,
    span: Range<usize>,
    slice: &'i str,
}

impl<'i> Display for SpanToken<'i> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Token::Error = self.kind {
            f.write_str(self.slice)
        } else {
            f.write_str(&self.kind.to_string())
        }
    }
}

pub struct Parser<'i> {
    tokens: Vec<SpanToken<'i>>,
    cursor: usize,
}

impl<'i> Parser<'i> {
    pub fn new(input: &'i str) -> Self {
        let lexer = Token::lexer(input).spanned().map(|argument| {
            let (kind, span) = argument;
            let slice = &input[span.clone()];
            SpanToken { kind, span, slice }
        });

        Parser {
            tokens: lexer.collect(),
            cursor: 0,
        }
    }

    fn get_or_error(&self, index: usize) -> ParseResult<'i, SpanToken<'i>> {
        self.tokens
            .get(index)
            .cloned()
            .ok_or_else(ParseError::end_of_input)
    }

    /// Returns `true` if the parser is at EOF.
    pub fn at_eof(&self) -> bool {
        self.cursor >= self.tokens.len()
    }

    /// Peeks at the current token and returns it. This does not progress the parser.
    pub fn peek(&self) -> ParseResult<'i, SpanToken<'i>> {
        self.get_or_error(self.cursor)
    }

    /// Peeks at the token `n` positions ahead and returns it. This does not progress the parser.
    pub fn lookahead(&self, n: usize) -> ParseResult<'i, SpanToken<'i>> {
        self.get_or_error(self.cursor + n)
    }

    /// Consumes the current token and returns it. This progresses the parser.
    pub fn consume(&mut self) -> ParseResult<'i, SpanToken<'i>> {
        let result = self.get_or_error(self.cursor);
        self.cursor += 1;
        result
    }

    /// Consumes the current token and returns it. Returns an error if the token is not of kind `expected`.
    /// This only progresses the parser if the token is of the expected kind.
    pub fn expect(&mut self, expected: Token) -> ParseResult<'i, SpanToken<'i>> {
        // TODO: Proper error.
        let token = self
            .peek()
            .map_err(|_| ParseError::with(format!("Expected {}, but reached EOF", expected)))?;

        if token.kind == expected {
            self.cursor += 1;
            Ok(token)
        } else {
            Err(ParseError::with(format!(
                "Expected {}, but found {} instead",
                expected, token.kind
            ))) // TODO: Proper error
        }
    }

    /// Calls `f(self)` to parse an expression. If parsing fails, the parser does not progress.
    /// This can be used to selectively allow backtracking.
    pub fn try_parse<T, F>(&mut self, f: F) -> ParseResult<'i, T>
    where
        F: FnOnce(&mut Self) -> ParseResult<'i, T>,
    {
        let old_cursor = self.cursor;
        let result = f(self);
        if result.is_err() {
            self.cursor = old_cursor;
        }

        result
    }

    /// Returns the next token wrapped in `Some` if the parser is allowed to progress with the current precedence.
    /// I.e, if the current precedence is >= the precedence of the current token, or the parser is at EOF, this returns
    /// `None`. This is the main conditional used in a Pratt parsing implementation.
    pub fn next_token_with(&mut self, current_precedence: usize) -> Option<SpanToken<'i>> {
        let token = self.peek().ok()?;
        let precedence = get_infix_parser(&token).map_or(0, |parser| parser.get_precedence());

        (current_precedence < precedence).then(|| {
            self.cursor += 1;
            token
        })
    }

    // This is a fairly standard TDOP / Pratt implementation.
    pub fn parse_expression_with(&mut self, current_precedence: usize) -> ParseResult<'i> {
        // TODO: proper errors. this consume call is probably fine though
        let token = self.consume()?;
        let prefix_parser =
            get_prefix_parser(&token).ok_or_else(|| ParseError::unexpected_token(&token))?;
        let mut left = prefix_parser.parse(self, token)?;

        while let Some(token) = self.next_token_with(current_precedence) {
            // We won't have made it to the body of this loop if there's not an infix parser for this token, so this
            // unwrap is safe. In the future we might want suffix parsers so this behaviour may have to change then.
            // But hey. It's fine for now.
            left = get_infix_parser(&token).unwrap().parse(self, left, token)?;
        }

        Ok(left)
    }

    pub fn parse_expression(&mut self) -> ParseResult<'i> {
        let result = self.parse_expression_with(Precedence::Start as usize)?;
        self.expect(Token::SymbolSemicolon)?;

        Ok(result)
    }

    pub fn parse_block(&mut self) -> ParseResult<'i, Vec<AstNode<'i>>> {
        self.expect(Token::SymbolLeftBrace)?;
        let mut results = Vec::new();

        loop {
            let token = self.peek()?;
            if let Token::SymbolRightBrace = token.kind {
                // We wouldn't be here if we were at EOF, so the unwrap is safe.
                self.cursor += 1;
                break;
            }

            results.push(self.parse_item()?)
        }

        Ok(results)
    }

    pub fn parse_item(&mut self) -> ParseResult<'i> {
        let token = self.peek()?;

        // if need be, an item parser will consume `;`. We don't have to do that ourselves here.
        match get_item_parser(&token) {
            Some(item_parser) => item_parser.parse(self),
            None => self.parse_expression(),
        }
    }

    pub fn parse(&mut self) -> ParseResult<'i, Vec<AstNode<'i>>> {
        let mut results = Vec::new();
        while !self.at_eof() {
            results.push(self.parse_item()?);
        }

        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn expect_ast(input: &'static str, expected: AstNode<'static>) -> Result<(), ParseError> {
        let mut parser = Parser::new(input);
        assert_eq!(parser.parse_item()?, expected);
        Ok(())
    }

    #[test]
    fn test_sequence() -> Result<(), ParseError> {
        let expected = AstNode::Sequence {
            left: Box::new(AstNode::Integer("1")),
            right: Some(Box::new(AstNode::Sequence {
                left: Box::new(AstNode::Integer("2")),
                right: Some(Box::new(AstNode::Integer("3"))),
            })),
        };

        expect_ast("1, 2, 3;", expected)
    }
}
