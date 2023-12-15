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
    for c in line.chars() {
        if c.is_digit(10) {
            if first == 0 {
                first = c.to_digit(10).unwrap();
                last = first;
            } else {
                last = c.to_digit(10).unwrap();
            }
        }
    }
    first * 10 + last
}

#[cfg(test)]
mod test {
    use crate::part1::{parse_line, solve};

    #[test]
    fn parsing() {
        assert_eq!(parse_line("1abc2"), 12);
        assert_eq!(parse_line("pqr3stu8vwx"), 38);
        assert_eq!(parse_line("a1b2c3d4e5f"), 15);
        assert_eq!(parse_line("treb7uchet"), 77);
    }

    #[test]
    fn sum() {
        assert_eq!(solve("sample.txt"), 142);
    }
}