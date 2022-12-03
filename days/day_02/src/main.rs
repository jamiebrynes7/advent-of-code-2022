fn main() {
    let input = include_str!("./data.txt");
    part1::result(input);
    part2::result(input);
}

mod part2 {
    use std::str::FromStr;

    pub fn result(input: &str) {
        let games = input
            .split("\n")
            .filter(|s| !s.is_empty())
            .map(Game::from_str)
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        let points: u32 = games.iter().map(Game::points).sum();
        println!("Part 1 result: {points}");
    }

    struct Game {
        opponent: Shape,
        outcome: Outcome,
    }

    impl Game {
        pub fn points(&self) -> u32 {
            let mut score = match self.outcome {
                Outcome::Win => 6,
                Outcome::Draw => 3,
                Outcome::Loss => 0,
            };

            score += match (&self.outcome, &self.opponent) {
                (Outcome::Win, Shape::Rock) => 2,
                (Outcome::Win, Shape::Paper) => 3,
                (Outcome::Win, Shape::Scissors) => 1,
                (Outcome::Draw, Shape::Rock) => 1,
                (Outcome::Draw, Shape::Paper) => 2,
                (Outcome::Draw, Shape::Scissors) => 3,
                (Outcome::Loss, Shape::Rock) => 3,
                (Outcome::Loss, Shape::Paper) => 1,
                (Outcome::Loss, Shape::Scissors) => 2,
            };

            score
        }
    }

    impl FromStr for Game {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut parts = s.split(" ");

            let opponent = parts.next().ok_or(format!("Cannot parse {s}"))?;
            let outcome = parts.next().ok_or(format!("Cannot parse {s}"))?;

            Ok(Game {
                opponent: Shape::from_str(opponent)?,
                outcome: Outcome::from_str(outcome)?,
            })
        }
    }

    #[derive(PartialEq, Eq)]
    enum Shape {
        Rock,
        Paper,
        Scissors,
    }

    impl FromStr for Shape {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "A" => Ok(Shape::Rock),
                "B" => Ok(Shape::Paper),
                "C" => Ok(Shape::Scissors),
                _ => Err(format!("Unknown shape: {s}")),
            }
        }
    }

    enum Outcome {
        Win,
        Draw,
        Loss,
    }

    impl FromStr for Outcome {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "X" => Ok(Outcome::Loss),
                "Y" => Ok(Outcome::Draw),
                "Z" => Ok(Outcome::Win),
                _ => Err(format!("Unknown outcome {s}")),
            }
        }
    }
}

mod part1 {
    use std::str::FromStr;

    pub fn result(input: &str) {
        let games = input
            .split("\n")
            .filter(|s| !s.is_empty())
            .map(Game::from_str)
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        let points: u32 = games.iter().map(Game::points).sum();
        println!("Part 1 result: {points}");
    }

    struct Game {
        you: Shape,
        opponent: Shape,
    }

    impl Game {
        pub fn points(&self) -> u32 {
            let mut score = match self.you {
                Shape::Rock => 1,
                Shape::Paper => 2,
                Shape::Scissors => 3,
            };

            score += match (&self.you, &self.opponent) {
                (Shape::Rock, Shape::Rock) => 3,
                (Shape::Rock, Shape::Paper) => 0,
                (Shape::Rock, Shape::Scissors) => 6,
                (Shape::Paper, Shape::Rock) => 6,
                (Shape::Paper, Shape::Paper) => 3,
                (Shape::Paper, Shape::Scissors) => 0,
                (Shape::Scissors, Shape::Rock) => 0,
                (Shape::Scissors, Shape::Paper) => 6,
                (Shape::Scissors, Shape::Scissors) => 3,
            };

            score
        }
    }

    impl FromStr for Game {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut parts = s.split(" ");

            let opponent = parts.next().ok_or(format!("Cannot parse {s}"))?;
            let you = parts.next().ok_or(format!("Cannot parse {s}"))?;

            Ok(Game {
                you: Shape::from_str(you)?,
                opponent: Shape::from_str(opponent)?,
            })
        }
    }

    #[derive(PartialEq, Eq)]
    enum Shape {
        Rock,
        Paper,
        Scissors,
    }

    impl FromStr for Shape {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "A" | "X" => Ok(Shape::Rock),
                "B" | "Y" => Ok(Shape::Paper),
                "C" | "Z" => Ok(Shape::Scissors),
                _ => Err(format!("Unknown shape: {s}")),
            }
        }
    }

    #[cfg(test)]
    mod test {
        use std::str::FromStr;

        use super::Game;

        #[test]
        fn test_points() {
            let game = Game::from_str("A Y").unwrap();
            assert_eq!(game.points(), 8);

            let game = Game::from_str("B X").unwrap();
            assert_eq!(game.points(), 1);

            let game = Game::from_str("C Z").unwrap();
            assert_eq!(game.points(), 6);
        }
    }
}
