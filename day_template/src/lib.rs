#![expect(unused_variables)]
use anyhow::Result;

pub fn part_1(input: &str) -> Result<usize> {
    Ok(0)
}

pub fn part_2(input: &str) -> Result<usize> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use crate::{part_1, part_2};
    use anyhow::Result;

    const INPUT: &str = "";

    #[test]
    fn test_day_part_1() -> Result<()> {
        assert_eq!(part_1(INPUT)?, 0);
        Ok(())
    }

    #[test]
    fn test_day_part_2() -> Result<()> {
        assert_eq!(part_2(INPUT)?, 0);
        Ok(())
    }
}
