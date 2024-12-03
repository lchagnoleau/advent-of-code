use regex::Regex;

#[aoc(day3, part1)]
fn part1(input: &str) -> u32 {
    let mut result: u32 = 0;

    for line in input.lines() {
        for mul in get_mul_tuples(line).iter() {
            result += mul.0 * mul.1;
        }
    }

    result
}

fn get_mul_tuples(input: &str) -> Vec<(u32, u32)> {
    let mut result = Vec::new();
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    for (_, [a, b]) in re.captures_iter(input).map(|c| c.extract()) {
        result.push((a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap()));
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_input() {
        let data = include_str!("../input/day3.txt");
        assert_eq!(part1(data), 173529487);
    }

    #[test]
    fn test_get_mul_tuples() {
        let data = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let result = vec![(2, 4), (5, 5), (11, 8), (8, 5)];
        assert_eq!(get_mul_tuples(data), result);
    }
}
