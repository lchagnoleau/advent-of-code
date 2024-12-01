use std::result;

#[aoc(day1, part1)]
fn part1(input: &str) -> u32 {
    let mut result: u32 = 0;
    let mut first_list = Vec::new();
    let mut second_list = Vec::new();

    for line in input.lines() {
        let (first, second) = split_line(line);
        first_list.push(first);
        second_list.push(second);
    }

    first_list.sort();
    second_list.sort();

    for i in 0..first_list.len() {
        let diff: i32 = first_list[i] as i32 - second_list[i] as i32;
        result += diff.abs() as u32;
    }

    result
}

fn split_line(line: &str) -> (u32, u32) {
    let first = line.split("   ").nth(0).unwrap().parse::<u32>().unwrap();
    let second = line.split("   ").nth(1).unwrap().parse::<u32>().unwrap();
    (first, second)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_input() {
        let data = include_str!("../input/day1.txt");
        assert_eq!(part1(data), 2285373);
    }

    #[test]
    fn test_split_line() {
        let data = split_line("33242   97663");
        assert_eq!(data, (33242, 97663));
    }
}
