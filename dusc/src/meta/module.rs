use super::StaticCell;

use ast::AstNode;

#[derive(Debug)]
pub struct Module<'i> {
    source: StaticCell<String>,
    ast: Vec<AstNode<'i>>,
}

impl<'i> Module<'i> {
    pub fn new(source: StaticCell<String>, ast: Vec<AstNode<'i>>) -> Self {
        Module { source, ast }
    }
}
