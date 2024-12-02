#[aoc(day2, part1)]
fn part1(input: &str) -> u32 {
    let mut result: u32 = 0;

    for line in input.lines() {
        if list_increase(&split_line(line)) || list_decrease(&split_line(line)) {
            result += 1;
        }
    }

    result
}

fn split_line(line: &str) -> Vec<u32> {
    let mut result = Vec::new();

    line.split_whitespace().for_each(|x| {
        result.push(x.parse::<u32>().unwrap());
    });

    result
}

fn list_increase(list: &Vec<u32>) -> bool {
    for i in 1..list.len() {
        if list[i] <= list[i - 1] {
            return false;
        }
        if list[i] - list[i - 1] > 3 {
            return false;
        }
    }

    true
}

fn list_decrease(list: &Vec<u32>) -> bool {
    for i in 1..list.len() {
        if list[i] >= list[i - 1] {
            return false;
        }
        if list[i - 1] - list[i] > 3 {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_input() {
        let data = include_str!("../input/day2.txt");
        assert_eq!(part1(data), 598);
    }

    #[test]
    fn test_split_line() {
        let data = split_line("41 43 46 48 51 53 55 573");
        assert_eq!(data, vec![41, 43, 46, 48, 51, 53, 55, 573]);
    }

    #[test]
    fn test_list_increase() {
        let list = vec![1, 2, 3, 4, 5];
        assert_eq!(list_increase(&list), true);
        let list = vec![1, 2, 3, 4, 4];
        assert_eq!(list_increase(&list), false);

        let list = vec![1, 2, 3, 4, 7];
        assert_eq!(list_increase(&list), true);

        let list = vec![1, 2, 3, 4, 8];
        assert_eq!(list_increase(&list), false);

        let list = vec![1, 2, 3, 4, 2];
        assert_eq!(list_increase(&list), false);
    }

    #[test]
    fn test_list_decrease() {
        let list = vec![5, 4, 3, 2, 1];
        assert_eq!(list_decrease(&list), true);
        let list = vec![5, 5, 3, 2, 1];
        assert_eq!(list_decrease(&list), false);

        let list = vec![5, 4, 3, 0];
        assert_eq!(list_decrease(&list), true);

        let list = vec![5, 6, 4, 3, 2];
        assert_eq!(list_decrease(&list), false);

        let list = vec![9, 4, 3, 2, 1];
        assert_eq!(list_decrease(&list), false);
    }
}
