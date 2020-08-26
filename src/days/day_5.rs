const INPUT: &str = include_str!("day_5_input");

pub fn solve() {
    let mut polymer = INPUT.to_owned();
    println!("Final polymer length: {}", react_polymer(&mut polymer));
}

pub fn solve_extra() {
    let min_polymer = (b'a'..b'z')
        .map(|x| {
            let mut polymer = remove_all(x, INPUT.to_owned());
            let react = react_polymer(&mut polymer);
            (x, react)
        })
        .min_by(|a, b| a.1.cmp(&b.1))
        .unwrap();
    println!(
        "Final best polymer length without '{}': {}",
        char::from(min_polymer.0),
        min_polymer.1
    );
}

fn react_polymer(polymer: &mut String) -> usize {
    while let Some(m) = find_reaction(polymer) {
        polymer.replace_range(m..=(m + 1), "");
    }
    polymer.len()
}

fn find_reaction(i: &str) -> Option<usize> {
    let len = i.len();
    let bytes = i.as_bytes();
    for x in 0..(len - 1) {
        let chars = &bytes[x..=(x + 1)];
        let first = chars[0];
        let second = chars[1];
        if first.to_ascii_uppercase() == second.to_ascii_uppercase() {
            if first.is_ascii_uppercase() != second.is_ascii_uppercase() {
                return Some(x);
            }
        }
    }
    None
}

fn remove_all(ch: u8, i: String) -> String {
    let mut new = String::with_capacity(i.len());
    for b in i.as_bytes() {
        if *b != ch && b.to_ascii_lowercase() != ch {
            new.push(char::from(*b));
        }
    }
    new
}
