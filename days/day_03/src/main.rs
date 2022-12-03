use std::collections::HashSet;

fn main() {
    let input = include_str!("./data.txt");
    let rucksacks = input
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| Rucksack { data: s.into() })
        .collect::<Vec<_>>();

    part1(&rucksacks);
    part2(&rucksacks);
}

fn part1(rucksacks: &[Rucksack]) {
    let sum: i32 = rucksacks.iter().map(|r| priority(r.common_item())).sum();
    println!("Part 1 result: {sum}");
}

fn part2(rucksacks: &[Rucksack]) {
    let sum: i32 = rucksacks
        .chunks(3)
        .map(|chunk| Rucksack::badge(chunk))
        .map(priority)
        .sum();

    println!("Result 2 result: {sum}");
}

struct Rucksack {
    data: String,
}

impl Rucksack {
    fn common_item(&self) -> char {
        let half_length = self.data.len() / 2;

        let first_half = self.data[0..half_length].chars().collect::<HashSet<_>>();
        let second_half = self.data[half_length..].chars().collect::<HashSet<_>>();

        let common = first_half.intersection(&second_half).collect::<Vec<_>>();

        assert_eq!(common.len(), 1);

        common[0].clone()
    }

    fn badge(elves: &[Rucksack]) -> char {
        let common = elves
            .iter()
            .map(|r| r.data.chars().collect::<HashSet<_>>())
            .reduce(|acc, item| acc.intersection(&item).cloned().collect::<HashSet<_>>())
            .expect("No rucksacks provided");

        assert_eq!(common.len(), 1);

        common.iter().next().unwrap().clone()
    }
}

fn priority(c: char) -> i32 {
    let is_upper = c.is_uppercase();
    // lowercase z is 122, so find the negative offset and add it to 26.
    let val = 26 + (c.to_ascii_lowercase() as i32 - 122);

    if is_upper {
        val + 26
    } else {
        val
    }
}

#[cfg(test)]
mod test {
    use crate::priority;

    #[test]
    fn test_priority() {
        assert_eq!(priority('a'), 1);
        assert_eq!(priority('z'), 26);
        assert_eq!(priority('A'), 27);
        assert_eq!(priority('Z'), 52);
    }
}
