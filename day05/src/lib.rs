#![expect(unused_variables)]
use anyhow::Result;

fn get_rules(rules: &[(usize, usize)], e: &usize) -> (Vec<usize>, Vec<usize>) {
    let mut pre = vec![];
    let mut post = vec![];
    for (p, n) in rules {
        if e == p {
            post.push(*n)
        } else if e == n {
            pre.push(*p);
        }
    }
    (pre, post)
}

pub fn part_1(input: &str) -> Result<usize> {
    let (rules, updates) = input.split_once("\n\n").unwrap();
    let rules: Vec<_> = rules
        .lines()
        .map(|l| {
            let (f, s) = l.split_once("|").unwrap();
            (f.parse::<usize>().unwrap(), s.parse::<usize>().unwrap())
        })
        .collect();
    let updates = updates
        .lines()
        .map(|l| {
            l.split(",")
                .map(|w| w.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|values| {
            values.iter().enumerate().all(|(i, e)| {
                let (pre_rules, post_rules) = get_rules(&rules, e);
                !(&values[i + 1..]).iter().any(|x| pre_rules.contains(x))
                    && !(&values[..i]).iter().any(|x| post_rules.contains(x))
            })
        })
        .map(|values| values[values.len() / 2])
        .sum();

    Ok(updates)
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
            "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"
        };
        assert_eq!(part_1(input)?, 143);
        assert_eq!(part_2(input)?, 0);
        Ok(())
    }
}
