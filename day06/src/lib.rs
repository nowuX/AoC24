use anyhow::Result;
use std::collections::HashSet;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    fn new() -> Self {
        Self::Up
    }

    fn next(&mut self) {
        *self = self.get_next();
    }

    // Versión inmutable que devuelve el siguiente
    fn get_next(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

fn get_coords(map: &[Vec<u8>]) -> (usize, usize) {
    map.iter()
        .enumerate()
        .find_map(|(y, row)| row.iter().position(|&c| c == b'^').map(|x| (x, y)))
        .unwrap()
}

fn simulate_guard(map: &[Vec<u8>], obstacle_pos: (usize, usize)) -> bool {
    let (ox, oy) = obstacle_pos;

    let cell = map[oy][ox];
    if cell == b'#' || cell == b'^' {
        return false;
    }

    let mut visited_states = HashSet::new();
    let mut coords = get_coords(map);
    let mut di = Direction::new();

    loop {
        let state = (coords.0, coords.1, di.clone()); // Assume Direction has as_u8()

        if !visited_states.insert(state) {
            return true; // Loop detected
        }

        let (x, y) = coords;
        let (next_x, next_y) = match di {
            Direction::Up => (x, y.wrapping_sub(1)),
            Direction::Down => (x, y + 1),
            Direction::Right => (x + 1, y),
            Direction::Left => (x.wrapping_sub(1), y),
        };

        if next_y >= map.len() || next_x >= map[0].len() {
            return false;
        }

        if next_y >= map.len() || next_x >= map[0].len() {
            return false;
        }

        let is_wall = (next_x, next_y) == obstacle_pos || map[next_y][next_x] == b'#';

        if is_wall {
            di.next();
        } else {
            coords = (next_x, next_y);
        }
    }
}

#[inline]
fn mark_vertical_range(map: &mut [Vec<u8>], x: usize, start_y: usize, end_y: usize) {
    for y in map.iter_mut().take(end_y + 1).skip(start_y) {
        y[x] = b'X';
    }
}

#[inline]
fn mark_horizontal_range(map: &mut [Vec<u8>], y: usize, start_x: usize, end_x: usize) {
    for x in start_x..=end_x {
        map[y][x] = b'X';
    }
}

#[inline]
fn count_x_bytes(row: &[u8]) -> usize {
    row.iter().filter(|b| **b == b'X').count()
}

pub fn part_1(input: &str) -> Result<usize> {
    let mut map: Vec<Vec<u8>> = input.lines().map(|l| l.bytes().collect()).collect();
    let mut di = Direction::new();
    let (mut x, mut y) = get_coords(&map);

    loop {
        match di {
            Direction::Up => match (0..=y).rev().find(|ty| map[*ty][x] == b'#') {
                Some(pos) => {
                    mark_vertical_range(&mut map, x, pos + 1, y);
                    y = pos + 1;
                }
                None => {
                    mark_vertical_range(&mut map, x, 0, y);
                    break;
                }
            },
            Direction::Down => {
                let len = map.len();
                match ((y + 1)..map.len()).find(|ty| map[*ty][x] == b'#') {
                    Some(wall_y) => {
                        mark_vertical_range(&mut map, x, y, wall_y - 1);
                        y = wall_y - 1;
                    }
                    None => {
                        mark_vertical_range(&mut map, x, y, len - 1);
                        break;
                    }
                }
            }
            Direction::Right => {
                let len = map[y].len();
                match ((x + 1)..map[y].len()).find(|tx| map[y][*tx] == b'#') {
                    Some(wall_x) => {
                        mark_horizontal_range(&mut map, y, x, wall_x - 1);
                        x = wall_x - 1;
                    }
                    None => {
                        mark_horizontal_range(&mut map, y, x, len - 1);
                        break;
                    }
                }
            }
            Direction::Left => match (0..x).rev().find(|tx| map[y][*tx] == b'#') {
                Some(wall_x) => {
                    mark_horizontal_range(&mut map, y, wall_x + 1, x);
                    x = wall_x + 1;
                }
                None => {
                    mark_horizontal_range(&mut map, y, 0, x);
                    break;
                }
            },
        }
        di.next();
    }

    let result = map.iter().map(|row| count_x_bytes(row)).sum();

    Ok(result)
}

pub fn part_2(input: &str) -> Result<usize> {
    let original_map: Vec<Vec<u8>> = input.lines().map(|line| line.bytes().collect()).collect();
    let start_coords = get_coords(&original_map);
    let height = original_map.len();
    let width = original_map[0].len();

    let positions: Vec<(usize, usize)> = (0..height)
        .flat_map(|y| (0..width).map(move |x| (x, y)))
        .filter(|&pos| pos != start_coords && original_map[pos.1][pos.0] != b'#')
        .collect();

    let loop_positions = positions
        .iter()
        .filter(|&&pos| simulate_guard(&original_map, pos))
        .count();

    Ok(loop_positions)
}

#[cfg(test)]
mod tests {
    use crate::{part_1, part_2};
    use anyhow::Result;

    #[test]
    fn test_day() -> Result<()> {
        let input = {
            "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."
        };
        assert_eq!(part_1(input)?, 41);
        assert_eq!(part_2(input)?, 6);
        Ok(())
    }
}
