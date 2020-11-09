use std::{convert::TryFrom, ops::Index};

const INPUT: &str = include_str!("../inputs/day_13_input");

#[derive(Debug, Copy, Clone)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
}

#[derive(Debug, Copy, Clone)]
enum TurnDir {
    Rising,
    Falling,
}

#[derive(Debug, Copy, Clone)]
enum TrackDir {
    Vertical,
    Horizontal,
}

#[derive(Debug, Copy, Clone)]
enum Tile {
    Track(TrackDir),
    Turn(TurnDir),
    Intersection,
    None,
}

impl Tile {
    fn is_vertical(&self) -> bool {
        match self {
            Tile::Intersection | Tile::Track(TrackDir::Vertical) => true,
            _ => false,
        }
    }

    fn is_horizontal(&self) -> bool {
        match self {
            Tile::Intersection | Tile::Track(TrackDir::Horizontal) => true,
            _ => false,
        }
    }
}

impl TryFrom<u8> for Tile {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let tile = match value {
            b'v' | b'^' | b'|' => Tile::Track(TrackDir::Vertical),
            b'<' | b'>' | b'-' => Tile::Track(TrackDir::Horizontal),
            b'/' => Tile::Turn(TurnDir::Rising),
            b'\\' => Tile::Turn(TurnDir::Falling),
            b'+' => Tile::Intersection,
            b' ' => Tile::None,
            _ => return Err(()),
        };
        Ok(tile)
    }
}

#[derive(Debug)]
struct Cart {
    dir: Direction,
    x: usize,
    y: usize,
    turn_num: usize,
}

impl Cart {
    fn new(x: usize, y: usize, dir: Direction) -> Self {
        Self { x, y, dir, turn_num: 0 }
    }
}

#[derive(Debug)]
struct Map {
    inner: Vec<Tile>,
    size_x: usize,
    size_y: usize,
}

impl Map {
    fn from_str(input: &str) -> (Self, Vec<Cart>) {
        let mut lines = input.lines();

        let size_x = lines.next().unwrap().len();
        let size_y = lines.count() + 1;

        let mut inner = Vec::with_capacity(size_x * size_y);
        let mut carts = Vec::with_capacity(20);

        let mut x = 0;
        let mut y = 0;

        for byte in input.bytes() {
            if byte == b'\n' {
                y += 1;
                continue;
            }
            let tile = Tile::try_from(byte).expect(&format!("Unknown map tile: {}", byte));

            // now detect carts
            match byte {
                b'v' => carts.push(Cart::new(x, y, Direction::Down)),
                b'^' => carts.push(Cart::new(x, y, Direction::Up)),
                b'<' => carts.push(Cart::new(x, y, Direction::Left)),
                b'>' => carts.push(Cart::new(x, y, Direction::Right)),
                _ => {}
            }

            inner.push(tile);
            x = (x + 1) % size_x;
        }

        // add intersecions back
        let index = |x: usize, y: usize| x + y * size_x;
        for cart in &carts {
            let Cart { x, y, .. } = cart;
            let above = inner[index(*x, *y - 1)];
            let below = inner[index(*x, *y + 1)];
            let left = inner[index(*x - 1, *y)];
            let right = inner[index(*x + 1, *y)];

            if above.is_vertical() && left.is_horizontal() && below.is_vertical() && right.is_horizontal() {
                *inner.get_mut(index(*x, *y)).unwrap() = Tile::Intersection
            }
        }

        (Self { size_x, size_y, inner }, carts)
    }

    fn render(&self, carts: &[Cart]) {
        for y in 0..self.size_y {
            for x in 0..self.size_x {
                let cart = carts.iter().filter(|c| c.x == x && c.y == y).nth(0);
                if let Some(cart) = cart {
                    match cart.dir {
                        Direction::Left => print!("<"),
                        Direction::Up => print!("^"),
                        Direction::Right => print!(">"),
                        Direction::Down => print!("v"),
                    }
                    continue;
                }
                match self[(x, y)] {
                    Tile::Track(TrackDir::Horizontal) => print!("-"),
                    Tile::Track(TrackDir::Vertical) => print!("|"),
                    Tile::Turn(t) => match t {
                        TurnDir::Rising => print!("/"),
                        TurnDir::Falling => print!("\\"),
                    },
                    Tile::Intersection => print!("+"),
                    Tile::None => print!(" "),
                }
            }
            println!();
        }
    }
}

