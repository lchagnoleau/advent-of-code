use std::ops::Index;

#[derive(PartialEq, Debug)]
struct Rule {
    before: u32,
    after: u32,
}
#[derive(PartialEq, Debug)]
struct Rules {
    rules: Vec<Rule>,
}
impl Rules {
    fn parse(input: &str) -> Self {
        let rules = input
            .lines()
            .map(|x| {
                let mut l = x.split('|');
                Rule {
                    before: l.next().unwrap().parse::<u32>().unwrap(),
                    after: l.next().unwrap().parse::<u32>().unwrap(),
                }
            })
            .collect::<Vec<Rule>>();
        Rules { rules }
    }
    fn before(&self, page: u32) -> Vec<u32> {
        self.rules
            .iter()
            .filter(|rule| rule.before == page)
            .map(|rule| rule.after)
            .collect()
    }
    fn after(&self, page: u32) -> Vec<u32> {
        self.rules
            .iter()
            .filter(|rule| rule.after == page)
            .map(|rule| rule.before)
            .collect()
    }
}
impl Index<usize> for Rules {
    type Output = Rule;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.rules[index]
    }
}

fn sort(pages: &[u32], rules: &Rules) -> Vec<u32> {
    let mut sorted_pages = pages.to_owned();
    sorted_pages.sort_by(|a, b| {
        let a_before = rules.before(*a);
        let b_before = rules.before(*b);
        if a_before.contains(b) {
            std::cmp::Ordering::Greater
        } else if b_before.contains(a) {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Equal
        }
    });
    sorted_pages
}

fn get_middle_of_list(pages: &[u32]) -> u32 {
    pages[pages.len() / 2]
}

fn check_line(line: &Vec<u32>, rules: &Rules) -> bool {
    for page in line {
        let before = rules.before(*page);
        let after = rules.after(*page);

        for l in line {
            if page == l {
                break;
            }
            for b in &before {
                if b == l {
                    return false;
                }
            }
        }
        for l in line.iter().take(0).skip(line.len()) {
            if page == l {
                break;
            }
            for a in &after {
                if a == l {
                    return false;
                }
            }
        }
    }

    true
}

#[aoc(day5, part1)]
fn part1(input: &str) -> u32 {
    let mut correct_pages: Vec<Vec<u32>> = Vec::new();

    let inputs: Vec<&str> = input.split("\n\n").collect();
    let rules = Rules::parse(inputs[0]);

    let pages_line = inputs[1]
        .lines()
        .map(|x| {
            x.split(',')
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    for line in pages_line {
        if check_line(&line, &rules) {
            correct_pages.push(line);
        }
    }

    correct_pages.iter().map(|x| get_middle_of_list(x)).sum()
}

#[aoc(day5, part2)]
fn part2(input: &str) -> u32 {
    let mut bad_pages: Vec<Vec<u32>> = Vec::new();

    let inputs: Vec<&str> = input.split("\n\n").collect();
    let rules = Rules::parse(inputs[0]);

    let pages_line = inputs[1]
        .lines()
        .map(|x| {
            x.split(',')
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    for line in pages_line {
        if !check_line(&line, &rules) {
            bad_pages.push(line);
        }
    }

    bad_pages
        .iter()
        .map(|x| get_middle_of_list(&sort(x, &rules)))
        .sum()
}

#[test]
fn test_example() {
    let data = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
    assert_eq!(part1(data), 143);
    assert_eq!(part2(data), 123);
}

#[test]
fn test_middle() {
    let list = vec![1, 2, 3, 4, 5];
    assert_eq!(get_middle_of_list(&list), 3);
    let list = vec![1, 2, 3];
    assert_eq!(get_middle_of_list(&list), 2);
}

#[test]
fn test_rules_check() {
    let rules = Rules {
        rules: vec![
            Rule {
                before: 47,
                after: 53,
            },
            Rule {
                before: 97,
                after: 13,
            },
            Rule {
                before: 47,
                after: 61,
            },
            Rule {
                before: 97,
                after: 47,
            },
        ],
    };
    assert_eq!(rules.before(47), vec![53, 61]);
    assert_eq!(rules.before(97), vec![13, 47]);
    assert_eq!(rules.before(53), vec![]);
    assert_eq!(rules.after(53), vec![47]);
    assert_eq!(rules.after(61), vec![47]);
    assert_eq!(rules.after(4), vec![]);
}

#[test]
fn test_parse_input() {
    let input = "47|53
97|13
97|61
97|47

75,47,61,53,29
97,61,53,29,13";

    let inputs: Vec<&str> = input.split("\n\n").collect();
    let rules = Rules::parse(inputs[0]);

    assert_eq!(
        rules[0],
        Rule {
            before: 47,
            after: 53
        }
    );
    assert_eq!(
        rules[1],
        Rule {
            before: 97,
            after: 13
        }
    );
    assert_eq!(
        rules[2],
        Rule {
            before: 97,
            after: 61
        }
    );
    assert_eq!(
        rules[3],
        Rule {
            before: 97,
            after: 47
        }
    );

    let pages_line = inputs[1]
        .lines()
        .map(|x| {
            x.split(',')
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    assert_eq!(pages_line[0], vec![75, 47, 61, 53, 29]);
    assert_eq!(pages_line[1], vec![97, 61, 53, 29, 13]);
}
