const INPUT: &'static str = include_str!("../input.txt");

fn main() {
    println!(
        "The sum of all of the part numbers in the engine schematic: {:?}",
        find_part_numbers(INPUT).iter().sum::<u32>()
    );
}

fn find_part_numbers(s: &str) -> Vec<u32> {
    let line_tokens = normalized_lines(s)
        .map(|l| LineParser::new(l).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut numbers = Vec::new();
    for (l, ts) in line_tokens.iter().enumerate() {
        for (i, ParsedToken { ref token, .. }) in ts.iter().enumerate() {
            if let Token::Number(n) = token {
                if !adjacent_symbols(i, l, &line_tokens).is_empty() {
                    numbers.push(*n);
                }
            }
        }
    }

    numbers
}

fn adjacent_symbols(
    current_token_index: usize,
    current_line_index: usize,
    tokens_per_line: &Vec<Vec<ParsedToken>>,
) -> Vec<char> {
    let current_line_tokens = &tokens_per_line[current_line_index];
    let current_token = &current_line_tokens[current_token_index];
    let rstart = 1.max(current_token.start) - 1;
    let rend = current_token.start + current_token.len;
    let mut found_symbols = Vec::new();

    // Above
    if current_line_index > 0 {
        tokens_per_line[current_line_index - 1]
            .iter()
            .filter_map(|pt| match pt.token {
                Token::Symbol(c) if pt.contains(rstart, rend) => Some(c),
                _ => None,
            })
            .for_each(|c| found_symbols.push(c));
    }

    // Same line
    if current_token_index > 0 {
        if let Token::Symbol(c) = current_line_tokens[current_token_index - 1].token {
            found_symbols.push(c);
        }
    }
    if let Some(Token::Symbol(c)) = current_line_tokens
        .get(current_token_index + 1)
        .map(|pt| &pt.token)
    {
        found_symbols.push(*c);
    }

    // Below
    if let Some(below) = tokens_per_line.get(current_line_index + 1) {
        below
            .iter()
            .filter_map(|pt| match pt.token {
                Token::Symbol(c) if pt.contains(rstart, rend) => Some(c),
                _ => None,
            })
            .for_each(|c| found_symbols.push(c));
    }

    found_symbols
}

struct LineParser<'a> {
    s: &'a str,
    i: usize,
}
impl LineParser<'_> {
    fn new(s: &str) -> LineParser {
        LineParser { s, i: 0 }
    }

    fn parse_space(&mut self) -> ParsedToken {
        let count = self.s[self.i..].chars().take_while(|c| *c == '.').count();

        let parsed_token = ParsedToken::new(self.i, count, Token::Space);
        self.i += count;

        parsed_token
    }

    fn parse_symbol(&mut self) -> ParsedToken {
        let parsed_token = ParsedToken::new(
            self.i,
            1,
            Token::Symbol(self.s.chars().nth(self.i).expect("Symbol")),
        );

        self.i += 1;

        parsed_token
    }

    fn parse_number(&mut self) -> ParsedToken {
        let number = self.s[self.i..]
            .chars()
            .take_while(|c| c.is_digit(10))
            .collect::<String>();

        let token = Token::Number(u32::from_str_radix(&number, 10).expect("Number"));
        let parsed_token = ParsedToken::new(self.i, number.len(), token);
        self.i += number.len();

        parsed_token
    }
}
impl<'a> Iterator for LineParser<'_> {
    type Item = ParsedToken;

    fn next(&mut self) -> Option<Self::Item> {
        match self.s.chars().nth(self.i) {
            None => return None,
            Some('.') => Some(self.parse_space()),
            Some(n) if n.is_digit(10) => Some(self.parse_number()),
            _ => Some(self.parse_symbol()),
        }
    }
}

#[derive(Debug, PartialEq)]
struct ParsedToken {
    start: usize,
    len: usize,
    token: Token,
}
impl ParsedToken {
    fn new(start: usize, len: usize, token: Token) -> Self {
        Self { start, len, token }
    }

    fn contains(&self, n1: usize, n2: usize) -> bool {
        (self.start <= n1 && n1 <= self.end())
            || (self.start <= n2 && n2 <= self.end())
            || (n1 <= self.start && self.start <= n2)
            || (n1 <= self.end() && self.end() <= n2)
    }

