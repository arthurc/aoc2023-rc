const INPUT: &'static str = include_str!("../input.txt");

fn main() {
    println!(
        "The sum of all of the calibration values: {}",
        INPUT.lines().map(to_calibration_value).sum::<u32>()
    );
}

fn to_calibration_value(line: &str) -> u32 {
    let mut iter = line.chars().flat_map(|c| c.to_digit(10).into_iter());
    let first = iter.next().expect("First value");
    let last = iter.last().unwrap_or(first);
    first * 10 + last
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(to_calibration_value("1abc2"), 12);
        assert_eq!(to_calibration_value("pqr3stu8vwx"), 38);
        assert_eq!(to_calibration_value("a1b2c3d4e5f"), 15);
        assert_eq!(to_calibration_value("treb7uchet"), 77);
    }
}
