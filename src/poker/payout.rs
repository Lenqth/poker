use super::hand::PokerHands;

pub trait Payout {
    fn name(&self, hand: &PokerHands) -> &str;
    fn payout(&self, hand: &PokerHands) -> f64;
}

pub struct JB96 {}

impl Payout for JB96 {
    fn payout(&self, hand: &PokerHands) -> f64 {
        match hand {
            PokerHands::RoyalFlush => 250.0,
            PokerHands::FiveOfAKind => todo!(),
            PokerHands::StraightFlush => 50.0,
            PokerHands::FourOfAKind => 25.0,
            PokerHands::FullHouse => 9.0,
            PokerHands::Flush => 6.0,
            PokerHands::Straight => 4.0,
            PokerHands::ThreeOfAKind => 3.0,
            PokerHands::TwoPair => 2.0,
            PokerHands::OnePair(x) => {
                if *x == 1 || *x >= 11 {
                    1.0
                } else {
                    0.0
                }
            }
            PokerHands::None => 0.0,
        }
    }
    fn name(&self, hand: &PokerHands) -> &str {
        match hand {
            PokerHands::RoyalFlush => "RoyalFlush",
            PokerHands::FiveOfAKind => "FiveOfAKind",
            PokerHands::StraightFlush => "StraightFlush",
            PokerHands::FourOfAKind => "FourOfAKind",
            PokerHands::FullHouse => "FullHouse",
            PokerHands::Flush => "Flush",
            PokerHands::Straight => "Straight",
            PokerHands::ThreeOfAKind => "ThreeOfAKind",
            PokerHands::TwoPair => "TwoPair",
            PokerHands::OnePair(x) => {
                if *x == 1 || *x >= 11 {
                    "JB"
                } else {
                    "nothing"
                }
            }
            PokerHands::None => "nothing",
        }
    }
}
