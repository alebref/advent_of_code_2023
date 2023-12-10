const VALID_DIGITS: [&str; 18] = [
    "1", "2", "3", "4", "5", "6", "7", "8" ,"9",
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn index_to_digit(index: usize) -> u32 {
    (index % 9 + 1) as u32
}

#[derive(Debug, PartialEq)]
struct DigitMatches {
    digit: u32,
    first_index: usize,
    last_index: usize,
}

fn get_digit_matches(line: &str) -> Vec<DigitMatches> {
    VALID_DIGITS.iter().enumerate()
        .filter_map(|(index, digit)| {
            let matches = line.match_indices(digit).collect::<Vec<_>>();
            if matches.is_empty() {
                return None;
            }
            Some(DigitMatches {
                digit: index_to_digit(index),
                first_index: matches[0].0,
                last_index: matches[matches.len() - 1].0,
            })
        })
        .collect()
}

fn get_number(line: &str) -> u32 {
    let matches = get_digit_matches(line);
    let (first_digit, _, last_digit, _) = matches.iter()
        .fold((0, usize::MAX, 0, usize::MIN), |(first_digit, first_index, last_digit, last_index), matches| {
            let mut new_first_digit = first_digit;
            let mut new_first_index = first_index;
            let mut new_last_digit = last_digit;
            let mut new_last_index = last_index;
            if matches.first_index < first_index {
                new_first_digit = matches.digit;
                new_first_index = matches.first_index;
            }
            if matches.last_index > last_index {
                new_last_digit = matches.digit;
                new_last_index = matches.last_index;
            }
            (new_first_digit, new_first_index, new_last_digit, new_last_index)
        });
    first_digit * 10 + last_digit
}

pub(crate) fn solve(lines: &[String]) -> u32 {
    lines.iter()
        .map(|line| get_number(line))
        .sum()
}

#[cfg(test)]
mod test {
    use std::cmp::Ordering;
    use super::*;

    #[test]
    fn test_index_to_digit() {
        const DIGITS: [u32; VALID_DIGITS.len()] = [
            1, 2, 3, 4, 5, 6, 7, 8, 9,
            1, 2, 3, 4, 5, 6, 7, 8, 9,
        ];

        for index in 0..VALID_DIGITS.len() {
            assert_eq!(DIGITS[index], index_to_digit(index));
        }
    }

    #[test]
    fn test_get_digit_matches() {
        const COMPARE_DIGITS: for<'a, 'b> fn(&'a DigitMatches, &'b DigitMatches) -> Ordering = |a, b| a.digit.cmp(&b.digit);

        const LINE_1: &str = "4nineeightseven2";

        let mut expected_matches = vec![
            DigitMatches { digit: 4, first_index: 0, last_index: 0 },
            DigitMatches { digit: 9, first_index: 1, last_index: 1 },
            DigitMatches { digit: 8, first_index: 5, last_index: 5 },
            DigitMatches { digit: 7, first_index: 10, last_index: 10 },
            DigitMatches { digit: 2, first_index: 15, last_index: 15 },
        ];
        expected_matches.sort_by(COMPARE_DIGITS);

        let mut matches =  get_digit_matches(LINE_1);
        matches.sort_by(COMPARE_DIGITS);

        assert_eq!(expected_matches, matches);

        const LINE_2: &str = "2x2x2";

        let mut expected_matches = vec![
            DigitMatches { digit: 2, first_index: 0, last_index: 4 },
        ];
        expected_matches.sort_by(COMPARE_DIGITS);

        let mut matches =  get_digit_matches(LINE_2);
        matches.sort_by(COMPARE_DIGITS);

        assert_eq!(expected_matches, matches);
    }

    #[test]
    fn test_get_number_from_line() {
        const LINES: [&str; 7] = [
            "two1nine",
            "eightwothree",
            "abcone2threexyz",
            "xtwone3four",
            "4nineeightseven2",
            "zoneight234",
            "7pqrstsixteen",
        ];

        const NUMS: [u32; 7] = [29, 83, 13, 24, 42, 14, 76];

        for (line, num) in LINES.iter().zip(NUMS) {
            assert_eq!(num, get_number(line));
        }
    }

    #[test]
    fn test_solve() {
        let lines = [
            "two1nine".to_string(),
            "eightwothree".to_string(),
            "abcone2threexyz".to_string(),
            "xtwone3four".to_string(),
            "4nineeightseven2".to_string(),
            "zoneight234".to_string(),
            "7pqrstsixteen".to_string(),
        ];

        assert_eq!(281, solve(lines.as_ref()));
    }
}
