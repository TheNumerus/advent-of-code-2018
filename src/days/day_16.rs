use std::collections::{HashMap, HashSet};

use nom::{
    bytes::complete::tag,
    character::complete::{char, digit1, space1},
    combinator::map,
    multi::many1,
    sequence::tuple,
    IResult,
};

const INPUT: &str = include_str!("../inputs/day_16_input");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct CPUState {
    reg: [i32; 4],
}

impl CPUState {
    fn parse(i: &str) -> IResult<&str, Self> {
        let (i, _) = char('[')(i)?;
        let (i, r0) = digit1(i)?;
        let (i, _) = tag(", ")(i)?;
        let (i, r1) = digit1(i)?;
        let (i, _) = tag(", ")(i)?;
        let (i, r2) = digit1(i)?;
        let (i, _) = tag(", ")(i)?;
        let (i, r3) = digit1(i)?;
        let (i, _) = char(']')(i)?;

        Ok((
            (i),
            Self {
                reg: [
                    r0.parse().unwrap(),
                    r1.parse().unwrap(),
                    r2.parse().unwrap(),
                    r3.parse().unwrap(),
                ],
            },
        ))
    }

    fn execute_instr(&mut self, instr: &Instruction) {
        let a = instr.a as usize;
        let b = instr.b as usize;
        let c = instr.c as usize;

        match instr.opcode {
            Opcode::Addr => self.reg[c] = self.reg[a] + self.reg[b],
            Opcode::Addi => self.reg[c] = self.reg[a] + instr.b,
            Opcode::Mulr => self.reg[c] = self.reg[a] * self.reg[b],
            Opcode::Muli => self.reg[c] = self.reg[a] * instr.b,
            Opcode::Banr => self.reg[c] = self.reg[a] & self.reg[b],
            Opcode::Bani => self.reg[c] = self.reg[a] & instr.b,
            Opcode::Borr => self.reg[c] = self.reg[a] | self.reg[b],
            Opcode::Bori => self.reg[c] = self.reg[a] | instr.b,
            Opcode::Setr => self.reg[c] = self.reg[a],
            Opcode::Seti => self.reg[c] = instr.a,
            Opcode::Gtir => self.reg[c] = (instr.a > self.reg[b]) as i32,
            Opcode::Gtri => self.reg[c] = (self.reg[a] > instr.b) as i32,
            Opcode::Gtrr => self.reg[c] = (self.reg[a] > self.reg[b]) as i32,
            Opcode::Eqir => self.reg[c] = (instr.a == self.reg[b]) as i32,
            Opcode::Eqri => self.reg[c] = (self.reg[a] == instr.b) as i32,
            Opcode::Eqrr => self.reg[c] = (self.reg[a] == self.reg[b]) as i32,
            Opcode::Unknown(_) => panic!("Executing unknown opcode"),
        }
    }

