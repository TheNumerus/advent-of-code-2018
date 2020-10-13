use std::collections::HashMap;

use nom::bytes::complete::tag;
use nom::IResult;

const INPUT: &str = include_str!("../inputs/day_12_input");

fn parse_rule(i: &str) -> IResult<&str, ([bool; 5], bool)> {
    use nom::multi::many_m_n;

    let (i, pots) = many_m_n(5, 5, pot)(i)?;

    // i don't like this
    let mut pattern = [false; 5];
    for i in 0..5 {
        pattern[i] = pots[i];
    }

    let (i, _) = tag(" => ")(i)?;
    let (i, result) = pot(i)?;

    Ok((i, (pattern, result)))
}

fn parse_pots(i: &str) -> IResult<&str, Vec<bool>> {
    use nom::multi::fold_many0;

    let (i, _) = tag("initial state: ")(i)?;
    let (i, test) = fold_many0(pot, Vec::new(), |mut acc, item| {
        acc.push(item);
        acc
    })(i)?;
    let (i, _) = tag("\n")(i)?;

    Ok((i, test))
}

fn pot(i: &str) -> IResult<&str, bool> {
    use nom::character::complete::one_of;
    use nom::combinator::map;

    map(one_of(".#"), |a| a == '#')(i)
}

pub fn solve() {
    let (i, pots_input) = parse_pots(INPUT).unwrap();
    let rules = i
        .lines()
        .skip(1)
        .map(|i| parse_rule(i).unwrap().1)
        .collect::<HashMap<_, _>>();

    assert_eq!(32, rules.len(), "Invalid number of rules, should be 32");

    let mut pots = vec![false; 800];
    pots.splice(400..400, pots_input);

    for _ in 0..20 {
        let p_clone = pots.clone();
        for x in 2..898 {
            let key = [
                p_clone[x - 2],
                p_clone[x - 1],
                p_clone[x],
                p_clone[x + 1],
                p_clone[x + 2],
            ];
            pots[x] = rules[&key];
        }
    }

    let sum = pot_sum(&pots, 400);

    println!("Pot sum: {}", sum);
}

fn pot_sum(pots: &[bool], center: i32) -> i32 {
    pots.iter().enumerate().fold(0, |mut acc, (i, pot)| {
        if *pot {
            acc += i as i32 - center;
        }
        acc
    })
}

pub fn solve_extra() {
    let (i, pots_input) = parse_pots(INPUT).unwrap();
    let rules = i
        .lines()
        .skip(1)
        .map(|i| parse_rule(i).unwrap().1)
        .collect::<HashMap<_, _>>();

    assert_eq!(32, rules.len(), "Invalid number of rules, should be 32");

    let mut pots = vec![false; 80000];
    pots.splice(40000..40000, pots_input);

    let mut last_sum = 0;

    let size = pots.len();

    let mut last_adds = Vec::with_capacity(100);

    let mut stable_gen = 0;

    for i in 0..10000 {
        let p_clone = pots.clone();
        for x in 2..(size - 2) {
            let key = [
                p_clone[x - 2],
                p_clone[x - 1],
                p_clone[x],
                p_clone[x + 1],
                p_clone[x + 2],
            ];
            pots[x] = rules[&key];
        }
        let sum = pot_sum(&pots, 40000);
        println!("{}", sum - last_sum);
        last_adds.push(sum - last_sum);
        if last_adds.len() == 101 {
            let mut eq = true;
            for win in last_adds.windows(2) {
                if win[0] != win[1] {
                    eq = false;
                    break;
                }
            }
            if eq {
                stable_gen = i;
                break;
            }
            last_adds.remove(0);
        }
        last_sum = sum;
    }

    let mut sum = pot_sum(&pots, 40000) as isize;

    sum += (50_000_000_000 - stable_gen as isize - 1) * last_adds[0] as isize;

    println!("Pot sum: {}", sum);
}
