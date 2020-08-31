use std::collections::HashMap;

use nom::IResult;

const INPUT: &str = include_str!("day_6_input");

#[derive(Debug)]
struct Coords(u32, u32);

impl Coords {
    fn parse(i: &str) -> IResult<&str, Self> {
        use nom::bytes::complete::tag;
        use nom::character::complete::digit1;
        use nom::combinator::map;
        use nom::sequence::tuple;
        let (i, state) = map(
            tuple((digit1, tag(", "), digit1)),
            |(x, _, y): (&str, &str, &str)| Self(x.parse().unwrap(), y.parse().unwrap()),
        )(i)?;
        Ok((i, state))
    }
}

pub fn solve() {
    let coords = get_coords();
    let (mut field, x_size, y_size) = get_field(&coords);

    let index_to_coords = |i| (i % x_size, i / x_size);

    for (i, elem) in field.iter_mut().enumerate() {
        let (x, y) = index_to_coords(i);
        let mut distances = coords
            .iter()
            .enumerate()
            .map(|(index, a)| {
                (
                    index,
                    manhattan((a.0 as i32, a.1 as i32), (x as i32, y as i32)),
                )
            })
            .collect::<Vec<_>>();

        distances.sort_by(|a, b| a.1.cmp(&b.1));

        if distances[0].1 != distances[1].1 {
            *elem = distances[0].0
        }
    }

    let mut stats = HashMap::new();

    for elem in &field {
        *stats.entry(*elem).or_insert(0) += 1;
    }

    stats.remove(&usize::MAX);

    // now remove all entries which touch the borders
    for x in 0..x_size {
        stats.remove(&field[x]);
    }
    for x in ((y_size - 1) * x_size)..((y_size - 1) * x_size + x_size) {
        stats.remove(&field[x]);
    }
    for y in 0..y_size {
        let base = y * x_size;
        stats.remove(&field[base]);
        stats.remove(&field[base + x_size - 1]);
    }

    let mut sorted_stats = stats.iter().collect::<Vec<_>>();
    sorted_stats.sort_by(|a, b| b.1.cmp(&a.1));

    println!("Biggest region size is {}", sorted_stats[0].1);
}

pub fn solve_extra() {
    let coords = get_coords();
    let (mut field, x_size, _y_size) = get_field(&coords);

    let index_to_coords = |i| (i % x_size, i / x_size);

    for (i, elem) in field.iter_mut().enumerate() {
        let (x, y) = index_to_coords(i);
        let dist = coords
            .iter()
            .map(|a| manhattan((a.0 as i32, a.1 as i32), (x as i32, y as i32)))
            .sum::<i32>() as usize;
        *elem = dist;
    }
    let reg_size = field.iter().filter(|a| **a < 10_000).count();

    println!("Closest region size is {}", reg_size);
}

fn get_coords() -> Vec<Coords> {
    INPUT
        .lines()
        .map(|i| Coords::parse(i).unwrap().1)
        .collect::<Vec<_>>()
}

fn get_field(coords: &[Coords]) -> (Vec<usize>, usize, usize) {
    let (mut max_x, mut max_y) = (0, 0);

    for center in coords {
        if center.0 > max_x {
            max_x = center.0;
        }

        if center.1 > max_y {
            max_y = center.1;
        }
    }
    let x_size = (max_x + 1) as usize;
    let y_size = (max_y + 1) as usize;

    let field = vec![usize::MAX; x_size * y_size];

    (field, x_size, y_size)
}

fn manhattan((a_x, a_y): (i32, i32), (b_x, b_y): (i32, i32)) -> i32 {
    (a_x - b_x).abs() + (a_y - b_y).abs()
}
