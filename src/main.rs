use rand::seq::SliceRandom;

fn random_word() -> String {
    let mut rng = rand::thread_rng();
    let words: [&str; 3] = ["foo", "bar", "baz"];
    words.choose(&mut rng).unwrap().to_string()
}

fn main() {
    println!("Hello, world!");
    println!("Our random word is {}", random_word());
}
