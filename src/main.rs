use std::{cmp::Ordering, collections::HashMap, convert::TryInto, io::{Write, stdin, stdout}, mem::MaybeUninit};

use itertools::Itertools;
use poker::poker::{
    cards::{Cards, Suit},
    hand::PokerHands,
    payout::{Payout, JB96},
    utils::random_gen,
};

#[macro_use]
extern crate timeit;

fn all_holding(payout_calc: &impl Payout, cards: &[Cards; 5]) -> [(f64, Vec<Cards>); 32] {
    let mut card_ids = cards.map(|x| x.get_id());
    card_ids.sort();
    card_ids.reverse();
    let card_ids = card_ids;
    let mut rest: Vec<i32> = (0..52).collect();
    for bit_pos in 0..5usize {
        rest.remove(card_ids[bit_pos] as usize);
    }

    let mut res: [(f64, Vec<Cards>); 32] = Default::default();

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
        res[holding_id as usize] = (t / (n as f64), drawn_cards[0..(popcount as usize)].to_vec());
    }

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

fn input_str(s: &str) -> Result<Vec<Cards>, String> {
    let mut buff = Vec::new();
    let mut mode: Option<Suit> = None;
    for (pos, c) in s.trim_end().chars().into_iter().enumerate() {
        if let Some(suit) = mode {
            let num = match c.to_ascii_lowercase() {
                'a' => 1,
                '1' => 1,
                '2' => 2,
                '3' => 3,
                '4' => 4,
                '5' => 5,
                '6' => 6,
                '7' => 7,
                '8' => 8,
                '9' => 9,
                'x' => 10,
                'j' => 11,
                'q' => 12,
                'k' => 13,
                x => return Err(format!("Error at {} : {}", pos, c)),
            };
            buff.push(Cards::from_suit_num(suit, num));
            mode = None;
        } else {
            match c.to_ascii_lowercase() {
                's' => mode = Some(Suit::Spade),
                'c' => mode = Some(Suit::Club),
                'd' => mode = Some(Suit::Diamond),
                'h' => mode = Some(Suit::Heart),
                x => return Err(format!("Error at {} : {}", pos, c)),
            }
        }
    }
    Ok(buff)
}

fn main() {
    let payout_calc = JB96 {};

    loop {
        let mut s = String::new();
        print!(">");
        stdout().flush();
        stdin().read_line(&mut s);
        match input_str(&s) {
            Ok(hand_v) => {
                if let Ok(x) = hand_v.try_into() {
                    let hand: [Cards; 5] = x;
                    let res = all_holding(&payout_calc, &hand);
                    for (v, holded) in res[0..5].iter() {
                        for item in holded.iter() {
                            print!("{} ", item);
                        }
                        print!("\n");
                        println!("{}", v);
                    }
                } else {
                    println!("Supported only exactly 5 cards");
                    continue;
                }
            }
            Err(e) => println!("{}", e),
        }
    }
}
