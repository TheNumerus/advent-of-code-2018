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
    crashed: bool,
}

impl Cart {
    fn new(x: usize, y: usize, dir: Direction) -> Self {
        Self {
            x,
            y,
            dir,
            turn_num: 0,
            crashed: false,
        }
    }

    pub fn move_on_map(&mut self, map: &Map) {
        let track = map[(self.x, self.y)];

        // that match would be `self` sea otherwise
        let Cart { x, y, dir, .. } = self;

        let left = (*x - 1, *y, Direction::Left);
        let right = (*x + 1, *y, Direction::Right);
        let up = (*x, *y - 1, Direction::Up);
        let down = (*x, *y + 1, Direction::Down);

        let (new_x, new_y, new_dir) = match (&dir, track) {
            (Direction::Right, Tile::Track(_)) => right,
            (Direction::Left, Tile::Track(_)) => left,
            (Direction::Up, Tile::Track(_)) => up,
            (Direction::Down, Tile::Track(_)) => down,
            (Direction::Right, Tile::Turn(TurnDir::Rising)) => up,
            (Direction::Right, Tile::Turn(TurnDir::Falling)) => down,
            (Direction::Left, Tile::Turn(TurnDir::Rising)) => down,
            (Direction::Left, Tile::Turn(TurnDir::Falling)) => up,
            (Direction::Up, Tile::Turn(TurnDir::Rising)) => right,
            (Direction::Up, Tile::Turn(TurnDir::Falling)) => left,
            (Direction::Down, Tile::Turn(TurnDir::Rising)) => left,
            (Direction::Down, Tile::Turn(TurnDir::Falling)) => right,
            (Direction::Up, Tile::Intersection) => {
                let coords = match self.turn_num % 3 {
                    0 => left,
                    1 => up,
                    2 => right,
                    _ => unreachable!(),
                };
                self.turn_num += 1;
                coords
            }
            (Direction::Right, Tile::Intersection) => {
                let coords = match self.turn_num % 3 {
                    0 => up,
                    1 => right,
                    2 => down,
                    _ => unreachable!(),
                };
                self.turn_num += 1;
                coords
            }
            (Direction::Down, Tile::Intersection) => {
                let coords = match self.turn_num % 3 {
                    0 => right,
                    1 => down,
                    2 => left,
                    _ => unreachable!(),
                };
                self.turn_num += 1;
                coords
            }
            (Direction::Left, Tile::Intersection) => {
                let coords = match self.turn_num % 3 {
                    0 => down,
                    1 => left,
                    2 => up,
                    _ => unreachable!(),
                };
                self.turn_num += 1;
                coords
            }
            (_, Tile::None) => panic!("cart out of track, on [{}, {}]", x, y),
        };
        *self = Self {
            x: new_x,
            y: new_y,
            dir: new_dir,
            ..*self
        };
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

    // here for debugging
    #[allow(dead_code)]
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
        let mut i = 0;
        // this isn't for loop, because we need to check all carts after any move
        // for loop would borrow `carts` mutably for the whole loop
        loop {
            let (x, y);
            {
                let cart = &mut carts[i];
                cart.move_on_map(&map);
                x = cart.x;
                y = cart.y;
            }

            let overlaps = carts.iter().filter(|c| c.x == x && c.y == y).count();
            if overlaps == 2 {
                println!("Collision on [{}:{}]", x, y);
                break 'tick;
            }

            i += 1;
            if i >= carts.len() {
                break;
            }
        }

        // every tick starts sim from the top, we need to update
        carts.sort_by(|a, b| a.y.cmp(&b.y).then(a.x.cmp(&b.x)));
    }
}

pub fn solve_extra() {
    let (map, mut carts) = Map::from_str(INPUT);

    'tick: loop {
        let mut i = 0;
        // this isn't for loop, because we need to check all carts after any move
        // for loop would borrow `carts` mutably for the whole loop
        loop {
            let (x, y);
            {
                let cart = &mut carts[i];
                cart.move_on_map(&map);
                x = cart.x;
                y = cart.y;
            }

            let overlaps = carts.iter().filter(|c| c.x == x && c.y == y && !c.crashed).count();
            if overlaps == 2 {
                // mark carts as crashed
                carts
                    .iter_mut()
                    .filter(|c| c.x == x && c.y == y)
                    .for_each(|c| c.crashed = true);
            }

            i += 1;
            if i >= carts.len() {
                break;
            }
        }
        // remove crashed carts
        carts.retain(|c| !c.crashed);

        if carts.len() == 1 {
            println!("Last cart on [{}:{}]", carts[0].x, carts[0].y);
            break 'tick;
        }
        // every tick starts sim from the top, we need to update
        carts.sort_by(|a, b| a.y.cmp(&b.y).then(a.x.cmp(&b.x)));
    }
}
