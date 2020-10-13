use std::ops::Index;

const INPUT: &str = include_str!("../inputs/day_13_input");

#[derive(Debug)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
}

#[derive(Debug)]
enum Tile {
    Track,
    Intersection,
    Cart(Direction),
    Nothing,
}

#[derive(Debug)]
struct Map {
    inner: Vec<Tile>,
    size: (usize, usize),
}

impl Map {
    fn from_str(input: &str) -> Self {
        let x = 0;
        let y = 0;
        let mut inner = Vec::new();

        for byte in input.bytes() {
            match byte {
                _ => panic!("AAAAAAAAAAAAAAAA"),
            }
        }

        //TODO fix size

        Self {
            size: (x, y),
            inner,
        }
    }
}

impl Index<(usize, usize)> for Map {
    type Output = Tile;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let index = index.0 + index.1 * self.size.0;
        self.inner.index(index)
    }
}

pub fn solve() {
    dbg!(std::mem::size_of::<Tile>() * 150 * 150);
    let map = Map::from_str(INPUT);
    dbg!(&map);
}

pub fn solve_extra() {}
