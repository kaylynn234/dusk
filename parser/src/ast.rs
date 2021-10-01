use crate::{span::Span, visitor::Visitor};
use codegen::{Visitor, Spanned};

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

#[derive(Debug, Clone, Spanned)]
#[span(self.span)]
pub struct OperatorInfo<T> {
    operator: T,
    span: Span
}

#[derive(Debug, Clone)]
pub enum UnaryOperator {
    Not,
    Positive,
    Negative,
}

#[derive(Debug, Clone)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    Equal,
    NotEqual,
    And,
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

literal_impl! {
    pub IntegerLiteral,
    pub FloatLiteral,
    pub StringLiteral,
}

#[derive(Debug, Clone, Spanned)]
#[span(self.0)]
pub struct BoolLiteral(Span, bool);

impl BoolLiteral {
    pub fn span(&self) -> Span {
        self.0
    }

    pub fn value(&self) -> bool {
        self.1
    }
}

#[derive(Debug, Clone, Spanned)]
pub enum LiteralExpression {
    Integer(IntegerLiteral),
    Float(FloatLiteral),
    String(StringLiteral),
}

#[derive(Debug, Clone, Visitor, Spanned)]
#[visit(base)]
pub enum Expression {
    Unary(UnaryExpression),
    Binary(BinaryExpression),
    Is(IsExpression),
    Block(BlockExpression),
    Literal(LiteralExpression),
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
pub enum Pattern {}

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
