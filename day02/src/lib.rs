use anyhow::Result;
use utils::parse_split;

fn is_sorted(line: &[i64]) -> bool {
    line.is_sorted_by(|a, b| a < b) || line.is_sorted_by(|a, b| a > b)
}

fn hasnt_bad_level(line: &[i64]) -> bool {
    line.windows(2).all(|pair| {
        let diff = (pair[0] - pair[1]).abs();
        (1..=3).contains(&diff)
    })
}
fn is_safe(line: &[i64]) -> bool {
    is_sorted(line) && hasnt_bad_level(line)
}

fn is_safe_without_n(line: &[i64]) -> bool {
    if is_safe(line) {
        return true;
    }

    (0..line.len()).any(|i| {
        let without_i = [&line[0..i], &line[i + 1..]].concat();
        is_safe(&without_i)
    })
}

pub fn part_1(input: &str) -> Result<usize> {
    let reports = input
        .lines()
        .map(|line| parse_split(line, " "))
        .filter(|line| {
            let sorted = is_sorted(line);
            let no_bad_level = hasnt_bad_level(line);
            sorted & no_bad_level
        })
        .count();
    Ok(reports)
}

pub fn part_2(input: &str) -> Result<usize> {
    let reports = input
        .lines()
        .map(|line| parse_split(line, " "))
        .filter(|line| is_safe_without_n(line))
        .count();
    Ok(reports)
}

/*
fn check_condition<F, F2>(line: &[i64], condition_1: F, condition_2: F2) -> bool
where
    F: Fn(&[i64]) -> bool,
    F2: Fn(&[i64]) -> bool,
{
    let mut used = false;
    let mut line = line.into_iter().cloned().collect::<Vec<_>>();
    let first = if condition_1(&line) {
        true
    } else {
        let mut i_set = None;
        for i in 0..line.len() {
            let without_n = [&line[0..i], &line[i + 1..]].concat();
            if condition_1(&without_n) {
                i_set = Some(i);
                break;
            }
        }
        match i_set {
            Some(i) => {
                line = [&line[0..i], &line[i + 1..]].concat();
                used = true;
                true
            }
            None => false,
        }
    };

    let second = if condition_2(&line) {
        true
    } else {
        if used {
            false
        } else {
            (0..line.len()).any(|i| {
                let without_n = [&line[0..i], &line[i + 1..]].concat();
                condition_2(&without_n)
            })
        }
    };
    first && second
}
 */

#[cfg(test)]
mod tests {
    use crate::{hasnt_bad_level, is_safe_without_n, is_sorted, part_1, part_2};
    use anyhow::Result;

    #[test]
    fn test_day() -> Result<()> {
        let input = "7 6 4 2 1\n1 2 7 8 9\n9 7 6 2 1\n1 3 2 4 5\n8 6 4 4 1\n1 3 6 7 9";
        assert_eq!(part_1(input)?, 2);
        assert_eq!(part_2(input)?, 4);
        Ok(())
    }

    #[test]
    fn sort() {
        // reverse and normal
        assert!(is_sorted(&[1, 2, 3, 4]));
        assert!(is_sorted(&[4, 3, 2, 1]));
        // double value
        assert!(!is_sorted(&[1, 3, 3, 4]));
        assert!(!is_sorted(&[4, 3, 3, 1]));
    }

    #[test]
    fn bad_level() {
        // normal 1 diff
        assert!(hasnt_bad_level(&[1, 2, 3, 4, 5]));
        assert!(hasnt_bad_level(&[5, 4, 3, 2, 1]));
        // normal 2 diff
        assert!(hasnt_bad_level(&[1, 3, 5, 7, 9]));
        assert!(hasnt_bad_level(&[9, 7, 5, 3, 1]));
        // normal 3 diff
        assert!(hasnt_bad_level(&[1, 4, 7, 10, 13]));
        assert!(hasnt_bad_level(&[13, 10, 7, 4, 1]));
        // reject 0 diff (dups)
        assert!(!hasnt_bad_level(&[1, 1, 2, 2, 3]));
        assert!(!hasnt_bad_level(&[3, 2, 2, 1, 1]));
        // reject 4 diff
        assert!(!hasnt_bad_level(&[0, 4, 8, 12, 16]));
        // reject +3 diff
        assert!(!hasnt_bad_level(&[0, 5, 11, 18, 27]));
    }

    #[test]
    fn tolerate_bad_level() {
        let mat = [
            // element in any place in
            vec![1, 3, 2, 3, 4, 5],
            vec![5, 6, 4, 3, 2, 1],
            // elem in sides
            vec![2, 1, 2, 3, 4, 5],
            vec![4, 5, 4, 3, 2, 1],
            // diff check
            vec![1, 2, 3, 4, 5],
        ];
        for line in mat {
            assert!(is_safe_without_n(&line))
        }

        // el error era que verifica conficion por condicion aunque igual medio sus
    }
}
