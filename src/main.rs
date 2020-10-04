mod wordlist;

use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use wordlist as wl;

fn random_pluralnoun(rng: &mut ThreadRng) -> String {
    wl::PLURALNOUN.choose(rng).unwrap().to_string()
}

fn main() {
    let mut rng: ThreadRng = rand::thread_rng();
    println!("Hello, world!");
    println!("Our random word is {}", random_pluralnoun(&mut rng));
    println!("Our next random word is {}", random_pluralnoun(&mut rng));
}
