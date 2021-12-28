// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut dict = HashMap::new();

    for el in magazine.iter() {
        let mut c = dict.entry(el).or_insert(0);
        *c += 1;
    }

    let mut ans: bool = true;

    for el in note.iter() {
        let w = dict.get(el);
        ans = match w {
            None => false,
            Some(0) => false,
            Some(&c) => {
                dict.insert(el, c - 1);
                true
            }
        };

        if ans == false {
            break;
        };
    }
    ans
}
