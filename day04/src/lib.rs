use anyhow::Result;

fn has_xmas(chars: &[char]) -> bool {
    matches!(chars, ['X', 'M', 'A', 'S'] | ['S', 'A', 'M', 'X'])
}

fn linear_check(matriz: &Vec<Vec<char>>, counter: &mut usize) {
    for line in matriz {
        for chars in line.windows(4) {
            if has_xmas(chars) {
                *counter += 1;
            }
        }
    }
}

fn rotate_90(matrix: &[Vec<char>]) -> Vec<Vec<char>> {
    (0..matrix[0].len())
        .rev()
        .map(|col| (0..matrix.len()).map(|row| matrix[row][col]).collect())
        .collect()
}

pub fn part_1(input: &str) -> Result<usize> {
    let mut counter = 0;
    let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    // Horizontalmente
    linear_check(&matrix, &mut counter);

    // Vertocal
    let rotated = rotate_90(&matrix);
    linear_check(&rotated, &mut counter);

    // Diagonal left + top -> right + bottom
    for i in 0..=matrix.len() - 4 {
        for j in 0..=matrix[0].len() - 4 {
            let chars: Vec<char> = (0..=3).map(|delta| matrix[i + delta][j + delta]).collect();
            if has_xmas(&chars) {
                counter += 1;
            }
        }
    }

    // Diagonal right + top -> left + bottom
    for i in 0..=matrix.len() - 4 {
        for j in 0..=matrix[0].len() - 4 {
            let chars: Vec<char> = (0..=3)
                .map(|delta| matrix[i + 3 - delta][j + delta])
                .collect();
            if has_xmas(&chars) {
                counter += 1;
            }
        }
    }

    Ok(counter)
}

pub fn part_2(input: &str) -> Result<usize> {
    let mut counter = 0;
    let mut matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    for r in 0..2 {
        // todo 4
        if r != 0 {
            matrix = rotate_90(&matrix);
        }
        for i in 0..=matrix.len() - 3 {
            for j in 0..=matrix[0].len() - 3 {
                let chars: Vec<char> = vec![
                    matrix[i][j],
                    matrix[i][j + 2],
                    matrix[i + 1][j + 1],
                    matrix[i + 2][j],
                    matrix[i + 2][j + 2],
                ];
                if matches!(
                    chars.as_slice(),
                    ['M', 'M', 'A', 'S', 'S'] | ['S', 'S', 'A', 'M', 'M']
                ) {
                    counter += 1;
                }
            }
        }
    }

    Ok(counter)
}

#[cfg(test)]
mod tests {
    use crate::{part_1, part_2};
    use anyhow::Result;

    #[test]
    fn test_day() -> Result<()> {
        let input = "MMMSXXMASM\nMSAMXMSMSA\nAMXSXMAAMM\nMSAMASMSMX\nXMASAMXAMM\nXXAMMXXAMA\nSMSMSASXSS\nSAXAMASAAA\nMAMMMXMMMM\nMXMXAXMASX";
        assert_eq!(part_1(input)?, 18);
        assert_eq!(part_2(input)?, 9);
        Ok(())
    }
}
