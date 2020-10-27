use std::{cmp::Ordering, convert::TryFrom, ops::Index};

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
enum Tile {
    Track,
    Turn(TurnDir),
    Intersection,
    None,
}

impl TryFrom<u8> for Tile {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let tile = match value {
            b'|' | b'-' | b'v' | b'^' | b'<' | b'>' => Tile::Track,
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

        (Self { size_x, size_y, inner }, carts)
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
        // list carts
        for cart in &mut carts {
            let Cart { dir, x, y, turn_num } = cart;

            //println!("cart on [{:3},{:3}], dir {:?}, turn num {}", x, y, dir, turn_num);

            let (new_x, new_y) = match (&dir, map[(*x, *y)]) {
                (Direction::Right, Tile::Track) => (*x + 1, *y),
                (Direction::Left, Tile::Track) => (*x - 1, *y),
                (Direction::Up, Tile::Track) => (*x, *y - 1),
                (Direction::Down, Tile::Track) => (*x, *y + 1),
                (Direction::Right, Tile::Turn(TurnDir::Rising)) => {
                    *dir = Direction::Up;
                    (*x, *y + 1)
                }
                (Direction::Right, Tile::Turn(TurnDir::Falling)) => {
                    *dir = Direction::Down;
                    (*x, *y - 1)
                }
                (Direction::Left, Tile::Turn(TurnDir::Rising)) => {
                    *dir = Direction::Down;
                    (*x, *y - 1)
                }
                (Direction::Left, Tile::Turn(TurnDir::Falling)) => {
                    *dir = Direction::Up;
                    (*x, *y + 1)
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
                println!("Collision on [{}:{}]", a.x, b.x);
                break 'tick;
            }
        }
    }
}

pub fn solve_extra() {}
