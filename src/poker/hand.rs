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
    fn suit(&self) -> Suit {
        match self {
            Cards::Heart(_) => Suit::Heart,
            Cards::Diamond(_) => Suit::Diamond,
            Cards::Spade(_) => Suit::Spade,
            Cards::Club(_) => Suit::Club,
        }
    }
    fn number(&self) -> i32 {
        match self {
            Cards::Heart(x) => *x,
            Cards::Diamond(x) => *x,
            Cards::Spade(x) => *x,
            Cards::Club(x) => *x,
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
pub enum PokerHands {
    RoyalFlush,
    FiveOfAKind,
    StraightFlush,
    FourOfAKind,
    FullHouse,
    Flush,
    Straight,
    ThreeOfAKind,
    TwoPair,
    OnePair(i32),
    None,
}

impl PokerHands {
    fn flushed(cards: &[Cards; 5]) -> bool {
        let suit = cards[0].suit();
        for i in 1..5 {
            if suit != cards[i].suit() {
                return false;
            }
        }
        true
    }
    fn streighted(cards: &[Cards; 5]) -> bool {
        let mut sorted_num = cards.map(|x| x.number());
        sorted_num.sort();

        let mut a = sorted_num[0];
        if a == 1 {
            let b = sorted_num[1];
            if b == 10 {
                a = 10;
                for i in 2..5 {
                    if sorted_num[i] != a + 1 {
                        return false;
                    }
                    a += 1;
                }
            } else if b == 2 {
                a = 2;
                for i in 2..5 {
                    if sorted_num[i] != a + 1 {
                        return false;
                    }
                    a += 1;
                }
            }
        } else {
            for i in 1..5 {
                if sorted_num[i] != a + 1 {
                    return false;
                }
                a += 1;
            }
        }
        true
    }

    pub fn cards_to_hand(cards: [Cards; 5]) -> PokerHands {
        let flushed = Self::flushed(&cards);
        let streighted = Self::streighted(&cards);
        PokerHands::None
    }
}

#[cfg(test)]
mod tests {
    use super::{Cards, PokerHands};

    #[test]
    fn flush() {
        assert!(PokerHands::flushed( 
            &[
                Cards::Spade(1),
                Cards::Spade(4),
                Cards::Spade(2),
                Cards::Spade(8),
                Cards::Spade(11)
            ]
        ));
        assert!(PokerHands::flushed( 
            &[
                Cards::Heart(7),
                Cards::Heart(4),
                Cards::Heart(2),
                Cards::Heart(8),
                Cards::Heart(11)
            ]
        ));
        assert!(!PokerHands::flushed( 
            &[
                Cards::Heart(1),
                Cards::Spade(4),
                Cards::Heart(2),
                Cards::Spade(8),
                Cards::Spade(11)
            ]
        ));
        assert!(!PokerHands::flushed( 
            &[
                Cards::Spade(1),
                Cards::Spade(4),
                Cards::Heart(2),
                Cards::Spade(8),
                Cards::Spade(11)
            ]
        ));
    }
    #[test]
    fn streighted() {
        assert!(PokerHands::streighted( 
            &[
                Cards::Spade(1),
                Cards::Spade(4),
                Cards::Spade(2),
                Cards::Spade(5),
                Cards::Spade(3)
            ]
        ));
        assert!(PokerHands::streighted( 
            &[
                Cards::Club(10),
                Cards::Heart(13),
                Cards::Spade(11),
                Cards::Diamond(1),
                Cards::Heart(12)
            ]
        ));
        assert!(!PokerHands::streighted( 
            &[
                Cards::Heart(1),
                Cards::Spade(4),
                Cards::Heart(2),
                Cards::Spade(8),
                Cards::Spade(11)
            ]
        ));
        assert!(!PokerHands::streighted( 
            &[
                Cards::Spade(11),
                Cards::Spade(13),
                Cards::Heart(12),
                Cards::Spade(2),
                Cards::Spade(1)
            ]
        ));
    }
}