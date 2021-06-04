use parser::Error as ParseError;

pub enum ErrorKind {
    ParseError(ParseError),
}

