use std::fmt::Display;

use category_derive::Category;
use logos::Logos;

#[derive(Category, Logos, Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Token {
    #[category(Symbol)]
    #[token("{")]
    SymbolLeftBrace,

    #[category(Symbol)]
    #[token("}")]
    SymbolRightBrace,

    #[category(Symbol)]
    #[token("(")]
    SymbolLeftParen,

    #[category(Symbol)]
    #[token(")")]
    SymbolRightParen,

    #[category(Symbol)]
    #[token("[")]
    SymbolLeftBracket,

    #[category(Symbol)]
    #[token("]")]
    SymbolRightBracket,

    #[category(Symbol)]
    #[token(",")]
    SymbolComma,

    #[category(Symbol)]
    #[token(".")]
    SymbolDot,

    #[category(Symbol)]
    #[token(":")]
    SymbolColon,

    #[category(Symbol)]
    #[token(";")]
    SymbolSemicolon,

    #[category(Symbol)]
    #[token("->")]
    SymbolArrow,

    #[category(Symbol)]
    #[token("#")]
    SymbolHash,

    #[category(Symbol)]
    #[token("!")]
    SymbolExclamation,

    #[category(Symbol)]
    #[token("=")]
    SymbolEquals,

    #[category(BinaryOperator, UnaryOperator, Sum)]
    #[token("+")]
    SymbolPlus,

    #[category(BinaryOperator, UnaryOperator, Sum)]
    #[token("-")]
    SymbolMinus,

    #[category(BinaryOperator, Product)]
    #[token("*")]
    SymbolAsterisk,

    #[category(BinaryOperator, Product)]
    #[token("/")]
    SymbolSlash,

    #[category(BinaryOperator, ValueComparison)]
    #[token("<")]
    SymbolLesser,

    #[category(BinaryOperator, ValueComparison)]
    #[token("<=")]
    SymbolLesserEqual,

    #[category(BinaryOperator, ValueComparison)]
    #[token(">")]
    SymbolGreater,

    #[category(BinaryOperator, ValueComparison)]
    #[token(">=")]
    SymbolGreaterEqual,

    #[category(BinaryOperator, ValueComparison)]
    #[token("==")]
    SymbolEqualsEquals,

    #[category(BinaryOperator, ValueComparison)]
    #[token("!=")]
    SymbolNotEqual,

    #[category(BinaryOperator)]
    #[token("and")]
    KeywordAnd,

    #[category(BinaryOperator)]
    #[token("or")]
    KeywordOr,

    #[category(UnaryOperator)]
    #[token("not")]
    KeywordNot,

    #[category(Keyword)]
    #[token("struct")]
    KeywordStruct,

    #[category(Keyword)]
    #[token("fn")]
    KeywordFunction,

    #[category(Keyword)]
    #[token("let")]
    KeywordLet,

    #[category(Keyword, Atom)]
    #[token("True")]
    KeywordTrue,

    #[category(Keyword, Atom)]
    #[token("False")]
    KeywordFalse,

    #[category(Keyword)]
    #[token("module")]
    KeywordModule,

    #[category(Atom)]
    #[regex("[_a-zA-Z]+[_a-zA-Z0-9]*", priority = 2)]
    Identifier,

    #[category(Atom)]
    #[regex(r#""([^"\\]*(\\.[^"\\]*)*)""#)]
    LiteralString,

    #[category(Atom)]
    #[regex(r"[_0-9]+")]
    LiteralInteger,

    #[category(Atom)]
    #[regex(r"[_0-9]+\.[0-9_]+")]
    LiteralFloat,

    #[regex(r"//[^\n]*", logos::skip)]
    LineComment,

    #[error]
    #[regex(r"[ \t\n\f\s]+", logos::skip)]
    Error,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let as_string = match self {
            Token::SymbolLeftBrace => "{",
            Token::SymbolRightBrace => "}",
            Token::SymbolLeftParen => "(",
            Token::SymbolRightParen => ")",
            Token::SymbolLeftBracket => "[",
            Token::SymbolRightBracket => "]",
            Token::SymbolComma => ",",
            Token::SymbolDot => ".",
            Token::SymbolColon => ":",
            Token::SymbolSemicolon => ";",
            Token::SymbolArrow => "->",
            Token::SymbolHash => "#",
            Token::SymbolExclamation => "!",
            Token::SymbolEquals => "=",
            Token::SymbolPlus => "+",
            Token::SymbolMinus => "-",
            Token::SymbolAsterisk => "*",
            Token::SymbolSlash => "/",
            Token::SymbolLesser => "<",
            Token::SymbolLesserEqual => "<=",
            Token::SymbolGreater => ">",
            Token::SymbolGreaterEqual => ">=",
            Token::SymbolEqualsEquals => "==",
            Token::SymbolNotEqual => "!=",
            Token::KeywordAnd => "and",
            Token::KeywordOr => "or",
            Token::KeywordNot => "not",
            Token::KeywordStruct => "struct",
            Token::KeywordFunction => "fn",
            Token::KeywordLet => "let",
            Token::KeywordTrue => "True",
            Token::KeywordFalse => "False",
            Token::KeywordModule => "module",
            Token::Identifier => "identifier",
            Token::LiteralString => "string",
            Token::LiteralInteger => "integer",
            Token::LiteralFloat => "float",
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
