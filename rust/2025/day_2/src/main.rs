use std::io::Read;

fn extract_ranges(input: &str) -> Vec<(usize, usize)> {
    input
        .split(",")
        .collect::<Vec<&str>>()
        .iter()
        .map(|&range| {
            let (start, end) = range.split_once("-").unwrap();
            (start.trim().parse().unwrap(), end.trim().parse().unwrap())
        })
        .collect()
}

fn part1(input: &str) {
    let ranges = extract_ranges(input);
    let mut bad_ids = std::collections::HashSet::new();
    for (start, end) in ranges {
        for id in start..=end {
            let id_string = id.to_string();
            if !id_string.len().is_multiple_of(2) {
                continue;
            }
            let split_string = id_string.split_at(id_string.len() / 2);
            if split_string.0 == split_string.1 {
                bad_ids.insert(id);
            }
        }
    }
    let mut total = 0;
    for id in bad_ids {
        total += id;
    }
    println!("part1: {total}");
}

fn part2(input: &str) {
    let ranges = extract_ranges(input);
    let mut bad_ids: std::collections::HashSet<usize> = std::collections::HashSet::new();
    for (start, end) in ranges {
        for id in start..=end {
            let id_string = id.to_string();
            for i in 1..id_string.len() {
                let split_check = id_string
                    .as_bytes()
                    .chunks(i)
                    .map(str::from_utf8)
                    .collect::<Result<std::collections::HashSet<&str>, _>>()
                    .unwrap();
                if split_check.len() == 1 {
                    bad_ids.insert(id);
                    continue;
                }
            }
        }
    }
    let mut total = 0;
    for id in bad_ids {
        total += id;
    }
    println!("part 2: {total}");
}

fn main() {
    let mut file = std::fs::File::open("./input").unwrap();
    let mut input = String::new();
    let _ = file.read_to_string(&mut input);
    part1(&input);
    part2(&input);
}
