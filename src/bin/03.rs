use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("inputs/03.txt").expect("Unable to read file");
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn part_1(input: &str) -> usize {
    let mut position = (0, 0);
    let mut visited = HashSet::from([position]);
    for dir in input.chars() {
        position = move_pos(position, dir);
        visited.insert(position);
    }
    return visited.len();
}

fn part_2(input: &str) -> usize {
    let mut santa_pos = (0, 0);
    let mut robo_santa_pos = (0, 0);
    let mut visited = HashSet::from([santa_pos, robo_santa_pos]);
    for (i, dir) in input.char_indices() {
        if i % 2 == 0 {
            santa_pos = move_pos(santa_pos, dir);
        } else {
            robo_santa_pos = move_pos(robo_santa_pos, dir);
        }
        visited.insert(santa_pos);
        visited.insert(robo_santa_pos);
    }
    return visited.len();
}

fn move_pos(p: (i32, i32), d: char) -> (i32, i32) {
    return match d {
        '^' => (p.0-1, p.1),
        'v' => (p.0+1, p.1),
        '>' => (p.0, p.1+1),
        '<' => (p.0, p.1-1),
        _ => p
    }
}
