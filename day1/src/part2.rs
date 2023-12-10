use std::fs;

pub fn solve(path: &str) -> u32 {
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(parse_line)
        .sum()
}

fn parse_line(line: &str) -> u32 {
    let mut first = 0;
    let mut last = 0;
    for (i, c) in line.chars().enumerate() {
        if c.is_digit(10) {
            if first == 0 {
                first = c.to_digit(10).unwrap();
                last = first;
            } else {
                last = c.to_digit(10).unwrap();
            }
        } else {
            let fulltext = fulltext_digit(&line[i..]);
            if fulltext.is_some() {
                if first == 0 {
                    first = fulltext.unwrap();
                    last = first;
                } else {
                    last = fulltext.unwrap();
                }
            }
        }
    }
    first * 10 + last
}

fn fulltext_digit(slice: &str) -> Option<u32> {
    if slice.starts_with("one") {
        Some(1)
    } else if slice.starts_with("two") {
        Some(2)
    } else if slice.starts_with("three") {
        Some(3)
    } else if slice.starts_with("four") {
        Some(4)
    } else if slice.starts_with("five") {
        Some(5)
    } else if slice.starts_with("six") {
        Some(6)
    } else if slice.starts_with("seven") {
        Some(7)
    } else if slice.starts_with("eight") {
        Some(8)
    } else if slice.starts_with("nine") {
        Some(9)
    } else if slice.starts_with("zero") {
        Some(0)
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use crate::part2::{fulltext_digit, parse_line, solve};

    #[test]
    fn parsing() {
        assert_eq!(parse_line("two1nine"), 29);
        assert_eq!(parse_line("eightwothree"), 83);
        assert_eq!(parse_line("abcone2threexyz"), 13);
        assert_eq!(parse_line("xtwone3four"), 24);
        assert_eq!(parse_line("4nineeightseven2"), 42);
        assert_eq!(parse_line("zoneight234"), 14);
        assert_eq!(parse_line("7pqrstsixteen"), 76);
    }

    #[test]
    fn sum() {
        assert_eq!(solve("sample2.txt"), 281);
    }

    #[test]
    fn digit_conversion() {
        assert_eq!(fulltext_digit("one"), Some(1));
        assert_eq!(fulltext_digit("abc"), None);
    }
}
