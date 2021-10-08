use derive_more::Display;
use logos::Logos;
use token_macro_derive::TokenInfo;

#[derive(TokenInfo, Logos, Debug, Display, PartialEq, Eq, Clone, Copy, Hash)]
#[repr(u8)]
pub enum Token {
    #[category(Symbol, FirstTokenOfExpression)]
    #[token("{")]
    #[display(fmt = "`{{`")]
    OpeningBrace,

    #[category(Symbol, ClosingBracket)]
    #[token("}")]
    #[display(fmt = "`}}`")]
    ClosingBrace,

    #[category(Symbol, FirstTokenOfExpression)]
    #[token("(")]
    #[display(fmt = "`(`")]
    OpeningParen,

    #[category(Symbol, ClosingBracket)]
    #[token(")")]
    #[display(fmt = "`)`")]
    ClosingParen,

    #[category(Symbol, FirstTokenOfExpression)]
    #[token("[")]
    #[display(fmt = "`[`")]
    OpeningBracket,

    #[category(Symbol, ClosingBracket)]
    #[token("]")]
    #[display(fmt = "`]`")]
    ClosingBracket,

    #[category(Symbol)]
    #[token(",")]
    #[display(fmt = "`,`")]
    Comma,

    #[category(Symbol, Path)]
    #[token(".")]
    #[display(fmt = "`.`")]
    Dot,

    #[category(Symbol)]
    #[token(":")]
    #[display(fmt = "`:`")]
    Colon,

    #[category(Symbol, Path)]
    #[token("::")]
    #[display(fmt = "`::`")]
    ColonColon,

    #[category(Symbol)]
    #[token(";")]
    #[display(fmt = "`;`")]
    Semicolon,

    #[category(Symbol)]
    #[token("->")]
    #[display(fmt = "`->`")]
    Arrow,

    #[category(Symbol)]
    #[token("=>")]
    #[display(fmt = "`=>`")]
    FatArrow,

    #[category(Symbol)]
    #[token("#")]
    #[display(fmt = "`#`")]
    Hash,

    #[category(Symbol)]
    #[token("!")]
    #[display(fmt = "`!`")]
    Exclamation,

    #[category(Symbol)]
    #[token("=")]
    #[display(fmt = "`=`")]
    Equals,

    #[category(CompoundOperator)]
    #[token("+=")]
    #[display(fmt = "`+=`")]
    PlusEquals,

    #[category(CompoundOperator)]
    #[token("-=")]
    #[display(fmt = "`-=`")]
    MinusEquals,

    #[category(CompoundOperator)]
    #[token("*=")]
    #[display(fmt = "`*=`")]
    AsteriskEquals,

    #[category(CompoundOperator)]
    #[token("/=")]
    #[display(fmt = "`/=`")]
    SlashEquals,

    #[category(BinaryOperator, UnaryOperator, SumOperator, FirstTokenOfExpression)]
    #[token("+")]
    #[display(fmt = "`+`")]
    Plus,

    #[category(BinaryOperator, UnaryOperator, SumOperator, FirstTokenOfExpression)]
    #[token("-")]
    #[display(fmt = "`-`")]
    Minus,

    #[category(BinaryOperator, ProductOperator)]
    #[token("*")]
    #[display(fmt = "`*`")]
    Asterisk,

    #[category(BinaryOperator, ProductOperator)]
    #[token("/")]
    #[display(fmt = "`/`")]
    Slash,

    #[category(BinaryOperator, ComparisonOperator)]
    #[token("<")]
    #[display(fmt = "`<`")]
    Lesser,

    #[category(BinaryOperator, ComparisonOperator)]
    #[token("<=")]
    #[display(fmt = "`<=`")]
    LesserEqual,

    #[category(BinaryOperator, ComparisonOperator)]
    #[token(">")]
    #[display(fmt = "`>`")]
    Greater,

    #[category(BinaryOperator, ComparisonOperator)]
    #[token(">=")]
    #[display(fmt = "`>=`")]
    GreaterEqual,

    #[category(BinaryOperator, ComparisonOperator)]
    #[token("==")]
    #[display(fmt = "`==`")]
    EqualsEquals,

    #[category(BinaryOperator, ComparisonOperator)]
    #[token("!=")]
    #[display(fmt = "`!=`")]
    NotEqual,

    #[category(BinaryOperator)]
    #[token("and")]
    #[display(fmt = "the keyword `and`")]
    And,

    #[category(BinaryOperator)]
    #[token("or")]
    #[display(fmt = "the keyword `or`")]
    Or,

    #[category(UnaryOperator, FirstTokenOfExpression)]
    #[token("not")]
    #[display(fmt = "the keyword `not`")]
    Not,

    #[category(Keyword, ItemKeyword)]
    #[token("struct")]
    #[display(fmt = "the keyword `struct`")]
    Struct,

    #[category(Keyword, ItemKeyword)]
    #[token("fn")]
    #[display(fmt = "the keyword `fn`")]
    Function,

    #[category(Keyword)]
    #[token("let")]
    #[display(fmt = "the keyword `let`")]
    Let,

    #[category(Keyword)]
    #[token("module")]
    #[display(fmt = "the keyword `module`")]
    Module,

    #[category(FirstTokenOfExpression)]
    #[regex("[_a-zA-Z]+[_a-zA-Z0-9]*", priority = 2)]
    #[display(fmt = "an identifier")]
    Identifier,

    #[category(Literal, FirstTokenOfExpression)]
    #[regex(r#""([^"\\]*(\\.[^"\\]*)*)""#)]
    #[display(fmt = "a string literal")]
    String,

    #[category(Literal, FirstTokenOfExpression)]
    #[regex(r"[_0-9]+")]
    #[display(fmt = "an integer literal")]
    Integer,

    #[category(Literal, FirstTokenOfExpression)]
    #[regex(r"[_0-9]+\.[0-9_]+")]
    #[display(fmt = "a float literal")]
    Float,

    #[regex(r"//[^\n]*", logos::skip)]
    #[display(fmt = "a comment")]
    LineComment,

    #[error]
    #[regex(r"[ \t\n\f\s]+", logos::skip)]
    #[display(fmt = "<error>")]
    Error,
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

// It also emits a `macro_rules!` macro named `token_category_slice`, which looks a bit similar to the below:

#[cfg(never)]
macro_rules! token_category {
    (ProductOperator) => {
        &[Token::SymbolAsterisk, Token::SymbolSlash]
    }; // ...
}

// Given a category name, this macro expands to a slice containing the enum variants in that category.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer() {
        let _lexer = Token::lexer("source");
    }
}
