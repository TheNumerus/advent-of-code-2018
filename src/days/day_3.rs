use nom::IResult;

const INPUT: &str = include_str!("../inputs/day_3_input");

#[derive(Debug)]
struct Claim {
    id: u32,
    left: u32,
    top: u32,
    width: u32,
    height: u32,
}

impl Claim {
    pub fn parse(i: &str) -> IResult<&str, Self> {
        use nom::bytes::complete::tag;
        use nom::character::complete::*;
        use nom::sequence::tuple;

        let (i, (_, id, _, left, _, top, _, width, _, height)) = tuple((
            char('#'),
            digit1,
            tag(" @ "),
            digit1,
            char(','),
            digit1,
            tag(": "),
            digit1,
            char('x'),
            digit1,
        ))(i)?;
        Ok((
            i,
            Self {
                id: id.parse().unwrap(),
                left: left.parse().unwrap(),
                top: top.parse().unwrap(),
                width: width.parse().unwrap(),
                height: height.parse().unwrap(),
            },
        ))
    }
}

fn get_populated_fabric() -> (Vec<Claim>, Vec<u8>) {
    let lines = INPUT.lines();
    let mut claims = Vec::with_capacity(lines.clone().count());
    let mut fabric = vec![0_u8; 1024 * 1024];
    let map = |x, y| y * 1024 + x;
    for line in lines {
        let claim = Claim::parse(line).unwrap().1;
        claims.push(claim);
    }

    for claim in &claims {
        for x in (claim.left)..(claim.left + claim.width) {
            for y in (claim.top)..(claim.top + claim.height) {
                let index = map(x, y) as usize;
                fabric[index] += 1;
            }
        }
    }

    (claims, fabric)
}

pub fn solve() {
    let (_, fabric) = get_populated_fabric();

    let overlaps = fabric.iter().filter(|x| **x > 1).count();
    println!("{} overlaps total.", overlaps);
}

pub fn solve_extra() {
    let (claims, fabric) = get_populated_fabric();

    let map = |x, y| y * 1024 + x;

    let mut claims = claims.iter();

    'claim: loop {
        let claim = claims.next();
        if let None = claim {
            break;
        }
        let claim = claim.unwrap();
        for x in (claim.left)..(claim.left + claim.width) {
            for y in (claim.top)..(claim.top + claim.height) {
                let index = map(x, y) as usize;
                if fabric[index] > 1 {
                    continue 'claim;
                }
            }
        }
        println!("Claim {} is not contested.", claim.id);
    }
}
