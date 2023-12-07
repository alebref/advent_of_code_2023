fn main() {
    let result = solve("/media/alebref/DATA/dev/aoc-2023/aoc-01/src/input.txt");
    println!("{}", result);
}

fn parse(path: &str) -> Vec<String> {
    std::fs::read_to_string(path)
        .unwrap()
        .split_ascii_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
}

fn get_number_from_line(line: &str) -> u32 {
    let digits = line.chars()
        .into_iter()
        .filter(|c| c.is_ascii_digit())
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<u32>>();
    digits[0] * 10 + digits[digits.len() - 1]
}

fn solve(path: &str) -> u32 {
    let lines = parse(path);
    lines.iter()
        .map(|line| get_number_from_line(line))
        .sum()
}

#[test]
fn test() {
    let lines = parse("/media/alebref/DATA/dev/aoc-2023/aoc-01/src/test_input.txt");
    assert_eq!(4, lines.len());

    const LINES: [&str; 4] = [
        "1abc2",
        "pqr3stu8vwx",
        "a1b2c3d4e5f",
        "treb7uchet",
    ];
    const NUMS: [u32; 4] = [12, 38, 15, 77];
    
    for (index, line) in lines.iter().enumerate() {
        assert_eq!(LINES[index], line);
        let num = get_number_from_line(line);
        assert_eq!(NUMS[index], num);
    }

    let result = solve("/media/alebref/DATA/dev/aoc-2023/aoc-01/src/test_input.txt");
    assert_eq!(142, result);
}