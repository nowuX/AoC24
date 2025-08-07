use anyhow::Result;
use utils::parse_split_once;

pub fn part_1(input: &str) -> Result<i64> {
    let (mut xs, mut ys): (Vec<i64>, Vec<i64>) =
        parse_split_once::<i64>(input, "   ").into_iter().unzip();
    xs.sort();
    ys.sort();
    let sum = xs
        .into_iter()
        .zip(ys)
        .fold(0, |acc, (x, y)| acc + (x - y).abs());

    Ok(sum)
}

pub fn part_2(input: &str) -> Result<i64> {
    let (xs, ys): (Vec<i64>, Vec<i64>) = parse_split_once::<i64>(input, "   ").into_iter().unzip();
    let sum = xs.iter().fold(0, |acc, x| {
        acc + (x * ys.iter().filter(|y| *y == x).count() as i64)
    });

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use crate::{part_1, part_2};
    use anyhow::Result;

    #[test]
    fn test_day() -> Result<()> {
        let input = "3   4\n4   3\n2   5\n1   3\n3   9\n3   3";
        assert_eq!(part_1(input)?, 11);
        assert_eq!(part_2(input)?, 31);
        Ok(())
    }
}
