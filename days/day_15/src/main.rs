use std::{ops::RangeInclusive, str::FromStr};

use once_cell::sync::Lazy;
use regex::Regex;
use utils::parse_lines;

fn main() {
    let input = include_str!("./data.txt");
    let sensors: Vec<Sensor> = parse_lines(input).unwrap();
    part1(&sensors);
    part2(&sensors);
}

fn part1(sensors: &[Sensor]) {
    let detection_range = sensors
        .iter()
        .map(|s| s.detection_range_x())
        .reduce(|acc, item| {
            let min = acc.start().min(item.start());
            let max = acc.end().max(item.end());

            *min..=*max
        })
        .unwrap();

    const Y: i64 = 2000000;

    let impossible_elements = detection_range
        .map(|x| (x, Y))
        .filter(|coords| sensors.iter().any(|s| s.is_impossible(*coords)))
        .filter(|coords| sensors.iter().all(|s| s.closest_beacon != *coords))
        .count();

    println!("Part 1 result: {}", impossible_elements);
}

fn part2(sensors: &[Sensor]) {
    const MAX: i64 = 4000000;

    for y in 0..=MAX {
        let ranges = sensors
            .iter()
            .filter_map(|s| s.detection_range_for_y(y))
            .collect::<Vec<RangeInclusive<i64>>>();

        let mut x = 0;

        while x <= MAX {
            if let Some(range) = ranges.iter().find(|r| r.contains(&x)) {
                x = range.end() + 1;
            } else {
                let result = x * 4000000 + y;
                println!("Part 2 result: {result}");
                return;
            }
        }
    }
}

type Coords = (i64, i64);

fn manhattan_dist(first: Coords, second: Coords) -> i64 {
    (first.0 - second.0).abs() + (first.1 - second.1).abs()
}

#[derive(Debug)]
struct Sensor {
    coords: Coords,
    closest_beacon: Coords,
}

impl Sensor {
    pub fn detection_range_x(&self) -> RangeInclusive<i64> {
        let dist = manhattan_dist(self.coords, self.closest_beacon);

        self.coords.0 - dist..=(self.coords.0 + dist)
    }

    pub fn detection_range_for_y(&self, y: i64) -> Option<RangeInclusive<i64>> {
        let dist = manhattan_dist(self.coords, self.closest_beacon);

        let y_diff = (self.coords.1 - y).abs();
        let x_diff = dist - y_diff;

        if x_diff < 0 {
            None
        } else {
            Some(self.coords.0 - x_diff..=(self.coords.0 + x_diff))
        }
    }

    pub fn is_impossible(&self, coords: Coords) -> bool {
        manhattan_dist(self.coords, self.closest_beacon) >= manhattan_dist(self.coords, coords)
    }
}

impl FromStr for Sensor {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        static RE: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r#"Sensor at x=([-0-9]*), y=([-0-9]*): closest beacon is at x=([-0-9]*), y=([-0-9]*)"#).unwrap()
        });

        let captures = RE.captures(s).unwrap();

        let self_x = captures.get(1).unwrap().as_str().parse::<i64>().unwrap();
        let self_y = captures.get(2).unwrap().as_str().parse::<i64>().unwrap();

        let beacon_x = captures.get(3).unwrap().as_str().parse::<i64>().unwrap();
        let beacon_y = captures.get(4).unwrap().as_str().parse::<i64>().unwrap();

        Ok(Self {
            coords: (self_x, self_y),
            closest_beacon: (beacon_x, beacon_y),
        })
    }
}
