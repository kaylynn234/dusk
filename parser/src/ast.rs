use crate::{
    error::ErrorVariant,
    span::{Span, Spanned, SpannedToken},
    visitor::Visitor,
};
use codegen::{Spanned, Visitor};
use derive_more::{Display, From};
use lexer::Token;
use std::convert::{TryFrom, TryInto};

#[derive(Debug, Clone, Spanned)]
#[span(self.span)]
pub struct OperatorInfo<T> {
    operator: T,
    span: Span,
}

#[derive(Debug, Display, Clone)]
pub enum UnaryOperator {
    #[display(fmt = "not")]
    Not,
    #[display(fmt = "+")]
    Positive,
    #[display(fmt = "-")]
    Negative,
}

#[derive(Debug, Display, Clone)]
pub enum BinaryOperator {
    #[display(fmt = "+")]
    Add,
    #[display(fmt = "-")]
    Subtract,
    #[display(fmt = "*")]
    Multiply,
    #[display(fmt = "/")]
    Divide,
    #[display(fmt = "<")]
    LessThan,
    #[display(fmt = "<=")]
    LessThanOrEqual,
    #[display(fmt = ">")]
    GreaterThan,
    #[display(fmt = ">=")]
    GreaterThanOrEqual,
    #[display(fmt = "==")]
    Equal,
    #[display(fmt = "!=")]
    NotEqual,
    #[display(fmt = "and")]
    And,
    #[display(fmt = "or")]
    Or,
}

#[derive(Debug, Clone, Visitor, Spanned)]
#[visit(node = Expression)]
#[span(self.span)]
pub struct UnaryExpression {
    pub span: Span,
    pub operator: OperatorInfo<UnaryOperator>,
    #[visit]
    pub operand: Box<Expression>,
}

#[derive(Debug, Clone, Visitor, Spanned)]
#[visit(node = Expression)]
#[span(self.span)]
pub struct BinaryExpression {
    pub span: Span,
    pub operator: OperatorInfo<BinaryOperator>,
    #[visit]
    pub left: Box<Expression>,
    #[visit]
    pub right: Box<Expression>,
}

#[derive(Debug, Clone, Visitor, Spanned)]
#[visit(node = Expression)]
#[span(self.span)]
pub struct IsExpression {
    pub span: Span,
    #[visit]
    pub expression: Box<Expression>,
    pub pattern: Pattern,
}

#[derive(Debug, Clone, Visitor, Spanned)]
#[visit(node = Expression)]
#[span(self.span)]
pub struct BlockExpression {
    pub span: Span,
    #[visit]
    pub statements: Vec<Expression>,
    #[visit]
    pub tail: Option<Box<Expression>>,
}

macro_rules! literal_impl {
    ($($vis:vis $name:ident,)+) => { literal_impl! { $($vis $name),* } };
    ($($vis:vis $name:ident),*) => {
        $(
            #[derive(Debug, Clone)]
            $vis struct $name(Span);

            impl $crate::span::Spanned for $name {
                fn span(&self) -> $crate::span::Span {
                    self.0
                }
            }
        )*
    };
}

literal_impl! {
    pub IntegerLiteral,
    pub FloatLiteral,
    pub StringLiteral,
    pub Identifer,
}

#[derive(Debug, Clone, From, Spanned)]
pub enum LiteralExpression {
    Integer(IntegerLiteral),
    Float(FloatLiteral),
    String(StringLiteral),
}

#[derive(Debug, Clone, Visitor, Spanned)]
#[visit(node = Expression)]
#[span(self.span)]
pub struct Call {
    span: Span,
    #[visit]
    operand: Box<Expression>,
    // TODO: visit for enums
    arguments: Vec<Argument>,
}

#[derive(Debug, Clone, Visitor, Spanned)]
#[visit(node = Expression)]
#[span(self.span)]
pub struct NamedArgument {
    span: Span,
    name: Identifer,
    #[visit]
    expression: Box<Expression>,
}

#[derive(Debug, Clone, Spanned)]
pub enum Argument {
    Named(NamedArgument),
    Positional(Expression)
}

#[derive(Debug, Clone, From, Visitor, Spanned)]
#[visit(base)]
pub enum Expression {
    Unary(UnaryExpression),
    Binary(BinaryExpression),
    Is(IsExpression),
    Block(BlockExpression),
    Literal(LiteralExpression),
    Identifier(Identifer),
    Call(Call),
    Error(Span),
}

