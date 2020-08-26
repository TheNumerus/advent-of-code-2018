use std::collections::HashMap;

const INPUT: &str = include_str!("day_2_input");

pub fn solve() {
    let lines = INPUT.split_whitespace();

    let mut pairs = 0;
    let mut triplets = 0;

    for line in lines {
        let mut hashmap = HashMap::new();
        for char in line.chars() {
            let counter = hashmap.entry(char).or_insert(0);
            *counter += 1;
        }

        if hashmap.values().find(|val| **val == 2).is_some() {
            pairs += 1;
        }

        if hashmap.values().find(|val| **val == 3).is_some() {
            triplets += 1;
        }
    }
    println!("Checksum: {}", pairs * triplets);
}

pub fn solve_extra() {
    let lines = INPUT.split_whitespace();

    let mut comparisons = Vec::with_capacity(lines.clone().count() / 2);

    for (i, line) in lines.clone().enumerate() {
        for (j, another) in lines.clone().enumerate() {
            if i > j {
                comparisons.push((line, another));
            }
        }
    }

    for comparison in comparisons {
        let (a, b) = comparison;
        let (difs, string) = compare(a, b);
        if difs == 1 {
            println!("Best match = {}", string);
            break;
        }
    }
}

fn compare(a: &str, b: &str) -> (usize, String) {
    let mut result = String::with_capacity(a.len());
    let mut same = 0;

    for (c, h) in a.chars().zip(b.chars()) {
        if c == h {
            result.push(c);
        } else {
            same += 1
        }
    }
    (same, result)
}
