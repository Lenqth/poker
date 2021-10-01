use itertools::Itertools;

use super::{cards::Cards, hand::PokerHands};

pub struct Cached {
    cache: Vec<PokerHands>,
}

impl Cached {
    pub fn make_caches() -> Self {
        let combin = (0..52).combinations(5);
        let mut cache: Vec<PokerHands> = Vec::with_capacity(2598960);
        for (i, card_ids) in combin.enumerate() {
            let mut cards = [Cards::Heart(0); 5];

            for (i, x) in card_ids.iter().enumerate() {
                cards[i] = Cards::from_id(*x);
            }

            let hand = PokerHands::cards_to_hand(&cards);
            cache.push(hand);
        }
        Self { cache: cache }
    }

    pub fn lookup_hand(&self, cards: [Cards; 5]) -> PokerHands {
        let mut sorted_cards = cards.map(|x| x.get_id());
        sorted_cards.sort();
        let comb_id = inv_nth(&sorted_cards);

        self.cache[comb_id as usize].clone()
    }
}

pub fn inv_nth(items: &[i32; 5]) -> i32 {
    // https://math.stackexchange.com/questions/1363239/fast-way-to-get-a-position-of-combination-without-repetitions
    let mut n = binomial(52, 5) - 1;
    for (i, k) in items.iter().enumerate() {
        n -= binomial(52 - (k + 1), (5 - i) as i32);
    }
    n
}

pub fn binomial(n: i32, k: i32) -> i32 {
    (0..n + 1).rev().zip(1..k + 1).fold(1, |mut r, (n, d)| {
        r *= n;
        r /= d;
        r
    })
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use crate::poker::cached::inv_nth;

    use super::{binomial, Cards, PokerHands};

    #[test]
    fn test_binomial() {
        assert_eq!(binomial(52, 5), 2598960);
    }

    #[test]
    fn test_combin() {
        let mut combin = (0..52).combinations(5);
        let mut n_0: [i32; 5] = [0i32; 5];
        n_0.copy_from_slice(combin.nth(0).unwrap().as_slice());
        println!("{:#?}", n_0);

        let mut combin = (0..52).combinations(5);
        let mut n_1024: [i32; 5] = [0i32; 5];
        n_1024.copy_from_slice(combin.nth(1024).unwrap().as_slice());

        let mut combin = (0..52).combinations(5);
        let mut n_114514: [i32; 5] = [0i32; 5];
        n_114514.copy_from_slice(combin.nth(114514).unwrap().as_slice());

        let mut combin = (0..52).combinations(5);
        let mut n_final: [i32; 5] = [0i32; 5];
        n_final.copy_from_slice(combin.last().unwrap().as_slice());

        assert_eq!(1024, inv_nth(&n_1024));
        assert_eq!(114514, inv_nth(&n_114514));
        assert_eq!(2598959, inv_nth(&n_final));
    }
}
