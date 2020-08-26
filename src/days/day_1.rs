use std::collections::HashSet;

const INPUT: &str = include_str!("day_1_input");

pub fn solve() {
    let mut start = 0;
    let values = INPUT
        .split_whitespace()
        .map(|num| num.parse::<isize>().unwrap());
    for value in values {
        start += value;
    }
    println!("Result is {}", start);
}

pub fn solve_extra() {
    let mut start = 0;
    let values = INPUT
        .split_whitespace()
        .map(|num| num.parse::<isize>().unwrap());
    let mut set: HashSet<isize> = HashSet::new();
    'l: loop {
        let values = values.clone();
        for value in values {
            start += value;
            if set.contains(&start) {
                break 'l;
            }
            set.insert(start);
        }
    }
    println!("Found {} twice", start);
}
