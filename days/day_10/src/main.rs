use std::str::FromStr;
use utils::parse_lines;

fn main() {
    let input = include_str!("./data.txt");
    let instructions: Vec<Instruction> = parse_lines(input).unwrap();

    part1(&instructions);
    part2(&instructions);
}

fn part1(insts: &[Instruction]) {
    let mut machine = Machine::new();
    let cycles = machine.apply(insts);

    let sum: i32 = (19..cycles.len())
        .step_by(40)
        .map(|idx| cycles[idx] * (1 + idx) as i32)
        .sum();

    println!("Part 1 result: {sum}");
}

fn part2(insts: &[Instruction]) {
    let mut machine = Machine::new();
    let cycles = machine.apply(insts);

    let mut chars = vec![];

    for y in 0..6 {
        let mut row = vec![];
        for x in 0..40 {
            let cycle_val = cycles[40 * y + x as usize];

            if cycle_val >= x - 1 && cycle_val <= x + 1 {
                row.push('■');
            } else {
                row.push(' ');
            }
        }

        chars.push(row);
    }

    for row in chars {
        println!("{}", row.iter().collect::<String>());
    }
}

struct Machine {
    register: i32,
}

impl Machine {
    pub fn new() -> Machine {
        Machine { register: 1 }
    }

    pub fn apply(&mut self, insts: &[Instruction]) -> Vec<i32> {
        let mut cycles = vec![];

        for inst in insts {
            match inst {
                Instruction::NoOp => cycles.push(self.register),
                Instruction::AddX(val) => {
                    cycles.push(self.register);
                    cycles.push(self.register);
                    self.register += val;
                }
            }
        }

        cycles
    }
}

enum Instruction {
    NoOp,
    AddX(i32),
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(" ").collect::<Vec<_>>();

        let inst = match parts[0] {
            "noop" => Instruction::NoOp,
            "addx" => Instruction::AddX(parts[1].parse().unwrap()),
            _ => panic!("Unknown instruction"),
        };

        Ok(inst)
    }
}
