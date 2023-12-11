pub(crate) mod adjacency;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Number {
    pub(crate) value: u32,
    line_index: usize,
    start_index: usize,
    end_index: usize,
}

impl Number {
    pub(crate) fn single_digit(line_index: usize, start_index: usize, digit_char: char) -> Self {
        Self {
            value: digit_char.to_digit(10).unwrap(),
            line_index,
            start_index,
            end_index: start_index,
        }
    }

    pub(crate) fn adding_digit(self, digit_char: char) -> Self {
        Self {
            value: self.value * 10 + digit_char.to_digit(10).unwrap(),
            end_index: self.end_index + 1,
            ..self
        }
    }
}
