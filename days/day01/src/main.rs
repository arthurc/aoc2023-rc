const INPUT: &'static str = include_str!("../input.txt");

fn main() {
    println!("Part 1:");
    println!(
        "The sum of all of the calibration values: {}",
        INPUT.lines().map(part1::to_calibration_value).sum::<u32>()
    );
    println!();

    println!("Part 2:");
    println!(
        "The sum of all of the calibration values: {}",
        INPUT.lines().map(part2::to_calibration_value).sum::<u32>()
    )
}

mod part1 {
    pub fn to_calibration_value(line: &str) -> u32 {
        let mut iter = line.chars().flat_map(|c| c.to_digit(10).into_iter());
        let first = iter.next().expect("First value");
        let last = iter.last().unwrap_or(first);
        first * 10 + last
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_example() {
            assert_eq!(to_calibration_value("1abc2"), 12);
            assert_eq!(to_calibration_value("pqr3stu8vwx"), 38);
            assert_eq!(to_calibration_value("a1b2c3d4e5f"), 15);
            assert_eq!(to_calibration_value("treb7uchet"), 77);
        }
    }
}

mod part2 {
    use super::part1;

    const NUMBERS_AS_TEXT: &'static [(&'static str, &'static str)] = &[
        ("1", "one"),
        ("2", "two"),
        ("3", "three"),
        ("4", "four"),
        ("5", "five"),
        ("6", "six"),
        ("7", "seven"),
        ("8", "eight"),
        ("9", "nine"),
    ];

    pub fn to_calibration_value(line: &str) -> u32 {
        let mut line = line.to_owned();
        for (n, t) in NUMBERS_AS_TEXT {
            // HACK: This replaces {text} with {text}{number}{text} to be able to take into
            //       account, for instance, "oneight". If we just replace with {number} we
            //       end up with "1ight" or "on8" depending on which number we replace with
            //       first. If we replace with {number}{text} we get "1oneeight" or
            //       "on8eight", which still messes up some cases one way or the other.
            //       {text}{number}{text} results in "one1oneight8eight" and thus we are
            //       able to extract the numbers in the correct order, regardless of the
            //       order we replace the stringified numbers.
            line = line.replace(t, &format!("{}{}{}", t, n, t));
        }

        part1::to_calibration_value(&line)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_example() {
            assert_eq!(to_calibration_value("two1nine"), 29);
            assert_eq!(to_calibration_value("eightwothree"), 83);
            assert_eq!(to_calibration_value("abcone2threexyz"), 13);
            assert_eq!(to_calibration_value("xtwone3four"), 24);
            assert_eq!(to_calibration_value("4nineeightseven2"), 42);
            assert_eq!(to_calibration_value("zoneight234"), 14);
            assert_eq!(to_calibration_value("7pqrstsixteen"), 76);
        }

        #[test]
        fn test_edge_cases() {
            assert_eq!(to_calibration_value("1"), 11);
            assert_eq!(to_calibration_value("one"), 11);
            assert_eq!(to_calibration_value("oneight"), 18);
            assert_eq!(to_calibration_value("twone"), 21);
            assert_eq!(to_calibration_value("eightwo"), 82);
        }
    }
}
