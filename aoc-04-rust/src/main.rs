use std::collections::HashSet;

fn main() {
    let lines = get_input_lines("./src/input/input.txt");
    let result = solve(lines.as_slice());
    println!("{}", result);
}

fn get_input_lines(path: &str) -> Vec<String> {
    std::fs::read_to_string(path).expect("The input couldn't be read")
        .trim_end()
        .lines()
        .map(str::to_string)
        .collect()
}

fn solve(lines: &[String]) -> u32 {
    lines.iter()
        .map(|line| {
            let (_, line) = line.split_once(':').expect("Bad line format, expected ':'");
            let (winning_numbers, my_numbers) = line.split_once('|').expect("Bad line format, expected '|'");

            let collect_numbers = |string: &str| string
                .trim()
                .split_whitespace()
                .map(|part| part.parse::<u32>().expect("Expected a number"))
                .collect::<HashSet<_>>();

            let (winning_numbers, my_numbers) = (
                collect_numbers(winning_numbers),
                collect_numbers(my_numbers)
            );

            let count_winning_numbers = winning_numbers
                .intersection(&my_numbers)
                .collect::<Vec<_>>()
                .len() as u32;

            if count_winning_numbers == 0 {
                0
            } else {
                2u32.pow(count_winning_numbers - 1)
            }
        })
        .sum()
}