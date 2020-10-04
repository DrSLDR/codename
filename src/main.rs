mod wordlist;

use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use wordlist as wl;

fn random_adjective(rng: &mut ThreadRng) -> String {
    wl::ADJECTIVE.choose(rng).unwrap().to_string()
}

fn random_adverb(rng: &mut ThreadRng) -> String {
    wl::ADVERB.choose(rng).unwrap().to_string()
}

fn random_pluralnoun(rng: &mut ThreadRng) -> String {
    wl::PLURALNOUN.choose(rng).unwrap().to_string()
}

fn random_verb(rng: &mut ThreadRng) -> String {
    wl::VERB.choose(rng).unwrap().to_string()
}

fn main() {
    let mut rng: ThreadRng = rand::thread_rng();
    println!("Random adjective: {}", random_adjective(&mut rng));
    println!("Random adverb: {}", random_adverb(&mut rng));
    println!("Random pluralnoun: {}", random_pluralnoun(&mut rng));
    println!("Random verb: {}", random_verb(&mut rng));
}
