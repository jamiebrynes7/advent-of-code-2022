use std::{collections::HashSet, str::FromStr};

use utils::parse_lines;

fn main() {
    let input = include_str!("./data.txt");
    let directions: Vec<Movement> = parse_lines(input).unwrap();

    part1(&directions);
    part2(&directions);
}

type Coords = (i32, i32);

fn part1(movements: &Vec<Movement>) {
    let mut visited: HashSet<Coords> = HashSet::new();

    let mut rope = Rope::new(2);

    visited.insert(rope.tail_pos());

    for movement in movements {
        for _ in 0..movement.magnitude {
            rope.apply(movement.direction);
            visited.insert(rope.tail_pos());
        }
    }

    println!("Part 1 result: {}", visited.len());
}

fn part2(movements: &Vec<Movement>) {
    let mut visited: HashSet<Coords> = HashSet::new();

    let mut rope = Rope::new(10);

    visited.insert(rope.tail_pos());

    for movement in movements {
        for _ in 0..movement.magnitude {
            rope.apply(movement.direction);
            visited.insert(rope.tail_pos());
        }
    }

    println!("Part 2 result: {}", visited.len());
}

#[derive(Debug)]
struct Rope {
    elements: Vec<Coords>,
}

impl Rope {
    pub fn new(size: usize) -> Rope {
        Rope {
            elements: (0..size).map(|_| (0, 0)).collect(),
        }
    }

    pub fn apply(&mut self, dir: Direction) {
        let head_pos = &mut self.elements[0];

        match dir {
            Direction::Left => head_pos.0 -= 1,
            Direction::Right => head_pos.0 += 1,
            Direction::Up => head_pos.1 += 1,
            Direction::Down => head_pos.1 -= 1,
        }

        for idx in 0..self.elements.len() - 1 {
            let leader = self.elements[idx];
            let mut follower = &mut self.elements[idx + 1];

            let x_diff: i32 = leader.0 - follower.0;
            let y_diff: i32 = leader.1 - follower.1;

            if x_diff.abs() == 2 {
                follower.0 += x_diff / 2;
                if y_diff.abs() == 1 {
                    follower.1 += y_diff;
                } else if y_diff.abs() == 2 {
                    follower.1 += y_diff / 2;
                }
            } else if y_diff.abs() == 2 {
                follower.1 += y_diff / 2;
                if x_diff.abs() == 1 {
                    follower.0 += x_diff;
                } else if x_diff.abs() == 2 {
                    follower.0 += x_diff / 2;
                }
            }
        }
    }

    pub fn tail_pos(&self) -> Coords {
        self.elements.last().unwrap().clone()
    }
}

struct Movement {
    direction: Direction,
    magnitude: usize,
}

impl FromStr for Movement {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(" ").collect::<Vec<_>>();

        let direction = match parts[0] {
            "L" => Direction::Left,
            "R" => Direction::Right,
            "U" => Direction::Up,
            "D" => Direction::Down,
            _ => panic!("Unknown direction"),
        };

        let magnitude = parts[1].parse().unwrap();

        Ok(Movement {
            direction,
            magnitude,
        })
    }
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}
