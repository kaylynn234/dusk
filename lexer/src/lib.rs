use std::fmt::Display;

use logos::Logos;
use token_macro_derive::TokenInfo;

#[derive(TokenInfo, Logos, Debug, PartialEq, Eq, Clone, Copy, Hash)]
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
    #[token("=>")]
    FatArrow,

    #[category(Symbol)]
    #[token("#")]
    Hash,

    #[category(Symbol)]
    #[token("!")]
    Exclamation,

    #[category(Symbol)]
    #[token("=")]
    Equals,

    #[category(CompoundOperator)]
    #[token("+=")]
    PlusEquals,

    #[category(CompoundOperator)]
    #[token("-=")]
    MinusEquals,

    #[category(CompoundOperator)]
    #[token("*=")]
    AsteriskEquals,

    #[category(CompoundOperator)]
    #[token("/=")]
    SlashEquals,

    #[category(BinaryOperator, UnaryOperator, SumOperator)]
    #[token("+")]
    Plus,

    #[category(BinaryOperator, UnaryOperator, SumOperator)]
    #[token("-")]
    Minus,

    #[category(BinaryOperator, ProductOperator)]
    #[token("*")]
    Asterisk,

    #[category(BinaryOperator, ProductOperator)]
    #[token("/")]
    Slash,

    #[category(BinaryOperator, ComparisonOperator)]
    #[token("<")]
    Lesser,

    #[category(BinaryOperator, ComparisonOperator)]
    #[token("<=")]
    LesserEqual,

    #[category(BinaryOperator, ComparisonOperator)]
    #[token(">")]
    Greater,

    #[category(BinaryOperator, ComparisonOperator)]
    #[token(">=")]
    GreaterEqual,

    #[category(BinaryOperator, ComparisonOperator)]
    #[token("==")]
    EqualsEquals,

    #[category(BinaryOperator, ComparisonOperator)]
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

    #[category(Keyword, ItemKeyword)]
    #[token("struct")]
    Struct,

    #[category(Keyword, ItemKeyword)]
    #[token("fn")]
    Function,

    #[category(Keyword)]
    #[token("let")]
    Let,

    #[category(Keyword, Literal)]
    #[token("True")]
    True,

    #[category(Keyword, Literal)]
    #[token("False")]
    False,

    #[category(Keyword)]
    #[token("module")]
    Module,

    #[regex("[_a-zA-Z]+[_a-zA-Z0-9]*", priority = 2)]
    Identifier,

    #[category(Literal)]
    #[regex(r#""([^"\\]*(\\.[^"\\]*)*)""#)]
    String,

    #[category(Literal)]
    #[regex(r"[_0-9]+")]
    Integer,

    #[category(Literal)]
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
        use Token::*;
        f.write_str(match self {
            OpeningBrace => "`{`",
            ClosingBrace => "`}`",
            OpeningParen => "`(`",
            ClosingParen => "`)`",
            OpeningBracket => "`[`",
            ClosingBracket => "`]`",
            Comma => "`,`",
            Dot => "`.`",
            Colon => "`:`",
            ColonColon => "`::`",
            Semicolon => "`;`",
            Arrow => "`->`",
            FatArrow => "`=>`",
            Hash => "`#`",
            Exclamation => "`!`",
            Equals => "`=`",
            PlusEquals => "`+=`",
            MinusEquals => "`-=`",
            AsteriskEquals => "`*=`",
            SlashEquals => "`/=`",
            Plus => "`+`",
            Minus => "`-`",
            Asterisk => "`*`",
            Slash => "`/`",
            Lesser => "`<`",
            LesserEqual => "`<=`",
            Greater => "`>`",
            GreaterEqual => "`>=`",
            EqualsEquals => "`==`",
            NotEqual => "`!=`",
            And => "keyword `and`",
            Or => "keyword `or`",
            Not => "keyword `not`",
            Struct => "keyword `struct`",
            Function => "keyword `fn`",
            Let => "keyword `let`",
            True => "keyword `True`",
            False => "keyword `False`",
            Module => "keyword `module`",
            Identifier => "identifier",
            String => "string",
            Integer => "integer",
            Float => "float",
            LineComment => "comment",
            Error => "<error>",
        })
    }
}

// You may be wondering something along the lines of "what the hell how is this macro here and where does it come from"
// if you've been reading the rest of the source. The answer is that the `category_derive` macro emits a `macro_rules!`
// macro named `token_category`. It looks a bit similar to the below:

#[cfg(never)]
macro_rules! token_category {
    (ProductOperator) => {
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
