use nom::bytes::complete::tag;
use nom::sequence::tuple;
use nom::IResult;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Copy, Clone)]
struct Timestamp {
    year: u32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
}

impl Timestamp {
    fn parse(i: &str) -> IResult<&str, Self> {
        use nom::character::complete::*;

        let (i, (_, year, _, month, _, day, _, hour, _, minute, _)) = tuple((
            char('['),
            digit1,
            char('-'),
            digit1,
            char('-'),
            digit1,
            space1,
            digit1,
            char(':'),
            digit1,
            tag("] "),
        ))(i)?;
        Ok((
            i,
            Self {
                year: year.parse().unwrap(),
                month: month.parse().unwrap(),
                day: day.parse().unwrap(),
                hour: hour.parse().unwrap(),
                minute: minute.parse().unwrap(),
            },
        ))
    }
}

#[derive(Debug)]
struct GuardShift {
    id: u32,
    start: Timestamp,
    asleep: Vec<(Timestamp, Timestamp)>,
}

#[derive(Debug)]
enum GuardAction {
    Begin(u32),
    Wakes,
    Sleeps,
}

impl GuardAction {
    fn parse(i: &str) -> IResult<&str, Self> {
        use nom::branch::alt;
        use nom::character::complete::digit1;
        use nom::combinator::map;
        let (i, state) = alt((
            map(tag("wakes up"), |_| Self::Wakes),
            map(tag("falls asleep"), |_| Self::Sleeps),
            map(
                tuple((tag("Guard #"), digit1, tag(" begins shift"))),
                |(_, num, _): (_, &str, _)| Self::Begin(num.parse().unwrap()),
            ),
        ))(i)?;
        Ok((i, state))
    }
}

const INPUT: &str = include_str!("../inputs/day_4_input");

pub fn solve() {
    let shifts = get_parsed_shifts();

    let mut sleeping = HashMap::new();

    for shift in &shifts {
        let mut total_sleep = 0;
        for sleep in &shift.asleep {
            total_sleep += sleep.1.minute - sleep.0.minute;
        }
        *sleeping.entry(shift.id).or_insert(0) += total_sleep;
    }

    let sleepiest = sleeping.iter().max_by(|x, y| x.1.cmp(&y.1)).unwrap();

    println!(
        "Guard #{} is the sleepiest with {} minutes",
        sleepiest.0, sleepiest.1
    );

    let sleepy_shifts = shifts
        .iter()
        .filter(|a| a.id == *sleepiest.0)
        .collect::<Vec<_>>();

    let mut minutes = [0; 60];

    for shift in &sleepy_shifts {
        for (start, end) in &shift.asleep {
            for x in start.minute..end.minute {
                minutes[x as usize] += 1;
            }
        }
    }

    let max = minutes
        .iter()
        .enumerate()
        .max_by(|a, b| a.1.cmp(b.1))
        .unwrap();

    println!("{} sleeping days on minute {}", max.1, max.0);

    println!("ID x minute = {}", sleepiest.0 * max.0 as u32);
}

pub fn solve_extra() {
    let shifts = get_parsed_shifts();

    let mut shift_minutes = HashMap::new();

    for shift in &shifts {
        for (start, end) in &shift.asleep {
            for x in start.minute..end.minute {
                *shift_minutes.entry((shift.id, x)).or_insert(0) += 1;
            }
        }
    }

    let top_guard = shift_minutes.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap();
    println!(
        "Guard #{} sleeps on minute {} the most, {}",
        (top_guard.0).0,
        (top_guard.0).1,
        (top_guard.0).0 * (top_guard.0).1
    );
}

fn get_parsed_shifts() -> Vec<GuardShift> {
    let lines = INPUT.lines();
    let mut shifts = Vec::new();
    let mut actions = Vec::with_capacity(lines.clone().count());
    for line in lines {
        let (i, timestamp) = Timestamp::parse(line).unwrap();
        let (_, action) = GuardAction::parse(i).unwrap();
        actions.push((timestamp, action));
    }
    actions.sort_by(|a, b| a.0.cmp(&b.0));

    let mut shift = None;
    let mut actions_iter = actions.iter();

    loop {
        match actions_iter.next() {
            Some(a) => match a.1 {
                GuardAction::Begin(id) => {
                    if let Some(_) = shift {
                        let s = shift.take().unwrap();
                        shifts.push(s);
                    }
                    shift = Some(GuardShift {
                        id,
                        start: a.0,
                        asleep: Vec::new(),
                    });
                }
                GuardAction::Wakes => panic!("This should not happen"),
                GuardAction::Sleeps => {
                    let next_action = actions_iter.next().unwrap();
                    if let Some(s) = &mut shift {
                        s.asleep.push((a.0, next_action.0));
                    }
                }
            },
            None => {
                let s = shift.take();
                shifts.push(s.unwrap());
                break;
            }
        }
    }

    shifts
}
