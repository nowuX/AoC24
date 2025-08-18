#![expect(unused)]

const DATA: &'static str = include_str!("../../input/08.in");

fn main() {
    // let (p1, p2) = part_1_and_2(&DATA);
    let now = std::time::Instant::now();
    let p1 = part_1(&DATA);
    let p2 = part_2(&DATA);
    let elapsed = now.elapsed();
    println!("Part 1: {p1:?}\nPart 2: {p2:?}\nTime: {elapsed:?}");
}

fn part_1(data: &str) -> usize {
    0
}

fn part_2(data: &str) -> usize {
    0
}

// fn part_1_and_2(data: &str) -> (usize, usize) { let p1 = 0; let p2 = 0; (p1, p2) }

#[cfg(test)]
mod tests {
    use crate::{part_1, part_2};

    const INPUT: &str = "";

    #[test]
    fn test_part_1() {
        let p1 = part_1(INPUT);
        assert_eq!(p1, 0);
    }

    #[test]
    fn test_part_2() {
        let p2 = part_2(INPUT);
        assert_eq!(p2, 0);
    }
}
