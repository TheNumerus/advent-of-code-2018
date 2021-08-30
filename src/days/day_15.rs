use std::{
    collections::{HashMap, VecDeque},
    convert::TryFrom,
};

const INPUT: &str = include_str!("../inputs/day_15_input");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    fn from_tuple((x, y): (usize, usize)) -> Self {
        Self { x, y }
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.y.cmp(&other.y).then(self.x.cmp(&other.x)))
    }
}

impl Ord for Position {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
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
    hp: i32,
    attack: i32,
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
            EntityType::Elf => |e: &&Entity| e.side == EntityType::Goblin && e.hp > 0,
            EntityType::Goblin => |e: &&Entity| e.side == EntityType::Elf && e.hp > 0,
        };

        entities.iter().filter(filter).collect::<Vec<_>>()
    }

    fn locate_target(&self, reach_tree: &ReachTree, targets: &[&Entity], map: &Map) -> Option<Position> {
        // list all targets
        let mut positions = targets
            .iter()
            .flat_map(|e| find_pos_in_range(e.pos.x, e.pos.y, map))
            .map(Position::from_tuple)
            .collect::<Vec<_>>();
        positions.sort_unstable();
        positions.dedup();

        // remove unreachable
        positions.retain(|p| reach_tree.tree.contains_key(p));

        // find closest
        let min = positions.iter().map(|pos| reach_tree.tree.get(pos).unwrap()).min();

        let min = min?;

        let mut closest = Vec::new();
        for pos in &positions {
            let pos_cost = reach_tree.tree.get(&pos);
            if let Some(cost) = pos_cost {
                if cost == min {
                    closest.push(pos);
                }
            }
        }

        if closest.is_empty() {
            return None;
        }

        closest.sort();

        Some(*closest[0])
    }

    fn analyze_turn(&self, entities: &[Entity], map: &Map) -> TurnAction {
        let targets = self.list_targets(entities);

        let reach_tree = ReachTree::build_from_pos(&self.pos, entities, map);

        let target = self.locate_target(&reach_tree, &targets, map);

        if target.is_none() {
            return TurnAction::Idle;
        }
        let target = target.unwrap();

        let close = find_pos_in_range(self.pos.x, self.pos.y, map);

        let mut min_hp = i32::MAX;
        let mut min_pos = None;

        for c in &close {
            let pos = Position::from_tuple(*c);
            for entity in &targets {
                if entity.pos == pos && entity.hp > 0 {
                    if min_hp > entity.hp {
                        min_hp = entity.hp;
                        min_pos = Some(pos);
                    }
                }
            }
        }

        if let Some(pos) = min_pos {
            return TurnAction::AttackOn(pos);
        }

        let distance_tree = ReachTree::build_from_pos(&target, entities, map);

        let mut closest = i32::MAX;
        let mut closest_tile = None;

        for tile in close {
            let tile_pos = Position::from_tuple(tile);

            if reach_tree.tree.contains_key(&tile_pos) {
                let cost = distance_tree.tree.get(&tile_pos);
                if let Some(cost) = cost {
                    if *cost < closest {
                        closest = *cost;
                        closest_tile = Some(tile);
                    }
                }
            }
        }

        match closest_tile {
            Some(pos) => {
                let close = find_pos_in_range(pos.0, pos.1, map);

                let mut min_hp = i32::MAX;
                let mut min_pos = None;

                for c in &close {
                    let pos = Position::from_tuple(*c);
                    for entity in &targets {
                        if entity.pos == pos && entity.hp > 0 {
                            if min_hp > entity.hp {
                                min_hp = entity.hp;
                                min_pos = Some(pos);
                            }
                        }
                    }
                }

                if let Some(att_pos) = min_pos {
                    return TurnAction::MoveAndAttack(Position::from_tuple(pos), att_pos);
                }

                TurnAction::MoveTo(Position::from_tuple(pos))
            }
            None => TurnAction::Idle,
        }
    }
}

#[derive(Debug)]
enum TurnAction {
    Idle,
    MoveTo(Position),
    AttackOn(Position),
    MoveAndAttack(Position, Position),
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

