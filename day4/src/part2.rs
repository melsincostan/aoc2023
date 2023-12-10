use std::collections::HashSet;
use std::fs;

#[derive(Debug, PartialEq)]
struct Scratchcard {
    id: u32,
    winning: HashSet<u32>,
    own: HashSet<u32>,
}

pub fn solve(input: &str) -> u32 {
    let cards: Vec<Scratchcard> = fs::read_to_string(input).unwrap().lines().map(parse_card).collect();
    let mut played = vec![0; cards.len()];
    for (pos, card) in cards.iter().enumerate() {
        played[pos] += 1;
        for w in 0..score_card(card) {
            played[w+pos+1] += played[pos];
        }
    }
    played.iter().sum()
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

fn score_card(card: &Scratchcard) -> usize {
    card.winning.intersection(&card.own).count()
}

#[cfg(test)]
mod test {
    use crate::part2::solve;

    #[test]
    fn full() {
        assert_eq!(solve("sample.txt"), 30);
    }
}