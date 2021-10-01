#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum Cards {
    Heart(i32),
    Diamond(i32),
    Spade(i32),
    Club(i32),
}

impl Cards {
    pub fn suit(&self) -> Suit {
        match self {
            Cards::Heart(_) => Suit::Heart,
            Cards::Diamond(_) => Suit::Diamond,
            Cards::Spade(_) => Suit::Spade,
            Cards::Club(_) => Suit::Club,
        }
    }
    pub fn number(&self) -> i32 {
        match self {
            Cards::Heart(x) => *x,
            Cards::Diamond(x) => *x,
            Cards::Spade(x) => *x,
            Cards::Club(x) => *x,
        }
    }
}