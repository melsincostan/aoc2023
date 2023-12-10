use std::cmp;
use std::fs;

struct Pull {
    color: String,
    number: i32,
}

#[derive(Debug)]
struct Play {
    red: i32,
    green: i32,
    blue: i32,
}

#[derive(PartialEq, Debug)]
struct Game {
    id: u32,
    plays: Vec<Play>,
}

impl PartialEq for Play {
    fn eq(&self, other: &Self) -> bool {
        self.red == other.red && self.green == other.green && self.blue == other.blue
    }
}

pub fn solve(input: &str) -> i32 {
    let games: Vec<Game> = fs::read_to_string(input).unwrap().lines().map(parse_game).collect();
    let minimums: Vec<Play> = games.iter().map(min_cubes).collect();
    minimums.iter().map(cubes_power).sum()
    
}

fn cubes_power(play: &Play) -> i32 {
    play.red * play.green * play.blue
}

fn min_cubes(game: &Game) -> Play {
    let mut max_red = 0;
    let mut max_green = 0;
    let mut max_blue = 0;
    for play in &game.plays {
        max_red = cmp::max(max_red, play.red);
        max_green = cmp::max(max_green, play.green);
        max_blue = cmp::max(max_blue, play.blue);
    }
    Play {
        red: max_red,
        green: max_green,
        blue: max_blue,
    }
}

fn parse_game(line: &str) -> Game {
    let raw_meta = line.split(": ").next().unwrap();
    let raw_game = line.split(": ").last().unwrap();

    let game_id = raw_meta
        .strip_prefix("Game ")
        .unwrap()
        .parse::<u32>()
        .unwrap();
    let plays: Vec<Play> = raw_game.split("; ").map(parse_play).collect();

    Game {
        id: game_id,
        plays: plays,
    }
}

fn parse_play(play: &str) -> Play {
    let pulls: Vec<Pull> = play.split(", ").map(parse_pull).collect();
    let mut total = Play {
        red: 0,
        green: 0,
        blue: 0,
    };
    for pull in pulls {
        match pull.color.as_str() {
            "red" => total.red += pull.number,
            "green" => total.green += pull.number,
            "blue" => total.blue += pull.number,
            _ => panic!("Parsing error: {} unmatched!", pull.color),
        }
    }
    total
}

fn parse_pull(pull: &str) -> Pull {
    let raw_number = pull.split(" ").next().unwrap();
    let color = pull.split(" ").last().unwrap();
    let number = raw_number.parse::<i32>().unwrap();
    Pull {
        color: color.to_string(),
        number: number,
    }
}

#[cfg(test)]
mod test {
    use crate::part2::{min_cubes, parse_game, Play, cubes_power};

    #[test]
    fn cube_opt() {
        let game1 = parse_game("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
        assert_eq!(
            min_cubes(&game1),
            Play {
                red: 4,
                green: 2,
                blue: 6
            }
        );
        let game2 = parse_game("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue");
        assert_eq!(
            min_cubes(&game2),
            Play {
                red: 1,
                green: 3,
                blue: 4
            }
        );
    }

    #[test]
    fn combo_power() {
        let combo1 = min_cubes(&parse_game("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"));
        let combo2 = min_cubes(&parse_game("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"));

        assert_eq!(cubes_power(&combo1), 48);
        assert_eq!(cubes_power(&combo2), 12);
    }
}
