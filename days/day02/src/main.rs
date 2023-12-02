use aoc2023::prelude::*;
use std::str::FromStr;

const INPUT: &'static str = include_str!("../input.txt");

fn main() {
    println!("The sum of the IDs of those games: {}", part1::run(INPUT));

    println!("The sum of the power of these sets: {}", part2::run(INPUT))
}

fn game_iter<'a>(s: &'a str) -> impl Iterator<Item = Game> + 'a {
    s.lines().flat_map(|s| Game::from_str(s).into_iter())
}

mod part1 {
    use super::*;

    pub fn run(s: &str) -> usize {
        let config = Set::new(12, 13, 14);

        game_iter(s)
            .filter(|g| g.is_possible(&config))
            .map(|g| g.id)
            .sum()
    }
}

mod part2 {
    use super::*;

    pub fn run(s: &str) -> usize {
        game_iter(s)
            .map(|g| g.minimum_required_configuration().power())
            .sum()
    }
}

#[derive(PartialEq, Debug)]
struct Game {
    id: usize,
    sets: Vec<Set>,
}
impl Game {
    fn is_possible(&self, config: &Set) -> bool {
        self.sets.iter().all(|s| s.is_possible(config))
    }

    fn minimum_required_configuration(&self) -> Set {
        self.sets.iter().fold(Set::new(0, 0, 0), |acc, s| {
            Set::new(s.r.max(acc.r), s.g.max(acc.g), s.b.max(acc.b))
        })
    }
}
impl FromStr for Game {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(":");
        let game_id_part = iter.next().ok_or("Expected game id part")?;
        let sets_part = iter.next().ok_or("Expected sets part")?;

        let id = extract_number(game_id_part) as usize;

        let sets = sets_part
            .split(";")
            .flat_map(|s| Set::from_str(s).into_iter())
            .collect();

        Ok(Game { id, sets })
    }
}

#[derive(PartialEq, Debug)]
struct Set {
    r: usize,
    g: usize,
    b: usize,
}
impl Set {
    fn new(r: usize, g: usize, b: usize) -> Self {
        Self { r, g, b }
    }

    fn is_possible(&self, config: &Set) -> bool {
        self.r <= config.r && self.g <= config.g && self.b <= config.b
    }

    fn power(&self) -> usize {
        self.r * self.g * self.b
    }
}
impl FromStr for Set {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut r = 0;
        let mut g = 0;
        let mut b = 0;

        for cubes in s.split(",") {
            if cubes.contains("red") {
                r = extract_number(cubes) as usize;
            } else if cubes.contains("green") {
                g = extract_number(cubes) as usize;
            } else if cubes.contains("blue") {
                b = extract_number(cubes) as usize;
            }
        }

        Ok(Self::new(r, g, b))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_game1() -> Game {
        Game {
            id: 1,
            sets: vec![Set::new(4, 0, 3), Set::new(1, 2, 6), Set::new(0, 2, 0)],
        }
    }

    fn example_game2() -> Game {
        Game {
            id: 2,
            sets: vec![Set::new(0, 2, 1), Set::new(1, 3, 4), Set::new(0, 1, 1)],
        }
    }

    #[test]
    fn test_game_from_str() {
        assert_eq!(
            Game::from_str("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            Ok(example_game1()),
        );

        assert_eq!(
            Game::from_str("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"),
            Ok(example_game2()),
        )
    }

    #[test]
    fn test_is_possible() {
        let game = Game {
            id: 1,
            sets: vec![Set::new(10, 0, 20)],
        };

        assert!(game.is_possible(&Set::new(10, 10, 30)));
        assert!(game.is_possible(&Set::new(10, 0, 30)));

        assert!(!game.is_possible(&Set::new(9, 0, 50)));
        assert!(!game.is_possible(&Set::new(0, 0, 0)));
    }

    #[test]
    fn test_minimum_required_configuration() {
        assert_eq!(
            example_game1().minimum_required_configuration(),
            Set::new(4, 2, 6)
        )
    }

    #[test]
    fn test_power() {
        assert_eq!(example_game1().minimum_required_configuration().power(), 48);
        assert_eq!(example_game2().minimum_required_configuration().power(), 12);
    }

    #[test]
    fn test_answer_part1() {
        assert_eq!(part1::run(INPUT), 2406);
    }

    #[test]
    fn test_answer_part2() {
        assert_eq!(part2::run(INPUT), 78375);
    }
}
