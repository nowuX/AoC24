use anyhow::Result;

fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    input
        .lines()
        .map(|line| {
            let mut ns = line.split_whitespace();
            let n1 = ns.next().unwrap();
            let n2 = ns.last().unwrap();
            (n1.parse::<i32>().unwrap(), n2.parse::<i32>().unwrap())
        })
        .unzip()
}

pub fn part_1(input: &str) -> Result<i32> {
    let (mut xs, mut ys) = parse_input(input);
    xs.sort();
    ys.sort();
    let sum = xs
        .into_iter()
        .zip(ys)
        .fold(0, |acc, (x, y)| acc + (x - y).abs());

    Ok(sum)
}

pub fn part_2(input: &str) -> Result<i32> {
    let (xs, ys) = parse_input(input);
    let sum = xs.iter().fold(0, |acc, x| {
        acc + (x * ys.iter().filter(|y| *y == x).count() as i32)
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
