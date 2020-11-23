const INPUT: usize = 990941;
const INPUT_SLICED: [u8; 6] = [9, 9, 0, 9, 4, 1];

pub fn solve() {
    let mut recipes = Vec::with_capacity(INPUT + 12);
    recipes.extend_from_slice(&[3_u8, 7]);
    let mut elves = (0, 1);
    loop {
        let score = recipes[elves.0] + recipes[elves.1];
        if score >= 10 {
            recipes.push(1);
        }
        recipes.push(score % 10);
        elves.0 = (elves.0 + recipes[elves.0] as usize + 1) % recipes.len();
        elves.1 = (elves.1 + recipes[elves.1] as usize + 1) % recipes.len();
        if recipes.len() >= INPUT + 10 {
            break;
        }
    }
    println!("{:?}", &recipes[INPUT..(INPUT + 10)]);
}

pub fn solve_extra() {
    let mut recipes = Vec::with_capacity(INPUT + 12);
    recipes.extend_from_slice(&[3_u8, 7]);
    let mut elves = (0, 1);
    let off_by_one = loop {
        let score = recipes[elves.0] + recipes[elves.1];
        if score >= 10 {
            recipes.push(1);
        }
        recipes.push(score % 10);
        elves.0 = (elves.0 + recipes[elves.0] as usize + 1) % recipes.len();
        elves.1 = (elves.1 + recipes[elves.1] as usize + 1) % recipes.len();
        if score >= 10 {
            let start = if (recipes.len() - 1) < INPUT_SLICED.len() {
                0
            } else {
                recipes.len() - INPUT_SLICED.len() - 1
            };

            let end = if start + INPUT_SLICED.len() >= recipes.len() {
                recipes.len()
            } else {
                start + INPUT_SLICED.len()
            };

            if recipes[start..end] == INPUT_SLICED {
                break true;
            }
        } else {
            let start = if recipes.len() < INPUT_SLICED.len() {
                0
            } else {
                recipes.len() - INPUT_SLICED.len()
            };

            if recipes[start..] == INPUT_SLICED {
                break false;
            }
        }
    };

    if off_by_one {
        println!("{:?}", recipes.len() - INPUT_SLICED.len() - 1);
    } else {
        println!("{:?}", recipes.len() - INPUT_SLICED.len());
    }
}
