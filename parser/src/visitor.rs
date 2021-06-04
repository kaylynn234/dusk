use super::ast::*;

#[allow(unused_variables)]
pub trait Visitor {
    /// Called each time a deeper node in the tree is visited
    fn visit(&mut self, item: &AstNode) {
        self.visit_node(item)
    }

    fn visit_node(&mut self, item: &AstNode) {
        match item {
            AstNode::Bool(value) => self.visit_bool(value),
            AstNode::String(value) => self.visit_string(value),
            AstNode::Integer(value) => self.visit_integer(value),
            AstNode::Float(value) => self.visit_float(value),
            AstNode::Identifier(value) => self.visit_identifier(value),
            AstNode::Path(value) => self.visit_path(value),
            AstNode::Unary(value) => self.visit_unary(value),
            AstNode::Binary(value) => self.visit_binary(value),
            AstNode::Tuple(value) => self.visit_tuple(value),
            AstNode::Pair(value) => self.visit_pair(value),
            AstNode::Call(value) => self.visit_call(value),
            AstNode::Assignment(value) => self.visit_assignment(value),
            AstNode::Function(value) => self.visit_function(value),
            AstNode::Struct(value) => self.visit_struct(value),
            AstNode::Module(value) => self.visit_module(value),
        }
    }

    fn visit_bool(&mut self, item: &LiteralBool) {}
    fn visit_string(&mut self, item: &LiteralString) {}
    fn visit_integer(&mut self, item: &LiteralInteger) {}
    fn visit_float(&mut self, item: &LiteralFloat) {}
    fn visit_identifier(&mut self, item: &Identifier) {}

    fn visit_path(&mut self, item: &Path) {
        self.visit(&item.first);
        for (_, identifier) in &item.rest {
            self.visit_identifier(identifier);
        }
    }

    fn visit_unary(&mut self, item: &UnaryExpression) {
        self.visit(&item.operand);
    }

    fn visit_binary(&mut self, item: &BinaryExpression) {
        self.visit(&item.left);
        self.visit(&item.right);
    }

    fn visit_tuple(&mut self, item: &LiteralTuple) {
        for element in &item.elements {
            self.visit(element);
        }
    }

    fn visit_pair(&mut self, item: &Pair) {
        self.visit(&item.left);
        self.visit(&item.right);
    }

    fn visit_call(&mut self, item: &Call) {
        self.visit(&item.subject);
        for element in &item.arguments {
            self.visit(element);
        }
    }

    fn visit_assignment(&mut self, item: &Assignment) {
        self.visit(&item.pattern);
        self.visit(&item.value);
    }

    fn visit_function(&mut self, item: &Function) {
        self.visit_identifier(&item.name);
        for element in &item.parameters {
            self.visit(element);
        }

        if let Some(return_type) = &item.return_type {
            self.visit(return_type);
        }

        if let Some(body) = &item.body {
            for element in body {
                self.visit(element);
            }
        }
    }

    fn visit_struct(&mut self, item: &Struct) {
        self.visit_identifier(&item.name);
        for element in &item.fields {
            self.visit(element);
        }
    }

    fn visit_module(&mut self, item: &Module) {
        self.visit_identifier(&item.name);
    }
}
