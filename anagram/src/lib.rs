use std::collections::{HashMap, HashSet};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let mut dict: HashMap<char, u8> = HashMap::new();

    let word = &word.to_lowercase();

    for c in word.chars() {
        let count = dict.entry(c).or_insert(0);
        *count += 1;
    }
    // dict.keys().to_lowercase();

    let mut res: HashSet<&'a str> = HashSet::new();

    for w in possible_anagrams {
        if &w.to_lowercase() == word {
            continue;
        }
        // let mut tmp_dict = dict.clone();
        let mut tmp_dict: HashMap<char, u8> = HashMap::new();

        for c in w.to_lowercase().chars() {
            let count = tmp_dict.entry(c).or_insert(0);
            *count += 1;
        }

        if tmp_dict == dict {
            res.insert(w);
        }
    }

    res
}
