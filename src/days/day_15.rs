use std::{convert::TryFrom, ops::Index};

const INPUT: &str = include_str!("../inputs/day_15_input");

#[derive(Debug, Copy, Clone)]
enum Tile {
    Wall,
    Floor,
}

impl TryFrom<u8> for Tile {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use Tile::*;
        Ok(match value {
            b'#' => Wall,
            b'.' | b'E' | b'G' => Floor,
            _ => return Err(()),
        })
    }
}

#[derive(Debug, PartialEq)]
enum EntityType {
    Goblin,
    Elf,
}

#[derive(Debug)]
struct Entity {
    hp: u32,
    attack: u32,
    side: EntityType,
    x: usize,
    y: usize,
}

impl Entity {
    fn new(x: usize, y: usize, side: EntityType) -> Self {
        Self {
            x,
            y,
            side,
            attack: 3,
            hp: 200,
        }
    }

    fn list_targets<'a>(&self, entities: &'a [Entity]) -> impl Iterator<Item = &'a Entity> {
        let filter = match self.side {
            EntityType::Elf => |e: &&Entity| e.side == EntityType::Goblin,
            EntityType::Goblin => |e: &&Entity| e.side == EntityType::Elf,
        };

        entities.iter().filter(filter)
    }
}

#[derive(Debug)]
struct Map {
    inner: Vec<Tile>,
    size_x: usize,
    size_y: usize,
}

impl Map {
    fn from_str_with_entities(input: &str) -> (Self, Vec<Entity>) {
        let mut lines = input.lines();

        let size_x = lines.next().unwrap().len();
        let size_y = lines.count() + 1;

        let mut inner = Vec::with_capacity(size_x * size_y);
        let mut entities = Vec::new();

        let mut x = 0;
        let mut y = 0;

        for byte in input.bytes() {
            if byte == b'\n' {
                y += 1;
                continue;
            }
            let tile = Tile::try_from(byte).expect(&format!("Unknown map tile: {}", byte));
            match byte {
                b'E' => entities.push(Entity::new(x, y, EntityType::Elf)),
                b'G' => entities.push(Entity::new(x, y, EntityType::Goblin)),
                _ => {}
            }

            inner.push(tile);
            x = (x + 1) % size_x;
        }

        (Self { size_x, size_y, inner }, entities)
    }
}

impl Index<(usize, usize)> for Map {
    type Output = Tile;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let index = index.0 + index.1 * self.size_x;
        self.inner.index(index)
    }
}

fn find_pos_in_range(entity: &Entity, map: &Map) -> Vec<(usize, usize)> {
    let Entity { x, y, .. } = entity;
    let mut output = Vec::with_capacity(4);
    // TODO check walls
    if *x > 0 {
        output.push((*x - 1, *y));
    }
    if *x < map.size_x - 1 {
        output.push((*x + 1, *y));
    }
    if *y > 0 {
        output.push((*x, *y - 1));
    }
    if *y < map.size_y - 1 {
        output.push((*x, *y + 1));
    }

    output
}

fn reachable((to_x, to_y): &(usize, usize), (from_x, from_y): (usize, usize), entities: &[Entity]) -> bool {
    // TODO write pathfinding
    unimplemented!();
}

pub fn solve() {
    let (map, mut entities) = Map::from_str_with_entities(INPUT);
    'turn: loop {
        for entity in &entities {
            let mut targets = entity
                .list_targets(&entities)
                .flat_map(|e| find_pos_in_range(e, &map))
                .filter(|to| reachable(to, (entity.x, entity.y), &entities))
                .collect::<Vec<_>>();
            targets.dedup();
            // TODO sort by distance
            targets.sort_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0)));
            let target = targets[0];
            dbg!(target);
        }
        entities.sort_by(|a, b| a.y.cmp(&b.y).then(a.x.cmp(&b.x)));
        break 'turn;
    }
    //dbg!(map, entities);
}

pub fn solve_extra() {}
