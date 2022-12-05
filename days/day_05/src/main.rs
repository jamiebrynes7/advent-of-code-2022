use std::str::FromStr;

use once_cell::sync::Lazy;
use regex::Regex;
use utils::parse_lines;

fn main() {
    let stacks = include_str!("./stacks.txt");
    let instructions = include_str!("./instructions.txt");

    let stacks = Stacks::from_str(stacks).unwrap();
    let instructions = parse_lines(instructions).unwrap();

    part1(stacks.clone(), &instructions);
    part2(stacks, &instructions);
}

fn part1(mut stacks: Stacks, insts: &[Instruction]) {
    for inst in insts {
        stacks.apply(inst);
    }

    println!("Part 1 result: {}", stacks.arrangement());
}

fn part2(mut stacks: Stacks, insts: &[Instruction]) {
    for inst in insts {
        stacks.apply_batched(inst);
    }

    println!("Part 2 result: {}", stacks.arrangement());
}

#[derive(Debug, Clone)]
struct Stacks {
    stacks: Vec<Vec<char>>,
}

impl Stacks {
    pub fn apply(&mut self, inst: &Instruction) {
        for _ in 0..inst.count {
            let elem = self.stacks[inst.source - 1].pop().unwrap();
            self.stacks[inst.target - 1].push(elem);
        }
    }

    pub fn apply_batched(&mut self, inst: &Instruction) {
        let point = self.stacks[inst.source - 1].len() - inst.count;
        let mut to_move = self.stacks[inst.source - 1].split_off(point);
        self.stacks[inst.target - 1].append(&mut to_move);
    }

    pub fn arrangement(&self) -> String {
        self.stacks.iter().map(|s| s.last().unwrap()).collect()
    }
}

impl FromStr for Stacks {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.split("\n").filter(|s| !s.is_empty()).collect::<Vec<_>>();
        let num_stacks = lines.last().unwrap().split_whitespace().count();

        let mut stacks = (0..num_stacks).map(|_| vec![]).collect::<Vec<_>>();

        for line in &lines[0..(lines.len() - 1)] {
            let chars = line.chars().collect::<Vec<_>>();
            for i in 0..num_stacks {
                let idx = 1 + 4 * i;

                match &chars[idx] {
                    'A'..='Z' => (&mut stacks[i]).push(chars[idx].clone()),
                    _ => continue,
                }
            }
        }

        for stack in &mut stacks {
            stack.reverse();
        }

        Ok(Stacks { stacks })
    }
}

#[derive(Debug)]
struct Instruction {
    count: usize,
    source: usize,
    target: usize,
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        static INST_RE: Lazy<Regex> =
            Lazy::new(|| Regex::new("move ([0-9]*) from ([0-9]*) to ([0-9]*)").unwrap());

        let captures = INST_RE.captures(s).expect("Did not match regex");

        let count = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let source = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();
        let target = captures.get(3).unwrap().as_str().parse::<usize>().unwrap();

        Ok(Instruction {
            count,
            source,
            target,
        })
    }
}
