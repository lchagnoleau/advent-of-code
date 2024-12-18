#[derive(PartialEq, Debug)]
pub struct Draw {
    red: u32,
    green: u32,
    blue: u32,
}

pub struct Game {
    id: u32,
    draws: Vec<Draw>,
}

fn parse_id(input: &str) -> u32 {
    input.split("Game ").nth(1).unwrap().parse().unwrap()
}

fn parse_draw(input: &str) -> Draw {
    let mut r: u32 = 0;
    let mut g: u32 = 0;
    let mut b: u32 = 0;

    for s in input.split(", ") {
        let number: u32 = s.split_whitespace().next().unwrap().parse().unwrap();
        let color: &str = s.split_whitespace().nth(1).unwrap();

        match color {
            "red" => r = number,
            "green" => g = number,
            "blue" => b = number,
            _ => (),
        }
    }

    Draw {
        red: r,
        green: g,
        blue: b,
    }
}

fn parse_line(line: &str) -> Game {
    let id = parse_id(line.split(':').next().unwrap());
    let draws = line
        .split(": ")
        .nth(1)
        .map(|draws_str| draws_str.split("; ").map(parse_draw).collect())
        .unwrap_or_default();

    Game { id, draws }
}

// Parse the input into a vector of Game structs.
// example:
// Game 1: 5 red, 1 green; 6 red, 3 blue; 9 red; 1 blue, 1 green, 4 red; 1 green, 2 blue; 2 blue, 1 red
// Game 2: 12 red, 2 green, 9 blue; 8 red, 12 blue; 9 red, 1 blue, 2 green; 12 blue, 8 red, 2 green; 4 red, 5 blue; 1 green, 9 blue, 10 red
#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Game> {
    input.lines().map(parse_line).collect()
}

//Only 12 red cubes, 13 green cubes, and 14 blue cubes
#[aoc(day2, part1)]
fn part1(games: &Vec<Game>) -> u32 {
    let mut sum_of_possible_id: u32 = 0;

    for game in games {
        let mut id: u32 = game.id;
        for draw in &game.draws {
            if draw.red > 12 || draw.green > 13 || draw.blue > 14 {
                id = 0;
                break;
            }
        }
        sum_of_possible_id += id;
    }

    sum_of_possible_id
}

#[aoc(day2, part2)]
fn part2(games: &Vec<Game>) -> u32 {
    let mut sum_power_set: u32 = 0;

    for game in games {
        let mut m_r: u32 = 0;
        let mut m_g: u32 = 0;
        let mut m_b: u32 = 0;
        for draw in &game.draws {
            if draw.red > m_r {
                m_r = draw.red;
            }
            if draw.green > m_g {
                m_g = draw.green;
            }
            if draw.blue > m_b {
                m_b = draw.blue;
            }
        }
        sum_power_set += m_r * m_g * m_b;
    }

    sum_power_set
}

#[test]
fn test_example() {
    let data = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    assert_eq!(part1(&input_generator(data)), 8);
    assert_eq!(part2(&input_generator(data)), 2286);
}

#[test]
fn test_parse_id() {
    assert_eq!(parse_id("Game 12"), 12);
    assert_eq!(parse_id("Game 1"), 1);
    assert_eq!(parse_id("Game 100"), 100);
}

#[test]
fn test_parse_draw() {
    assert_eq!(
        parse_draw("5 red, 1 green"),
        Draw {
            red: 5,
            green: 1,
            blue: 0
        }
    );
    assert_eq!(
        parse_draw("8 red, 12 blue"),
        Draw {
            red: 8,
            green: 0,
            blue: 12
        }
    );
}

#[test]
fn test_parse_line() {
    let line = "Game 1: 5 red, 1 green; 6 red, 3 blue; 9 red; 1 blue, 1 green, 4 red; 1 green, 2 blue; 2 blue, 1 red";
    let game = parse_line(line);

    assert_eq!(game.id, 1);

    assert_eq!(game.draws.len(), 6);

    assert_eq!(
        game.draws[0],
        Draw {
            red: 5,
            green: 1,
            blue: 0
        }
    );
    assert_eq!(
        game.draws[1],
        Draw {
            red: 6,
            green: 0,
            blue: 3
        }
    );
    assert_eq!(
        game.draws[2],
        Draw {
            red: 9,
            green: 0,
            blue: 0
        }
    );
    assert_eq!(
        game.draws[3],
        Draw {
            red: 4,
            green: 1,
            blue: 1
        }
    );
    assert_eq!(
        game.draws[4],
        Draw {
            red: 0,
            green: 1,
            blue: 2
        }
    );
    assert_eq!(
        game.draws[5],
        Draw {
            red: 1,
            green: 0,
            blue: 2
        }
    );
}
