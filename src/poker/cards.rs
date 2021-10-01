use std::fmt::Display;

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
impl Display for Cards {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ns = [
            "_", "A", "2", "3", "4", "5", "6", "7", "8", "9", "X", "J", "Q", "K",
        ];
        match self {
            Cards::Heart(x) => {
                f.write_str("♡")?;
                f.write_str(ns[*x as usize])
            }
            Cards::Diamond(x) => {
                f.write_str("♢")?;
                f.write_str(ns[*x as usize])
            }
            Cards::Spade(x) => {
                f.write_str("♠")?;
                f.write_str(ns[*x as usize])
            }
            Cards::Club(x) => {
                f.write_str("♣")?;
                f.write_str(ns[*x as usize])
            }
        }
    }
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

    pub fn get_id(&self) -> i32 {
        match self {
            Cards::Heart(x) => *x - 1,
            Cards::Diamond(x) => 13 + *x - 1,
            Cards::Spade(x) => 26 + *x - 1,
            Cards::Club(x) => 39 + *x - 1,
        }
    }

    pub fn from_id(x: i32) -> Self {
        let t = x / 13;
        let n = x % 13;
        match t {
            0 => Self::Heart(n + 1),
            1 => Self::Diamond(n + 1),
            2 => Self::Spade(n + 1),
            3 => Self::Club(n + 1),
            _ => panic!("IMPOSSIBLE"),
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::poker::cards::Cards;

    #[test]
    fn test_id() {
        for i in 0..52 {
            assert_eq!(i,Cards::from_id(i).get_id());
        }
    }
}