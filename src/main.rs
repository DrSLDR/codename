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

fn get_random(ls: &[&str], rng: &mut ThreadRng) -> String {
    ls.choose(rng).unwrap().to_string()
}

fn generate_codename(
    pattern: String,
    map: &HashMap<String, &[&str]>,
    rng: &mut ThreadRng,
) -> String {
    let mut pattern = pattern.clone();
    while pattern.find('#') != None {
        for cl in map.keys() {
            if pattern.find(cl) != None {
                let word = get_random(map.get(cl).unwrap(), rng);
                pattern = pattern.replacen(cl, &word, 1);
            }
        }
    }
    pattern
}

fn main() {
    let mut rng: ThreadRng = rand::thread_rng();
    let map = generate_classmap();

    let pattern: String = get_random(PATTERN, &mut rng);
    println!("{}", generate_codename(pattern, &map, &mut rng));
}
