use anyhow::Result;
use utils::parse_split;

fn op_combinations(n: u32, base: usize) -> impl Iterator<Item = impl Iterator<Item = usize>> {
    let total = base.pow(n);
    (0..total).map(move |i| {
        (0..n).map(move |j| {
            if base == 2 {
                (i >> (n - 1 - j)) & 1
            } else {
                (i / base.pow(n - 1 - j)) % base
            }
        })
    })
}

fn eval_rec<I>(ns: &[usize], ops: I, target: usize) -> bool
where
    I: Iterator<Item = usize>,
{
    if ns.len() == 1 {
        return ns[0] == target;
    }

    let n1 = ns[0];
    let n2 = ns[1];
    let mut ops_iter = ops;
    let op = ops_iter.next().unwrap();

    let buffer = match op {
        0 => n1 + n2,
        1 => n1 * n2,
        2 => {
            let mut multiplier = 1;
            let mut temp = n2;
            while temp > 0 {
                multiplier *= 10;
                temp /= 10;
            }
            n1 * multiplier + n2
        }
        _ => unreachable!(),
    };
    if buffer > target {
        return false;
    }

    if ns.len() == 2 {
        return buffer == target;
    }

    let mut next_ns = [0usize; 12];
    next_ns[0] = buffer;
    next_ns[1..(ns.len() - 1)].copy_from_slice(&ns[2..]);

    eval_rec(&next_ns[..(ns.len() - 1)], ops_iter, target)
}

fn solution(input: &str, base: usize) -> usize {
    input
        .lines()
        .filter_map(|l| {
            let (target, numbers) = l.split_once(": ")?;
            let target: usize = target.parse().ok()?;
            let numbers: Vec<usize> = parse_split(numbers, " ").collect();

            if op_combinations((numbers.len() - 1) as u32, base)
                .any(|ops| eval_rec(&numbers, ops, target))
            {
                Some(target)
            } else {
                None
            }
        })
        .sum()
}

pub fn part_1(input: &str) -> Result<usize> {
    let result = solution(input, 2);
    Ok(result)
}

pub fn part_2(input: &str) -> Result<usize> {
    let result = solution(input, 3);
    Ok(result)
}

#[cfg(test)]
mod tests {
    use crate::{part_1, part_2};
    use anyhow::Result;

    const INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
    #[test]
    fn test_day_part_1() -> Result<()> {
        assert_eq!(part_1(INPUT)?, 3749);
        Ok(())
    }

    #[test]
    fn test_day_part_2() -> Result<()> {
        assert_eq!(part_2(INPUT)?, 11387);
        Ok(())
    }
}
