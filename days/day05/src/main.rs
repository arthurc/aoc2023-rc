use std::{borrow::Cow, collections::BTreeMap, iter, str::FromStr};

use aoc2023::extract_number;

const INPUT: &'static str = include_str!("../input.txt");

fn main() {
    println!(
        "The lowest location number that corresponds to any of the initial seed numbers: {}",
        find_lowest_location_number(INPUT)
    );
}

fn find_lowest_location_number(s: &str) -> u64 {
    const CHAIN: &[&'static str] = &[
        "seed-to-soil",
        "soil-to-fertilizer",
        "fertilizer-to-water",
        "water-to-light",
        "light-to-temperature",
        "temperature-to-humidity",
        "humidity-to-location",
    ];

    let almanac = Almanac::from_str(s).expect("Almanac");

    almanac
        .seeds
        .iter()
        .map(|s| {
            CHAIN
                .iter()
                .fold(*s, |seed, name| almanac.map(name).get(seed))
        })
        .min()
        .unwrap_or_default()
}

struct Almanac {
    seeds: Vec<u64>,
    maps: BTreeMap<String, Map>,
}
impl Almanac {
    fn map(&self, name: &str) -> &Map {
        self.maps.get(name).unwrap_or(&EMPTY_MAP)
    }
}
impl FromStr for Almanac {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.trim().lines().map(|l| l.trim());

        let (key, seeds) = lines
            .next()
            .ok_or("Expected line")?
            .split_once(':')
            .ok_or("Expected :")?;
        if key != "seeds" {
            return Err("Expected 'seeds'");
        }

        let seeds = seeds
            .split_whitespace()
            .map(|n| extract_number(n) as u64)
            .collect::<Vec<_>>();

        let _ = lines.next();
        let maps = iter::from_fn(|| Map::from_lines(&mut lines).ok())
            .map(|m| (m.name.to_string(), m))
            .collect::<BTreeMap<_, _>>();

        Ok(Self { seeds, maps })
    }
}

static EMPTY_MAP: Map = Map {
    name: Cow::Borrowed(""),
    categories: vec![],
};

struct Category {
    source: u64,
    dest: u64,
    len: usize,
}
impl Category {
    fn new(source: u64, dest: u64, len: usize) -> Self {
        Self { source, dest, len }
    }

    fn get(&self, key: u64) -> Option<u64> {
        if self.source <= key && key <= (self.source + self.len as u64) {
            Some(self.dest + (key - self.source))
        } else {
            None
        }
    }
}

struct Map {
    name: Cow<'static, str>,
    categories: Vec<Category>,
}
impl Map {
    fn new(name: String, categories: Vec<Category>) -> Self {
        Self {
            name: Cow::Owned(name),
            categories,
        }
    }

    fn get(&self, key: u64) -> u64 {
        self.categories
            .iter()
            .find_map(|c| c.get(key))
            .unwrap_or(key)
    }

    fn from_lines<'a>(mut lines: impl Iterator<Item = &'a str>) -> Result<Self, &'static str> {
        let (name, _) = lines
            .next()
            .ok_or("Expected line")?
            .split_once(' ')
            .ok_or("Expected space")?;

        let mut categories = Vec::new();
        while let Some(line) = lines.next() {
            if line.trim().is_empty() {
                break;
            }

            let mut numbers = line.split_whitespace();
            let dest = extract_number(numbers.next().ok_or("Dest")?) as u64;
            let source = extract_number(numbers.next().ok_or("Source")?) as u64;
            let len = extract_number(numbers.next().ok_or("Length")?) as usize;

            categories.push(Category::new(source, dest, len))
        }

        Ok(Map::new(name.into(), categories))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &'static str = r#"
    seeds: 79 14 55 13

    seed-to-soil map:
    50 98 2
    52 50 48

    soil-to-fertilizer map:
    0 15 37
    37 52 2
    39 0 15

    fertilizer-to-water map:
    49 53 8
    0 11 42
    42 0 7
    57 7 4

    water-to-light map:
    88 18 7
    18 25 70

    light-to-temperature map:
    45 77 23
    81 45 19
    68 64 13

    temperature-to-humidity map:
    0 69 1
    1 0 69

    humidity-to-location map:
    60 56 37
    56 93 4"#;

    #[test]
    fn test_example() {
        let almanac = Almanac::from_str(EXAMPLE).unwrap();

        assert_eq!(almanac.seeds, vec![79, 14, 55, 13]);
        assert_eq!(almanac.map("seed-to-soil").get(0), 0);
        assert_eq!(almanac.map("seed-to-soil").get(49), 49);
        assert_eq!(almanac.map("seed-to-soil").get(50), 52);
    }

    #[test]
    fn test_example_part1() {
        assert_eq!(find_lowest_location_number(EXAMPLE), 35);
    }

    #[test]
    fn test_result_part1() {
        assert_eq!(find_lowest_location_number(INPUT), 382895070);
    }
}
