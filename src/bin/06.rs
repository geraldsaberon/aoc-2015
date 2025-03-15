const GRID_SIZE: usize = 1000;

fn main() {
    let input = std::fs::read_to_string("inputs/06.txt").expect("Unable to read file.");
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn part_1(input: &str) -> usize {
    let mut grid: Vec<bool> = vec![false; 1000 * 1000];
    for line in input.lines() {
        let square: Vec<usize> = line.split(|c: char| !c.is_numeric()).filter_map(|n| n.parse().ok()).collect();
        for i in square[0]..=square[2] {
            for j in square[1]..=square[3] {
                grid[i*GRID_SIZE+j] = if line.starts_with("turn on") { true }
                    else if line.starts_with("turn off") { false }
                    else { !grid[i*GRID_SIZE+j] };
            }
        }
    }
    let result = grid.iter().fold(0, |acc, x| acc + if *x { 1 } else { 0 });
    return result;
}

fn part_2(input: &str) -> u32 {
    let mut grid: Vec<u32> = vec![0; GRID_SIZE*GRID_SIZE];
    for line in input.lines() {
        let square: Vec<usize> = line.split(|c: char| !c.is_numeric()).filter_map(|n| n.parse().ok()).collect();
        for i in square[0]..=square[2] {
            for j in square[1]..=square[3] {
                let light = grid[i*GRID_SIZE+j];
                grid[i*GRID_SIZE+j] =
                    if line.starts_with("turn on") { light+1 }
                    else if line.starts_with("turn off") { light.saturating_sub(1) }
                    else { light+2 };
            }
        }
    }
    let result = grid.iter().sum();
    return result;
}
