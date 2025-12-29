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

#[test]
fn test_part1() {
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
}
