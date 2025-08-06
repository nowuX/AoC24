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

    #[test]
    fn test_day() -> Result<()> {
        let input = "";
        assert_eq!(part_1(input)?, 0);
        assert_eq!(part_2(input)?, 0);
        Ok(())
    }
}
