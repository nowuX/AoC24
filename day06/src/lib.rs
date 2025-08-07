#![expect(unused_variables, dead_code)]
use anyhow::Result;

enum Direccion {
    North,
    South,
    East,
    Oest,
}

struct Guardia {
    direccion: Direccion,
}

enum Map {
    Obstaculo,
    Guardia(Guardia),
    Vacio,
}

pub fn part_1(input: &str) -> Result<usize> {
    println!("==>");
    let map: Vec<Vec<Map>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match &c {
                    '.' => Map::Vacio,
                    '#' => Map::Obstaculo,
                    '^' => Map::Guardia(Guardia {
                        direccion: Direccion::North,
                    }),
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();
    for line in map {
        // println!("{line:?}");
    }
    println!("==>");
    Ok(0)
}

pub fn part_2(input: &str) -> Result<usize> {
    Ok(0)
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
        assert_eq!(part_2(input)?, 0);
        Ok(())
    }
}
