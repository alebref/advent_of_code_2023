use super::Number;

#[derive(Default, Debug, PartialEq)]
pub(crate) struct Adjacency {
    pub(crate) is_adjacent_to_a_symbol: bool,
    pub(crate) gear: Option<Gear>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub(crate) struct Gear {
    pub(crate) line_index: usize,
    pub(crate) char_index: usize,
}

impl Adjacency {
    fn merge(self, other: Self) -> Self {
        Self {
            is_adjacent_to_a_symbol: self.is_adjacent_to_a_symbol || other.is_adjacent_to_a_symbol,
            gear: self.gear.or(other.gear),
        }
    }
}

impl Number {
    fn get_adjacency_on_left_or_right_char(self, line: &str, char_index: usize) -> Adjacency {
        let adjacent_char = line.as_bytes()[char_index] as char;
        Adjacency {
            is_adjacent_to_a_symbol: is_symbol(adjacent_char),
            gear:
                if adjacent_char == '*' {
                    Some(Gear {
                        line_index: self.line_index,
                        char_index,
                    })
                } else {
                    None
                },
        }
    }

    fn get_adjacency_for_upper_or_lower_line(self, line: &str, line_index: usize) -> Adjacency {
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
        let adjacent_chars = line.get(leftmost_char_index..=rightmost_char_index).unwrap();
        let mut is_adjacent_to_a_symbol = false;
        let mut gear_index = None;
        for (offset, adjacent_char) in adjacent_chars.chars().enumerate() {
            if is_symbol(adjacent_char) {
                is_adjacent_to_a_symbol = true;
            }
            if adjacent_char == '*' {
                gear_index = Some(Gear {
                    line_index,
                    char_index: leftmost_char_index + offset
                });
                break;
            }
        }
        Adjacency {
            is_adjacent_to_a_symbol,
            gear: gear_index,
        }
    }

    pub(crate) fn get_adjacency(&self, lines: &[String]) -> Adjacency {
        let mut adjacency = Adjacency::default();
        let line = lines[self.line_index].as_str();
        if self.start_index > 0 {
            adjacency = adjacency.merge(self.get_adjacency_on_left_or_right_char(line, self.start_index - 1));
        }
        if self.end_index < line.len() - 1 {
            adjacency = adjacency.merge(self.get_adjacency_on_left_or_right_char(line, self.end_index + 1));
        }
        if self.line_index > 0 {
            let upper_line = lines[self.line_index - 1].as_str();
            adjacency = adjacency.merge(self.get_adjacency_for_upper_or_lower_line(upper_line, self.line_index - 1));
        }
        if self.line_index < lines.len() - 1 {
            let lower_line = lines[self.line_index + 1].as_str();
            adjacency = adjacency.merge(self.get_adjacency_for_upper_or_lower_line(lower_line, self.line_index + 1));
        }
        adjacency
    }
}

fn is_symbol(c: char) -> bool {
    !c.is_ascii_digit() && c != '.'
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_adjacency() {
        let lines = &[
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
        const NUMBER_467: Number = Number { value: 467, line_index: 0, start_index: 0, end_index: 2 };
        const ADJACENCY_467: Adjacency = Adjacency { is_adjacent_to_a_symbol: true, gear: Some(Gear { line_index: 1, char_index: 3 }) };
        assert_eq!(ADJACENCY_467, NUMBER_467.get_adjacency(lines));

        const NUMBER_114: Number = Number { value: 114, line_index: 0, start_index: 5, end_index: 7 };
        const ADJACENCY_114: Adjacency = Adjacency { is_adjacent_to_a_symbol: false, gear: None };
        assert_eq!(ADJACENCY_114, NUMBER_114.get_adjacency(lines));

        const NUMBER_617: Number = Number { value: 617, line_index: 4, start_index: 0, end_index: 2 };
        const ADJACENCY_617: Adjacency = Adjacency { is_adjacent_to_a_symbol: true, gear: Some(Gear { line_index: 4, char_index: 3 }) };
        assert_eq!(ADJACENCY_617, NUMBER_617.get_adjacency(lines));

        const NUMBER_58: Number = Number { value: 58, line_index: 5, start_index: 7, end_index: 8 };
        const ADJACENCY_58: Adjacency = Adjacency { is_adjacent_to_a_symbol: false, gear: None };
        assert_eq!(ADJACENCY_58, NUMBER_58.get_adjacency(lines));

        const NUMBER_664: Number = Number { value: 664, line_index: 9, start_index: 1, end_index: 3 };
        const ADJACENCY_664: Adjacency = Adjacency { is_adjacent_to_a_symbol: true, gear: None };
        assert_eq!(ADJACENCY_664, NUMBER_664.get_adjacency(lines));
    }
}
