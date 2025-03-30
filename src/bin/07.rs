use std::collections::HashMap;

fn main() {
    println!("Part 1: {}", part_1());
}

fn part_1() -> u16 {
    let input = std::fs::read_to_string("inputs/07.txt").expect("Unable to read file");
    let mut values: HashMap<&str, u16> = HashMap::new();

    let lines = input.lines().collect::<Vec<&str>>();
    let mut i: usize = 0;
    while values.len() != lines.len() {
        let mut split = lines[i%lines.len()].split(" -> ");
        let op = split.next().unwrap();
        let key = split.next().unwrap();
        let op_split = op.split(" ").collect::<Vec<&str>>();
        if let [v] = op_split.as_slice() {
            if let Some(n) = parse_or_get(v, &values) {
                values.insert(key, n);
            }
        } else if let [_, v] = op_split.as_slice() {
            if let Some(n) = parse_or_get(v, &values) {
                values.insert(key, !n);
            }
        } else if let [l, m, r] = op_split.as_slice() {
            let l = parse_or_get(l, &values);
            let r = parse_or_get(r, &values);
            if !l.is_none() && !r.is_none() {
                let l = l.unwrap();
                let r = r.unwrap();
                let res: Option<u16> = match *m {
                    "AND" => Some(l & r),
                    "OR" => Some(l | r),
                    "LSHIFT" => Some(l << r),
                    "RSHIFT" => Some(l >> r),
                    _ => None
                };
                if let Some(n) = res {
                    values.insert(key, n);
                }
            }
        }
        i += 1;
    }
    return *values.get("a").unwrap();
}

fn parse_or_get(s: &str, m: &HashMap<&str, u16>) -> Option<u16> {
    if let Ok(n) = s.parse::<u16>() {
        return Some(n);
    } else if let Some(n) = m.get(s) {
        return Some(*n);
    } else {
        return None;
    }
}