impl Index<(usize, usize)> for Map {
    type Output = Tile;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let index = index.0 + index.1 * self.size_x;
        self.inner.index(index)
    }
}

pub fn solve() {
    let (map, mut carts) = Map::from_str(INPUT);

    'tick: loop {
        println!("\n\n\n\n\n");
        map.render(&carts);
        // list carts
        for cart in &mut carts {
            let Cart { dir, x, y, turn_num } = cart;

            //println!("cart on [{:3},{:3}], dir {:?}, turn num {}", x, y, dir, turn_num);

            let (new_x, new_y) = match (&dir, map[(*x, *y)]) {
                (Direction::Right, Tile::Track(_)) => (*x + 1, *y),
                (Direction::Left, Tile::Track(_)) => (*x - 1, *y),
                (Direction::Up, Tile::Track(_)) => (*x, *y - 1),
                (Direction::Down, Tile::Track(_)) => (*x, *y + 1),
                (Direction::Right, Tile::Turn(TurnDir::Rising)) => {
                    *dir = Direction::Up;
                    (*x, *y - 1)
                }
                (Direction::Right, Tile::Turn(TurnDir::Falling)) => {
                    *dir = Direction::Down;
                    (*x, *y + 1)
                }
                (Direction::Left, Tile::Turn(TurnDir::Rising)) => {
                    *dir = Direction::Down;
                    (*x, *y + 1)
                }
                (Direction::Left, Tile::Turn(TurnDir::Falling)) => {
                    *dir = Direction::Up;
                    (*x, *y - 1)
                }
                (Direction::Up, Tile::Turn(TurnDir::Rising)) => {
                    *dir = Direction::Right;
                    (*x + 1, *y)
                }
                (Direction::Up, Tile::Turn(TurnDir::Falling)) => {
                    *dir = Direction::Left;
                    (*x - 1, *y)
                }
                (Direction::Down, Tile::Turn(TurnDir::Rising)) => {
                    *dir = Direction::Left;
                    (*x - 1, *y)
                }
                (Direction::Down, Tile::Turn(TurnDir::Falling)) => {
                    *dir = Direction::Right;
                    (*x + 1, *y)
                }
                (Direction::Up, Tile::Intersection) => {
                    let coords = match *turn_num % 3 {
                        0 => {
                            *dir = Direction::Left;
                            (*x - 1, *y)
                        }
                        1 => (*x, *y - 1),
                        2 => {
                            *dir = Direction::Right;
                            (*x + 1, *y)
                        }
                        _ => unreachable!(),
                    };
                    *turn_num += 1;
                    coords
                }
                (Direction::Right, Tile::Intersection) => {
                    let coords = match *turn_num % 3 {
                        0 => {
                            *dir = Direction::Up;
                            (*x, *y - 1)
                        }
                        1 => (*x + 1, *y),
                        2 => {
                            *dir = Direction::Down;
                            (*x, *y + 1)
                        }
                        _ => unreachable!(),
                    };
                    *turn_num += 1;
                    coords
                }
                (Direction::Down, Tile::Intersection) => {
                    let coords = match *turn_num % 3 {
                        0 => {
                            *dir = Direction::Right;
                            (*x + 1, *y)
                        }
                        1 => (*x, *y + 1),
                        2 => {
                            *dir = Direction::Left;
                            (*x - 1, *y)
                        }
                        _ => unreachable!(),
                    };
                    *turn_num += 1;
                    coords
                }
                (Direction::Left, Tile::Intersection) => {
                    let coords = match *turn_num % 3 {
                        0 => {
                            *dir = Direction::Down;
                            (*x, *y + 1)
                        }
                        1 => (*x - 1, *y),
                        2 => {
                            *dir = Direction::Up;
                            (*x, *y - 1)
                        }
                        _ => unreachable!(),
                    };
                    *turn_num += 1;
                    coords
                }
                (_, Tile::None) => panic!("cart out of track, on [{}, {}]", x, y),
            };
            *x = new_x;
            *y = new_y;
        }
        carts.sort_by(|a, b| a.y.cmp(&b.y).then(a.x.cmp(&b.x)));

        for carts in carts.windows(2) {
            let a = &carts[0];
            let b = &carts[1];
            if a.x == b.x && a.y == b.y {
                println!("Collision on [{}:{}]", a.x, b.y);
                break 'tick;
            }
        }
    }
}

pub fn solve_extra() {}
