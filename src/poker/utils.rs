use rand::{Rng, prelude::IteratorRandom};

use super::cards::Cards;
use rand::seq::SliceRandom;

pub fn random_gen() -> [Cards; 5] {
    let mut dest = [Cards::Heart(0); 5];
    let d = (0..52).choose_multiple(&mut rand::thread_rng(), 5);
    for (i,x) in d.iter().enumerate() {
        dest[i] = Cards::from_id(*x);
    }
    dest
}

struct IterCards {}

impl IterCards {
    fn new() {}
}
