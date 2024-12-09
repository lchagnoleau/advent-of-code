use std::ops::{Add, Sub};
use std::ops::{AddAssign, Index, IndexMut};

#[derive(Copy, Clone, Debug, PartialEq)]
struct Coor {
    x: i32,
    y: i32,
}
impl Coor {
    fn new(x: i32, y: i32) -> Self {
        Coor { x, y }
    }
}
impl Add for Coor {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Coor {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl AddAssign for Coor {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
impl Sub for Coor {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Coor {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

#[derive(PartialEq, Debug)]
enum Status {
    Running,
    Stuck,
    Out,
}

struct Map {
    width: i32,
    height: i32,
    log_pos: Vec<(char, Coor)>,
    value: Vec<char>,
}

impl Map {
    fn new(input: &str) -> Self {
        let lines: Vec<&str> = input.lines().collect();

        let width = lines[0].len() as i32;
        let height = lines.len() as i32;
        let log_pos = vec![];
        let value = lines.iter().flat_map(|l| l.chars()).collect();

        Map {
            width,
            height,
            log_pos,
            value,
        }
    }
    fn get_pos(&self) -> Coor {
        let mut x = 0;
        let mut y = 0;
        for i in 0..self.value.len() {
            if self.value[i] == '^'
                || self.value[i] == '>'
                || self.value[i] == '<'
                || self.value[i] == 'V'
            {
                x = i as i32 % self.width;
                y = i as i32 / self.width;
                break;
            }
        }

        Coor::new(x, y)
    }
    fn next_dir(&self, pos: Coor) -> Coor {
        match self[pos] {
            '^' => Coor::new(0, -1),
            '>' => Coor::new(1, 0),
            '<' => Coor::new(-1, 0),
            'V' => Coor::new(0, 1),
            _ => Coor::new(0, 0),
        }
    }
    fn run(&mut self) -> Status {
        let pos = self.get_pos();
        let next_pos = pos + self.next_dir(pos);

        // Seems we are stuck
        for log in &self.log_pos {
            if self[pos] == log.0 && pos == log.1 {
                return Status::Stuck;
            }
        }

        // We are out of map, so return false
        if next_pos.x >= self.width || next_pos.y >= self.height || next_pos.x < 0 || next_pos.y < 0
        {
            self[pos] = 'X';
            return Status::Out;
        }

        match self[next_pos] {
            '.' | 'X' => {
                self[next_pos] = self[pos]; //Continue moving in the same direction
                self[pos] = 'X'; //To mark we are visited this area
            }
            '#' => {
                self.log_pos.push((self[pos], pos));
                match self[pos] {
                    '^' => self[pos] = '>',
                    '>' => self[pos] = 'V',
                    '<' => self[pos] = '^',
                    'V' => self[pos] = '<',
                    _ => return Status::Out,
                }
            }
            _ => return Status::Out,
        }

        Status::Running
    }
    fn get_visited_area(&self) -> Vec<Coor> {
        let mut r = Vec::new();

        for x in 0..self.width {
            for y in 0..self.height {
                if self[Coor::new(x, y)] == 'X' {
                    r.push(Coor::new(x, y));
                }
            }
        }

        r
    }
}

impl Index<Coor> for Map {
    type Output = char;
    fn index(&self, index: Coor) -> &Self::Output {
        &self.value[(index.y * self.width + index.x) as usize]
    }
}

impl IndexMut<Coor> for Map {
    fn index_mut(&mut self, index: Coor) -> &mut Self::Output {
        &mut self.value[(self.width * index.y + index.x) as usize]
    }
}

#[aoc(day6, part1)]
fn part1(input: &str) -> u32 {
    let mut map = Map::new(input);
    while map.run() == Status::Running {}
    map.get_visited_area().len() as u32
}

#[aoc(day6, part2)]
fn part2(input: &str) -> u32 {
    let mut map = Map::new(input);
    while map.run() == Status::Running {}
    let mut result = 0;

    // try all possible obstacle position
    for pos in map.get_visited_area() {
        let mut new_map = Map::new(input);
        if new_map[pos] == '.' {
            new_map[pos] = '#';
        }

        let mut status = Status::Running;
        while status == Status::Running {
            status = new_map.run();
        }
        result += (status == Status::Stuck) as u32;
    }
    result
}

#[test]
fn test_example() {
    let data = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
    assert_eq!(part1(data), 41);
    assert_eq!(part2(data), 6);
}

#[test]
fn test_stuck() {
    let data = "....#.....
.........#
..........
..#.......
.......#..
..........
.#.#^.....
........#.
#.........
......#...";

    let mut map = Map::new(data);
    assert_eq!(map.get_pos(), Coor::new(4, 6));
    assert_eq!(map.next_dir(map.get_pos()), Coor::new(0, -1));
    let mut status = Status::Running;
    while status == Status::Running {
        status = map.run();
    }
    assert_eq!(status, Status::Stuck);
}

#[test]
fn test_stuck_2() {
    let data = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
......#.#.
#.........
......#...";

    let mut map = Map::new(data);
    assert_eq!(map.get_pos(), Coor::new(4, 6));
    assert_eq!(map.next_dir(map.get_pos()), Coor::new(0, -1));
    let mut status = Status::Running;
    while status == Status::Running {
        status = map.run();
    }
    assert_eq!(status, Status::Stuck);
}

#[test]
fn test_part2() {
    let data = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    let mut map = Map::new(data);
    while map.run() == Status::Running {}

    let mut result = 0;

    // try all possible obstacle position
    for pos in map.get_visited_area() {
        let mut new_map = Map::new(data);
        if new_map[pos] == '.' {
            new_map[pos] = '#';
        }

        let mut status = Status::Running;
        while status == Status::Running {
            status = new_map.run();
        }
        result += (status == Status::Stuck) as u32;
    }
    assert_eq!(result, 6);
}
