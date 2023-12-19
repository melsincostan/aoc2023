use std::fs;

pub fn solve(input: &str) -> u32 {
    fs::read_to_string(input)
        .unwrap()
        .trim()
        .split(",")
        .map(hash)
        .sum()
}

fn hash(string: &str) -> u32 {
    let mut hash_result = 0;
    for c in string.chars() {
        hash_result += c as u32;
        hash_result *= 17;
        hash_result %= 256;
    }
    hash_result
}

#[cfg(test)]
mod test {
    use crate::part1::{hash, solve};

    #[test]
    fn test_hash() {
        assert_eq!(hash("HASH"), 52);
        assert_eq!(hash("rn=1"), 30);
        assert_eq!(hash("cm-"), 253);
        assert_eq!(hash("qp=3"), 97);
        assert_eq!(hash("cm=2"), 47);
        assert_eq!(hash("qp-"), 14);
        assert_eq!(hash("pc=4"), 180);
        assert_eq!(hash("ot=9"), 9);
        assert_eq!(hash("ab=5"), 197);
        assert_eq!(hash("pc-"), 48);
        assert_eq!(hash("pc=6"), 214);
        assert_eq!(hash("ot=7"), 231);
    }

    #[test]
    fn test_solve() {
        assert_eq!(solve("sample.txt"), 1320);
    }
}
