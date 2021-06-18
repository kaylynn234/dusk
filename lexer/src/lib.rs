use std::fmt::Display;

use category_derive::Category;
use logos::Logos;

#[derive(Category, Logos, Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[repr(u8)]
pub enum Token {
    #[category(Symbol)]
    #[token("{")]
    OpeningBrace,

    #[category(Symbol, ClosingBracket)]
    #[token("}")]
    ClosingBrace,

    #[category(Symbol)]
    #[token("(")]
    OpeningParen,

    #[category(Symbol, ClosingBracket)]
    #[token(")")]
    ClosingParen,

    #[category(Symbol)]
    #[token("[")]
    OpeningBracket,

    #[category(Symbol, ClosingBracket)]
    #[token("]")]
    ClosingBracket,

    #[category(Symbol)]
    #[token(",")]
    Comma,

    #[category(Symbol, Path)]
    #[token(".")]
    Dot,

    #[category(Symbol)]
    #[token(":")]
    Colon,

    #[category(Symbol, Path)]
    #[token("::")]
    ColonColon,

    #[category(Symbol)]
    #[token(";")]
    Semicolon,

    #[category(Symbol)]
    #[token("->")]
    Arrow,

    #[category(Symbol)]
    #[token("#")]
    Hash,

    #[category(Symbol)]
    #[token("!")]
    Exclamation,

    #[category(Symbol)]
    #[token("=")]
    Equals,

    #[category(BinaryOperator, UnaryOperator, Sum)]
    #[token("+")]
    Plus,

    #[category(BinaryOperator, UnaryOperator, Sum)]
    #[token("-")]
    Minus,

    #[category(BinaryOperator, Product)]
    #[token("*")]
    Asterisk,

    #[category(BinaryOperator, Product)]
    #[token("/")]
    Slash,

    #[category(BinaryOperator, ValueComparison)]
    #[token("<")]
    Lesser,

    #[category(BinaryOperator, ValueComparison)]
    #[token("<=")]
    LesserEqual,

    #[category(BinaryOperator, ValueComparison)]
    #[token(">")]
    Greater,

    #[category(BinaryOperator, ValueComparison)]
    #[token(">=")]
    GreaterEqual,

    #[category(BinaryOperator, ValueComparison)]
    #[token("==")]
    EqualsEquals,

    #[category(BinaryOperator, ValueComparison)]
    #[token("!=")]
    NotEqual,

    #[category(BinaryOperator)]
    #[token("and")]
    And,

    #[category(BinaryOperator)]
    #[token("or")]
    Or,

    #[category(UnaryOperator)]
    #[token("not")]
    Not,

    #[category(Keyword)]
    #[token("struct")]
    Struct,

    #[category(Keyword)]
    #[token("fn")]
    Function,

    #[category(Keyword)]
    #[token("let")]
    Let,

    #[category(Keyword, Atom)]
    #[token("True")]
    True,

    #[category(Keyword, Atom)]
    #[token("False")]
    False,

    #[category(Keyword)]
    #[token("module")]
    Module,

    #[category(Atom)]
    #[regex("[_a-zA-Z]+[_a-zA-Z0-9]*", priority = 2)]
    Identifier,

    #[category(Atom)]
    #[regex(r#""([^"\\]*(\\.[^"\\]*)*)""#)]
    String,

    #[category(Atom)]
    #[regex(r"[_0-9]+")]
    Integer,

    #[category(Atom)]
    #[regex(r"[_0-9]+\.[0-9_]+")]
    Float,

    #[regex(r"//[^\n]*", logos::skip)]
    LineComment,

    #[error]
    #[regex(r"[ \t\n\f\s]+", logos::skip)]
    Error,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let as_string = match self {
            Token::OpeningBrace => "`{`",
            Token::ClosingBrace => "`}`",
            Token::OpeningParen => "`(`",
            Token::ClosingParen => "`)`",
            Token::OpeningBracket => "`[`",
            Token::ClosingBracket => "`]`",
            Token::Comma => "`,`",
            Token::Dot => "`.`",
            Token::Colon => "`:`",
            Token::ColonColon => "`::`",
            Token::Semicolon => "`;`",
            Token::Arrow => "`->`",
            Token::Hash => "`#`",
            Token::Exclamation => "`!`",
            Token::Equals => "`=`",
            Token::Plus => "`+`",
            Token::Minus => "`-`",
            Token::Asterisk => "`*`",
            Token::Slash => "`/`",
            Token::Lesser => "`<`",
            Token::LesserEqual => "`<=`",
            Token::Greater => "`>`",
            Token::GreaterEqual => "`>=`",
            Token::EqualsEquals => "`==`",
            Token::NotEqual => "`!=`",
            Token::And => "keyword `and`",
            Token::Or => "keyword `or`",
            Token::Not => "keyword `not`",
            Token::Struct => "keyword `struct`",
            Token::Function => "keyword `fn`",
            Token::Let => "keyword `let`",
            Token::True => "keyword `True`",
            Token::False => "keyword `False`",
            Token::Module => "keyword `module`",
            Token::Identifier => "identifier",
            Token::String => "string",
            Token::Integer => "integer",
            Token::Float => "float",
            Token::LineComment => "comment",
            Token::Error => "<error>",
        };

        f.write_str(as_string)
    }
}

// You may be wondering something along the lines of "what the hell how is this macro here and where does it come from"
// if you've been reading the rest of the source. The answer is that the `category_derive` macro emits a `macro_rules!`
// macro named `token_category`. It looks a bit similar to the below:

#[cfg(never)]
macro_rules! token_category {
    (Product) => {
        Token::SymbolAsterisk | Token::SymbolSlash
    }; // ...
}

// Given a category name, this macro expands to a pattern that matches the enum variants in that category. It exists
// exclusively to make some parts of the codebase prettier.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer() {
        let _lexer = Token::lexer("source");
    }
}
