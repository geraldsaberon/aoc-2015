fn main() {
    let mut input = String::from("1321131112");
    for _ in 0..40 {
        input = look_and_say(&input);
    }
    println!("Part 1: {}", input.len());
}

fn look_and_say(input: &str) -> String {
    let mut current_n = input.chars().nth(0).expect("Input is empty");
    let mut current_count = 0;
    let mut result = String::new();
    for n in input.chars() {
        if n == current_n {
            current_count += 1;
        } else {
            result = format!("{result}{current_count}{current_n}");
            current_n = n;
            current_count = 1;
        }
    }
    result = format!("{result}{current_count}{current_n}");
    return result;
}