    fn new() -> Self {
        Self { reg: [0, 0, 0, 0] }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Opcode {
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr,
    Unknown(u8),
}

impl Opcode {
    const LIST: [Self; 16] = {
        use Opcode::*;

        [
            Addr, Addi, Mulr, Muli, Banr, Bani, Borr, Bori, Setr, Seti, Gtir, Gtri, Gtrr, Eqir, Eqri, Eqrr,
        ]
    };

    fn num(&self) -> u8 {
        if let Opcode::Unknown(i) = self {
            return *i;
        }
        panic!("Num of invalid variant");
    }
}

#[derive(Debug, Clone, Copy)]
struct Instruction {
    opcode: Opcode,
    a: i32,
    b: i32,
    c: i32,
}

impl Instruction {
    fn parse(i: &str) -> IResult<&str, Self> {
        let (i, opcode) = digit1(i)?;
        let (i, _) = space1(i)?;
        let (i, a) = digit1(i)?;
        let (i, _) = space1(i)?;
        let (i, b) = digit1(i)?;
        let (i, _) = space1(i)?;
        let (i, c) = digit1(i)?;

        Ok((
            (i),
            Self {
                opcode: Opcode::Unknown(opcode.parse().unwrap()),
                a: a.parse().unwrap(),
                b: b.parse().unwrap(),
                c: c.parse().unwrap(),
            },
        ))
    }
}

#[derive(Debug)]
struct Sample {
    before: CPUState,
    instr: Instruction,
    after: CPUState,
}

impl Sample {
    fn parse(i: &str) -> IResult<&str, Self> {
        let (i, _) = tag("Before: ")(i)?;

        let (i, before) = CPUState::parse(i)?;

        let (i, _) = char('\n')(i)?;

        let (i, instr) = Instruction::parse(i)?;

        let (i, _) = tag("\nAfter:  ")(i)?;

        let (i, after) = CPUState::parse(i)?;

        let (i, _) = char('\n')(i)?;

        Ok(((i), Self { before, instr, after }))
    }
}

fn parse_samples(i: &str) -> Vec<Sample> {
    many1(map(tuple((Sample::parse, tag("\n"))), |(s, _)| s))(i).unwrap().1
}

fn parse_samples_and_program(i: &str) -> (Vec<Sample>, Vec<Instruction>) {
    let (i, samples) = many1(map(tuple((Sample::parse, tag("\n"))), |(s, _)| s))(i).unwrap();

    let (i, _) = tag::<&str, &str, nom::error::Error<&str>>("\n\n")(i).unwrap();

    let (_, program) = many1(map(tuple((Instruction::parse, tag("\n"))), |(s, _)| s))(i).unwrap();

    (samples, program)
}

fn print_instr_table(table: &HashSet<(Opcode, u8)>) {
    let mut buf = String::with_capacity(17 * 80);

    buf += "     ";

    for num in 0..16 {
        buf.push_str(&format!("{:3}", num));
    }
    buf += "\n";
    for opcode in Opcode::LIST {
        buf.push_str(&format!("{:?} ", opcode));

        for num in 0..16 {
            let val = table.contains(&(opcode, num));

            match val {
                true => buf += "  #",
                false => buf += "   ",
            }
        }

        buf += "\n";
    }

    println!("{}", buf);
}

fn filter_instr_table(table: &mut HashSet<(Opcode, u8)>) {
    // filter candidates with only one number
    let mut filtered = [false; 16];
    loop {
        for num in 0..16 {
            if filtered[num as usize] {
                continue;
            }
            let mut count = 0;
            let mut last_opcode = None;

            for opcode in Opcode::LIST {
                if table.contains(&(opcode, num)) {
                    count += 1;
                    last_opcode = Some(opcode);
                }
            }

            if count == 1 {
                for x in 0..16 {
                    if x != num {
                        table.remove(&(last_opcode.unwrap(), x));
                        filtered[num as usize] = true;
                    }
                }
            }
        }
        if filtered.iter().all(|f| *f) {
            break;
        }
    }
}

pub fn solve() {
    let samples = parse_samples(INPUT);

    let mut ambiguous_count = 0;

    for sample in &samples {
        let mut correct = 0;

        for opcode in Opcode::LIST {
            let mut state = sample.before;

            let instr = Instruction { opcode, ..sample.instr };

            state.execute_instr(&instr);

            if state == sample.after {
                correct += 1;
            }
        }

        if correct >= 3 {
            ambiguous_count += 1;
        }
    }

    println!("Ambiguous count: {}", ambiguous_count);
}

pub fn solve_extra() {
    let (samples, program) = parse_samples_and_program(INPUT);

    let mut table = HashSet::new();

    for opcode in Opcode::LIST {
        for x in 0..16 {
            table.insert((opcode, x));
        }
    }

    // sort
    for sample in &samples {
        for opcode in Opcode::LIST {
            let mut state = sample.before;

            let instr = Instruction { opcode, ..sample.instr };

            state.execute_instr(&instr);

            if state != sample.after {
                table.remove(&(opcode, sample.instr.opcode.num()));
            }
        }
    }

    filter_instr_table(&mut table);

    print_instr_table(&table);

    let mut mapping: HashMap<u8, Opcode> = HashMap::with_capacity(16);
    for opcode in Opcode::LIST {
        for x in 0..16 {
            if table.contains(&(opcode, x)) {
                mapping.insert(x, opcode);
            }
        }
    }

    let mut state = CPUState::new();

    for instr in &program {
        let mapped_instr = Instruction {
            opcode: *mapping.get(&instr.opcode.num()).unwrap(),
            ..*instr
        };

        state.execute_instr(&mapped_instr);
    }

    println!(
        "CPU State: [{}, {}, {}, {}]",
        state.reg[0], state.reg[1], state.reg[2], state.reg[3]
    );
}
