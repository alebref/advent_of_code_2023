mod part1;

fn main() {
    let lines = get_input_lines("./src/input/input.txt");

    let result = part1::solve(lines.as_slice());
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
        const LINES: [&str; 10] = [
            "467..114..",
            "...*......",
            "..35..633.",
            "......#...",
            "617*......",
            ".....+.58.",
            "..592.....",
            "......755.",
            "...$.*....",
            ".664.598..",
        ];

        assert_eq!(LINES, get_input_lines("./src/input/test_input.txt").as_slice());
    }
}