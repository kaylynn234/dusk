use crate::{span::Span, visitor::Visitor};
use visitor_derive::Visitor;

#[derive(Debug, Clone, Visitor)]
#[visit(node = Expression)]
pub struct UnaryExpression {
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

#[derive(Debug, Clone, Visitor)]
#[visit(node = Expression)]
pub struct BinaryExpression {
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

#[derive(Debug, Clone, Visitor)]
#[visit(node = Expression)]
pub struct IsExpression {
    #[visit]
    pub expression: Box<Expression>,
    pub pattern: Pattern,
}

#[derive(Debug, Clone, Visitor)]
#[visit(node = Expression)]
pub struct BlockExpression {
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

#[derive(Debug, Clone)]
pub struct BoolLiteral(Span, bool);

impl BoolLiteral {
    pub fn span(&self) -> Span {
        self.0
    }

    pub fn value(&self) -> bool {
        self.1
    }
}

#[derive(Debug, Clone)]
pub enum LiteralExpression {
    Integer(IntegerLiteral),
    Float(FloatLiteral),
    String(StringLiteral),
}

#[derive(Debug, Clone, Visitor)]
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
