use nom::character::complete::char;
use nom::combinator::map;
use nom::sequence::tuple;
use nom::IResult;

const INPUT: &str = include_str!("../inputs/day_10_input");

fn parse_num_pair(i: &str) -> IResult<&str, (i32, i32)> {
    use nom::character::complete::{space0, space1};

    Ok(map(
        tuple((
            char('<'),
            space0,
            parse_num,
            char(','),
            space1,
            parse_num,
            char('>'),
        )),
        |(_, _, x, _, _, y, _): (_, _, i32, _, _, i32, _)| (x, y),
    )(i)?)
}

fn parse_num(i: &str) -> IResult<&str, i32> {
    use nom::character::complete::digit1;
    use nom::combinator::opt;

    Ok(map(
        tuple((opt(char('-')), digit1)),
        |(sign, num): (_, &str)| {
            if let Some(_) = sign {
                -(num.parse::<i32>().unwrap())
            } else {
                num.parse().unwrap()
            }
        },
    )(i)?)
}

#[derive(Debug)]
struct Point {
    position: (i32, i32),
    velocity: (i32, i32),
}

impl Point {
    fn parse(i: &str) -> IResult<&str, Self> {
        use nom::bytes::complete::tag;

        let (i, point) = map(
            tuple((
                tag("position="),
                parse_num_pair,
                tag(" velocity="),
                parse_num_pair,
            )),
            |(_, position, _, velocity): (_, (i32, i32), _, _)| Self { position, velocity },
        )(i)?;

        Ok((i, point))
    }
}

pub fn solve() {
    let mut points = INPUT
        .lines()
        .map(|i| Point::parse(i).unwrap().1)
        .collect::<Vec<_>>();

    let mut min_delta = i32::MAX;
    let mut min_step = 0;

    for i in 0..80000 {
        advance_points(&mut points);
        let (min, max) = find_extrema(&points);

        let delta = (max.0 - min.0 + 1) + (max.1 - min.1 + 1);
        if delta < min_delta {
            min_delta = delta;
            min_step = i;
        }
    }

    let mut points = INPUT
        .lines()
        .map(|i| Point::parse(i).unwrap().1)
        .collect::<Vec<_>>();

    for _ in 0..=min_step {
        advance_points(&mut points);
    }

    let (min, max) = find_extrema(&points);

    for x in min.1..=max.1 {
        for y in min.0..=max.0 {
            let mut contains = false;
            for point in &points {
                if point.position == (y, x) {
                    contains = true;
                    break;
                }
            }
            if contains {
                print!("#");
            } else {
                print!(" ");
            }
        }
        print!("\n");
    }
}

fn advance_points(points: &mut [Point]) {
    for point in points {
        point.position.0 += point.velocity.0;
        point.position.1 += point.velocity.1;
    }
}

fn find_extrema(points: &[Point]) -> ((i32, i32), (i32, i32)) {
    let mut min = (i32::MAX, i32::MAX);
    let mut max = (i32::MIN, i32::MIN);
    for point in points {
        min.0 = min.0.min(point.position.0);
        max.0 = max.0.max(point.position.0);
        min.1 = min.1.min(point.position.1);
        max.1 = max.1.max(point.position.1);
    }
    (min, max)
}

pub fn solve_extra() {
    let mut points = INPUT
        .lines()
        .map(|i| Point::parse(i).unwrap().1)
        .collect::<Vec<_>>();

    let mut min_delta = i32::MAX;
    let mut min_step = 0;

    for i in 0..80000 {
        advance_points(&mut points);
        let (min, max) = find_extrema(&points);

        let delta = (max.0 - min.0 + 1) + (max.1 - min.1 + 1);
        if delta < min_delta {
            min_delta = delta;
            min_step = i;
        }
    }
    println!("Min steps: {}", min_step + 1);
}
