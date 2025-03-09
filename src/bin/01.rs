fn main() {
    let input = std::fs::read_to_string("inputs/01.txt").expect("Unable to read file");
    let mut floor: i32 = 0;
    let mut basement_pos: usize = 0;
    for (i, c) in input.char_indices() {
        match c {
            '(' => floor += 1,
            ')' => {
                floor -= 1;
                if floor == -1 && basement_pos == 0 {
                    basement_pos = i + 1
                }
            },
            _ => (),
        }
    }
    println!("Part 1: {}", floor);
    println!("Part 2: {}", basement_pos);
}
