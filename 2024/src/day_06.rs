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

struct Map {
    width: i32,
    height: i32,
    value: Vec<char>,
}

impl Map {
    fn new(input: &str) -> Self {
        let lines: Vec<&str> = input.lines().collect();

        let width = lines[0].len() as i32;
        let height = lines.len() as i32;
        let value = lines.iter().flat_map(|l| l.chars()).collect();

        Map {
            width,
            height,
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
            '^' => return Coor::new(0, -1),
            '>' => return Coor::new(1, 0),
            '<' => return Coor::new(-1, 0),
            'V' => return Coor::new(0, 1),
            _ => Coor::new(0, 0),
        }
    }
    fn run(&mut self) -> bool {
        let pos = self.get_pos();
        let next_pos = pos + self.next_dir(pos);

        // We are out of map, so return false
        if next_pos.x >= self.width || next_pos.y >= self.height || next_pos.x < 0 || next_pos.y < 0 {
            self[pos] = 'X';
            return false;
        }

        match self[next_pos] {
            '.' | 'X' => {
                self[next_pos] = self[pos]; //Continue moving in the same direction
                self[pos] = 'X'; //To mark we are visited this area
            }
            '#' => match self[pos] {
                '^' => self[pos] = '>',
                '>' => self[pos] = 'V',
                '<' => self[pos] = '^',
                'V' => self[pos] = '<',
                _ => return false,
            },
            _ => return false,
        }

        true
    }
    fn get_visited_area(&self) -> u32 {
        self.value.iter().filter(|x| **x == 'X').count() as u32
    }
    fn print_map(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", self[Coor::new(x, y)]);
            }
            println!();
        }
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

// fn navigate(mut map: Map, origin: Coor) {

// }

#[aoc(day6, part1)]
fn part1(input: &str) -> u32 {
    let mut map = Map::new(input);
    while map.run() {}
    map.get_visited_area()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_input() {
        let data = include_str!("../input/day6.txt");
        assert_eq!(part1(data), 5080);
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

        let mut map = Map::new(data);
        assert_eq!(map.get_pos(), Coor::new(4, 6));
        assert_eq!(map.next_dir(map.get_pos()), Coor::new(0, -1));
        while map.run() {
            map.print_map();
            println!();
        }
        assert_eq!(map.get_visited_area(), 41);
    }
}
