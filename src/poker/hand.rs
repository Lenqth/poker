use super::cards::{Cards, Suit};

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
    fn pair_fq(cards: &[Cards; 5]) -> Self {
        let mut fq = [0; 13];
        for c in cards {
            fq[(c.number() - 1) as usize] += 1;
        }
        let mut v: Option<(i32, i32)> = None;
        for i in 0..13 {
            if fq[i] >= 4 {
                return Self::FourOfAKind;
            }
            if fq[i] >= 2 {
                match v {
                    Some((_, num)) => {
                        if num == 3 {
                            return Self::FullHouse;
                        } else {
                            if fq[i] == 3 {
                                return Self::FullHouse;
                            } else {
                                return Self::TwoPair;
                            }
                        }
                    }
                    None => {
                        v = Some((i as i32 + 1, fq[i]));
                    }
                }
            }
        }
        match v {
            Some((_, 3)) => Self::ThreeOfAKind,
            Some((x, 2)) => Self::OnePair(x),
            Some((_, _)) => panic!("IMPOSSIBLE"),
            None => Self::None,
        }
    }
    fn flushed(cards: &[Cards; 5]) -> bool {
        let suit = cards[0].suit();
        for i in 1..5 {
            if suit != cards[i].suit() {
                return false;
            }
        }
        true
    }
    fn streighted(cards: &[Cards; 5]) -> Option<i32> {
        let mut sorted_num = cards.map(|x| x.number());
        sorted_num.sort();

        let mut a = sorted_num[0];
        if a == 1 {
            let b = sorted_num[1];
            if b == 10 {
                a = 11;
                for i in 2..5 {
                    if sorted_num[i] != a {
                        return None;
                    }
                    a += 1;
                }
                Some(14)
            } else if b == 2 {
                a = 3;
                for i in 2..5 {
                    if sorted_num[i] != a {
                        return None;
                    }
                    a += 1;
                }
                Some(a-1)
            } else {
                return None;
            }
        } else {
            a += 1;
            for i in 1..5 {
                if sorted_num[i] != a {
                    return None;
                }
                a += 1;
            }
            Some(a-1)
        }
    }

    pub fn cards_to_hand(cards: &[Cards; 5]) -> PokerHands {
        let flushed = Self::flushed(&cards);
        let streighted = Self::streighted(&cards);
        if flushed {
            if let Some(x) = streighted {
                if x == 14 {
                    Self::RoyalFlush
                }else{
                    Self::StraightFlush
                }
            } else {
                Self::Flush
            }
        } else {
            if let Some(_) = streighted {
                Self::Straight
            } else {
                Self::pair_fq(&cards)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Cards, PokerHands};

    #[test]
    fn flush() {
        assert!(PokerHands::flushed(&[
            Cards::Spade(1),
            Cards::Spade(4),
            Cards::Spade(2),
            Cards::Spade(8),
            Cards::Spade(11)
        ]));
        assert!(PokerHands::flushed(&[
            Cards::Heart(7),
            Cards::Heart(4),
            Cards::Heart(2),
            Cards::Heart(8),
            Cards::Heart(11)
        ]));
        assert!(!PokerHands::flushed(&[
            Cards::Heart(1),
            Cards::Spade(4),
            Cards::Heart(2),
            Cards::Spade(8),
            Cards::Spade(11)
        ]));
        assert!(!PokerHands::flushed(&[
            Cards::Spade(1),
            Cards::Spade(4),
            Cards::Heart(2),
            Cards::Spade(8),
            Cards::Spade(11)
        ]));
    }
    #[test]
    fn streighted() {
        assert_eq!(Some(5),PokerHands::streighted(&[
            Cards::Spade(1),
            Cards::Spade(4),
            Cards::Spade(2),
            Cards::Spade(5),
            Cards::Spade(3)
        ]));
        assert_eq!(Some(14),PokerHands::streighted(&[
            Cards::Club(10),
            Cards::Heart(13),
            Cards::Spade(11),
            Cards::Diamond(1),
            Cards::Heart(12)
        ]));
        assert_eq!(None,PokerHands::streighted(&[
            Cards::Heart(1),
            Cards::Spade(4),
            Cards::Heart(2),
            Cards::Spade(8),
            Cards::Spade(11)
        ]));
        assert_eq!(None,PokerHands::streighted(&[
            Cards::Spade(11),
            Cards::Spade(13),
            Cards::Heart(12),
            Cards::Spade(2),
            Cards::Spade(1)
        ]));
    }
    #[test]
    fn test_pair_fq_four() {
        assert_eq!(
            PokerHands::FourOfAKind,
            PokerHands::pair_fq(&[
                Cards::Club(1),
                Cards::Heart(6),
                Cards::Heart(1),
                Cards::Spade(1),
                Cards::Diamond(1),
            ])
        );
        assert_eq!(
            PokerHands::FourOfAKind,
            PokerHands::pair_fq(&[
                Cards::Club(13),
                Cards::Heart(12),
                Cards::Spade(13),
                Cards::Diamond(13),
                Cards::Heart(13),
            ])
        );
        assert_eq!(
            PokerHands::ThreeOfAKind,
            PokerHands::pair_fq(&[
                Cards::Club(1),
                Cards::Heart(6),
                Cards::Heart(5),
                Cards::Spade(1),
                Cards::Diamond(1),
            ])
        );
        assert_eq!(
            PokerHands::FullHouse,
            PokerHands::pair_fq(&[
                Cards::Club(13),
                Cards::Heart(5),
                Cards::Spade(5),
                Cards::Spade(13),
                Cards::Diamond(13),
            ])
        );
        assert_eq!(
            PokerHands::TwoPair,
            PokerHands::pair_fq(&[
                Cards::Club(13),
                Cards::Heart(5),
                Cards::Spade(5),
                Cards::Spade(13),
                Cards::Diamond(4),
            ])
        );
        assert_eq!(
            PokerHands::OnePair(8),
            PokerHands::pair_fq(&[
                Cards::Club(8),
                Cards::Heart(1),
                Cards::Spade(5),
                Cards::Spade(8),
                Cards::Diamond(4),
            ])
        );
        assert_eq!(
            PokerHands::OnePair(13),
            PokerHands::pair_fq(&[
                Cards::Club(13),
                Cards::Heart(1),
                Cards::Spade(5),
                Cards::Spade(13),
                Cards::Diamond(4),
            ])
        );
        assert_eq!(
            PokerHands::OnePair(1),
            PokerHands::pair_fq(&[
                Cards::Club(13),
                Cards::Heart(1),
                Cards::Spade(5),
                Cards::Spade(1),
                Cards::Diamond(4),
            ])
        );
    }
}
