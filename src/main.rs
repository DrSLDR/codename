use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;

const WORD: &[&str] = &["foo", "bar", "baz", "quz", "no"];

fn random_word(rng: &mut ThreadRng) -> String {
    WORD.choose(rng).unwrap().to_string()
}

fn main() {
    let mut rng: ThreadRng = rand::thread_rng();
    println!("Hello, world!");
    println!("Our random word is {}", random_word(&mut rng));
    println!("Our next random word is {}", random_word(&mut rng));
}
