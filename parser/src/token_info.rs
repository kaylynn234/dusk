use lexer::{token_category, Token};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Precedence(u32);

impl Precedence {
    pub const START: Self = Precedence(0);
    pub const CONDITIONAL: Self = Precedence(1);
    pub const IS: Self = Precedence(2);
    pub const OR: Self = Precedence(3);
    pub const AND: Self = Precedence(4);
    pub const COMPARISON: Self = Precedence(5);
    pub const SUM: Self = Precedence(6);
    pub const PRODUCT: Self = Precedence(7);
    pub const PREFIX: Self = Precedence(8);
    pub const POSTFIX: Self = Precedence(9);
    pub const CALL: Self = Precedence(10);

    pub fn up(self: Precedence) -> Self {
        Precedence(self.0 + 1)
    }

    pub fn down(self) -> Self {
        Precedence(self.0 - 1)
    }

    pub fn with(self, assoc: Associativity) -> Precedence {
        match assoc {
            Associativity::Left => self,
            Associativity::Right => self.down(),
        }
    }
}

pub enum Associativity {
    Left,
    Right,
}

pub trait TokenInfoExt {
    fn precedence(&self) -> Precedence;
    fn associativity(&self) -> Associativity;
}

impl TokenInfoExt for Token {
    fn precedence(&self) -> Precedence {
        let precedence = match self {
            Token::Or => Precedence::OR,
            Token::And => Precedence::AND,
            token_category![ComparisonOperator] => Precedence::COMPARISON,
            token_category![SumOperator] => Precedence::SUM,
            token_category![ProductOperator] => Precedence::PRODUCT,
            _ => Precedence::START,
        };

        precedence.with(self.associativity())
    }

    fn associativity(&self) -> Associativity {
        match self {
            _ => Associativity::Left,
        }
    }
}
