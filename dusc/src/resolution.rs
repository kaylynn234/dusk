// This entire file probably needs to go/be moved somewhere. Unsure.

use std::collections::HashMap;

pub struct SymbolTable<'i> {
    data: HashMap<&'i str, Option<Symbol<'i>>>,
    unresolved_symbols: usize,
}

impl<'i> SymbolTable<'i> {
    pub fn new() -> Self {
        SymbolTable {
            data: HashMap::new(),
            unresolved_symbols: 0,
        }
    }

    pub fn insert(&mut self, name: &'i str, symbol: Option<Symbol<'i>>) {
        let current = symbol.is_some();
        let previous = self
            .data
            .insert(name, symbol)
            .map_or(current, |inner| inner.is_some());

        match (previous, current) {
            (true, false) => self.unresolved_symbols += 1,
            (false, true) => self.unresolved_symbols -= 1,
            _ => {}
        }
    }
}

pub enum Symbol<'i> {
    Type(Box<Type<'i>>),
    // We'll extend this later. I'm too tired at the moment. Oh well.
}

// At the moment this doesn't contain much. While the AST might represent what the user typed, this trait more
// represents the actual meaning of what they typed and can help us answer questions - i.e, what can the type _ do?
// What's its size on the stack? And so forth.
pub enum Type<'i> {
    StructType(HashMap<&'i str, &'i Type<'i>>),
}
