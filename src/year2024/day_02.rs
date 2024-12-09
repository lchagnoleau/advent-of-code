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

#[aoc(day2, part2)]
fn part2(input: &str) -> u32 {
    let mut result: u32 = 0;

    for line in input.lines() {
        for l in get_all_possible(&split_line(line)) {
            if list_increase(&l) || list_decrease(&l) {
                result += 1;
                break;
            }
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

fn list_increase(list: &[u32]) -> bool {
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

fn list_decrease(list: &[u32]) -> bool {
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

fn get_all_possible(list: &[u32]) -> Vec<Vec<u32>> {
    let mut ret = Vec::new();

    for i in 0..list.len() {
        let mut list_clone = list.to_owned();
        list_clone.remove(i);
        ret.push(list_clone);
    }

    ret
}

#[test]
fn test_example() {
    let data = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
    assert_eq!(part1(data), 2);
    assert_eq!(part2(data), 4);
}

#[test]
fn test_split_line() {
    let data = split_line("41 43 46 48 51 53 55 573");
    assert_eq!(data, vec![41, 43, 46, 48, 51, 53, 55, 573]);
}

#[test]
fn test_list_increase() {
    let list = vec![1, 2, 3, 4, 5];
    assert!(list_increase(&list));
    let list = vec![1, 2, 3, 4, 4];
    assert!(!list_increase(&list));

    let list = vec![1, 2, 3, 4, 7];
    assert!(list_increase(&list));

    let list = vec![1, 2, 3, 4, 8];
    assert!(!list_increase(&list));

    let list = vec![1, 2, 3, 4, 2];
    assert!(!list_increase(&list));
}

#[test]
fn test_list_decrease() {
    let list = vec![5, 4, 3, 2, 1];
    assert!(list_decrease(&list));
    let list = vec![5, 5, 3, 2, 1];
    assert!(!list_decrease(&list));

    let list = vec![5, 4, 3, 0];
    assert!(list_decrease(&list));

    let list = vec![5, 6, 4, 3, 2];
    assert!(!list_decrease(&list));

    let list = vec![9, 4, 3, 2, 1];
    assert!(!list_decrease(&list));
}

#[test]
fn test_get_all_possible() {
    let lists = get_all_possible(&[1, 2, 3]);

    assert_eq!(lists, vec![vec![2, 3], vec![1, 3], vec![1, 2]]);
}
