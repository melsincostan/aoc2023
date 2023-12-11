use rayon::prelude::*;
// use std::collections::HashMap;
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
    let raw_seeds: Vec<i64> = blocks[0]
        .strip_prefix("seeds: ")
        .unwrap()
        .split_whitespace()
        .map(|seed| seed.parse::<i64>().unwrap())
        .collect();
    assert!(raw_seeds.len() % 2 == 0);
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
    let mut destinations: Vec<i64> = vec![];
    // let mut cache: HashMap<(usize, i64), i64> = Default::default();
    for i in (0..raw_seeds.len()).step_by(2) {
        println!("Going through seed space {} / {}", (i/2)+1, raw_seeds.len()/2);
        destinations.push((raw_seeds[i]..(raw_seeds[i]+raw_seeds[i + 1])).into_par_iter().map(|seed| {
            let mut scratch = seed;
            for block in &map_blocks {
                    for map in block {
                        if scratch >= map.src_range_start
                            && scratch <= map.length + map.src_range_start
                        {
                            scratch += map.dest_range_start - map.src_range_start;
                            break;
                        }
                    
                }
            }
            scratch
        }).min().unwrap());
    }
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
