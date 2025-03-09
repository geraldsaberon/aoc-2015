use md5::{Md5, Digest};

fn main() {
    println!("Part 1: {}", hash_starts_with("00000"));
    println!("Part 2: {}", hash_starts_with("000000"));
}

fn hash_starts_with(s: &str) -> i64 {
    let input = std::fs::read_to_string("inputs/04.txt")
        .expect("Unable to read file")
        .trim().to_string();
    for n in 0.. {
        let cat = format!("{input}{n}");
        let hash = md5hash(&cat);
        print!("{hash} {n}\r");
        if hash.starts_with(s) {
            println!();
            return n;
        }
    }
    unreachable!();
}

fn md5hash(d: &str) -> String {
    let mut hasher = Md5::new();
    hasher.update(d.trim());
    format!("{:x}", hasher.finalize())
}
