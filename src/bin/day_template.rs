#![expect(unused)]
use aoc_24::load_input;

fn main() {
    let data = load_input("<day>").unwrap();
    // let (part_1, part_2) = part_1_and_2(&data);
    let part_1 = part_1(&data);
    let part_2 = part_2(&data);
    println!("Part 1: {part_1}");
    println!("Part 2: {part_2}");
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
