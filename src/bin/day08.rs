use aoc_24::load_input;
use hashbrown::{HashMap, HashSet};

fn main() {
    let data = load_input("08").unwrap();
    let (part_1, part_2) = part_1_and_2(&data);
    println!("Part 1: {part_1}");
    println!("Part 2: {part_2}");
}

fn part_1_and_2(data: &str) -> (usize, usize) {
    let mut p1 = HashSet::new();
    let mut p2 = HashSet::new();

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

    let y_max = (data.lines().count() - 1) as isize;
    let x_max = (data.lines().next().unwrap().len() - 1) as isize;

    for (_, values) in hm.iter() {
        for value in values {
            for comp in values {
                if value != comp {
                    let step = (value.0 - comp.0, value.1 - comp.1);

                    let new_point = add_points(value, &step);
                    let (y, x) = new_point;
                    if in_bounds(new_point, y_max, x_max) {
                        p1.insert((y, x));
                    }

                    let mut pos = *value;
                    while in_bounds(pos, y_max, x_max) {
                        p2.insert(pos);
                        pos = add_points(&pos, &step);
                    }
                }
            }
        }
    }

    (p1.len(), p2.len())
}

type Point = (isize, isize);

#[inline]
fn add_points(point_a: &Point, point_b: &Point) -> Point {
    (point_a.0 + point_b.0, point_a.1 + point_b.1)
}

#[inline]
fn in_bounds(point: Point, y_max: isize, x_max: isize) -> bool {
    point.0 >= 0 && point.1 >= 0 && point.0 <= y_max && point.1 <= x_max
}

#[cfg(test)]
mod tests {
    use crate::part_1_and_2;

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
        let (p1, _) = part_1_and_2(INPUT);
        assert_eq!(p1, 14);
    }

    #[test]
    fn test_part_2() {
        let (_, p2) = part_1_and_2(INPUT);
        assert_eq!(p2, 34);
    }
}
