use std::collections::HashSet;

use anyhow::Result;

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

    fn to_char(&self) -> char {
        match self {
            Direction::Up => '^',
            Direction::Down => 'v',
            Direction::Right => '>',
            Direction::Left => '<',
        }
    }

    fn get_next_coords(&self, coords: (usize, usize)) -> Option<(usize, usize)> {
        let (x, y) = coords;
        match self {
            Direction::Up => y.checked_sub(1).map(|new_y| (x, new_y)),
            Direction::Down => Some((x, y + 1)),
            Direction::Right => Some((x + 1, y)),
            Direction::Left => x.checked_sub(1).map(|new_x| (new_x, y)),
        }
    }
}

fn get_coords(map: &[Vec<char>]) -> Option<(usize, usize)> {
    map.iter().enumerate().find_map(|(row_idx, row)| {
        row.iter()
            .position(|&ch| matches!(ch, '^' | '>' | '<' | 'v'))
            .map(|col_idx| (col_idx, row_idx))
    })
}

fn set_pos(map: &mut [Vec<char>], coords: (usize, usize), c: char) {
    let (x, y) = coords;
    if let Some(row) = map.get_mut(y) {
        if let Some(current_char) = row.get_mut(x) {
            *current_char = c;
        }
    }
}

// TODO needs a heavy optimization this thing O(n^3)
fn simulate_guard(map: &[Vec<char>], obstacle_pos: Option<(usize, usize)>) -> bool {
    let mut map_copy = map.to_vec();

    if let Some((ox, oy)) = obstacle_pos {
        if let Some(row) = map_copy.get_mut(oy) {
            if let Some(cell) = row.get_mut(ox) {
                if *cell != '#' && !matches!(*cell, '^' | 'v' | '>' | '<') {
                    *cell = '#';
                } else {
                    return false;
                }
            }
        }
    }

    let mut visited_states = HashSet::new();
    let mut coords = get_coords(&map_copy).unwrap();
    let mut di = Direction::new();

    loop {
        let state = (coords, di.clone());
        if visited_states.contains(&state) {
            return true; // Loop detected
        }
        visited_states.insert(state);

        match di.get_next_coords(coords) {
            Some(next_coords) => {
                let (next_x, next_y) = next_coords;
                match map_copy.get(next_y).and_then(|row| row.get(next_x)) {
                    Some('#') => {
                        di.next();
                    }
                    Some(_) => {
                        coords = next_coords;
                    }
                    None => {
                        return false;
                    }
                }
            }
            None => {
                return false;
            }
        }
    }
}

pub fn part_1(input: &str) -> Result<usize> {
    let mut map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut di = Direction::new();

    loop {
        let coords = get_coords(&map).unwrap();
        let next_coords = match di.get_next_coords(coords) {
            Some(v) => v,
            None => {
                set_pos(&mut map, coords, 'X');
                break;
            }
        };
        let (next_x, next_y) = next_coords;
        match map.get_mut(next_y).and_then(|row| row.get_mut(next_x)) {
            Some(target_char) => match target_char {
                '#' => {
                    set_pos(&mut map, coords, di.get_next().to_char());
                    di.next();
                }
                _ => {
                    *target_char = di.to_char();
                    set_pos(&mut map, coords, 'X');
                }
            },
            None => {
                set_pos(&mut map, coords, 'X');
                break;
            }
        }
    }
    let guard_positions = map
        .into_iter()
        .map(|row| row.into_iter().filter(|c| *c == 'X').count())
        .sum();
    Ok(guard_positions)
}

pub fn part_2(input: &str) -> Result<usize> {
    let original_map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let start_coords = get_coords(&original_map).unwrap();
    let mut loop_positions = 0;

    for y in 0..original_map.len() {
        for x in 0..original_map[0].len() {
            if (x, y) == start_coords || original_map[y][x] == '#' {
                continue;
            }

            if simulate_guard(&original_map, Some((x, y))) {
                loop_positions += 1;
            }
        }
    }

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
