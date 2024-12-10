trait IsOdd {
    fn is_odd(&self) -> bool;
}

impl IsOdd for usize {
    #[inline]
    fn is_odd(&self) -> bool {
        self % 2 != 0
    }
}

#[aoc_generator(day9)]
fn parse(input: &str) -> Vec<u8> {
    let disk_map: Vec<u8> = input
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|d| d as u8)
        .collect();

    let mut disk_block = Vec::new();
    let mut index = 0;
    for (i, block) in disk_map.iter().enumerate() {
        match i.is_odd() {
            false => {
                disk_block.resize(disk_block.len() + *block as usize, index);
                index += 1;
            }
            true => disk_block.resize(disk_block.len() + *block as usize, b'.'),
        }
    }

    disk_block
}

fn checksum(input: Vec<u8>) -> u64 {
    let mut crc = 0;
    for (pos, id) in input.into_iter().enumerate() {
        if id == b'.' { break; }
        crc += pos as u64 * id as u64;
    }

    crc
}

#[aoc(day9, part1)]
fn part1(disk_block: &[u8]) -> u64 {
    let mut disk_block = disk_block.to_owned();

    for forward in 0..disk_block.len() {
        if disk_block[forward] == b'.' {
            for backward in (0..disk_block.len()).rev() {
                if backward == forward {
                    return checksum(disk_block);
                }
                if disk_block[backward] != b'.' {
                    disk_block[forward] = disk_block[backward];
                    disk_block[backward] = b'.';
                    break;
                }
            }
        }
    }

    0
}

#[test]
fn test_example() {
    let test_input = "2333133121414131402";
    assert_eq!(part1(&parse(test_input)), 1928);
}

// #[test]
// fn test_true() {
//     let test_input = include_str!("../../input/2024/day9.txt");
//     assert_eq!(part1(&parse(test_input)), 0);
// }
