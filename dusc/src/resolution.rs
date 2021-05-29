// This entire file probably needs to go/be moved somewhere. Unsure.

use crate::meta::DuskPath;
use std::collections::HashMap;

pub enum TypeInfo {
    Ref()
}

type TypeId = usize;

pub struct Function<'i> {
    parameters: HashMap<&'i str, TypeId>
}

pub struct Struct<'i> {
    fields: HashMap<&'i str, TypeId>
}