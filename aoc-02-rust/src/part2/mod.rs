use crate::{Game, Set};

impl Set {
    fn power(&self) -> u32 {
        self.red  * self.green * self.blue
    }
}

pub(crate) fn solve(lines: &[String]) -> u32 {
    lines.iter()
        .map(|line| Game::from(line.as_str()).max_set.power())
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_power() {
        assert_eq!(24,Set { red: 2, green: 3, blue: 4 }.power());
    }

    #[test]
    fn test_solve() {
        let lines = [
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string(),
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue".to_string(),
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red".to_string(),
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red".to_string(),
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_string(),
        ];

       assert_eq!(2286, solve(lines.as_ref()));
    }
}
