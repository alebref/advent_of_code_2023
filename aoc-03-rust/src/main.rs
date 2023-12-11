mod number;

use std::collections::HashMap;
use number::Number;
use number::adjacency::Gear;

fn main() {
    let lines = get_input_lines("./src/input/input.txt");
    let result = solve(lines.as_slice());
    println!("{}", result.sum_of_part_numbers);
    println!("{}", result.sum_of_gear_ratios);
}

fn get_input_lines(path: &str) -> Vec<String> {
    std::fs::read_to_string(path).expect("The input couldn't be read")
        .trim_end()
        .lines()
        .map(str::to_string)
        .collect()
}

struct Result {
    sum_of_part_numbers: u32,
    sum_of_gear_ratios: u32,
}

#[derive(Clone, Copy)]
struct Ratio {
    value: u32,
    count: u32,
}

impl Ratio {
    fn multiply(self, number: Number) -> Self {
        Self {
            value: self.value * number.value,
            count: self.count + 1,
        }
    }
}

pub(crate) fn solve(lines: &[String]) -> Result {
    let mut part_numbers = Vec::new();
    let mut ratio_by_gear = HashMap::<Gear, Ratio>::new();

    let mut on_number_end = |number: Number| {
        let adjacency = number.get_adjacency(lines);
        if adjacency.is_adjacent_to_a_symbol {
            part_numbers.push(number);
            if let Some(gear) = adjacency.gear {
                let ratio = ratio_by_gear.get(&gear).unwrap_or(&Ratio { value: 1, count: 0 });
                ratio_by_gear.insert(gear, ratio.multiply(number));
            }
        }
    };

    for (line_index, line) in lines.iter().enumerate() {
        let mut current_number: Option<Number> = None;
        for (char_index, char) in line.chars().enumerate() {
            if char.is_ascii_digit() {
                if let Some(n) = current_number {
                    current_number = Some(n.adding_digit(char));
                } else {
                    current_number = Some(Number::single_digit(line_index, char_index, char));
                }
            } else if let Some(number) = current_number {
                on_number_end(number);
                current_number = None;
            }
        }
        if let Some(number) = current_number {
            on_number_end(number);
        }
    }

    let sum_of_part_numbers = part_numbers.iter()
        .map(|n| n.value)
        .sum();
    let sum_of_gear_ratios = ratio_by_gear.values()
        .filter_map(|ratio|
            if ratio.count == 2 {
               Some(ratio.value)
            } else {
                None
            })
        .sum();
    Result {
        sum_of_part_numbers,
        sum_of_gear_ratios,
    }
}
