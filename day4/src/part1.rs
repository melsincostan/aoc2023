use std::collections::HashSet;
use std::fs;
use std::cmp;

#[derive(Debug, PartialEq)]
struct Scratchcard {
    id: u32,
    winning: HashSet<u32>,
    own: HashSet<u32>,
}

pub fn solve(input: &str) -> u32 {
    fs::read_to_string(input).unwrap().lines().map(parse_card).map(score_card).sum()
}

fn parse_card(line: &str) -> Scratchcard {
    let raw_meta = line.split(": ").next().unwrap();
    let raw_game = line.split(": ").last().unwrap();

    let raw_winning = raw_game.split(" | ").next().unwrap();
    let raw_own = raw_game.split(" | ").last().unwrap();

    let winning = raw_winning
        .split_whitespace()
        .map(|num| num.parse::<u32>().unwrap())
        .collect();

    let own = raw_own
        .split_whitespace()
        .map(|num| num.parse::<u32>().unwrap())
        .collect();

    let id = raw_meta
        .strip_prefix("Card ")
        .unwrap()
        .trim()
        .parse::<u32>()
        .unwrap();
    Scratchcard {
        id: id,
        winning: winning,
        own: own,
    }
}

fn score_card(card: Scratchcard) -> u32 {
    let present = card.winning.intersection(&card.own).count();
    if present < 1 {
        0
    } else {
        1 << (present - 1) // dumb power of two bitwise tricks because i don't want to use something like pow or w/e
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use crate::part1::{parse_card, Scratchcard, score_card, solve};

    #[test]
    fn parsing() {
        let card1 = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let card1_winning = HashSet::from([41, 48, 83, 86, 17]);
        let card1_own = HashSet::from([83, 86, 6, 31, 17, 9, 48, 53]);
        assert_eq!(parse_card(card1), Scratchcard{
            id: 1,
            winning: card1_winning,
            own: card1_own,
        });
    }

    #[test]
    fn scoring() {
        let card1 = parse_card("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");
        let card2 = parse_card("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83");
        let card3 = parse_card("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11");

        assert_eq!(score_card(card1), 8);
        assert_eq!(score_card(card2), 1);
        assert_eq!(score_card(card3), 0);
    }

    #[test]
    fn full() {
        assert_eq!(solve("sample.txt"), 13);
    }
}