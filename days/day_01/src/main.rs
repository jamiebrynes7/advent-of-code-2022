fn main() {
    let input = include_str!("./data.txt");
    let lines = input.split("\n").collect::<Vec<_>>();
    let cals = count_calories(lines);

    part1(&cals);
    part2(&cals);
}

fn part1(cals: &[u32]) {
    let max = cals.iter().max().unwrap();
    println!("Part 1 answer is {max}");
}

fn part2(cals: &[u32]) {
    let mut sorted = cals.iter().cloned().collect::<Vec<u32>>();
    sorted.sort();
    sorted.reverse();
    let total = &sorted[0..=2].iter().sum::<u32>();

    println!("Part 2 answer is {total}");
}

fn count_calories(lines: Vec<&str>) -> Vec<u32> {
    let mut calories = vec![];

    let mut current = 0;

    for line in lines {
        if line.is_empty() {
            calories.push(current);
            current = 0;
            continue;
        }

        current += line.parse::<u32>().unwrap();
    }

    calories.push(current);

    calories
}