        (Self { inner, size_x, size_y }, entities)
    }

    #[allow(dead_code)]
    fn render_with_entities(&self, entities: &[Entity]) {
        let mut index;
        for y in 0..self.size_y {
            for x in 0..self.size_x {
                index = y * self.size_x + x;

                match self.inner[index] {
                    Tile::Wall => print!("#"),
                    Tile::Floor => {
                        let entity = entities
                            .iter()
                            .find(|e| e.pos == Position::from_tuple((x, y)) && e.hp > 0);

                        match entity {
                            Some(e) => match e.side {
                                EntityType::Goblin => print!("G"),
                                EntityType::Elf => print!("E"),
                            },
                            None => print!(" "),
                        }
                    }
                }
            }

            // list entity HP
            for x in 0..self.size_x {
                let entity = entities
                    .iter()
                    .find(|e| e.pos == Position::from_tuple((x, y)) && e.hp > 0);

                match entity {
                    Some(e) => match e.side {
                        EntityType::Goblin => print!("   G({})", e.hp),
                        EntityType::Elf => print!("   E({})", e.hp),
                    },
                    None => {}
                }
            }

            println!();
        }
    }
}

fn find_pos_in_range(x: usize, y: usize, map: &Map) -> Vec<(usize, usize)> {
    let mut output = Vec::with_capacity(4);

    let mut push_if_floor = |pos: (usize, usize)| {
        let index = pos.0 + pos.1 * map.size_x;
        if let Tile::Floor = map.inner[index] {
            output.push(pos);
        }
    };

    if y > 0 {
        push_if_floor((x, y - 1));
    }
    if x > 0 {
        push_if_floor((x - 1, y));
    }
    if x < map.size_x - 1 {
        push_if_floor((x + 1, y));
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
                    && !entities.iter().any(|e| e.pos.x == n.0 && e.pos.y == n.1 && e.hp > 0)
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

fn simulate_combat(map: &Map, entities: &mut Vec<Entity>) -> (EntityType, i32) {
    let mut round = 0;

    let winner = 'turn: loop {
        for cur_entity in 0..entities.len() {
            let action = {
                let entity = &entities[cur_entity];

                if entity.hp <= 0 {
                    continue;
                }

                let targets = entity.list_targets(&entities);
                if targets.is_empty() {
                    break 'turn entity.side;
                }

                entity.analyze_turn(&entities, &map)
            };

            match action {
                TurnAction::AttackOn(pos) => {
                    let attack_power = {
                        let entity = &entities[cur_entity];
                        entity.attack
                    };

                    let target = entities.iter_mut().find(|e| e.pos == pos && e.hp > 0).unwrap();
                    target.hp = (target.hp - attack_power).max(0);
                }
                TurnAction::Idle => {}
                TurnAction::MoveTo(pos) => {
                    let mut entity = &mut entities[cur_entity];
                    entity.pos = pos;
                }
                TurnAction::MoveAndAttack(move_pos, attack_pos) => {
                    let mut entity = &mut entities[cur_entity];
                    entity.pos = move_pos;

                    let attack_power = entity.attack;

                    let target = entities.iter_mut().find(|e| e.pos == attack_pos && e.hp > 0).unwrap();
                    target.hp = (target.hp - attack_power).max(0);
                }
            }
        }

        entities.retain(|e| e.hp > 0);

        entities.sort_by(|a, b| a.pos.cmp(&b.pos));
        round += 1;
    };

    entities.retain(|e| e.hp > 0);

    (winner, round)
}

pub fn solve() {
    let (map, mut entities) = Map::from_str_with_entities(INPUT);

    let (winner, rounds) = simulate_combat(&map, &mut entities);

    let total_hp = entities.iter().fold(0, |total, entity| total + entity.hp);

    println!("Combat ends after {} full rounds", rounds);
    match winner {
        EntityType::Goblin => print!("Goblins "),
        EntityType::Elf => print!("Elves "),
    }
    println!("win with {} total hit points left", total_hp);

    println!("Outcome: {} * {} = {}", rounds, total_hp, rounds * total_hp);
}

pub fn solve_extra() {
    let (_map, entities) = Map::from_str_with_entities(INPUT);

    let elves = entities.iter().filter(|e| e.side == EntityType::Elf).count();
    let mut attack = 3;

    let (winner, rounds, entities) = 'iteration: loop {
        let (map, mut entities) = Map::from_str_with_entities(INPUT);

        for entity in &mut entities {
            if entity.side == EntityType::Elf {
                entity.attack = attack;
            }
        }

        let (winner, rounds) = simulate_combat(&map, &mut entities);

        if winner != EntityType::Elf || entities.len() != elves {
            attack += 1;
        } else {
            break 'iteration (winner, rounds, entities);
        }
    };

    let total_hp = entities.iter().fold(0, |total, entity| total + entity.hp);

    println!("Combat ends after {} full rounds", rounds);
    match winner {
        EntityType::Goblin => print!("Goblins "),
        EntityType::Elf => print!("Elves "),
    }
    println!("win with {} total hit points left", total_hp);

    println!("Outcome: {} * {} = {}", rounds, total_hp, rounds * total_hp);
}
