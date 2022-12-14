use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use utils::parse_lines;

fn main() {
    let input = include_str!("./data.txt");
    let scans: Vec<Scan> = parse_lines(input).unwrap();
    let cave = Cave::from_scans(scans);

    part1(cave.clone());
    part2(cave);
}

fn part1(mut cave: Cave) {
    let units = {
        let mut units = 0;

        loop {
            if let Err(_) = cave.drop_sand() {
                break;
            }

            units += 1;
        }

        units
    };

    println!("Part 1 result: {units}");
}

fn part2(mut cave: Cave) {
    let units = {
        let mut units = 0;

        loop {
            units += 1;

            if cave.drop_sand_with_floor() == (500, 0) {
                break;
            }
        }

        units
    };

    println!("Part 2 result: {units}");
}

type Coords = (usize, usize);

#[derive(Debug)]
struct Scan {
    parts: Vec<Coords>,
}

impl FromStr for Scan {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s
            .split(" -> ")
            .map(|coords| {
                let mut coords_parts = coords.split(",");

                let x = coords_parts.next().unwrap().parse::<usize>().unwrap();
                let y = coords_parts.next().unwrap().parse::<usize>().unwrap();

                (x, y)
            })
            .collect();

        Ok(Scan { parts })
    }
}

#[derive(Debug, Clone)]
struct Cave {
    occupied: HashSet<Coords>,
    abyss_limit: usize,
    floor: usize,
}

impl Cave {
    pub fn from_scans(scans: Vec<Scan>) -> Self {
        let mut occupied = HashSet::new();
        let mut abyss_limit = 0;

        for scan in scans {
            for pair in scan.parts.windows(2) {
                let start = pair[0];
                let end = pair[1];

                let x_range = if end.0 > start.0 {
                    start.0..=end.0
                } else {
                    end.0..=start.0
                };

                let y_range = if end.1 > start.1 {
                    start.1..=end.1
                } else {
                    end.1..=start.1
                };

                for x in x_range {
                    for y in y_range.clone() {
                        occupied.insert((x, y));

                        if y > abyss_limit {
                            abyss_limit = y;
                        }
                    }
                }
            }
        }

        Cave {
            occupied,
            abyss_limit,
            floor: abyss_limit + 2,
        }
    }

    pub fn drop_sand(&mut self) -> Result<(), ()> {
        let mut position = (500usize, 0usize);

        loop {
            let downward = (position.0, position.1 + 1);

            if downward.1 > self.abyss_limit {
                return Err(());
            }

            if !self.occupied.contains(&downward) {
                position = downward;
                continue;
            }

            let diag_left = (position.0 - 1, position.1 + 1);
            if !self.occupied.contains(&diag_left) {
                position = diag_left;
                continue;
            }

            let diag_right = (position.0 + 1, position.1 + 1);
            if !self.occupied.contains(&diag_right) {
                position = diag_right;
                continue;
            }

            // Nowhere to go, stop here.
            self.occupied.insert(position);
            return Ok(());
        }
    }

    pub fn drop_sand_with_floor(&mut self) -> Coords {
        let mut position = (500usize, 0usize);

        loop {
            let downward = (position.0, position.1 + 1);

            if downward.1 == self.floor {
                self.occupied.insert(position);
                return position;
            }

            if !self.occupied.contains(&downward) {
                position = downward;
                continue;
            }

            let diag_left = (position.0 - 1, position.1 + 1);
            if !self.occupied.contains(&diag_left) {
                position = diag_left;
                continue;
            }

            let diag_right = (position.0 + 1, position.1 + 1);
            if !self.occupied.contains(&diag_right) {
                position = diag_right;
                continue;
            }

            // Nowhere to go, stop here.
            self.occupied.insert(position);
            return position;
        }
    }
}
