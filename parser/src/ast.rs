use crate::{span::Span, visitor::Visitor};
use codegen::{Visitor, Spanned};

macro_rules! literal_impl {
    ($($vis:vis $name:ident,)+) => { literal_impl! { $($vis $name),* } };
    ($($vis:vis $name:ident),*) => {
        $(
            #[derive(Debug, Clone)]
            $vis struct $name(Span);

            impl $name {
                $vis fn span(&self) -> Span {
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
}

#[derive(Debug, Clone, Visitor, Spanned)]
#[visit(node = Expression)]
#[span(self.span)]
pub struct UnaryExpression {
    pub span: Span,
    pub operator: UnaryOperator,
    #[visit]
    pub operand: Box<Expression>,
}

#[derive(Debug, Clone)]
pub enum UnaryOperator {
    Not,
    Positive,
    Negative,
}

#[derive(Debug, Clone, Visitor, Spanned)]
#[visit(node = Expression)]
#[span(self.span)]
pub struct BinaryExpression {
    pub span: Span,
    pub operator: BinaryOperator,
    #[visit]
    pub left: Box<Expression>,
    #[visit]
    pub right: Box<Expression>,
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
