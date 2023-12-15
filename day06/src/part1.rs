use std::fs;
use std::cmp;

#[derive(Debug, PartialEq)]
struct Boatrace {
    time: i64,
    record: i64,
}

pub fn solve(input: &str) -> i64 {
    let raw = open_file(input);
    let races = bind_lines(&raw[0], &raw[1]);
    races.iter().map(|race| {
        let (lower, higher) = win_values_race(&race).unwrap();
        higher-lower+1
    }).reduce(|acc, val| acc * val).unwrap()
}

fn open_file(input: &str) -> Vec<Vec<i64>> {
    fs::read_to_string(input).unwrap().lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> Vec<i64> {
    let split: Vec<&str> = line.split_whitespace().collect();
    split[1..split.len()].iter().map(|num| num.parse::<i64>().unwrap()).collect()
}

fn bind_lines(times: &Vec<i64>, distances: &Vec<i64>) -> Vec<Boatrace> {
    assert!(times.len() <= distances.len());
    let mut races: Vec<Boatrace> = vec![];
    for i in 0..times.len() {
        races.push(Boatrace{
            time: times[i],
            record: distances[i],
        })
    }
    races
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
    use crate::part1::{parse_line, open_file, Boatrace, disc_from_race, win_values_race, solve};

    use super::bind_lines;

    #[test]
    fn line_parsing() {
        assert_eq!(parse_line("Time:      7  15   30"), vec![7, 15, 30]);
        assert_eq!(parse_line("Distance:  9  40  200"), vec![9, 40, 200]);
    }

    #[test]
    fn file_opening() {
        assert_eq!(open_file("sample.txt").len(), 2);
    }

    #[test]
    fn lines_binding() {
        let lines = open_file("sample.txt");
        let races = bind_lines(&lines[0], &lines[1]);
        assert_eq!(races.len(), 3);
        assert_eq!(races[0], Boatrace{
            time: 7,
            record: 9,
        });
        assert_eq!(races[1], Boatrace{
            time: 15,
            record: 40,
        });
        assert_eq!(races[2], Boatrace{
            time: 30,
            record: 200,
        });
    }

    #[test]
    fn test_disc() {
        let race = Boatrace{
            time: 7,
            record: 9,
        };
        assert_eq!(disc_from_race(&race), 13);
    }

    #[test]
    fn test_hold() {
        let race_1 = Boatrace{
            time: 7,
            record: 9,
        };
        let race_2 = Boatrace{
            time: 15,
            record: 40,
        };
        let race_3 = Boatrace{
            time: 30,
            record: 200,
        };
        assert_eq!(win_values_race(&race_1), Some((2, 5)));
        assert_eq!(win_values_race(&race_2), Some((4, 11)));
        assert_eq!(win_values_race(&race_3), Some((11, 19)));
    }

    #[test]
    fn full() {
        assert_eq!(solve("sample.txt"), 288);
    }
}