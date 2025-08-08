#![expect(unused_variables)]
use anyhow::Result;
use utils::parse_split;

fn op_combinations(n: usize) -> Vec<Vec<bool>> {
    let total = 1 << n;
    (0..total)
        .map(|i| (0..n).map(|j| (i >> (n - 1 - j)) & 1 == 1).collect())
        .collect()
}

fn eval_expr(numbers: &[usize], operators: &[bool]) -> usize {
    let mut result = numbers[0];
    for (i, op) in operators.iter().enumerate() {
        match op {
            true => result += numbers[i + 1],
            false => result *= numbers[i + 1],
        }
    }
    result
}

fn eval_ecu(numbers: &[usize], target: &usize) -> Option<Vec<bool>> {
    op_combinations(numbers.len() - 1)
        .into_iter()
        .find(|ops| eval_expr(numbers, ops) == *target)
}

pub fn part_1(input: &str) -> Result<usize> {
    let result = input
        .lines()
        .map(|l| {
            l.split_once(": ")
                .map(|(x, y)| (x.parse::<usize>().unwrap(), parse_split::<usize>(y, " ")))
                .unwrap()
        })
        .filter(|(target, numbers)| eval_ecu(numbers, target).is_some())
        .map(|(target, _)| target)
        .sum();

    Ok(result)
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
            "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"
        };
        assert_eq!(part_1(input)?, 3749);
        assert_eq!(part_2(input)?, 0);
        Ok(())
    }
}