// Here we unfortunately run into a small spot where we're context-sensitive. I will not solve this with a lexer hack
// since I am not evil. Instead, this will be caught in a later compilation stage, where we start to understand
// semantics of a source file a bit more.
//
// I'll take a step back though. What does a WordPattern even represent? Well, consider something like this:
// ```
// if value is None {
//     /* ... */
// }
// ```
//
// Assuming we have the constructor `Maybe::None` in scope, should this bind to the name `None`, or check if `value` is
// the `Maybe::None` variant of `Maybe`? And therein lies the rub - we can't decide that yet. So we'll simply spit out
// what we know for the moment - the name it binds to - and get another compiler stage to solve things for us.
#[derive(Debug, Clone, Spanned)]
#[span(self.span)]
pub struct WordPattern {
    pub span: Span,
}

#[derive(Debug, Clone, Visitor)]
#[visit(base)]
pub enum Pattern {
    Word(WordPattern),
    Error(Span),
}

#[derive(Debug, Clone, Visitor)]
#[visit(base)]
pub enum AstNode {}

#[derive(Debug, Clone)]
pub enum PathAccess {
    // `::` access
    Scope,
    // `.` access
    Member,
}

impl ErrorVariant for Expression {
    fn error(span: Span) -> Self {
        Expression::Error(span)
    }

    fn is_error(&self) -> bool {
        matches!(self, Expression::Error(_))
    }
}

#[derive(Debug)]
pub struct TryFromTokenError {
    token: Token,
    type_name: &'static str,
}

impl TryFromTokenError {
    pub fn token(&self) -> Token {
        self.token
    }

    pub fn type_name(&self) -> &'static str {
        self.type_name
    }
}

impl TryFrom<Token> for UnaryOperator {
    type Error = TryFromTokenError;

    fn try_from(value: Token) -> Result<Self, Self::Error> {
        match value {
            Token::Not => Ok(UnaryOperator::Not),
            Token::Plus => Ok(UnaryOperator::Positive),
            Token::Minus => Ok(UnaryOperator::Negative),
            token => Err(TryFromTokenError {
                token,
                type_name: "UnaryOperator",
            }),
        }
    }
}

impl TryFrom<Token> for BinaryOperator {
    type Error = TryFromTokenError;

    fn try_from(value: Token) -> Result<Self, Self::Error> {
        match value {
            Token::Plus => Ok(BinaryOperator::Add),
            Token::Minus => Ok(BinaryOperator::Subtract),
            Token::Asterisk => Ok(BinaryOperator::Multiply),
            Token::Slash => Ok(BinaryOperator::Divide),
            Token::Lesser => Ok(BinaryOperator::LessThan),
            Token::LesserEqual => Ok(BinaryOperator::LessThanOrEqual),
            Token::Greater => Ok(BinaryOperator::GreaterThan),
            Token::GreaterEqual => Ok(BinaryOperator::GreaterThanOrEqual),
            Token::EqualsEquals => Ok(BinaryOperator::Equal),
            Token::NotEqual => Ok(BinaryOperator::NotEqual),
            Token::And => Ok(BinaryOperator::And),
            Token::Or => Ok(BinaryOperator::Or),
            token => Err(TryFromTokenError {
                token,
                type_name: "BinaryOperator",
            }),
        }
    }
}

impl<T> TryFrom<SpannedToken> for OperatorInfo<T>
where
    T: TryFrom<Token, Error = TryFromTokenError>,
{
    type Error = TryFromTokenError;

    fn try_from(value: SpannedToken) -> Result<Self, Self::Error> {
        Ok(OperatorInfo {
            operator: value.kind().try_into()?,
            span: value.span(),
        })
    }
}

impl TryFrom<SpannedToken> for LiteralExpression {
    type Error = TryFromTokenError;

    fn try_from(value: SpannedToken) -> Result<Self, Self::Error> {
        match value.kind() {
            Token::Integer => Ok(IntegerLiteral(value.span()).into()),
            Token::Float => Ok(FloatLiteral(value.span()).into()),
            Token::String => Ok(StringLiteral(value.span()).into()),
            token => Err(TryFromTokenError {
                token,
                type_name: "LiteralExpression",
            }),
        }
    }
}

impl TryFrom<SpannedToken> for Identifer {
    type Error = TryFromTokenError;

    fn try_from(value: SpannedToken) -> Result<Self, Self::Error> {
        match value.kind() {
            Token::Identifier => Ok(Identifer(value.span())),
            token => Err(TryFromTokenError {
                token,
                type_name: "Identifier",
            }),
        }
    }
}
