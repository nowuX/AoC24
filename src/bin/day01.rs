use anyhow::Result;
use aoc_24::{load_input, parse_split_once};
use itertools::Itertools;

fn main() -> Result<()> {
    let data = load_input("01")?;
    let (part_1, part_2) = part_1_and_2(&data);
    println!("Part 1: {part_1}");
    println!("Part 2: {part_2}");
    Ok(())
}

fn part_1_and_2(data: &str) -> (u64, u64) {
    let (xs, ys): (Vec<u64>, Vec<u64>) = parse_split_once::<u64>(data, "   ").unzip();

    let p2 = xs.iter().fold(0, |acc, x| {
        acc + (x * ys.iter().filter(|&y| y == x).count() as u64)
    });
    let p1 = xs
        .into_iter()
        .sorted()
        .zip(ys.into_iter().sorted())
        .fold(0, |acc, (x, y)| acc + (x).abs_diff(y));

    (p1, p2)
}

#[cfg(test)]
mod tests {
    use crate::part_1_and_2;

    const INPUT: &str = "3   4\n4   3\n2   5\n1   3\n3   9\n3   3";

    #[test]
    fn test_part_1() {
        let (p1, _) = part_1_and_2(INPUT);
        assert_eq!(p1, 11);
    }

    #[test]
    fn test_part_2() {
        let (_, p2) = part_1_and_2(INPUT);
        assert_eq!(p2, 31);
    }
}
