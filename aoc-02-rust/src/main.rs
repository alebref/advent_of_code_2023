mod part1;
mod part2;

#[derive(Debug, PartialEq)]
struct Game {
    id: u32,
    max_set: Set,
}

#[derive(Debug, Default, PartialEq)]
struct Set {
    red: u32,
    green: u32,
    blue: u32,
}

impl From<&str> for Game {
    fn from(line: &str) -> Self {
        let game_then_sets = line.split(": ")
            .collect::<Vec<&str>>();
        let id = game_then_sets[0].strip_prefix("Game ")
            .expect("Bad line format, should start with 'Game '")
            .parse::<u32>()
            .expect("Bad game id, should be an integer");
        let max_set =  game_then_sets[1].split("; ")
            .map(Set::from)
            .fold(Set::default(), |max_set, set| Set {
                red: max_set.red.max(set.red),
                green: max_set.green.max(set.green),
                blue: max_set.blue.max(set.blue),
            });
        Game {
            id,
            max_set,
        }
    }
}

impl From<&str> for Set {
    fn from(description: &str) -> Self {
        let mut set = Set::default();
        description.split(", ")
            .for_each(|part| {
                let (count, color) = part.split_once(' ')
                    .expect("Expected '<count> <color>'");
                let count = count.parse::<u32>()
                    .expect("Expected a count");
                match color {
                    "red" => set.red = count,
                    "green" => set.green = count,
                    "blue" => set.blue = count,
                    bad_color => panic!("Bad color: '{}'", bad_color)
                }
            });
        set
    }
}

fn main() {
    let lines = get_input_lines("./src/input/input.txt");

    let result = part1::solve(lines.as_slice());
    println!("{}", result);

    let result = part2::solve(lines.as_slice());
    println!("{}", result);
}

fn get_input_lines(path: &str) -> Vec<String> {
    std::fs::read_to_string(path).expect("The input couldn't be read")
        .trim_end()
        .lines()
        .map(str::to_string)
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_input_lines() {
        const LINES: [&str; 5] = [
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ];

        assert_eq!(LINES, get_input_lines("./src/input/test_input.txt").as_slice());
    }

    #[test]
    fn test_game_from() {
        const GAME: Game = Game {
            id: 1,
            max_set: Set { red: 4, green: 2, blue: 6 },
        };

        assert_eq!(GAME, Game::from("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"));
    }

    #[test]
    fn test_set_from() {
        assert_eq!(Set { red: 0, green: 2, blue: 0 }, Set::from("2 green"));
        assert_eq!(Set { red: 1, green: 2, blue: 6 }, Set::from("1 red, 2 green, 6 blue"));
    }
}
