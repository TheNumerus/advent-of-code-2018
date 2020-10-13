const INPUT: i32 = 3463;

pub fn solve() {
    let powers = (0..(300 * 300))
        .map(|a| pow_from_index(a))
        .collect::<Vec<_>>();

    let mut max_power = i32::MIN;
    let mut max_x = 1;
    let mut max_y = 1;

    let coord_to_index = |x, y| ((x - 1) + (y - 1) * 300) as usize;

    for start_x in 1..299 {
        for start_y in 1..299 {
            let mut power = 0;
            for x in start_x..(start_x + 3) {
                for y in start_y..(start_y + 3) {
                    power += powers[coord_to_index(x, y)];
                }
            }

            if power > max_power {
                max_power = power;
                max_x = start_x;
                max_y = start_y;
            }
        }
    }

    println!("Max power {} on {}, {}", max_power, max_x, max_y);
}

fn pow_from_index(index: i32) -> i32 {
    let x = (index % 300) + 1;
    let y = index / 300 + 1;

    let rack = x + 10;
    let mut power = rack * y;
    power += INPUT;
    power *= rack;
    power = (power / 100) % 10;
    power -= 5;
    power
}

pub fn solve_extra() {
    let powers = (0..(300 * 300))
        .map(|a| pow_from_index(a))
        .collect::<Vec<_>>();

    let mut max_power = i32::MIN;
    let mut max_x = 1;
    let mut max_y = 1;
    let mut max_size = 1;

    let coord_to_index = |x, y| ((x - 1) + (y - 1) * 300) as usize;
    for size in 1..300 {
        for start_x in 1..(300 - size) {
            for start_y in 1..(300 - size) {
                let mut power = 0;
                for x in start_x..(start_x + size) {
                    for y in start_y..(start_y + size) {
                        power += powers[coord_to_index(x, y)];
                    }
                }

                if power > max_power {
                    max_power = power;
                    max_x = start_x;
                    max_y = start_y;
                    max_size = size;
                }
            }
        }
    }

    println!(
        "Max power {} on {}, {} size {}",
        max_power, max_x, max_y, max_size
    );
}
