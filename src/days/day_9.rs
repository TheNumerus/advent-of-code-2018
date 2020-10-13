use std::collections::HashMap;

use nom::IResult;

const INPUT: &str = include_str!("../inputs/day_9_input");

#[derive(Debug)]
struct GameInput {
    players: u32,
    top_marble: u32,
}

impl GameInput {
    fn parse(i: &str) -> IResult<&str, Self> {
        use nom::bytes::complete::tag;
        use nom::character::complete::digit1;
        use nom::combinator::map;
        use nom::sequence::tuple;
        let (i, game_input) = map(
            tuple((
                digit1,
                tag(" players; last marble is worth "),
                digit1,
                tag(" points"),
            )),
            |(players, _, top_marble, _): (&str, _, _, _)| Self {
                players: players.parse().unwrap(),
                top_marble: top_marble.parse().unwrap(),
            },
        )(i)?;

        Ok((i, game_input))
    }
}

pub fn solve() {
    solve_inter(false);
}

pub fn solve_extra() {
    solve_inter(true);
}

fn solve_inter(larger: bool) {
    let mut game_input = GameInput::parse(INPUT).unwrap().1;

    if larger {
        game_input.top_marble *= 100;
    }

    let mut board = vec![0];

    let mut scores = HashMap::new();

    for i in 0..game_input.players {
        scores.insert(i + 1, 0);
    }

    let mut active = 0;

    // now play
    for stone in 1..=game_input.top_marble {
        let player = (stone - 1) % game_input.players + 1;
        if stone % 23 == 0 {
            // apply score to player on turn
            let new_active = (active - 7 + board.len()) % board.len() as usize;
            let second = board.remove(new_active);
            active = new_active;
            *scores.get_mut(&player).unwrap() += stone as u64 + second as u64;
        } else {
            let new_active = (active + 2) % board.len() as usize;
            board.insert(new_active, stone);
            active = new_active;
        }
    }

    let best_score = scores.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap();
    println!("Best score is {}", best_score.1);
}
