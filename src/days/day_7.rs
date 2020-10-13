use std::collections::HashMap;

use nom::IResult;

const INPUT: &str = include_str!("../inputs/day_7_input");

#[derive(Debug)]
struct Step {
    name: char,
    allows: char,
}

impl Step {
    fn parse(i: &str) -> IResult<&str, Self> {
        use nom::bytes::complete::tag;
        use nom::character::complete::anychar;
        use nom::combinator::map;
        use nom::sequence::tuple;
        let (i, step) = map(
            tuple((
                tag("Step "),
                anychar,
                tag(" must be finished before step "),
                anychar,
                tag(" can begin."),
            )),
            |(_, name, _, allows, _): (&str, char, &str, char, &str)| Self { name, allows },
        )(i)?;

        Ok((i, step))
    }
}

#[derive(Debug)]
struct Node {
    name: char,
    used: bool,
    requisites: HashMap<char, bool>,
}

#[derive(Debug)]
struct Worker {
    current_work: Option<char>,
    time_remaining: usize,
}

impl Worker {
    pub fn new() -> Self {
        Worker {
            current_work: None,
            time_remaining: 0,
        }
    }
}

pub fn solve() {
    let steps = INPUT
        .lines()
        .map(|i| Step::parse(i).unwrap().1)
        .collect::<Vec<_>>();

    let mut nodes = generate_nodes(&steps);

    let mut sequence = String::with_capacity(nodes.len());

    while let Some(n) = find_next(&nodes) {
        let next = n.name;
        apply_req(&mut nodes, next);
        sequence.push(next);
    }

    println!("Graph sequence is \"{}\"", sequence);
}

pub fn solve_extra() {
    let steps = INPUT
        .lines()
        .map(|i| Step::parse(i).unwrap().1)
        .collect::<Vec<_>>();

    let mut nodes = generate_nodes(&steps);

    let mut sequence = String::with_capacity(nodes.len());

    const WORKERS: usize = 5;

    let mut workers = Vec::with_capacity(WORKERS);
    workers.resize_with(WORKERS, || Worker::new());
    let mut time = 0;

    loop {
        for worker in &mut workers {
            if let Some(work) = worker.current_work {
                if worker.time_remaining > 1 {
                    // apply time step
                    worker.time_remaining -= 1;
                } else {
                    // work done
                    *worker = Worker::new();
                    apply_req(&mut nodes, work);
                    sequence.push(work);
                }
            }
        }
        // now try assign some work to every free worker
        for worker in workers.iter_mut().filter(|w| w.current_work.is_none()) {
            if let Some(work) = find_next_mut(&mut nodes) {
                work.used = true;
                worker.current_work = Some(work.name);
                worker.time_remaining = char_to_time(work.name);
            }
        }
        if sequence.len() == nodes.len() {
            break;
        }
        time += 1;
    }

    //dbg!(&workers);

    println!("Parallel graph sequence is \"{}\"", sequence);
    println!("Parallel graph took {} seconds", time);
}

// this will horribly break on everything else than 'A'..='Z'
fn char_to_time(c: char) -> usize {
    let mut buf = [0; 4];
    c.encode_utf8(&mut buf);
    (buf[0] - b'A') as usize + 61
}

fn find_next(nodes: &[Node]) -> Option<&Node> {
    let mut possible_nodes = nodes
        .iter()
        .filter(|n| n.requisites.iter().all(|(_, a)| *a) && !n.used)
        .collect::<Vec<_>>();

    if possible_nodes.len() == 0 {
        return None;
    }

    possible_nodes.sort_by(|a, b| a.name.cmp(&b.name));

    Some(possible_nodes[0])
}

fn find_next_mut(nodes: &mut [Node]) -> Option<&mut Node> {
    let mut possible_nodes = nodes
        .iter_mut()
        .filter(|n| n.requisites.iter().all(|(_, a)| *a) && !n.used)
        .collect::<Vec<_>>();

    if possible_nodes.len() == 0 {
        return None;
    }

    possible_nodes.sort_by(|a, b| b.name.cmp(&a.name));

    possible_nodes.pop()
}

fn apply_req(nodes: &mut [Node], finished: char) {
    for node in nodes {
        if node.name == finished {
            node.used = true;
        }
        if let Some(val) = node.requisites.get_mut(&finished) {
            *val = true;
        }
    }
}

fn generate_nodes(steps: &[Step]) -> Vec<Node> {
    let mut requisites = HashMap::new();
    for step in steps {
        let Step { name, allows } = step;
        requisites.entry(*allows).or_insert(Vec::new()).push(*name);
        requisites.entry(*name).or_insert(Vec::new());
    }
    let nodes = requisites
        .iter()
        .map(|(name, req)| Node {
            name: *name,
            used: false,
            requisites: req.iter().map(|a| (*a, false)).collect(),
        })
        .collect();
    nodes
}
