use std::fs;

#[derive(PartialEq, Debug)]
struct Maprange {
    dest_range_start: i64,
    src_range_start: i64,
    length: i64,
}

pub fn solve(input: &str) -> i64 {
    let file = fs::read_to_string(input).unwrap();
    let blocks: Vec<&str> = file.split("\n\n").collect();
    assert_eq!(blocks.len(), 8);
    let seeds: Vec<i64> = blocks[0]
        .strip_prefix("seeds: ")
        .unwrap()
        .split_whitespace()
        .map(|seed| seed.parse::<i64>().unwrap())
        .collect();
    let mut map_blocks: Vec<Vec<Maprange>> = vec![];
    for block in &blocks[1..] {
        map_blocks.push(
            block
                .split("\n")
                .filter(filter_line)
                .map(parse_map)
                .collect(),
        );
    }
    let destinations: Vec<i64> = seeds.iter().map(|seed| {
        let mut scratch = *seed;
        for block in &map_blocks {
            for map in block {
                if scratch >= map.src_range_start && scratch <= map.length + map.src_range_start {
                    scratch += map.dest_range_start - map.src_range_start;
                }
            }
        }
        scratch
    }).collect();
    *destinations.iter().min().unwrap()
}

fn filter_line(line: &&str) -> bool {
    line.chars().next().is_some() && line.chars().next().unwrap().is_digit(10)
}

fn parse_map(line: &str) -> Maprange {
    let raw_map: Vec<i64> = line
        .split_whitespace()
        .map(|num| num.parse::<i64>().unwrap())
        .collect();
    assert_eq!(raw_map.len(), 3);
    Maprange {
        dest_range_start: raw_map[0],
        src_range_start: raw_map[1],
        length: raw_map[2],
    }
}

#[cfg(test)]
mod test {
    use crate::part1::{filter_line, parse_map, Maprange};

    #[test]
    fn line_filter() {
        assert!(filter_line(&"0 11 42"));
        assert!(!filter_line(&"fertilizer-to-water map:"));
        assert!(!filter_line(&""));
    }

    #[test]
    fn line_parser() {
        assert_eq!(
            parse_map("0 15 37"),
            Maprange {
                dest_range_start: 0,
                src_range_start: 15,
                length: 37
            }
        );
        assert_eq!(
            parse_map("50 98 2"),
            Maprange {
                dest_range_start: 50,
                src_range_start: 98,
                length: 2
            }
        );
    }
}
