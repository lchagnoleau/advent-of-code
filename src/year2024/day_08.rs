use std::collections::HashMap;

use crate::utils::grid::*;

struct Node {
    grid: Grid,
    antennas: HashMap<u8, Vec<Coor>>,
}

#[aoc_generator(day8)]
fn parse(input: &str) -> Node {
    let grid = Grid::new(input);
    let mut antennas = HashMap::new();

    for x in 0..grid.width {
        for y in 0..grid.height {
            let coor = Coor::new(x, y);
            let freq = grid[coor];

            if freq != b'.' {
                antennas.entry(freq).or_insert_with(Vec::new).push(coor);
            }
        }
    }

    Node { grid, antennas }
}

#[aoc(day8, part1)]
fn part1(node: &Node) -> u32 {
    let mut antinode = node.grid.copy_with(0);

    for freq in node.antennas.values() {
        for &first in freq {
            for &second in freq {
                if first != second {
                    let distance = second - first;
                    let ant = second + distance;
                    if node.grid.in_grid(ant) {
                        antinode[ant] = 1;
                    }
                }
            }
        }
    }

    let mut r = 0;
    for ant in antinode.value {
        r += ant as u32;
    }

    r
}

#[aoc(day8, part2)]
fn part2(node: &Node) -> u32 {
    let mut antinode = node.grid.copy_with(0);

    for freq in node.antennas.values() {
        for &first in freq {
            for &second in freq {
                if first != second {
                    let distance = second - first;
                    let mut ant = second;
                    while node.grid.in_grid(ant) {
                        antinode[ant] = 1;
                        ant += distance;
                    }
                }
            }
        }
    }

    let mut r = 0;
    for ant in antinode.value {
        r += ant as u32;
    }

    r
}

#[test]
fn test_example() {
    let test_input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
    assert_eq!(part1(&parse(test_input)), 14);
    assert_eq!(part2(&parse(test_input)), 34);
}
