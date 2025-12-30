#[aoc(day1, part1)]
fn part1(input: &str) -> u32 {
    let mut password: u32 = 0;
    let mut pointing: i32 = 50;

    for line in input.lines() {
        let clicks: i32 = line[1..].parse().unwrap();
        let direction = line.as_bytes()[0];

        pointing += if direction == b'R' { clicks } else { -clicks };
        pointing = pointing.rem_euclid(100);
        password += (pointing == 0) as u32;
    }

    password
}

#[aoc(day1, part2)]
fn part2(input: &str) -> u32 {
    let mut password: u32 = 0;
    let mut pointing: u32 = 1000000 + 50;

    for line in input.lines() {
        let clicks: u32 = line[1..].parse().unwrap();
        let direction = line.as_bytes()[0];

        for _ in 0..clicks {
            if direction == b'R' {
                pointing += 1;
            } else {
                pointing -= 1;
            }
            password += (pointing % 100 == 0) as u32;
        }
    }

    password
}

#[test]
fn test_day() {
    let data = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
    assert_eq!(part1(data), 3);
    assert_eq!(part2(data), 6);
}
