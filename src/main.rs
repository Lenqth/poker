use std::collections::HashMap;

use itertools::Itertools;
use poker::poker::{
    cached::Cached,
    cards::Cards,
    hand::PokerHands,
    payout::{Payout, JB96},
    utils::random_gen,
};

#[macro_use]
extern crate timeit;

fn all_combinations() {
    timeit!({
        let payout_calc = JB96 {};
        let mut total = 0.0f64;
        let mut cnt: HashMap<&str, i32> = HashMap::new();
        let mut c = 0;

        for card_ids in (0..52).combinations(5) {
            c += 1;
            let mut cards = [Cards::Heart(0); 5];

            for (i, x) in card_ids.iter().enumerate() {
                cards[i] = Cards::from_id(*x);
            }

            let hand = PokerHands::cards_to_hand(&cards);
            let v = payout_calc.payout(&hand);
            total += v;
            let hand_name = payout_calc.name(&hand);
            *cnt.entry(&hand_name).or_insert(0) += 1;
        }
        println!("Total : {} / {}", total, c);
        println!("{:#?}", cnt);
    })
}

fn random_cache() {
    let cache = Box::new(Cached::make_caches());
    timeit!({
        for _ in 0..1000000 {
            let cards = random_gen();

            let hand = PokerHands::cards_to_hand(&cards);
            let cached_hand = cache.lookup_hand(cards);

            if hand != cached_hand {
                println!(
                    "{} {} {} {} {}",
                    cards[0], cards[1], cards[2], cards[3], cards[4]
                );
            }
        }
    });
    let mut testcases: Vec<[Cards; 5]> = Vec::new();
    for _ in 0..1000000 {
        testcases.push(random_gen());
    }

    timeit!({
        let payout_calc = JB96 {};
        let mut total = 0.0f64;
        let mut cnt: HashMap<&str, i32> = HashMap::new();

        for cards in testcases.iter() { 
            let hand = cache.lookup_hand(*cards);

            let v = payout_calc.payout(&hand);
            total += v;
            let hand_name = payout_calc.name(&hand);
            *cnt.entry(&hand_name).or_insert(0) += 1;
        }
        println!("{}", total);
    });
    timeit!({
        let payout_calc = JB96 {};
        let mut total = 0.0f64;
        let mut cnt: HashMap<&str, i32> = HashMap::new();

        for cards in testcases.iter() { 
            let hand = PokerHands::cards_to_hand(&cards);

            let v = payout_calc.payout(&hand);
            total += v;
            let hand_name = payout_calc.name(&hand);
            *cnt.entry(&hand_name).or_insert(0) += 1;
        }
        println!("{}", total);
    })
}

fn main() {
    random_cache();
}
