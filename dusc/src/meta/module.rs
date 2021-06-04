use parser::AstNode;

#[derive(Debug)]
pub struct Module {
    source: String,
    ast: Vec<AstNode>,
}

impl<'i> Module {
    pub fn new(source: String, ast: Vec<AstNode>) -> Self {
        Module { source, ast }
    }
}
