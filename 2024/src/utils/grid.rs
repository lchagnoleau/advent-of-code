use std::ops::{Add, Sub};
use std::ops::{AddAssign, Index, IndexMut};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Coor {
    pub x: i32,
    pub y: i32,
}
impl Coor {
    pub fn new(x: i32, y: i32) -> Self {
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grid {
    pub width: i32,
    pub height: i32,
    pub value: Vec<u8>,
}

impl Grid {
    pub fn new(input: &str) -> Self {
        let lines: Vec<&str> = input.lines().collect();

        let width = lines[0].len() as i32;
        let height = lines.len() as i32;
        let value = lines.iter().flat_map(|l| l.bytes()).collect();

        Grid {
            width,
            height,
            value,
        }
    }

    pub fn in_grid(&self, coor: Coor) -> bool {
        coor.x >= 0 && coor.x < self.width && coor.y >= 0 && coor.y < self.height
    }

    pub fn copy_with(&self, value: u8) -> Self {
        Grid {
            width: self.width,
            height: self.height,
            value: vec![value; (self.height * self.width) as usize],
        }
    }
}

impl Index<Coor> for Grid {
    type Output = u8;
    fn index(&self, index: Coor) -> &Self::Output {
        &self.value[(index.y * self.width + index.x) as usize]
    }
}

impl IndexMut<Coor> for Grid {
    #[inline]
    fn index_mut(&mut self, index: Coor) -> &mut Self::Output {
        &mut self.value[(self.width * index.y + index.x) as usize]
    }
}
