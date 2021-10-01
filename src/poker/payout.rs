use super::hand::PokerHands;


trait Payout {

    fn payout(&self, hand: &PokerHands) -> f64;

}

struct JB96 {

}

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
            },
            PokerHands::None => 0.0
        }
    }
}