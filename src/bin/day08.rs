#![expect(unused)]
use anyhow::Result;
use aoc_24::load_input;
use hashbrown::{HashMap, HashSet};
use rayon::prelude::*;

fn main() -> Result<()> {
    let data = load_input("08")?;
    // let (part_1, part_2) = part_1_and_2(&data);
    let part_1 = part_1(&data);
    let part_2 = part_2(&data);
    println!("Part 1: {part_1}");
    println!("Part 2: {part_2}");
    Ok(())
}

fn part_1(data: &str) -> usize {
    let mut antinodes_positions = HashSet::new();

    let mut hm: HashMap<char, Vec<(isize, isize)>> = HashMap::new();
    for (y, line) in data.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch != '.' {
                hm.entry(ch)
                    .or_insert_with(Vec::new)
                    .push((y as isize, x as isize));
            }
        }
    }

    let y_max = data.lines().count() - 1;
    let x_max = data.lines().next().unwrap().len() - 1;

    for key in hm.keys() {
        let (key, values) = hm.get_key_value(key).unwrap();
        for value in values {
            for comp in values {
                if value != comp {
                    let step = (value.0 - comp.0, value.1 - comp.1);
                    let new_point = (value.0 + step.0, value.1 + step.1);
                    let (y, x) = new_point;
                    if y >= 0 && x >= 0 && y <= y_max as isize && x <= x_max as isize {
                        antinodes_positions.insert((y, x));
                    }
                }
            }
        }
    }

    antinodes_positions.len()
}

fn part_2(data: &str) -> usize {
    0
}

// fn part_1_and_2(data: &str) -> (usize, usize) { let sol = { let p1 = 0; let p2 = 0; (p1, p2) }; sol }

#[cfg(test)]
mod tests {
    use crate::{part_1, part_2};

    const INPUT: &str = "............
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

    #[test]
    fn test_part_1() {
        let p1 = part_1(INPUT);
        assert_eq!(p1, 14);
    }

    #[test]
    fn test_part_2() {
        let p2 = part_2(INPUT);
        assert_eq!(p2, 0);
    }
}
