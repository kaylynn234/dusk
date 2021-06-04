use self::module::Module;

pub mod module;
pub mod package;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DuskPath {
    Root,
    Name(String),
    Member {
        left: Box<DuskPath>,
        right: Box<DuskPath>,
    },
    Scope {
        left: Box<DuskPath>,
        right: Box<DuskPath>,
    },
}

#[derive(Debug)]
pub enum Item {
    Module(Module),
    // ...
}
