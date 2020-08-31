use nom::IResult;

const INPUT: &str = include_str!("day_7_input");

#[derive(Debug)]
struct Step {
    name: char,
    allows: char,
}

impl Step {
    fn parse(i: &str) -> IResult<&str, Self> {
        use nom::bytes::complete::tag;
        use nom::character::complete::anychar;
        use nom::combinator::map;
        use nom::sequence::tuple;
        let (i, step) = map(
            tuple((
                tag("Step "),
                anychar,
                tag(" must be finished before step "),
                anychar,
                tag(" can begin."),
            )),
            |(_, name, _, allows, _): (&str, char, &str, char, &str)| Self { name, allows },
        )(i)?;

        Ok((i, step))
    }
}

pub fn solve() {
    let steps = INPUT
        .lines()
        .map(|i| Step::parse(i).unwrap().1)
        .collect::<Vec<_>>();

    dbg!(&steps);
}

pub fn solve_extra() {}
