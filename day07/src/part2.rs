use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, PartialEq)]
enum Hand {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Debug, PartialEq)]
struct HandWithBet {
    hand: String,
    optimized_hand: String,
    bet: u32,
}

pub fn solve(input: &str) -> u32 {
    let mut hands: Vec<HandWithBet> = fs::read_to_string(input)
        .unwrap()
        .lines()
        .map(parse_line)
        .collect();
    hands.sort_by(compare_hands);
    let mut acc = 0;
    for (rank, hand) in hands.iter().enumerate() {
        acc += hand.bet * (rank as u32 + 1);
    }
    acc
}

fn compare_hands(a: &HandWithBet, b: &HandWithBet) -> Ordering {
    let t_a = hand_type(&a.optimized_hand).unwrap();
    let t_b = hand_type(&b.optimized_hand).unwrap();
    let p_a = hand_power(t_a);
    let p_b = hand_power(t_b);
    if p_a < p_b {
        return Ordering::Less;
    } else if p_a > p_b {
        return Ordering::Greater;
    } else {
        return compare_hand_cards(&a.hand, &b.hand);
    }
}

fn compare_hand_cards(a: &str, b: &str) -> Ordering {
    for i in 0..a.len() {
        let c_a = card_power(a.chars().nth(i).unwrap());
        let c_b = card_power(b.chars().nth(i).unwrap());
        if c_a < c_b {
            return Ordering::Less;
        } else if c_a > c_b {
            return Ordering::Greater;
        }
    }
    Ordering::Equal
}

fn parse_line(line: &str) -> HandWithBet {
    let parts: Vec<&str> = line.split_whitespace().collect();
    HandWithBet {
        hand: parts[0].to_string(),
        optimized_hand: optimize_hand(parts[0]),
        bet: parts[1].parse::<u32>().unwrap(),
    }
}

fn optimize_hand(hand: &str) -> String {
    let old_type = hand_type(hand).unwrap();
    if !hand.contains("J") || old_type == Hand::FiveOfAKind {
        return hand.to_string();
    }
    let mut parsed_hand: HashMap<char, u8> = Default::default();
    for c in hand.chars() {
            let val;
            if parsed_hand.contains_key(&c) {
                val = parsed_hand.get(&c).unwrap() + 1;
            } else {
                val = 1
            }
            parsed_hand.insert(c, val);
    }
    if old_type == Hand::FourOfAKind || old_type == Hand::FullHouse {
        parsed_hand.remove(&'J');
        return parsed_hand.keys().last().unwrap().to_string().repeat(5);
    } else if old_type == Hand::ThreeOfAKind {
        parsed_hand.remove(&'J');
        let rest: Vec<&char> = parsed_hand.keys().collect();
        return rest[1].to_string().repeat(4) + &rest[0].to_string();
    } else if old_type == Hand::TwoPair {
        if *parsed_hand.get(&'J').unwrap() == 1 {
            parsed_hand.remove(&'J');
            let rest: Vec<&char> = parsed_hand.keys().collect();
            return rest[1].to_string().repeat(3) + &rest[0].to_string().repeat(2);
        } else {
            parsed_hand.remove(&'J');
            let mut res = "".to_string();
            for (k, v) in parsed_hand {
                if v == 2 {
                    res += &k.to_string().repeat(4);
                } else {
                    res += &k.to_string().repeat(v as usize);
                }
            }
            return res;
        }
    } else if old_type == Hand::OnePair {
        if *parsed_hand.get(&'J').unwrap() == 2 {
            parsed_hand.remove(&'J');
            let rest: Vec<&char> = parsed_hand.keys().collect();
            return rest[0].to_string().repeat(3) + &rest[1].to_string() + &rest[2].to_string();
        } else {
            parsed_hand.remove(&'J');
            let mut res = "".to_string();
            for (k, v) in parsed_hand {
                if v == 2 {
                    res += &k.to_string().repeat(3);
                } else {
                    res += &k.to_string().repeat(v as usize);
                }
            }
            return res;
        }
    } else {
        parsed_hand.remove(&'J');
        let rest: Vec<&char> = parsed_hand.keys().collect();
        return rest[0].to_string().repeat(2) + &rest[1].to_string() + &rest[2].to_string() + &rest[3].to_string();
    }
}

fn card_power(card: char) -> u32 {
    match card {
        'J' => 0,
        '2' => 1,
        '3' => 2,
        '4' => 3,
        '5' => 4,
        '6' => 5,
        '7' => 6,
        '8' => 7,
        '9' => 8,
        'T' => 9,
        'Q' => 11,
        'K' => 12,
        'A' => 13,
        _ => 0,
    }
}

fn hand_power(hand: Hand) -> u32 {
    match hand {
        Hand::HighCard => 1,
        Hand::OnePair => 2,
        Hand::TwoPair => 3,
        Hand::ThreeOfAKind => 4,
        Hand::FullHouse => 5,
        Hand::FourOfAKind => 6,
        Hand::FiveOfAKind => 7,
    }
}

