use super::{Span, SpanToken};

#[derive(Debug)]
pub struct LiteralBool {
    pub value: bool,
}
#[derive(Debug)]
pub struct LiteralString {
    pub span: Span,
}
#[derive(Debug)]
pub struct LiteralInteger {
    pub span: Span,
}
#[derive(Debug)]
pub struct LiteralFloat {
    pub span: Span,
}
#[derive(Debug)]
pub struct Identifier {
    pub span: Span,
}

#[derive(Debug)]
pub enum PathAccess {
    // `::` access
    Scope,
    // `.` access
    Member,
}

#[derive(Debug)]
pub struct Path {
    pub first: Box<AstNode>,
    pub rest: Vec<(PathAccess, Identifier)>,
}

#[derive(Debug)]
pub enum UnaryOperator {
    Not,
    Positive,
    Negative,
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct UnaryExpression {
    pub operator: UnaryOperator,
    pub operand: Box<AstNode>,
}

#[derive(Debug)]
pub struct BinaryExpression {
    pub operator: BinaryOperator,
    pub left: Box<AstNode>,
    pub right: Box<AstNode>,
}

#[derive(Debug)]
pub struct LiteralTuple {
    pub elements: Vec<AstNode>
}

#[derive(Debug)]
pub struct Pair {
    pub left: Box<AstNode>,
    pub right: Box<AstNode>,
}

#[derive(Debug)]
pub struct Call {
    pub subject: Box<AstNode>,
    pub arguments: Vec<AstNode>,
}

#[derive(Debug)]
pub enum AssignmentType {
    Value,
    Scope,
}

#[derive(Debug)]
pub struct Assignment {
    pub ty: AssignmentType,
    pub pattern: Box<AstNode>,
    pub value: Box<AstNode>,
}

#[derive(Debug)]
pub struct Function {
    pub name: Identifier,
    pub parameters: Vec<AstNode>,
    pub return_type: Option<Box<AstNode>>,
    pub body: Option<Vec<AstNode>>,
}

#[derive(Debug)]
pub struct Struct {
    pub name: Identifier,
    pub fields: Vec<AstNode>,
}

#[derive(Debug)]
pub struct Module {
    pub name: Identifier,
}

#[derive(Debug)]
pub enum AstNode {
    // Literals & atoms
    Bool(LiteralBool),
    String(LiteralString),
    Integer(LiteralInteger),
    Float(LiteralFloat),
    Identifier(Identifier),
    Path(Path),
    // Simple expressions
    Unary(UnaryExpression),
    Binary(BinaryExpression),
    // Collection literals
    Tuple(LiteralTuple),
    // ...
    Pair(Pair),
    // Postfix expressions - slicing, indexing and calls.
    Call(Call),
    // ....
    // Statements
    Assignment(Assignment),
    // Items
    // ....
    Function(Function),
    Struct(Struct),
    Module(Module),
}

