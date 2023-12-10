mod part1;
mod part2;

fn main() {
    let lines = get_input_lines("./src/input/input.txt");

    let result = part1::solve(lines.as_slice());
    println!("{}", result);

    let result = part2::solve(lines.as_slice());
    println!("{}", result);
}

fn get_input_lines(path: &str) -> Vec<String> {
    std::fs::read_to_string(path).expect("the input couldn't be read")
        .trim_end()
        .lines()
        .map(str::to_string)
        .collect::<Vec<String>>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_input_lines() {
        const LINES: [&str; 4] = [
            "1abc2",
            "pqr3stu8vwx",
            "a1b2c3d4e5f",
            "treb7uchet",
        ];

        assert_eq!(LINES, get_input_lines("./src/input/test_input_part1.txt").as_slice());
    }
}
