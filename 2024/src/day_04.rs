use std::ops::{AddAssign, Index};
use std::ops::{Add, Sub};

#[derive(Copy, Clone)]
struct Coor {
    x: i32,
    y: i32,
}
impl Coor {
    fn new(coor: (i32, i32)) -> Self {
        let x = coor.0;
        let y = coor.1;
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

struct Matrix {
    width: i32,
    height: i32,
    value: Vec<u8>,
}

impl Matrix {
    fn new(input: &str) -> Self {
        let lines: Vec<&str> = input.lines().collect();

        let width = lines[0].len() as i32;
        let height = lines.len() as i32;
        let value = lines.iter().flat_map(|l| l.bytes()).collect();

        Matrix {
            width,
            height,
            value,
        }
    }
}

impl Index<Coor> for Matrix {
    type Output = u8;
    fn index(&self, index: Coor) -> &Self::Output {
        &self.value[(index.y * self.width + index.x) as usize]
    }
}

#[aoc(day4, part1)]
fn part1(input: &str) -> u32 {
    let mut result: u32 = 0;

    let matrix = Matrix::new(input);

    // We assume that width and height are equal
    let size = matrix.height;

    for i in 0..size {
        //search forward and backward
        let dir = Coor::new((0, 1));
        result += search(&matrix, Coor::new((i, 0)), dir, size);
        //search down and up
        let dir = Coor::new((1, 0));
        result += search(&matrix, Coor::new((0, i)), dir, size);
    }

    // search in diagonal
    for i in 0..size - 3 {
        let dir = Coor::new((1, 1));
        result += search(&matrix, Coor::new((i, 0)), dir, size - i);
        result += search(&matrix, Coor::new((0, i + 1)), dir, size - 1 - i);
        let dir = Coor::new((-1, 1));
        result += search(&matrix, Coor::new((size - 1 - i, 0)), dir, size - i);
        result += search(&matrix, Coor::new((size - 1, i + 1)), dir, size - 1 - i);
    }

    result
}

fn search(matrix: &Matrix, mut coor: Coor, dir: Coor, size: i32) -> u32 {
    let mut result = 0;
    let mut word: u32 = 0;
    let xmas: u32 = 0x584d4153;
    let samx: u32 = 0x53414d58;
    for _ in 0..size {
        word = (word << 8) | matrix[coor] as u32;
        coor += dir;
        if word == xmas || word == samx {
            result += 1;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_input() {
        let data = include_str!("../input/day4.txt");
        assert_eq!(part1(data), 2462);
    }

    #[test]
    fn test_coor_ope() {
        let a = Coor::new((1, 2));
        let b = Coor::new((3, 4));
        let add = a + b;
        assert_eq!(add.x, 4);
        assert_eq!(add.y, 6);
        let a = Coor::new((1, 2));
        let b = Coor::new((3, 4));
        let sub = b - a;
        assert_eq!(sub.x, 2);
        assert_eq!(sub.y, 2);
    }

    #[test]
    fn test_matrix_parse() {
        let data = "XMAS
XMAS
XMAS
XMAS";
        let matrix = Matrix::new(data);
        assert_eq!(matrix.width, 4);
        assert_eq!(matrix.height, 4);
        assert_eq!(
            matrix.value,
            vec![
                b'X', b'M', b'A', b'S', b'X', b'M', b'A', b'S', b'X', b'M', b'A', b'S', b'X', b'M',
                b'A', b'S',
            ]
        );
    }

    #[test]
    fn test_matrix_index() {
        let data = "ABCD
EFGH";
        let matrix = Matrix::new(data);
        assert_eq!(matrix[Coor::new((0, 0))], b'A');
        assert_eq!(matrix[Coor::new((1, 0))], b'B');
        assert_eq!(matrix[Coor::new((2, 0))], b'C');
        assert_eq!(matrix[Coor::new((3, 0))], b'D');
        assert_eq!(matrix[Coor::new((0, 1))], b'E');
        assert_eq!(matrix[Coor::new((1, 1))], b'F');
        assert_eq!(matrix[Coor::new((2, 1))], b'G');
        assert_eq!(matrix[Coor::new((3, 1))], b'H');
    }
}
