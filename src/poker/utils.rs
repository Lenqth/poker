use rand::Rng;

use super::cards::Cards;

pub fn random_gen() -> [Cards; 5] {
    let mut dest = [Cards::Heart(0); 5];
    let d = rand::distributions::Uniform::from(0..52);
    for (i, x) in rand::thread_rng().sample_iter(d).take(5).enumerate() {
        dest[i] = Cards::from_id(x);
    }
    dest
}
