use std::sync::mpsc::{channel, Receiver, Sender};

fn main() {
    part1();
    part2();
}

fn part1() {
    let mut monkeys = setup_monkeys();
    let channels = generate_channels();

    for _ in 0..20 {
        do_round(&mut monkeys, &channels)
    }

    let mut inspections: Vec<usize> = monkeys.iter().map(|m| m.inspections).collect();
    inspections.sort();
    inspections.reverse();

    let answer: usize = inspections[0..=1].iter().product();

    println!("Part 1 result: {answer}")
}

fn part2() {
    let mut monkeys = setup_monkeys();
    let channels = generate_channels();

    for _ in 0..10000 {
        do_round(&mut monkeys, &channels)
    }

    let mut inspections: Vec<usize> = monkeys.iter().map(|m| m.inspections).collect();
    inspections.sort();
    inspections.reverse();

    let answer: usize = inspections[0..=1].iter().product();

    println!("Part 2 result: {answer}")
}

fn do_round(monkeys: &mut Vec<Monkey>, channels: &Vec<(Sender<i64>, Receiver<i64>)>) {
    for (idx, monkey) in monkeys.iter_mut().enumerate() {
        let recv = &channels[idx].1;

        while let Some((target_idx, item)) = monkey.inspect(recv) {
            channels[target_idx].0.send(item).unwrap();
        }
    }
}

struct Monkey {
    operation: Box<dyn Fn(i64) -> i64>,
    test: Box<dyn Fn(i64) -> bool>,
    iftrue: usize,
    iffalse: usize,

    inspections: usize,
}

impl Monkey {
    pub fn inspect(&mut self, rx: &Receiver<i64>) -> Option<(usize, i64)> {
        let mut item = match rx.try_recv() {
            Ok(i) => (self.operation)(i),
            Err(_) => return None,
        };

        // Uncomment below for part 1 to be correct.
        // item /= 3;
        item = item % KINDA_LCM;
        self.inspections += 1;

        if (self.test)(item) {
            Some((self.iftrue, item))
        } else {
            Some((self.iffalse, item))
        }
    }
}

fn generate_channels() -> Vec<(Sender<i64>, Receiver<i64>)> {
    let starting_values = vec![
        vec![59, 74, 65, 86],
        vec![62, 84, 72, 91, 68, 78, 51],
        vec![78, 84, 96],
        vec![97, 86],
        vec![50],
        vec![73, 65, 69, 65, 51],
        vec![69, 82, 97, 93, 82, 84, 58, 63],
        vec![81, 78, 82, 76, 79, 80],
    ];

    starting_values
        .into_iter()
        .map(|vals| {
            let (tx, rx) = channel();

            for val in vals {
                tx.send(val).unwrap();
            }

            (tx, rx)
        })
        .collect()
}

const KINDA_LCM: i64 = 7 * 2 * 19 * 3 * 13 * 11 * 5 * 17;

fn setup_monkeys() -> Vec<Monkey> {
    vec![
        Monkey {
            operation: Box::new(|old| old * 19),
            test: Box::new(|val| val % 7 == 0),
            iftrue: 6,
            iffalse: 2,
            inspections: 0,
        },
        Monkey {
            operation: Box::new(|old| old + 1),
            test: Box::new(|val| val % 2 == 0),
            iftrue: 2,
            iffalse: 0,
            inspections: 0,
        },
        Monkey {
            operation: Box::new(|old| old + 8),
            test: Box::new(|val| val % 19 == 0),
            iftrue: 6,
            iffalse: 5,
            inspections: 0,
        },
        Monkey {
            operation: Box::new(|old| old * old),
            test: Box::new(|val| val % 3 == 0),
            iftrue: 1,
            iffalse: 0,
            inspections: 0,
        },
        Monkey {
            operation: Box::new(|old| old + 6),
            test: Box::new(|val| val % 13 == 0),
            iftrue: 3,
            iffalse: 1,
            inspections: 0,
        },
        Monkey {
            operation: Box::new(|old| old * 17),
            test: Box::new(|val| val % 11 == 0),
            iftrue: 4,
            iffalse: 7,
            inspections: 0,
        },
        Monkey {
            operation: Box::new(|old| old + 5),
            test: Box::new(|val| val % 5 == 0),
            iftrue: 5,
            iffalse: 7,
            inspections: 0,
        },
        Monkey {
            operation: Box::new(|old| old + 3),
            test: Box::new(|val| val % 17 == 0),
            iftrue: 3,
            iffalse: 4,
            inspections: 0,
        },
    ]
}
