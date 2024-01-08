#[aoc(day1, part1)]
fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(sum_first_and_last_digit)
        .sum()
}

fn sum_first_and_last_digit(line: &str) -> u32 {
    let mut digits = line.chars().filter_map(|s| s.to_digit(10));
    let first_digit = digits.next().unwrap();
    let last_digit = digits.last().unwrap_or(first_digit);

    first_digit * 10 + last_digit
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let data = include_str!("../input/2023/day1.txt");
        assert_eq!(part1(data), 55108);
    }
}
