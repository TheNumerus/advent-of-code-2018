use std::{
    collections::{HashSet, VecDeque},
    convert::TryFrom,
    ops::Index,
};

const INPUT: &str = include_str!("../inputs/day_15_input");

#[derive(Debug, Copy, Clone, PartialEq)]
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

    fn accesible_targets(&self, entities: &[Entity], map: &Map) -> Vec<(usize, usize)> {
        let rtree = build_reach_tree((self.x, self.y), &entities, map);

        self.list_targets(&entities)
            .flat_map(|e| find_pos_in_range(e.x, e.y, map))
            .filter(|to| rtree.contains(to))
            .collect::<Vec<_>>()
    }

    fn locate_target(&self, entities: &[Entity], map: &Map) -> Option<Entity> {
        todo!()
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

fn find_pos_in_range(x: usize, y: usize, map: &Map) -> Vec<(usize, usize)> {
    let mut output = Vec::with_capacity(4);

    let mut push_if_floor = |pos: (usize, usize)| {
        if let Tile::Floor = map[pos] {
            output.push(pos);
        }
    };

    if x > 0 {
        push_if_floor((x - 1, y));
    }
    if x < map.size_x - 1 {
        push_if_floor((x + 1, y));
    }
    if y > 0 {
        push_if_floor((x, y - 1));
    }
    if y < map.size_y - 1 {
        push_if_floor((x, y + 1));
    }

    output
}

/*fn reachable((to_x, to_y): &(usize, usize), (from_x, from_y): (usize, usize), entities: &[Entity], map: &Map) -> bool {
    let walkable = |x: usize, y: usize| map[(x, y)] == Tile::Floor && !entities.iter().any(|a| a.x == x && a.y == y);

    // TODO write pathfinding
    unimplemented!();
}*/

fn build_reach_tree((from_x, from_y): (usize, usize), entities: &[Entity], map: &Map) -> HashSet<(usize, usize)> {
    let mut frontier = VecDeque::new();
    frontier.push_back((from_x, from_y));
    let mut visited = HashSet::new();

    while !frontier.is_empty() {
        // unwrap can be done, because this loop won't happen if frontier is empty
        let pos = frontier.pop_front().unwrap();
        visited.insert(pos);
        let neighbours = find_pos_in_range(pos.0, pos.1, map);
        for n in neighbours {
            if !frontier.contains(&n) && !visited.contains(&n) && !entities.iter().any(|e| e.x == n.0 && e.y == n.1) {
                frontier.push_back(n);
            }
        }
    }

    visited
}

#[allow(dead_code)]
fn render_reach_tree(tree: &HashSet<(usize, usize)>) {
    let min_x = tree.iter().min_by(|a, b| a.0.cmp(&b.0)).unwrap().0;
    let min_y = tree.iter().min_by(|a, b| a.1.cmp(&b.1)).unwrap().1;

    let max_x = tree.iter().max_by(|a, b| a.0.cmp(&b.0)).unwrap().0;
    let max_y = tree.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap().1;

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if tree.contains(&(x, y)) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn man_dist(from: (usize, usize), to: (usize, usize)) -> usize {
    ((from.0 as i32 - to.0 as i32).abs() + (from.1 as i32 - to.1 as i32).abs()) as usize
}

pub fn solve() {
    let (map, mut entities) = Map::from_str_with_entities(INPUT);
    'turn: loop {
        for entity in &entities {
            let mut targets = entity.accesible_targets(&entities, &map);
            if targets.is_empty() {
                continue;
            }
            //dedup only removes consecutive duplicates
            targets.sort_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0)));
            targets.dedup();

            // sort by distance
            // TODO STUPID FIX - distance does not have map info
            let mut targets = targets
                .iter()
                .map(|pos| (man_dist(*pos, (entity.x, entity.y)), *pos))
                .collect::<Vec<_>>();
            targets.sort_by(|a, b| a.0.cmp(&b.0));
            let targets = targets.iter().map(|(_, pos)| *pos).collect::<Vec<_>>();

            let target = targets[0];
            dbg!(target);
        }
        entities.sort_by(|a, b| a.y.cmp(&b.y).then(a.x.cmp(&b.x)));
        break 'turn;
    }
    //dbg!(map, entities);
}

pub fn solve_extra() {}
