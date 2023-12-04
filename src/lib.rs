pub fn extract_number(s: &str) -> u32 {
    s.chars()
        .flat_map(|c| c.to_digit(10).into_iter())
        .rev()
        .enumerate()
        .map(|(i, n)| 10u32.pow(i as u32) * n)
        .sum()
}

pub fn normalized_lines(s: &str) -> impl Iterator<Item = &str> {
    s.lines().map(|s| s.trim()).filter(|s| !s.is_empty())
}

pub mod prelude {
    pub use super::extract_number;
}