    #[inline]
    fn end(&self) -> usize {
        self.start + self.len - 1
    }
}

#[derive(Debug, PartialEq)]
enum Token {
    Number(u32),
    Space,
    Symbol(char),
}

fn normalized_lines(s: &str) -> impl Iterator<Item = &str> {
    s.lines().map(|s| s.trim()).filter(|s| !s.is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &'static str = r#"
    467..114..
    ...*......
    ..35..633.
    ......#...
    617*......
    .....+.58.
    ..592.....
    ......755.
    ...$.*....
    .664.598..
    "#;

    #[test]
    fn test_line_parser() {
        assert_eq!(
            LineParser::new("..11.@..33.").collect::<Vec<_>>(),
            vec![
                ParsedToken::new(0, 2, Token::Space),
                ParsedToken::new(2, 2, Token::Number(11)),
                ParsedToken::new(4, 1, Token::Space),
                ParsedToken::new(5, 1, Token::Symbol('@')),
                ParsedToken::new(6, 2, Token::Space),
                ParsedToken::new(8, 2, Token::Number(33)),
                ParsedToken::new(10, 1, Token::Space)
            ]
        );
    }

    #[test]
    fn test_example() {
        assert_eq!(
            find_part_numbers(EXAMPLE),
            vec![467, 35, 633, 617, 592, 755, 664, 598]
        );
    }

    #[test]
    fn test_find_part_numbers_same_line() {
        assert_eq!(find_part_numbers("1"), vec![]);
        assert_eq!(find_part_numbers(".1"), vec![]);
        assert_eq!(find_part_numbers("1."), vec![]);
        assert_eq!(find_part_numbers(".1."), vec![]);

        assert_eq!(find_part_numbers(""), vec![]);
        assert_eq!(find_part_numbers("@"), vec![]);
        assert_eq!(find_part_numbers("@1"), vec![1]);
        assert_eq!(find_part_numbers("1@"), vec![1]);

        assert_eq!(find_part_numbers("1.@"), vec![]);
        assert_eq!(find_part_numbers("1.@.2"), vec![]);
        assert_eq!(find_part_numbers("1.@2"), vec![2]);
        assert_eq!(find_part_numbers("1@2"), vec![1, 2]);
    }

    #[test]
    fn test_find_part_numbers_above() {
        assert_eq!(
            find_part_numbers(
                r#"
                "#
            ),
            vec![]
        );
        assert_eq!(
            find_part_numbers(
                r#"
                1"#
            ),
            vec![]
        );
        assert_eq!(
            find_part_numbers(
                r#"
                @..
                .1."#
            ),
            vec![1]
        );
        assert_eq!(
            find_part_numbers(
                r#"
                .@.
                .1."#
            ),
            vec![1]
        );
        assert_eq!(
            find_part_numbers(
                r#"
                ..@
                .1."#
            ),
            vec![1]
        );
        assert_eq!(
            find_part_numbers(
                r#"
                ...@
                .1.."#
            ),
            vec![]
        );
    }

    #[test]
    fn test_find_part_numbers_below() {
        assert_eq!(
            find_part_numbers(
                r#"
                "#
            ),
            vec![]
        );
        assert_eq!(
            find_part_numbers(
                r#"
                1
                "#
            ),
            vec![]
        );
        assert_eq!(
            find_part_numbers(
                r#"
                .1.
                @..
                "#
            ),
            vec![1]
        );
        assert_eq!(
            find_part_numbers(
                r#"
                .1.
                .@.
                "#
            ),
            vec![1]
        );
        assert_eq!(
            find_part_numbers(
                r#"
                .1.
                ..@
                "#
            ),
            vec![1]
        );
        assert_eq!(
            find_part_numbers(
                r#"
                .1..
                ...@
                "#
            ),
            vec![]
        );
        assert_eq!(
            find_part_numbers(
                r#"
                ..1
                *..
                "#
            ),
            vec![]
        );
        assert_eq!(
            find_part_numbers(
                r#"
                9..
                ..*
                "#
            ),
            vec![]
        );
    }

    #[test]
    fn test_result_part1() {
        assert_eq!(find_part_numbers(INPUT).iter().sum::<u32>(), 528799);
    }
}
