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
    let coords = INPUT
        .lines()
        .map(|i| Coords::parse(i).unwrap().1)
        .collect::<Vec<_>>();
    dbg!(&coords);
}

pub fn solve_extra() {}
