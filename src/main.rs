mod wordlist;

use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use std::collections::HashMap;
use wordlist as wl;

const PATTERN: &[&str] = &["#ADJECTIVE##PLURALNOUN##VERB##ADVERB#"];

fn generate_classmap() -> HashMap<String, &'static [&'static str]> {
    let mut map: HashMap<String, &[&str]> = HashMap::new();
    map.insert("#ADJECTIVE#".to_string(), wl::ADJECTIVE);
    map.insert("#ADVERB#".to_string(), wl::ADVERB);
    map.insert("#PLURALNOUN#".to_string(), wl::PLURALNOUN);
    map.insert("#VERB#".to_string(), wl::VERB);
    map
}

fn random(ls: &[&'static str], rng: &mut ThreadRng) -> &'static str {
    ls.choose(rng).unwrap()
}

fn generate_codename(
    mut pattern: String,
    map: &HashMap<String, &[&'static str]>,
    rng: &mut ThreadRng,
) -> String {
    while pattern.find('#') != None {
        for cl in map.keys() {
            if pattern.find(cl) != None {
                let word = random(map.get(cl).unwrap(), rng);
                pattern = pattern.replacen(cl, &word, 1);
            }
        }
    }
    pattern
}

fn main() {
    let mut rng: ThreadRng = rand::thread_rng();
    let map = generate_classmap();

    let pattern: &str = random(PATTERN, &mut rng);
    println!("{}", generate_codename(pattern.to_string(), &map, &mut rng));
}
