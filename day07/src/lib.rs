use anyhow::Result;
use utils::parse_split;

#[derive(Debug)]
enum Op {
    Add,
    Mul,
    Comb,
}

fn op_combinations(n: usize, base: usize) -> impl Iterator<Item = Vec<Op>> {
    let total = base.pow(n as u32);
    (0..total).map(move |i| {
        (0..n)
            .map(move |j| {
                let digit = if base == 2 {
                    (i >> (n - 1 - j)) & 1
                } else {
                    (i / base.pow((n - 1 - j) as u32)) % base
                };
                match digit {
                    0 => Op::Add,
                    1 => Op::Mul,
                    2 => Op::Comb,
                    _ => unreachable!(),
                }
            })
            .collect()
    })
}

// target op wait
fn eval_rec(ns: &[usize], ops: &[Op]) -> usize {
    if ns.len() == 1 {
        return ns[0];
    }
    let (n, ns) = (ns[0], &ns[1..]);
    let (op, ops) = (&ops[0], if ops.len() > 1 { &ops[1..] } else { &[] });
    // println!("n={n:?},op={op:?}\tns={ns:?},ops={ops:?}");

    match op {
        Op::Add => n + eval_rec(ns, ops),
        Op::Mul => n * eval_rec(ns, ops),
        Op::Comb => {
            let new_n = format!("{n}{}", ns[0]).parse::<usize>().unwrap();
            // println!("new_n={new_n:?}");
            if ns.len() == 1 {
                // println!("\toperators[1..].is_empty()");
                new_n
            } else {
                let ns = {
                    let mut n = vec![new_n];
                    n.extend_from_slice(&ns[1..]);
                    n
                };
                eval_rec(&ns, ops)
            }
        }
    }
}

fn eval_expr(numbers: &[usize], operators: &[Op], target: &usize) -> bool {
    let result = eval_rec(&numbers, operators);
    // println!("\tresult={result:?}");
    result == *target
}

fn valid_ecu(target: &usize, numbers: &[usize], base: usize) -> bool {
    println!("\n==> target={target:?}, numbers={numbers:?}");
    if op_combinations(numbers.len() - 1, base).any(|ops| eval_expr(numbers, &ops, target)) {
        true
    } else {
        false
    }
}

pub fn part_1(input: &str) -> Result<usize> {
    let ecu = input.lines().map(|l| {
        l.split_once(": ")
            .map(|(n, ns)| {
                let n = n.parse::<usize>().unwrap();
                let ns = parse_split::<usize>(ns, " ");
                (n, ns)
            })
            .unwrap()
    });
    let result = ecu
        .filter(|(target, numbers)| {
            let numbers = {
                let mut numbers = numbers.clone();
                numbers.reverse();
                numbers
            };
            let x = valid_ecu(target, &numbers, 2);
            if x {
                println!("✅✅, {target:?}");
            }
            x
        })
        .map(|(target, _)| target)
        .sum();

    Ok(result)
}

pub fn part_2(input: &str) -> Result<usize> {
    println!("=>");
    let ecu = input.lines().map(|l| {
        l.split_once(": ")
            .map(|(n, ns)| {
                let n = n.parse::<usize>().unwrap();
                let ns = parse_split::<usize>(ns, " ");
                (n, ns)
            })
            .unwrap()
    });
    let result = ecu
        .filter(|(target, numbers)| {
            let x = valid_ecu(target, &numbers, 3);
            if x {
                println!("✅✅, {target:?}");
            }
            x
        })
        .map(|(target, _)| target)
        .sum();
    println!("=>");

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
