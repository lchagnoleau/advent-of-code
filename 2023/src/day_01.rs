#[aoc(day1, part1)]
fn part1(input: &str) -> u32 {
    input.lines().map(sum_first_and_last_digit).sum()
}

fn sum_first_and_last_digit(line: &str) -> u32 {
    let mut digits = line.chars().filter_map(|s| s.to_digit(10));
    let first_digit = digits.next().unwrap();
    let last_digit = digits.last().unwrap_or(first_digit);

    first_digit * 10 + last_digit
}

const TRANSLATIONS: [(&str, &str); 9] = [
    ("one", "o1e"),
    ("two", "t2o"),
    ("three", "t3e"),
    ("four", "f4r"),
    ("five", "f5e"),
    ("six", "s6x"),
    ("seven", "s7n"),
    ("eight", "e8t"),
    ("nine", "n9e"),
];

#[aoc(day1, part2)]
fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            TRANSLATIONS
                .iter()
                .fold(line.to_string(), |acc, (word, translation)| {
                    acc.replace(word, translation)
                })
        })
        .map(|line| sum_first_and_last_digit(line.as_str()))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_input() {
        let data = include_str!("../input/2023/day1.txt");
        assert_eq!(part1(data), 55108);
    }

    #[test]
    fn part2_input() {
        let data = include_str!("../input/2023/day1.txt");
        assert_eq!(part2(data), 56324);
    }
}
