use std::collections::HashSet;

fn main() {
    let input = include_str!("./data.txt");
    let grid = input
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    part1(&grid);
    part2(&grid);
}

type Coordinates = (usize, usize);

fn part1(grid: &Vec<Vec<u32>>) {
    let mut visible: HashSet<Coordinates> = HashSet::new();

    let height = grid.len();
    let width = grid[0].len();

    // Add all the items on th edge of the grid.
    for x in 0..width {
        visible.insert((x, 0));
        visible.insert((x, height - 1));
    }

    for y in 0..height {
        visible.insert((0, y));
        visible.insert((width - 1, y));
    }

    for y in 0..height {
        let mut largest_left = 0;
        for x in 0..width {
            let val = grid[y][x];

            if val > largest_left {
                visible.insert((x, y));
                largest_left = val;
            }
        }

        let mut largest_right = 0;
        for x in (0..width).rev() {
            let val = grid[y][x];

            if val > largest_right {
                visible.insert((x, y));
                largest_right = val;
            }
        }
    }

    for x in 0..width {
        let mut largest_above = 0;
        for y in 0..height {
            let val = grid[y][x];

            if val > largest_above {
                visible.insert((x, y));
                largest_above = val;
            }
        }

        let mut largest_below = 0;
        for y in (0..height).rev() {
            let val = grid[y][x];

            if val > largest_below {
                visible.insert((x, y));
                largest_below = val;
            }
        }
    }

    println!("Part 1 result: {}", visible.len());
}

fn part2(grid: &Vec<Vec<u32>>) {
    let height = grid.len();
    let width = grid[0].len();

    let max = (0..width)
        .flat_map(|x| (0..height).map(move |y| (x, y)))
        .map(|coords| scenic_score(coords, grid))
        .max()
        .unwrap();

    println!("Part 2 result: {}", max);
}

fn scenic_score(coords: Coordinates, grid: &Vec<Vec<u32>>) -> usize {
    let height = grid.len();
    let width = grid[0].len();

    let val = grid[coords.1][coords.0];

    // Look left
    let mut left = (0..coords.0)
        .rev()
        .take_while(|x| grid[coords.1][*x] < val)
        .count();

    if left != coords.0 {
        left += 1
    }

    // Look right
    let mut right = ((coords.0 + 1)..width)
        .take_while(|x| grid[coords.1][*x] < val)
        .count();

    if right != (width - coords.0 - 1) {
        right += 1
    }

    // Look up
    let mut up = (0..coords.1)
        .rev()
        .take_while(|y| grid[*y][coords.0] < val)
        .count();

    if up != coords.1 {
        up += 1;
    }

    // Look down
    let mut down = ((coords.1 + 1)..height)
        .take_while(|y| grid[*y][coords.0] < val)
        .count();

    if down != (height - coords.1 - 1) {
        down += 1;
    }

    left * right * up * down
}