fn hand_type(hand: &str) -> Option<Hand> {
    assert_eq!(hand.len(), 5);
    let mut parsed_hand: HashMap<char, u8> = Default::default();
    for c in hand.chars() {
        let val;
        if parsed_hand.contains_key(&c) {
            val = parsed_hand.get(&c).unwrap() + 1;
        } else {
            val = 1
        }
        parsed_hand.insert(c, val);
    }
    if parsed_hand.len() == 1 {
        return Some(Hand::FiveOfAKind);
    } else if parsed_hand.len() == 2 {
        // four of a kind or full house
        let first_val = parsed_hand.values().last().unwrap();
        if *first_val == 4 || *first_val == 1 {
            return Some(Hand::FourOfAKind);
        } else {
            return Some(Hand::FullHouse);
        }
    } else if parsed_hand.len() == 3 {
        // three of a kind or two pair
        for val in parsed_hand.values() {
            if *val == 3 {
                // three of a kind: 3, 1, 1
                return Some(Hand::ThreeOfAKind);
            } else if *val == 2 {
                // two pair: 2, 2, 1
                return Some(Hand::TwoPair);
            }
        }
    } else if parsed_hand.len() == 4 {
        return Some(Hand::OnePair);
    } else if parsed_hand.len() == 5 {
        return Some(Hand::HighCard);
    }
    None
}

#[cfg(test)]
mod test {
    use std::cmp::Ordering;

    use crate::part2::{
        compare_hand_cards, compare_hands, hand_type, parse_line, solve, Hand, HandWithBet, optimize_hand,
    };

    #[test]
    fn full_test() {
        assert_eq!(solve("sample.txt"), 5905);
    }

    #[test]
    fn hand_parser() {
        assert_eq!(hand_type("AAAAA"), Some(Hand::FiveOfAKind));
        assert_eq!(hand_type("AA8AA"), Some(Hand::FourOfAKind));
        assert_eq!(hand_type("23332"), Some(Hand::FullHouse));
        assert_eq!(hand_type("TTT98"), Some(Hand::ThreeOfAKind));
        assert_eq!(hand_type("23432"), Some(Hand::TwoPair));
        assert_eq!(hand_type("A23A4"), Some(Hand::OnePair));
        assert_eq!(hand_type("23456"), Some(Hand::HighCard));
    }

    #[test]
    fn line_parser() {
        assert_eq!(
            parse_line("32T3K 765"),
            HandWithBet {
                hand: "32T3K".to_string(),
                optimized_hand: "32T3K".to_string(),
                bet: 765
            }
        );
        assert_eq!(
            parse_line("KK677 28"),
            HandWithBet {
                hand: "KK677".to_string(),
                optimized_hand: "KK677".to_string(),
                bet: 28
            }
        );
    }

    #[test]
    fn card_comparison() {
        assert_eq!(compare_hand_cards("KK677", "KTJJT"), Ordering::Greater);
        assert_eq!(compare_hand_cards("KTJJT", "KK677"), Ordering::Less);
        assert_eq!(compare_hand_cards("KK677", "KK677"), Ordering::Equal);
    }

    #[test]
    fn hand_comparison() {
        let c1 = HandWithBet {
            hand: "32T3K".to_string(),
            optimized_hand: "32T3K".to_string(),
            bet: 123,
        };
        let c2 = HandWithBet {
            hand: "KK677".to_string(),
            optimized_hand: "KK677".to_string(),
            bet: 123,
        };
        let c3 = HandWithBet {
            hand: "KTJJT".to_string(),
            optimized_hand: "KTTTT".to_string(),
            bet: 123,
        };
        assert_eq!(compare_hands(&c1, &c2), Ordering::Less);
        assert_eq!(compare_hands(&c2, &c1), Ordering::Greater);
        assert_eq!(compare_hands(&c1, &c1), Ordering::Equal);
        assert_eq!(compare_hands(&c2, &c3), Ordering::Less);
        assert_eq!(compare_hands(&c3, &c2), Ordering::Greater);
    }

    // #[test] // some of those tests are not deterministic (output relies on the order of the keys in an HashMap)
    fn check_optimization() {
        assert_eq!(optimize_hand("QQQQJ"), "QQQQQ".to_string()); // four of a kind
        assert_eq!(optimize_hand("JJJJQ"), "QQQQQ".to_string());
        assert_eq!(optimize_hand("JJQQQ"), "QQQQQ".to_string()); // full house
        assert_eq!(optimize_hand("QJJJ2"), "QQQQ2".to_string()); // three of a kind
        assert_eq!(optimize_hand("QQQJ2"), "QQQQ2".to_string());
        assert_eq!(optimize_hand("QQ22J"), "QQQ22".to_string()); // two pair
        assert_eq!(optimize_hand("QQJJ2"), "QQQQ2".to_string());
    }
}
