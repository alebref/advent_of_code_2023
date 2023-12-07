#[derive(Debug)]
struct Game {
    id: u8,
    sets: Vec<Set>,
}

#[derive(Debug)]
#[derive(PartialEq)]
struct Set {
    red: u8,
    green: u8,
    blue: u8,
}

impl From<&str> for Game {
    fn from(line: &str) -> Self {
        let game_then_sets = line.split(": ")
            .collect::<Vec<&str>>();
        let id = game_then_sets[0].strip_prefix("Game ")
            .unwrap()
            .parse::<u8>()
            .unwrap();
        let sets =  game_then_sets[1].split("; ")
            .map(|set| Set::from(set))
            .collect::<Vec<Set>>();
        Game {
            id,
            sets,
        }
    }
}

impl Default for Set {
    fn default() -> Self {
        Self {
            red: 0,
            green: 0,
            blue: 0,
        }
    }
}

impl From<&str> for Set {
    fn from(description: &str) -> Self {
        let mut set = Set::default();
        description.split(", ")
            .into_iter()
            .for_each(|part| {
                let count_then_color = part.split_ascii_whitespace()
                    .collect::<Vec<&str>>();
                let count = count_then_color[0].parse::<u8>().unwrap();
                match count_then_color[1] {
                    "red" => set.red = count,
                    "green" => set.green = count,
                    "blue" => set.blue = count,
                    bad_color => panic!("bad color: '{}'", bad_color)
                }
            });
        set
    }
}

impl PartialEq for Game {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id 
            && self.sets.iter()
                .enumerate()
                .all(|(index, set)| *set == other.sets[index])
    }
}

fn main() {
    let result = solve("/media/alebref/DATA/dev/aoc-2023/aoc-02/src/input.txt");
    println!("{}", result);
}

fn solve(path: &str) -> u32 {
    let lines = parse(path);
    lines.iter()
        .map(|line| Game::from(line.as_str()))
        .filter(|game| {
            let max_set = compute_max_set(&game.sets);
            max_set.red <= 12
                && max_set.green <= 13
                && max_set.blue <= 14
        })
        .map(|game| game.id as u32)
        .sum()
}

fn parse(path: &str) -> Vec<String> {
    std::fs::read_to_string(path)
        .unwrap()
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
}

fn compute_max_set(sets: &[Set]) -> Set {
    let mut max_set = Set::default();
    for set in sets {
        max_set.red = max_set.red.max(set.red);
        max_set.green = max_set.green.max(set.green);
        max_set.blue = max_set.blue.max(set.blue);
    }
    max_set
}

#[test]
fn test() {
    let lines = parse("/media/alebref/DATA/dev/aoc-2023/aoc-02/src/test_input.txt");
    assert_eq!(5, lines.len());

    const LINE: &str = "Game 5: 19 red, 1 green; 7 red, 1 green, 1 blue; 7 red; 13 red, 2 green";
    const SET1: Set = Set { red: 19, green: 1, blue: 0 };
    const SET2: Set = Set { red: 7, green: 1, blue: 1 };
    const SET3: Set = Set { red: 7, green: 0, blue: 0 };
    const SET4: Set = Set { red: 13, green: 2, blue: 0 };

    assert_eq!(SET1, Set::from("19 red, 1 green"));
    assert_eq!(SET2, Set::from("7 red, 1 green, 1 blue"));
    assert_eq!(SET3, Set::from("7 red"));

    let game = Game { id: 5, sets: vec![ SET1, SET2, SET3, SET4 ] };
    assert_eq!(game, Game::from(LINE));

    let max_set = compute_max_set(&game.sets);
    const MAX_SET: Set = Set { red: 19, green: 2, blue: 1 };
    assert_eq!(MAX_SET, max_set);

    let result = solve("/media/alebref/DATA/dev/aoc-2023/aoc-02/src/test_input.txt");
    assert_eq!(8, result)
}