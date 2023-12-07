fn main() {
    let result = solve_part1("/media/alebref/DATA/dev/aoc-2023/aoc-01/src/input.txt");
    println!("{}", result);

    let result = solve_part2("/media/alebref/DATA/dev/aoc-2023/aoc-01/src/input.txt");
    println!("{}", result);
}

fn parse(path: &str) -> Vec<String> {
    std::fs::read_to_string(path)
        .unwrap()
        .split_ascii_whitespace()
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
}

fn get_number_from_line_part1(line: &str) -> u32 {
    let digits = line.chars()
        .into_iter()
        .filter(|c| c.is_ascii_digit())
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<u32>>();
    digits[0] * 10 + digits[digits.len() - 1]
}

fn solve_part1(path: &str) -> u32 {
    let lines = parse(path);
    lines.iter()
        .map(|line| get_number_from_line_part1(line))
        .sum()
}

const VALID_DIGITS: [&str; 18] = [
    "1", "2", "3", "4", "5", "6", "7", "8" ,"9",
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn get_number_from_line_part2(line: &str) -> u32 {
    let matches_of_each_digit = VALID_DIGITS
        .map(|digit: &str| {
            let matches = line.match_indices(&digit).collect::<Vec<(usize, &str)>>();
            if matches.is_empty() {
                return None
            }
            let index_of_first_match = matches[0].0;
            let index_of_last_match = matches[matches.len() - 1].0;
            Some((index_of_first_match, index_of_last_match))
        });
    let (first_matched_digit, _) = matches_of_each_digit.iter()
        .enumerate()
        .fold(None, |optional_first_match, (index_in_valid_digits, optional_matches)| {
            if let Some((index_of_first_match, _)) = optional_matches {
                if let Some((_, first_match_index)) = optional_first_match {
                    if *index_of_first_match < first_match_index {
                        return Some(((index_in_valid_digits % 9 + 1) as u32, *index_of_first_match));
                    }
                } else {
                    return Some(((index_in_valid_digits % 9 + 1) as u32, *index_of_first_match));;
                }
            }
            optional_first_match
        })
        .unwrap();
    let (last_matched_digit, _) = matches_of_each_digit.iter()
        .enumerate()
        .fold(None, |optional_last_match, (index_in_valid_digits, optional_matches)| {
            if let Some((_, index_of_last_match)) = optional_matches {
                if let Some((_, last_match_index)) = optional_last_match {
                    if *index_of_last_match > last_match_index {
                        return Some(((index_in_valid_digits % 9 + 1) as u32, *index_of_last_match));
                    }
                } else {
                    return Some(((index_in_valid_digits % 9 + 1) as u32, *index_of_last_match));
                }
            }
            optional_last_match
        })
        .unwrap();
    first_matched_digit * 10 + last_matched_digit
}

fn solve_part2(path: &str) -> u32 {
    let lines = parse(path);
    lines.iter()
        .map(|line| get_number_from_line_part2(line))
        .sum()
}

#[test]
fn test() {
    let lines_part1 = parse("/media/alebref/DATA/dev/aoc-2023/aoc-01/src/test_input_part1.txt");
    assert_eq!(4, lines_part1.len());

    const LINES_PART1: [&str; 4] = [
        "1abc2",
        "pqr3stu8vwx",
        "a1b2c3d4e5f",
        "treb7uchet",
    ];
    const NUMS_PART1: [u32; 4] = [12, 38, 15, 77];
    
    for (index, line) in lines_part1.iter().enumerate() {
        assert_eq!(LINES_PART1[index], line);
        let num = get_number_from_line_part1(line);
        assert_eq!(NUMS_PART1[index], num);
    }

    let result = solve_part1("/media/alebref/DATA/dev/aoc-2023/aoc-01/src/test_input_part1.txt");
    assert_eq!(142, result);

    let lines_part2 = parse("/media/alebref/DATA/dev/aoc-2023/aoc-01/src/test_input_part2.txt");
    assert_eq!(7, lines_part2.len());

    const LINES_PART2: [&str; 7] = [
        "two1nine",
        "eightwothree",
        "abcone2threexyz",
        "xtwone3four",
        "4nineeightseven2",
        "zoneight234",
        "7pqrstsixteen",
    ];
    const NUMS_PART2: [u32; 7] = [29, 83, 13, 24, 42, 14, 76];

    for (index, line) in lines_part2.iter().enumerate() {
        assert_eq!(LINES_PART2[index], line);
        let num = get_number_from_line_part2(line);
        assert_eq!(NUMS_PART2[index], num);
    }

    let result = solve_part2("/media/alebref/DATA/dev/aoc-2023/aoc-01/src/test_input_part2.txt");
    assert_eq!(281, result);
}