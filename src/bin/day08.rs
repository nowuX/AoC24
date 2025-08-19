use hashbrown::{HashMap, HashSet};

const DATA: &str = include_str!("../../input/08.in");

fn main() {
    let now = std::time::Instant::now();
    let (p1, p2) = part_1_and_2(DATA);
    let elapsed = now.elapsed();
    println!("Part 1: {p1:?}\nPart 2: {p2:?}\nTime: {elapsed:?}");
}

fn part_1_and_2(data: &str) -> (usize, usize) {
    let grid = data.lines().map(|l| l.as_bytes()).collect::<Vec<_>>();
    let mut nodes: HashMap<_, Vec<_>> = HashMap::new();
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] != b'.' {
                nodes
                    .entry(grid[i][j])
                    .or_default()
                    .push((i as isize, j as isize));
            }
        }
    }

    let y_max = grid.len() as isize;
    let x_max = grid[0].len() as isize;

    let mut p1 = HashSet::new();
    let mut p2 = HashSet::new();
    for values in nodes.values() {
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
    point.0 >= 0 && point.1 >= 0 && point.0 < y_max && point.1 < x_max
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
