use parser::AstNode;
use std::sync::Arc;

#[derive(Debug)]
pub struct Module {
    source: Arc<String>,
    ast: Vec<AstNode>,
}

impl Module {
    pub fn new(source: Arc<String>, ast: Vec<AstNode>) -> Self {
        Module { source, ast }
    }
}
