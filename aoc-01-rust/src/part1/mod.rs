fn get_number(line: &str) -> u32 {
    let digits = line.chars()
        .filter_map(|c| c.to_digit(10))
        .collect::<Vec<u32>>();
    digits[0] * 10 + digits[digits.len() - 1]
}

pub(crate) fn solve(lines: &[String]) -> u32 {
    lines.iter()
        .map(|line| get_number(line))
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_number() {
        const LINES: [&str; 4] = [
            "1abc2",
            "pqr3stu8vwx",
            "a1b2c3d4e5f",
            "treb7uchet",
        ];

        const NUMS: [u32; 4] = [12, 38, 15, 77];

        for (line, num) in LINES.iter().zip(NUMS) {
            assert_eq!(num, get_number(line));
        }
    }

    #[test]
    fn test_solve() {
        let lines = [
            "1abc2".to_string(),
            "pqr3stu8vwx".to_string(),
            "a1b2c3d4e5f".to_string(),
            "treb7uchet".to_string(),
        ];

        assert_eq!(142, solve(lines.as_ref()));
    }
}
