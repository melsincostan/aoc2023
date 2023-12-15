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

pub fn solve(input: &str) -> u32 {
    fs::read_to_string(input)
        .unwrap()
        .lines()
        .map(parse_game)
        .map(|game| if game_possible(&game) {game.id} else {0})
        .sum()
}

fn game_possible(game: &Game) -> bool {
    // Contraints: 12 reds, 13 greens, 14 blue
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;
    for play in &game.plays {
        if play.red > max_red || play.green > max_green || play.blue > max_blue {
            return false;
        }
    }
    true
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
    use crate::part1::{game_possible, parse_game, parse_play, parse_pull, Game, Play};

    #[test]
    fn pull_parsing() {
        assert_eq!(parse_pull("2 green").color, "green");
        assert_eq!(parse_pull("2 green").number, 2);
    }

    #[test]
    fn play_parsing() {
        assert_eq!(parse_play("3 blue, 4 red").blue, 3);
        assert_eq!(parse_play("3 blue, 4 red").red, 4);
        assert_eq!(parse_play("3 blue, 4 red").green, 0);
    }

    #[test]
    fn game_parsing() {
        assert_eq!(
            parse_game("Game 12: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"),
            Game {
                id: 12,
                plays: vec![
                    Play {
                        blue: 1,
                        green: 2,
                        red: 0,
                    },
                    Play {
                        blue: 4,
                        green: 3,
                        red: 1,
                    },
                    Play {
                        blue: 1,
                        green: 1,
                        red: 0,
                    }
                ],
            }
        )
    }

    #[test]
    fn game_testing() {
        let parsed_1 = parse_game("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
        let parsed_2 =
            parse_game("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red");
        assert!(game_possible(&parsed_1));
        assert!(!game_possible(&parsed_2));
    }
}
