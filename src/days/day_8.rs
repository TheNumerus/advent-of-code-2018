const INPUT: &str = include_str!("../inputs/day_8_input");

#[derive(Debug)]
struct Node {
    child_entries: u8,
    metadata_entries: u8,
    children: Vec<Node>,
    metadata: Vec<u8>,
}

impl Node {
    fn parse(i: &mut impl Iterator<Item = u8>) -> Self {
        let child_entries = i.next().unwrap();
        let metadata_entries = i.next().unwrap();

        let mut children = Vec::with_capacity(child_entries as usize);
        for _ in 0..child_entries {
            children.push(Node::parse(i));
        }
        Self {
            child_entries,
            metadata_entries,
            children,
            metadata: i.take(metadata_entries as usize).collect(),
        }
    }

    fn metadata_sum(&self) -> u32 {
        let mut sum = self.metadata.iter().map(|a| *a as u32).sum();

        for child in &self.children {
            sum += child.metadata_sum();
        }

        sum
    }

    fn adv_sum(&self) -> u32 {
        if self.child_entries == 0 {
            self.metadata.iter().map(|a| *a as u32).sum()
        } else {
            let mut sum = 0;
            for data in &self.metadata {
                if *data != 0 && *data <= self.child_entries {
                    sum += self.children[*data as usize - 1].adv_sum();
                }
            }
            sum
        }
    }
}

pub fn solve() {
    let mut nums = INPUT.split_whitespace().map(|a| a.parse().unwrap());

    let root = Node::parse(&mut nums);

    println!("Metadata sum = {}", root.metadata_sum());
}

pub fn solve_extra() {
    let mut nums = INPUT.split_whitespace().map(|a| a.parse().unwrap());

    let root = Node::parse(&mut nums);

    println!("Advanced sum = {}", root.adv_sum());
}
