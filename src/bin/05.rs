use std::collections::{HashMap, HashSet};

fn main() {
    let input = std::fs::read_to_string("inputs/05.txt").expect("Unable to read file");
    let mut part1: u32 = 0;
    let mut part2: u32 = 0;
    for line in input.lines() {
        if check_string_1(line.trim()) {
            part1 += 1;
        }
        if check_string_2(line.trim()) {
            part2 += 1;
        }
    }
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn check_string_1(s: &str) -> bool {
    let mut n_vowels: usize = 0;
    let mut has_double: bool = false;
    let not_allowed = HashSet::from(["ab", "cd", "pq", "xy"]);
    let vowels = HashSet::from(['a', 'e', 'i', 'o', 'u']);

    let chars: Vec<char> = s.chars().collect();
    let mut i: usize = 0;
    while i < s.len() {
        let c = chars[i];
        if vowels.contains(&c) {
            n_vowels += 1;
        }
        if i < s.len()-1 {
            let d=  chars[i+1];
            if c == d {
                has_double = true;
            }
            let pair: String = [c, d].iter().collect();
            if not_allowed.contains(&pair.as_str()) {
                return false;
            }
        }
        i += 1;
    }
    return n_vowels > 2 && has_double;
}

fn check_string_2(s: &str) -> bool {
    let mut has_pair = false;
    let mut has_repeat = false;
    let mut pairs: HashMap<String, usize> = HashMap::new();
    let chars: Vec<char> = s.chars().collect();
    let mut i = 0;
    while i < s.len() && !(has_pair && has_repeat) {
        let c = chars[i];
        if i < s.len()-1 {
            let pair: String = [c, chars[i+1]].iter().collect();
            if pairs.contains_key(&pair) {
                let j = pairs.get(&pair).unwrap();
                if i - j > 1 {
                    has_pair = true;
                }
            } else {
                pairs.insert(pair, i);
            }
        }
        if i < s.len()-2 {
            let d = chars[i+2];
            if c == d {
                has_repeat = true;
            }
        }
        i += 1;
    }
    return has_pair && has_repeat;
}
