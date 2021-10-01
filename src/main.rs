use std::collections::HashMap;

use poker::poker::{
    hand::PokerHands,
    payout::{Payout, JB96},
    utils::random_gen,
};

#[macro_use]
extern crate timeit;


fn main() {
    timeit!({
        let payout_calc = JB96 {};
        let mut total = 0.0f64;
        let mut cnt: HashMap<&str, i32> = HashMap::new();
        for _ in 0..1000000 {
            let cards = random_gen();
            let hand = PokerHands::cards_to_hand(&cards);
            let v = payout_calc.payout(&hand);
            total += v;
            let hand_name = payout_calc.name(&hand);
            *cnt.entry(&hand_name).or_insert(0) += 1;
        }
        println!("Total : {}", total);
        println!("{:#?}", cnt);
})
}
