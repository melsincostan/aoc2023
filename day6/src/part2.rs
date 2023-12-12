use std::fs;
use std::cmp;

#[derive(Debug, PartialEq)]
struct Boatrace {
    time: i64,
    record: i64,
}

pub fn solve(input: &str) -> i64 {
    let raw = open_file(input);
    let race = Boatrace{
        time: raw[0],
        record: raw[1],
    };
    let (lower, upper) = win_values_race(&race).unwrap();
    upper-lower+1
}

fn open_file(input: &str) -> Vec<i64> {
    fs::read_to_string(input).unwrap().lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> i64 {
    let split: Vec<&str> = line.split_whitespace().collect();
    split[1..split.len()].join("").parse::<i64>().unwrap()
}

fn disc_from_race(race: &Boatrace) -> i64 { // part 1 of the quadratic formula for second degree polynomial solving
    // don't mind me repeating stuff so it is easier to figure out the formula in 6 months
    // x amount of time the button is pressed
    // t duration of the race
    // r record of the race
    // distance travelled is: x * (t-x) = tx - x^2 = -x^2 + t
    // x will result in a win if the above minus r is greater than 0
    // so the final polynomial is: -x^2 + t - r
    // using the quadratic formula we can get the two points at which it is 0
    // smallest rounded to next int is lower bound of winning holds
    // highest rounded to previous int is higher bound of winning holds
    // i could have probably brute-forced this way faster
    let a = -1;
    let b = race.time;
    let c = 0-(race.record);
    b*b - 4*a*c
}

fn win_values_race(race: &Boatrace) -> Option<(i64, i64)>{
    // see disc_from_race to see which value is which
    let disc = disc_from_race(race) as f64;
    if disc > 0.0 { // don't want to deal with complex numbers here
        let bound_1 = ((-race.time as f64) + disc.sqrt()) / -2.0;
        let bound_2 = ((-race.time as f64) - disc.sqrt()) / -2.0;

        let lower = cmp::min(bound_1.floor() as i64, bound_2.floor() as i64) + 1; // floor then add 1 to deal with integers
        let higher = cmp::max(bound_1.ceil() as i64, bound_2.ceil() as i64) - 1;  // ceil then substract 1 to deal with integers
        return Some((lower, higher));
    }
    None
}

#[cfg(test)]
mod test {
    use crate::part2::{parse_line, solve};

    #[test]
    fn line_parsing() {
        assert_eq!(parse_line("Time:      7  15   30"), 71530);
        assert_eq!(parse_line("Distance:  9  40  200"), 940200);
    }

    #[test]
    fn full() {
        assert_eq!(solve("sample.txt"), 71503);
    }
}