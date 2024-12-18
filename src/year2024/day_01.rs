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
        result += diff.unsigned_abs();
    }

    result
}

fn split_line(line: &str) -> (u32, u32) {
    let first = line.split("   ").next().unwrap().parse::<u32>().unwrap();
    let second = line.split("   ").nth(1).unwrap().parse::<u32>().unwrap();
    (first, second)
}

#[aoc(day1, part2)]
fn part2(input: &str) -> u32 {
    let mut result: u32 = 0;
    let mut first_list = Vec::new();
    let mut second_list = Vec::new();

    for line in input.lines() {
        let (first, second) = split_line(line);
        first_list.push(first);
        second_list.push(second);
    }

    for i in first_list.iter() {
        let mut occurences = 0;
        for j in second_list.iter() {
            if i == j {
                occurences += 1;
            }
        }
        result += occurences * i;
    }

    result
}

#[test]
fn test_example() {
    let data = "3   4
4   3
2   5
1   3
3   9
3   3";
    assert_eq!(part1(data), 11);
    assert_eq!(part2(data), 31);
}

#[test]
fn test_split_line() {
    let data = split_line("33242   97663");
    assert_eq!(data, (33242, 97663));
}
