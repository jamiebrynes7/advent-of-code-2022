use std::cmp::Ordering;

fn main() {
    let input = include_str!("./data.txt");
    let sequences = input
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| Sequence::parse(s))
        .collect::<Vec<_>>();

    part1(sequences.chunks(2).collect());
    part2(sequences);
}

fn part1(pairs: Vec<&[Sequence]>) {
    let sum: usize = pairs
        .iter()
        .enumerate()
        .filter(|(_, pair)| pair[0] < pair[1])
        .map(|(idx, _)| idx + 1)
        .sum();

    println!("Part 1 result: {sum}");
}

fn part2(mut sequences: Vec<Sequence>) {
    let dividers = vec![
        Sequence::List(vec![Sequence::List(vec![Sequence::Val(2)])]),
        Sequence::List(vec![Sequence::List(vec![Sequence::Val(6)])]),
    ];

    sequences.append(&mut dividers.clone());
    sequences.sort();

    let decoder_key: usize = dividers
        .iter()
        .map(|d| sequences.iter().position(|e| e == d).unwrap() + 1)
        .product();

    println!("Part 2 result: {}", decoder_key);
}

#[derive(Debug, Clone, PartialEq, Eq, Ord)]
enum Sequence {
    Val(u32),
    List(Vec<Sequence>),
}

impl Sequence {
    pub fn parse(s: &str) -> Self {
        let mut chars = s.chars();
        if let Sequence::List(mut vals) = Sequence::parse_iter(&mut chars) {
            return vals.remove(0);
        }

        panic!("Broken assumption")
    }

    fn parse_iter<I: Iterator<Item = char>>(iter: &mut I) -> Self {
        let mut elems = vec![];

        let mut num = String::from("");
        while let Some(char) = iter.next() {
            match char {
                '0'..='9' => num.push(char),
                ',' => {
                    if num != "" {
                        elems.push(Sequence::Val(num.parse().unwrap()));
                        num = String::from("");
                    }
                }
                '[' => elems.push(Sequence::parse_iter(iter)),
                ']' => {
                    if num != "" {
                        elems.push(Sequence::Val(num.parse().unwrap()));
                    }
                    return Sequence::List(elems);
                }
                _ => panic!("Unknown char: {}", char),
            }
        }

        Sequence::List(elems)
    }

    pub fn as_list(self) -> Sequence {
        match self {
            Sequence::Val(_) => Sequence::List(vec![self]),
            Sequence::List(_) => self,
        }
    }
}

impl PartialOrd for Sequence {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Sequence::Val(val), Sequence::Val(other_val)) => val.partial_cmp(other_val),
            (Sequence::Val(_), Sequence::List(_)) => self.clone().as_list().partial_cmp(other),
            (Sequence::List(_), Sequence::Val(_)) => self.partial_cmp(&other.clone().as_list()),
            (Sequence::List(vals), Sequence::List(other_vals)) => {
                if other_vals.len() == 0 && vals.len() > 0 {
                    return Some(Ordering::Greater);
                }

                for idx in 0..vals.len() {
                    if idx > other_vals.len() - 1 {
                        return Some(Ordering::Greater);
                    }

                    let val = &vals[idx];
                    let other_val = &other_vals[idx];

                    match val.partial_cmp(other_val).unwrap() {
                        Ordering::Equal => continue,
                        x => return Some(x),
                    }
                }

                if other_vals.len() > vals.len() {
                    return Some(Ordering::Less);
                }

                Some(Ordering::Equal)
            }
        }
    }
}
