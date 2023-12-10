#[derive(Debug, PartialEq)]
struct MaybePartNumber {
    number: u32,
    line_index: usize,
    start_index: usize,
    end_index: usize,
}

impl MaybePartNumber {
    fn new(line_index: usize, start_index: usize, digit_char: char) -> Self {
        MaybePartNumber {
            number: digit_char.to_digit(10).unwrap(),
            line_index,
            start_index,
            end_index: start_index,
        }
    }

    fn with_next_digit(self, digit_char: char) -> Self {
        MaybePartNumber {
            number: self.number * 10 + digit_char.to_digit(10).unwrap(),
            end_index: self.end_index + 1,
            ..self
        }
    }

    fn is_adjacent_to_a_symbol_in_upper_or_lower_line(&self, line: &str) -> bool {
        let leftmost_char_index = if self.start_index > 0 {
            self.start_index - 1
        } else {
            self.start_index
        };
        let rightmost_char_index = if self.end_index < line.len() - 1 {
            self.end_index + 1
        } else {
            self.end_index
        };
        let chars = line.get(leftmost_char_index..=rightmost_char_index);
        chars.unwrap().contains(is_symbol)
    }

    fn is_adjacent_to_a_symbol(&self, lines: &[String]) -> bool {
        let line = lines[self.line_index].as_str();
        (self.start_index > 0 && is_nth_char_a_symbol(line, self.start_index - 1))
            || (self.end_index < line.len() - 1 && is_nth_char_a_symbol(line, self.end_index + 1))
            || (self.line_index > 0 && self.is_adjacent_to_a_symbol_in_upper_or_lower_line(lines[self.line_index - 1].as_str()))
            || (self.line_index < lines.len() - 1 && self.is_adjacent_to_a_symbol_in_upper_or_lower_line(lines[self.line_index + 1].as_str()))
    }
}

fn is_symbol(c: char) -> bool {
    !c.is_ascii_digit() && c != '.'
}

fn is_nth_char_a_symbol(line: &str, char_index: usize) -> bool {
    is_symbol(line.as_bytes()[char_index] as char)
}

pub(crate) fn solve(lines: &[String]) -> u32 {
    let mut result = 0;
    for (line_index, line) in lines.iter().enumerate() {
        let mut current_number: Option<MaybePartNumber> = None;
        for (char_index, char) in line.chars().enumerate() {
            if char.is_ascii_digit() {
                if let Some(n) = current_number {
                    current_number = Some(n.with_next_digit(char));
                } else {
                    current_number = Some(MaybePartNumber::new(line_index, char_index, char));
                }
            } else if let Some(n) = current_number {
                if n.is_adjacent_to_a_symbol(lines) {
                    result += n.number;
                }
                current_number = None;
            }
        }
        // end of line but the latest number hasn't been checked yet
        if let Some(n) = current_number {
            if n.is_adjacent_to_a_symbol(lines) {
                result += n.number;
            }
        }
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_maybe_part_number_new() {
        const NUMBER: MaybePartNumber = MaybePartNumber {
            number: 9, line_index: 13, start_index: 42, end_index: 42,
        };

        assert_eq!(NUMBER, MaybePartNumber::new(13, 42, '9'));
    }

    #[test]
    fn test_with_next_digit() {
        const NUMBER_1: MaybePartNumber = MaybePartNumber {
            number: 9, line_index: 13, start_index: 42, end_index: 42,
        };
        const NUMBER_2: MaybePartNumber = MaybePartNumber {
            number: 92, end_index: 43, ..NUMBER_1
        };

        assert_eq!(NUMBER_2, NUMBER_1.with_next_digit('2'));
    }

    #[test]
    fn test_is_symbol() {
        assert_eq!(false, is_symbol('0'));
        assert_eq!(false, is_symbol('9'));
        assert_eq!(false, is_symbol('.'));
        assert_eq!(true, is_symbol('*'));
        assert_eq!(true, is_symbol('#'));
    }

    #[test]
    fn test_is_nth_char_a_symbol() {
        assert_eq!(false, is_nth_char_a_symbol("..3.#", 0));
        assert_eq!(false, is_nth_char_a_symbol("..3.#", 2));
        assert_eq!(true, is_nth_char_a_symbol("..3.#", 4));
    }

    #[test]
    fn test_is_adjacent_to_a_symbol_in_upper_or_lower_line() {
        const NUMBER: MaybePartNumber = MaybePartNumber {
            number: 987, line_index: 1, start_index: 2, end_index: 4,
        };
        const LINE_1: &str = ".*.....";
        const LINE_2: &str = "...*...";
        const LINE_3: &str = ".....*.";
        const LINE_4: &str = "*......";
        const LINE_5: &str = "......*";

        assert_eq!(true, NUMBER.is_adjacent_to_a_symbol_in_upper_or_lower_line(LINE_1));
        assert_eq!(true, NUMBER.is_adjacent_to_a_symbol_in_upper_or_lower_line(LINE_2));
        assert_eq!(true, NUMBER.is_adjacent_to_a_symbol_in_upper_or_lower_line(LINE_3));
        assert_eq!(false, NUMBER.is_adjacent_to_a_symbol_in_upper_or_lower_line(LINE_4));
        assert_eq!(false, NUMBER.is_adjacent_to_a_symbol_in_upper_or_lower_line(LINE_5));
    }

    #[test]
    fn test_is_adjacent_to_a_symbol() {
        const NUMBER_114: MaybePartNumber = MaybePartNumber {
            number: 114, line_index: 0, start_index: 5, end_index: 7,
        };
        const NUMBER_58: MaybePartNumber = MaybePartNumber {
            number: 58, line_index: 5, start_index: 7, end_index: 8,
        };
        const NUMBER_617: MaybePartNumber = MaybePartNumber {
            number: 617, line_index: 4, start_index: 0, end_index: 2,
        };
        const NUMBER_633: MaybePartNumber = MaybePartNumber {
            number: 633, line_index: 2, start_index: 6, end_index: 8,
        };
        const NUMBER_467: MaybePartNumber = MaybePartNumber {
            number: 467, line_index: 0, start_index: 0, end_index: 2,
        };
        let lines = [
            "467..114..".to_string(),
            "...*......".to_string(),
            "..35..633.".to_string(),
            "......#...".to_string(),
            "617*......".to_string(),
            ".....+.58.".to_string(),
            "..592.....".to_string(),
            "......755.".to_string(),
            "...$.*....".to_string(),
            ".664.598..".to_string(),
        ];

        assert_eq!(false, NUMBER_114.is_adjacent_to_a_symbol(&lines));
        assert_eq!(false, NUMBER_58.is_adjacent_to_a_symbol(&lines));
        assert_eq!(true, NUMBER_617.is_adjacent_to_a_symbol(&lines));
        assert_eq!(true, NUMBER_633.is_adjacent_to_a_symbol(&lines));
        assert_eq!(true, NUMBER_467.is_adjacent_to_a_symbol(&lines));
    }

    #[test]
    fn test_solve() {
        let lines = [
            "467..114..".to_string(),
            "...*......".to_string(),
            "..35..633.".to_string(),
            "......#...".to_string(),
            "617*......".to_string(),
            ".....+.58.".to_string(),
            "..592.....".to_string(),
            "......755.".to_string(),
            "...$.*....".to_string(),
            ".664.598..".to_string(),
        ];

        assert_eq!(4361, solve(lines.as_ref()));
    }
}