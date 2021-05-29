use lexer::Token;
use std::{convert::TryFrom, fmt::Display};

#[derive(Debug, PartialEq, Eq)]
pub enum UnaryOperator {
    Not,
    Positive,
    Negative,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BinaryOperator {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    Equal,
    NotEqual,
    And,
    Or,
}

#[derive(Debug, PartialEq, Eq)]
pub enum AssignmentType {
    // `global x = 2;`
    Global,
    // `let x = 2;`
    Scope,
    // `x = 2;`
    Value,
}

#[derive(Debug, PartialEq, Eq)]
pub enum PathType {
    // `.` access
    Member,
    // `::` access
    Scope,
}

#[derive(Debug, PartialEq, Eq)]
pub enum AstNode<'i> {
    // Literals come first
    Bool(bool),
    String(&'i str),
    Float(&'i str),
    Integer(&'i str),
    Identifier(&'i str),
    Path {
        left: Box<AstNode<'i>>,
        access: PathType,
        right: Box<AstNode<'i>>,
    },
    // Then simple expressions
    UnaryExpression {
        operator: UnaryOperator,
        subject: Box<AstNode<'i>>,
    },
    BinaryExpression {
        left: Box<AstNode<'i>>,
        operator: BinaryOperator,
        right: Box<AstNode<'i>>,
    },
    // Then slightly more complex expressions
    Call {
        subject: Box<AstNode<'i>>,
        parameters: Vec<AstNode<'i>>,
    },
    // Then pairs and sequences
    Pair {
        left: Box<AstNode<'i>>,
        right: Box<AstNode<'i>>,
    },
    Sequence {
        left: Box<AstNode<'i>>,
        right: Option<Box<AstNode<'i>>>,
    },
    // Followed by statement-esque things
    Assignment {
        scope: AssignmentType,
        subject: Box<AstNode<'i>>,
        value: Box<AstNode<'i>>,
    },
    // Then item definitions
    Module(&'i str),
    Struct {
        name: &'i str,
        fields: Box<AstNode<'i>>,
    },
    Function {
        name: &'i str,
        parameters: Option<Box<AstNode<'i>>>,
        return_type: Option<Box<AstNode<'i>>>,
        body: Option<Vec<AstNode<'i>>>,
    },
    // Some meta-y stuff
    ItemMetadata {
        inner: Box<AstNode<'i>>,
        subject: Box<AstNode<'i>>,
    },
    ModuleMetadata(Box<AstNode<'i>>),
}

impl<'i> Display for AstNode<'i> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let node_name = match self {
            AstNode::Bool(_) => "boolean",
            AstNode::String(_) => "string literal",
            AstNode::Float(_) => "float literal",
            AstNode::Integer(_) => "integer literal",
            AstNode::Identifier(_) => "identifier",
            AstNode::Path { .. } => "path",
            AstNode::UnaryExpression { .. } => "expression",
            AstNode::BinaryExpression { .. } => "expression",
            AstNode::Call { .. } => "function call",
            AstNode::Pair { .. } => "pair",
            AstNode::Sequence { .. } => "sequence",
            AstNode::Module(_) => "module declaration",
            AstNode::Struct { .. } => "struct definition",
            AstNode::Function { .. } => "function definition",
            AstNode::Assignment { .. } => "assignment",
            AstNode::ItemMetadata { .. } => "item metadata",
            AstNode::ModuleMetadata(_) => "module metadata",
        };

        f.write_str(node_name)
    }
}

impl<'i> AstNode<'i> {
    pub fn flatten_sequence(self, buffer: &mut Vec<AstNode<'i>>) {
        if let AstNode::Sequence { left, right } = self {
            left.flatten_sequence(buffer);
            if let Some(right) = right {
                right.flatten_sequence(buffer)
            }
        } else {
            buffer.push(self)
        }
    }
}
