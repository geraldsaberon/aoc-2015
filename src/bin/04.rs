use md5::{Md5, Digest};

fn main() {
    println!("Part 1: {}", hash_starts_with("00000"));
    println!("Part 2: {}", hash_starts_with("000000"));
}

fn hash_starts_with(s: &str) -> i64 {
    let input = std::fs::read_to_string("inputs/04.txt").expect("Unable to read file");
    let input = input.trim();
    for n in 0.. {
        let mut cat = String::new();
        cat.push_str(input);
        cat.push_str(n.to_string().as_str());
        let hash = md5hash(cat);
        print!("{hash} {n}\r");
        if hash.starts_with(s) {
            println!("");
            return n;
        }
    }
    return -1;
}

fn md5hash(d: String) -> String {
    let mut hasher = Md5::new();
    hasher.update(d.trim());
    let hash = format!("{:x}", hasher.finalize());
    return hash;
}
