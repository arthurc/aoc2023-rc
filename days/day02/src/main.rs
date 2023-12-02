use std::str::FromStr;

const INPUT: &'static str = include_str!("../input.txt");

fn main() {
    let config = Configuration::new(12, 13, 14);

    let id_sum: usize = INPUT
        .lines()
        .flat_map(|s| Game::from_str(s).into_iter())
        .filter(|g| g.is_possible(&config))
        .filter(|g| g.is_possible(&config))
        .map(|g| g.id)
        .sum();

    println!("The sum of the IDs of those games: {}", id_sum)
}

fn extract_number(s: &str) -> u32 {
    s.chars()
        .flat_map(|c| c.to_digit(10).into_iter())
        .rev()
        .enumerate()
        .map(|(i, n)| 10u32.pow(i as u32) * n)
        .sum()
}

#[derive(PartialEq, Debug)]
struct Game {
    id: usize,
    sets: Vec<Set>,
}
impl Game {
    fn is_possible(&self, config: &Configuration) -> bool {
        self.sets.iter().all(|s| s.is_possible(config))
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

    fn is_possible(&self, config: &Configuration) -> bool {
        self.r <= config.r && self.g <= config.g && self.b <= config.b
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

struct Configuration {
    r: usize,
    g: usize,
    b: usize,
}
impl Configuration {
    fn new(r: usize, g: usize, b: usize) -> Self {
        Self { r, b, g }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_from_str() {
        assert_eq!(
            Game::from_str("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            Ok(Game {
                id: 1,
                sets: vec![Set::new(4, 0, 3), Set::new(1, 2, 6), Set::new(0, 2, 0)],
            }),
        );

        assert_eq!(
            Game::from_str("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"),
            Ok(Game {
                id: 2,
                sets: vec![Set::new(0, 2, 1), Set::new(1, 3, 4), Set::new(0, 1, 1)],
            }),
        )
    }

    #[test]
    fn test_is_possible() {
        let game = Game {
            id: 1,
            sets: vec![Set::new(10, 0, 20)],
        };

        assert!(game.is_possible(&Configuration::new(10, 10, 30)));
        assert!(game.is_possible(&Configuration::new(10, 0, 30)));

        assert!(!game.is_possible(&Configuration::new(9, 0, 50)));
        assert!(!game.is_possible(&Configuration::new(0, 0, 0)));
    }
}
