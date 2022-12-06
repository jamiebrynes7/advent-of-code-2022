use std::collections::HashSet;

fn main() {
    let input = include_str!("./data.txt");
    part1(input);
    part2(input);
}

fn part1(data: &str) {
    let marker = find_unique_sequence(data, 4);
    println!("Part 1 result: {marker}");
}

fn part2(data: &str) {
    let marker = find_unique_sequence(data, 14);
    println!("Part 2 result: {marker}");
}

fn find_unique_sequence(data: &str, size: usize) -> usize {
    let mut start = 0;
    let mut end = size - 1;

    while end <= data.len() {
        let slice = &data[start..=end];

        if slice.chars().collect::<HashSet<_>>().len() == size {
            return end + 1;
        }

        start += 1;
        end += 1;
    }

    panic!("No start-of-packet marker found");
}
