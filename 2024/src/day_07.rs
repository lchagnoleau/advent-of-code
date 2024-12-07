struct Equation {
    test_value: u64,
    operators: Vec<u64>,
}

impl Equation {
    fn parse(input: &str) -> Self {
        let mut parts = input.split(": ");
        let test_value = parts.next().unwrap().parse().unwrap();
        let operators = parts
            .next()
            .unwrap()
            .split(" ")
            .map(|x| x.parse().unwrap())
            .collect();

        Equation {
            test_value,
            operators,
        }
    }
    fn is_valid(&self) -> bool {
        recursive_validation(self.operators.clone(), self.test_value, self.operators.len() - 1)
    }
    fn is_valid_2(&self) -> bool {
        recursive_validation_2(self.operators.clone(), self.test_value, self.operators.len() - 1)
    }
}

fn recursive_validation(op: Vec<u64>, test_val: u64, index: usize) -> bool {
    if index == 0 {
        return op[0] == test_val;
    }

    (test_val >= op[index] &&
        recursive_validation(op.clone(), test_val - op[index], index - 1)) ||
        (test_val % op[index] == 0 &&
        recursive_validation(op.clone(), test_val / op[index], index - 1))
}

fn recursive_validation_2(op: Vec<u64>, test_val: u64, index: usize) -> bool {
    if index == 0 {
        return op[0] == test_val;
    }

    (test_val >= op[index] &&
    recursive_validation_2(op.clone(), test_val - op[index], index - 1)) ||
    (test_val % op[index] == 0 &&
    recursive_validation_2(op.clone(), test_val / op[index], index - 1)) ||
    (test_val % next_power_of_ten(op[index]) == op[index] &&
    recursive_validation_2(op.clone(), test_val / next_power_of_ten(op[index]), index - 1))
}

fn next_power_of_ten(n: u64) -> u64 {
    if n < 10 {
        10
    } else if n < 100 {
        100
    } else {
        1000
    }
}

#[aoc(day7, part1)]
fn part1(input: &str) -> u64 {
    let mut r = 0;
    for line in input.lines() {
        let eq = Equation::parse(line);
        if eq.is_valid() {
            r += eq.test_value;
        }
    }
    r
}

#[aoc(day7, part2)]
fn part2(input: &str) -> u64 {
    let mut r = 0;
    for line in input.lines() {
        let eq = Equation::parse(line);
        if eq.is_valid_2() {
            r += eq.test_value;
        }
    }
    r
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_input() {
        let data = include_str!("../input/day7.txt");
        assert_eq!(part1(data), 7579994664753);
    }

    #[test]
    fn part2_input() {
        let data = include_str!("../input/day7.txt");
        assert_eq!(part2(data), 438027111276610);
    }

    #[test]
    fn test_example() {
        let test = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        assert_eq!(part1(test), 3759)
    }

    #[test]
    fn test_example_2() {
        let test = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        assert_eq!(part2(test), 11387)
    }

    #[test]
    fn test_parse() {
        let data = "190: 10 19";
        let eq = Equation::parse(data);
        assert_eq!(eq.test_value, 190);
        assert_eq!(eq.operators, vec![10, 19]);
    }

    #[test]
    fn test_is_valid() {
        let data = "190: 10 19";
        let mut eq = Equation::parse(data);
        assert_eq!(eq.is_valid(), true);
        let data = "7290: 6 8 6 15";
        let mut eq = Equation::parse(data);
        assert_eq!(eq.is_valid(), false);
    }

    #[test]
    fn test_is_valid_2() {
        let data = "190: 10 19";
        let mut eq = Equation::parse(data);
        assert_eq!(eq.is_valid_2(), true);
        let data = "7290: 6 8 6 15";
        let mut eq = Equation::parse(data);
        assert_eq!(eq.is_valid_2(), true);
        let data = "7290: 72 90";
        let mut eq = Equation::parse(data);
        assert_eq!(eq.is_valid_2(), true);
    }
}
