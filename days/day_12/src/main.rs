use std::{collections::HashSet, str::FromStr};

fn main() {
    let input = include_str!("./data.txt");
    let heightmap = Heightmap::from_str(input).unwrap();

    part1(&heightmap);
    part2(&heightmap);
}

fn part1(hm: &Heightmap) {
    println!("Part 1 result {}", hm.shortest_path(hm.start));
}

fn part2(hm: &Heightmap) {
    let candidates = hm
        .heights
        .iter()
        .enumerate()
        .flat_map(|(y, v)| {
            v.iter()
                .enumerate()
                .filter(|(_, val)| **val == 1)
                .map(move |(x, _)| (x, y))
        })
        .collect::<Vec<_>>();

    let min = candidates
        .into_iter()
        .map(|c| hm.shortest_path(c))
        .min()
        .unwrap();

    println!("Part 2 result: {}", min);
}

type Coords = (usize, usize);

#[derive(Debug)]
struct Heightmap {
    heights: Vec<Vec<usize>>,
    start: Coords,
    end: Coords,

    height: usize,
    width: usize,
}

impl Heightmap {
    pub fn shortest_path(&self, start: Coords) -> usize {
        let mut visited = HashSet::new();
        let mut to_visit = Vec::new();

        to_visit.push((start, 0));
        visited.insert(start);

        while !to_visit.is_empty() {
            let targets = to_visit.drain(0..).collect::<Vec<_>>();

            for (coords, dist) in targets {
                let val = self.val(coords);

                if coords == self.end {
                    return dist;
                }

                let neighbors = self
                    .neighbor_coords(coords)
                    .into_iter()
                    .filter(|c| !visited.contains(c))
                    .filter(|c| self.val(*c) <= val + 1)
                    .collect::<Vec<_>>();

                for n in neighbors {
                    visited.insert(n);
                    to_visit.push((n, dist + 1));
                }
            }
        }

        usize::MAX
    }

    fn val(&self, coords: Coords) -> usize {
        self.heights[coords.1][coords.0]
    }

    fn neighbor_coords(&self, target: Coords) -> Vec<Coords> {
        let mut res = vec![];

        if target.0 != 0 {
            res.push((target.0 - 1, target.1));
        }

        if target.0 != self.width - 1 {
            res.push((target.0 + 1, target.1));
        }

        if target.1 != 0 {
            res.push((target.0, target.1 - 1));
        }

        if target.1 != self.height - 1 {
            res.push((target.0, target.1 + 1));
        }

        res
    }
}

impl FromStr for Heightmap {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.split("\n").collect();

        let mut start = None;
        let mut end = None;

        let heights: Vec<Vec<usize>> = lines
            .iter()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| match c {
                        'a'..='z' => 26 - (122 - c as usize),
                        'S' => {
                            start = Some((x, y));
                            1
                        }
                        'E' => {
                            end = Some((x, y));
                            26
                        }
                        _ => panic!("Unknown char: {c}"),
                    })
                    .collect()
            })
            .collect();

        Ok(Heightmap {
            height: heights.len(),
            width: heights[0].len(),
            heights,
            start: start.unwrap(),
            end: end.unwrap(),
        })
    }
}
