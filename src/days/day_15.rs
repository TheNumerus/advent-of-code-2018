use std::{
    collections::{HashMap, VecDeque},
    convert::TryFrom,
    ops::Index,
};

const INPUT: &str = include_str!("../inputs/day_15_input");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    fn manhattan_distiance_to(&self, other: &Position) -> usize {
        let x = (self.x as i32 - other.x as i32).abs();
        let y = (self.y as i32 - other.y as i32).abs();

        (x + y) as usize
    }

    fn from_tuple((x, y): (usize, usize)) -> Self {
        Self { x, y }
    }
}

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

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum EntityType {
    Goblin,
    Elf,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Entity {
    hp: u32,
    attack: u32,
    side: EntityType,
    pos: Position,
}

impl Entity {
    fn new(x: usize, y: usize, side: EntityType) -> Self {
        Self {
            pos: Position { x, y },
            side,
            attack: 3,
            hp: 200,
        }
    }

    fn list_targets<'a>(&self, entities: &'a [Entity]) -> Vec<&'a Entity> {
        let filter = match self.side {
            EntityType::Elf => |e: &&Entity| e.side == EntityType::Goblin,
            EntityType::Goblin => |e: &&Entity| e.side == EntityType::Elf,
        };

        entities.iter().filter(filter).collect::<Vec<_>>()
    }

    fn locate_target(&self, reach_tree: &ReachTree, targets: &[&Entity], map: &Map) -> Option<Entity> {
        let mut positions = targets
            .iter()
            .flat_map(|e| find_pos_in_range(e.pos.x, e.pos.y, map))
            .filter(|pos| reach_tree.tree.contains_key(&Position::from_tuple(*pos)))
            .collect::<Vec<_>>();
        positions.sort();
        positions.dedup();

        // find closest
        let min = positions.values().min().unwrap();

        let mut closest = Vec::new();
        for pos in &positions {
            let pos_cost = reach_tree.tree.get(&Position::from_tuple(*pos));
            if let Some(cost) = pos_cost {
                if cost == min {
                    closest.push(pos);
                }
            }
        }

        dbg!(closest);

        todo!()
    }

    fn move_to_target(&self, reach: &ReachTree, target: &Entity) {
        todo!();
    }

    fn attack_entity(&self, other: &mut Entity) {
        other.hp -= self.attack;
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

struct ReachTree {
    tree: HashMap<Position, i32>,
}

impl ReachTree {
    fn build_from_pos(pos: &Position, entities: &[Entity], map: &Map) -> Self {
        let mut frontier = VecDeque::new();
        frontier.push_back((pos.x, pos.y, -1));
        let mut tree = HashMap::new();

        while let Some(front) = frontier.pop_front() {
            tree.insert(Position { x: front.0, y: front.1 }, front.2 + 1);
            let neighbours = find_pos_in_range(front.0, front.1, map);
            for n in neighbours {
                if !frontier.iter().any(|(x, y, _cost)| *x == n.0 && *y == n.1)
                    && !tree.contains_key(&Position { x: n.0, y: n.1 })
                    && !entities.iter().any(|e| e.pos.x == n.0 && e.pos.y == n.1)
                {
                    frontier.push_back((n.0, n.1, front.2 + 1));
                }
            }
        }

        Self { tree }
    }

    #[allow(dead_code)]
    /// Renders reach tree to terminal
    fn render(&self) {
        let min_x = self.tree.iter().min_by(|(a, _), (b, _)| a.x.cmp(&b.x)).unwrap().0.x;
        let min_y = self.tree.iter().min_by(|(a, _), (b, _)| a.y.cmp(&b.y)).unwrap().0.y;

        let max_x = self.tree.iter().max_by(|(a, _), (b, _)| a.x.cmp(&b.x)).unwrap().0.x;
        let max_y = self.tree.iter().max_by(|(a, _), (b, _)| a.y.cmp(&b.y)).unwrap().0.y;

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let tile = self.tree.get(&Position { x, y });
                match tile {
                    Some(cost) => print!("{:3}", cost),
                    None => print!("   "),
                }
            }
            println!();
        }
    }
}

pub fn solve() {
    let (map, mut entities) = Map::from_str_with_entities(INPUT);
    let mut round = 0;
    'turn: loop {
        for entity in &entities {
            let targets = entity.list_targets(&entities);
            if targets.is_empty() {
                break 'turn;
            }

            let reach_tree = ReachTree::build_from_pos(&entity.pos, &entities, &map);

            // now select target from list
            let mut target = entity.locate_target(&reach_tree, &targets, &map);

            if let Some(ref mut target) = target {
                // move if needed
                let close = false;
                entity.move_to_target(&reach_tree, &target);

                if close {
                    entity.attack_entity(target);
                }
            }
        }

        // remove dead entities
        entities.retain(|e| e.hp > 0);

        entities.sort_by(|a, b| a.pos.y.cmp(&b.pos.y).then(a.pos.x.cmp(&b.pos.x)));
        round += 1;
    }
    let total_hp = entities.iter().fold(0, |total, entity| total + entity.hp);

    println!("Outcome: {} * {} = {}", round, total_hp, round * total_hp);
}

pub fn solve_extra() {}
