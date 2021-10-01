use std::collections::HashMap;

use itertools::Itertools;
use poker::poker::{cards::Cards, hand::PokerHands, payout::{Payout, JB96}, utils::random_gen};


#[macro_use]
extern crate timeit;


fn main() {
    timeit!({
        let payout_calc = JB96 {};
        let mut total = 0.0f64;
        let mut cnt: HashMap<&str, i32> = HashMap::new();
        let mut c = 0;
        
        for card_ids in (0..52).combinations(5) {
            c += 1;
            let mut cards = [Cards::Heart(0); 5];

            for (i,x) in card_ids.iter().enumerate() {
                cards[i] = Cards::from_id(*x);
            }

            let hand = PokerHands::cards_to_hand(&cards);
            let v = payout_calc.payout(&hand);
            total += v;
            let hand_name = payout_calc.name(&hand);
            *cnt.entry(&hand_name).or_insert(0) += 1;
        }
        println!("Total : {} / {}", total , c);
        println!("{:#?}", cnt);
})
}
