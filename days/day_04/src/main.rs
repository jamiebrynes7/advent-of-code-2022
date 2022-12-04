use std::str::FromStr;

fn main() {
    let input = include_str!("./data.txt");
    let pairs = input
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(Pair::from_str)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    part1(&pairs);
    part2(&pairs);
}

fn part1(pairs: &[Pair]) {
    let count = pairs.iter().filter(|p| p.is_fully_overlapped()).count();
    println!("Part 1 result: {count}");
}

fn part2(pairs: &[Pair]) {
    let count = pairs.iter().filter(|p| p.overlaps()).count();
    println!("Part 2 result: {count}");
}

struct Pair {
    first: (i32, i32),
    second: (i32, i32),
}

impl Pair {
    pub fn is_fully_overlapped(&self) -> bool {
        (self.first.0 <= self.second.0 && self.first.1 >= self.second.1)
            || (self.second.0 <= self.first.0 && self.second.1 >= self.first.1)
    }

    pub fn overlaps(&self) -> bool {
        (self.first.0 <= self.second.0 && self.second.0 <= self.first.1)
            || (self.second.0 <= self.first.0 && self.first.0 <= self.second.1)
    }
}

impl FromStr for Pair {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(",");

        let first = parts.next().unwrap();
        let second = parts.next().unwrap();

        let mut first_pair = first.split("-");
        let mut second_pair = second.split("-");

        Ok(Pair {
            first: (
                first_pair.next().unwrap().parse().unwrap(),
                first_pair.next().unwrap().parse().unwrap(),
            ),
            second: (
                second_pair.next().unwrap().parse().unwrap(),
                second_pair.next().unwrap().parse().unwrap(),
            ),
        })
    }
}
