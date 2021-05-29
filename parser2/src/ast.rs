use super::Span;
use std::{
    error::Error,
    fmt::{Display, Write},
};

#[derive(Debug)]
pub struct LiteralBool(pub bool);
#[derive(Debug)]
pub struct LiteralString(pub Span);
#[derive(Debug)]
pub struct LiteralInteger(pub Span);
#[derive(Debug)]
pub struct LiteralFloat(pub Span);
#[derive(Debug)]
pub struct Identifier(pub Span);

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
pub struct LiteralTuple(pub Vec<AstNode>);

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
    Assignment(Assignment)
}

pub struct Pretty<'i> {
    input: &'i str,
    indent: usize,
    buffer: String,
}

impl<'i> Pretty<'i> {
    pub fn new(input: &'i str) -> Self {
        Pretty {
            input,
            indent: 0,
            buffer: String::new(),
        }
    }

    fn with_indent<F>(&mut self, f: F) -> Result<(), Box<dyn Error>>
    where
        F: FnOnce(&mut Self) -> Result<(), Box<dyn Error>>,
    {
        self.indent += 1;
        f(self)?;
        self.indent -= 1;
        Ok(())
    }

    fn write<T: Display>(&mut self, item: T) -> Result<(), Box<dyn Error>> {
        Ok(write!(self.buffer, "{}", item)?)
    }

    fn write_line<T: Display>(&mut self, item: T) -> Result<(), Box<dyn Error>> {
        Ok(write!(
            self.buffer,
            "\n{}{}",
            "  ".repeat(self.indent),
            item
        )?)
    }

    fn resolve_span(&self, span: &Span) -> String {
        (&self.input[span.clone()]).to_string()
    }

    fn visit(&mut self, node: &AstNode) -> Result<(), Box<dyn Error>> {
        match node {
            AstNode::Bool(bool) => self.write(bool.0),
            AstNode::String(literal) => self.write(self.resolve_span(&literal.0)),
            AstNode::Integer(literal) => self.write(self.resolve_span(&literal.0)),
            AstNode::Float(literal) => self.write(self.resolve_span(&literal.0)),
            AstNode::Identifier(identifier) => self.write(self.resolve_span(&identifier.0)),
            AstNode::Path(path) => {
                self.write_line("Path")?;
                self.with_indent(|visitor| {
                    visitor.visit(&path.first)?;
                    for (access, identifier) in &path.rest {
                        visitor.write_line(format!("{:?}", access))?;
                        visitor.write_line(visitor.resolve_span(&identifier.0))?;
                    }

                    Ok(())
                })
            }
            AstNode::Unary(unary) => {
                self.write_line("Unary")?;
                self.with_indent(|visitor| {
                    visitor.write_line(format!("Operator: {:?}", unary.operator))?;
                    visitor.visit(&unary.operand)
                })
            }
            AstNode::Binary(binary) => {
                self.write_line("BinaryExpression")?;
                self.with_indent(|visitor| {
                    visitor.write_line("Left: ")?;
                    visitor.with_indent(|visitor| visitor.visit(&binary.left))?;
                    visitor.write_line(format!("Operator: {:?}", binary.operator))?;
                    visitor.write_line("Right: ")?;
                    visitor.with_indent(|visitor| visitor.visit(&binary.right))
                })
            }
            AstNode::Tuple(tuple) => {
                self.write_line("(")?;
                self.with_indent(|visitor| {
                    for (index, item) in tuple.0.iter().enumerate() {
                        visitor.write_line(format!("Item #{}: ", index + 1))?;
                        visitor.with_indent(|visitor| visitor.visit(item))?;
                    }

                    Ok(())
                })?;

                self.write_line(")")
            }
            AstNode::Pair(pair) => {
                self.write_line("Pair")?;
                self.with_indent(|visitor| {
                    visitor.write_line("Left: ")?;
                    visitor.with_indent(|visitor| visitor.visit(&pair.left))?;
                    visitor.write_line("Right: ")?;
                    visitor.with_indent(|visitor| visitor.visit(&pair.right))
                })
            }
            AstNode::Call(call) => {
                self.write_line("Call")?;
                self.with_indent(|visitor| {
                    visitor.write_line("Subject: ")?;
                    visitor.with_indent(|visitor| visitor.visit(&call.subject))?;
                    for (index, item) in call.arguments.iter().enumerate() {
                        visitor.write_line(format!("Argument #{}: ", index + 1))?;
                        visitor.with_indent(|visitor| visitor.visit(item))?;
                    }

                    Ok(())
                })
            },
            AstNode::Assignment(assignment) => {
                self.write_line("Assignment")?;
                self.with_indent(|visitor| {
                    visitor.write_line("Pattern: ")?;
                    visitor.with_indent(|visitor| visitor.visit(&assignment.pattern))?;
                    visitor.write_line("Value: ")?;
                    visitor.with_indent(|visitor| visitor.visit(&assignment.value))
                })
            }
            
        }
    }

    pub fn pretty(mut self, node: &AstNode) -> Result<String, Box<dyn Error>> {
        self.visit(node)?;
        Ok(self.buffer)
    }
}
