use crate::{
    bail,
    error::ErrorVariant,
    span::{Spanned, SpannedToken},
    token_info::Precedence,
    BinaryExpression, Expression, Parser, SpannedTokenExt, TokenInfoExt, UnaryExpression,
};
use lexer::{token_category, Token};
use std::convert::TryInto;

// Something really bugs me about the actual parser implementation being in the same file as its definition, and I don't
// know why. I know it's a bit of a strange choice, but that's why this is a different module.

impl Parser<'_> {
    pub fn parse(&mut self) -> Expression {
        let result = self.parse_expression();
        self.add_delimiter_errors();
        result
    }

    pub fn parse_expression(&mut self) -> Expression {
        self.parse_expression_with(Precedence::START)
    }

    fn parse_expression_with(&mut self, precedence: Precedence) -> Expression {
        let token = bail!(self.next());
        let mut expr = self.parse_prefix_expression(token);

        while self.can_continue(precedence) {
            let token = bail!(self.next());
            expr = self.parse_infix_expression(expr, token);
        }

        expr
    }

    fn can_continue(&self, precedence: Precedence) -> bool {
        self.peek()
            .token()
            .map_or(false, |token| precedence < token.precedence())
    }

    #[inline]
    fn parse_prefix_expression(&mut self, token: SpannedToken) -> Expression {
        match token.kind() {
            Token::OpeningParen => self.parse_parenthesized_expression(token),
            token_category![UnaryOperator] => self.parse_unary_expression(token),
            // SAFETY: All tokens that match these pattern can be converted into a literal/identifier,
            // so the unwrap will not fail.
            token_category![Literal] => Expression::Literal(token.try_into().unwrap()),
            Token::Identifier => Expression::Identifier(token.try_into().unwrap()),
            _ => self
                .error()
                .location(token.span())
                .mismatch("expression", token)
                .finish()
                .unwrap(),
        }
    }

    #[inline]
    fn parse_infix_expression(&mut self, expr: Expression, token: SpannedToken) -> Expression {
        match token.kind() {
            token_category![BinaryOperator] => self.parse_binary_expression(expr, token),
            _ => Expression::error(token.span()),
        }
    }

    fn parse_unary_expression(&mut self, token: SpannedToken) -> Expression {
        let expression = self.parse_expression_with(Precedence::PREFIX);

        // SAFETY: Unwrapping below is safe, as this method will only be called if we have a token that we know can be
        // converted into a binary operator.
        UnaryExpression {
            span: token.span().union(expression.span()),
            operator: token.try_into().unwrap(),
            operand: Box::new(expression),
        }
        .into()
    }

    fn parse_binary_expression(&mut self, left: Expression, token: SpannedToken) -> Expression {
        let right = self.parse_expression_with(token.kind().precedence());

        // SAFETY: Unwrapping below is safe, as this method will only be called if we have a token that we know can be
        // converted into a binary operator.
        BinaryExpression {
            span: left.span().union(right.span()),
            operator: token.try_into().unwrap(),
            left: Box::new(left),
            right: Box::new(right),
        }
        .into()
    }

    fn parse_parenthesized_expression(&mut self, token: SpannedToken) -> Expression {
        self.unclosed_delimiters.push(token);
        let expression = self.parse_expression();
        match self.expect_matches(Token::ClosingParen) {
            Ok(_) => {
                self.unclosed_delimiters.pop();
                expression
            }
            Err(error) => ErrorVariant::error(error.span()),
        }
    }

    fn add_delimiter_errors(&mut self) {
        for unclosed in self.unclosed_delimiters.clone() {
            let error = self
                .error()
                .location(unclosed.span())
                .message("unclosed delimiter")
                .build()
                .unwrap();

            self.errors.push(error)
        }
    }
}
