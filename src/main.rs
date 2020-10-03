use rand::seq::SliceRandom;

const WORD: &[&str] = &["foo", "bar", "baz", "quz", "no"];


fn random_word() -> String {
    let mut rng = rand::thread_rng();
    WORD.choose(&mut rng).unwrap().to_string()
}

fn main() {
    println!("Hello, world!");
    println!("Our random word is {}", random_word());
}
