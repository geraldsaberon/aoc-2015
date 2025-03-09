fn main() {
    let input = std::fs::read_to_string("inputs/02.txt").expect("Unable to read file");
    let mut total_paper: u64 = 0;
    let mut total_ribbon: u64 = 0;
    for line in input.lines() {
        let s: Vec<&str> = line.split('x').collect();
        if let [l, w, h] = s.iter().map(|n| n.parse().expect("Unable to {n} to int")).collect::<Vec<u64>>().as_slice() {
            let (a, b, c) = (l * w, w * h, h * l);
            total_paper += 2*a + 2*b + 2*c + a.min(b).min(c);
            let mut xs = [l, w, h];
            xs.sort();
            total_ribbon += xs[0]*2 + xs[1]*2 + l*w*h;
        }
    }
    println!("Part 1: {}", total_paper);
    println!("Part 2: {}", total_ribbon);
}
