use std::{cmp::Ordering, collections::HashMap, mem::MaybeUninit};

use itertools::Itertools;
use poker::poker::{
    cards::Cards,
    hand::PokerHands,
    payout::{Payout, JB96},
    utils::random_gen,
};

#[macro_use]
extern crate timeit;

fn all_holding(payout_calc: &impl Payout, cards: [Cards; 5]) -> [(f64, Vec<Cards>); 32] {
    let mut card_ids = cards.map(|x| x.get_id());
    card_ids.sort();
    card_ids.reverse();
    let card_ids = card_ids;
    let mut rest: Vec<i32> = (0..52).collect();
    for bit_pos in 0..5usize {
        rest.remove(card_ids[bit_pos] as usize);
    }

    const UNINIT: MaybeUninit<(f64, Vec<Cards>)> = MaybeUninit::uninit();
    let mut res: [MaybeUninit<(f64, Vec<Cards>)>; 32] = [UNINIT; 32];

    for holding_id in 0..32i32 {
        let popcount = holding_id.count_ones();
        let mut t = 0.0f64;
        let mut n = 0;
        let mut drawn_cards = [Cards::Heart(0); 5];
        let mut n_hold = 0usize;

        for (bit_pos, x) in card_ids.iter().enumerate() {
            if (holding_id & (1 << bit_pos)) > 0 {
                drawn_cards[n_hold] = Cards::from_id(*x);
                n_hold += 1;
            }
        }

        for card_ids in rest.iter().combinations(5 - popcount as usize) {
            for (i, x) in card_ids.iter().enumerate() {
                drawn_cards[n_hold + i] = Cards::from_id(**x);
            }

            let hand = PokerHands::cards_to_hand(&drawn_cards);
            let v = payout_calc.payout(&hand);
            t += v;
            n += 1;
        }
        res[holding_id as usize]
            .write((t / (n as f64), drawn_cards[0..(popcount as usize)].to_vec()));
    }
    let mut res: [(f64, Vec<Cards>); 32] = unsafe { std::mem::transmute(res) };

    res.sort_by(|(v1, k1), (v2, k2)| v2.partial_cmp(v1).unwrap());
    res
}

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

fn main() {
    let payout_calc = JB96 {};

    let res = all_holding(
        &payout_calc,
        [
            Cards::Diamond(9),
            Cards::Club(5),
            Cards::Heart(6),
            Cards::Spade(3),
            Cards::Club(1),
        ],
    );
    for (v, holded) in res {
        for item in holded.iter() {
            print!("{}", item);
        }
        print!("\n");
        println!("{}", v);
    }
}
