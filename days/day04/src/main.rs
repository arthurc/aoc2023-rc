use std::{collections::HashSet, str::FromStr};

use aoc2023::{extract_number, normalized_lines};

const INPUT: &'static str = include_str!("../input.txt");

fn main() {
    println!("Points worth in total: {}", calculate_total_points(INPUT))
}

fn calculate_total_points(s: &str) -> u32 {
    normalized_lines(s)
        .flat_map(|l| Card::from_str(l).into_iter())
        .map(|c| c.points())
        .sum()
}

struct Card {
    number: u32,
    winning_numbers: Vec<u32>,
    my_numbers: Vec<u32>,
}
impl Card {
    fn points(&self) -> u32 {
        let unique_winning_numbers = self.winning_numbers.iter().copied().collect::<HashSet<_>>();
        let unique_my_numbers = self.my_numbers.iter().copied().collect::<HashSet<_>>();

        let intersecting_numbers = unique_winning_numbers
            .intersection(&unique_my_numbers)
            .count() as u32;

        if intersecting_numbers == 0 {
            0
        } else {
            2u32.pow(intersecting_numbers - 1)
        }
    }
}
impl FromStr for Card {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (card_number_part, rest) = s.split_once(':').ok_or("Expected ':'")?;
        let (winning_numbers_part, my_numbers_part) = rest.split_once('|').ok_or("Expected '|'")?;

        let number = extract_number(card_number_part);
        let winning_numbers = winning_numbers_part
            .split_whitespace()
            .map(extract_number)
            .collect::<Vec<_>>();
        let my_numbers = my_numbers_part
            .split_whitespace()
            .map(extract_number)
            .collect::<Vec<_>>();

        Ok(Card {
            number,
            winning_numbers,
            my_numbers,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &'static str = r#"
    Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
    Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
    Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
    Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
    Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
    "#;

    #[test]
    fn test_card_points() {
        assert_eq!(
            Card {
                number: 1,
                winning_numbers: vec![41, 48, 83, 86, 17],
                my_numbers: vec![83, 86, 6, 31, 17, 9, 48, 53]
            }
            .points(),
            8
        )
    }

    #[test]
    fn test_example() {
        assert_eq!(calculate_total_points(EXAMPLE), 13);
    }
}
