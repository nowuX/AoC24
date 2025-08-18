use std::iter::repeat;

const DATA: &str = include_str!("../../input/09.in");

fn main() {
    // let (p1, p2) = part_1_and_2(&DATA);
    let now = std::time::Instant::now();
    let p1 = part_1(DATA);
    let p2 = part_2(DATA);
    let elapsed = now.elapsed();
    println!("Part 1: {p1:?}\nPart 2: {p2:?}\nTime: {elapsed:?}");
}

pub fn display(vec: &[Option<u8>]) -> String {
    vec.iter()
        .map(|opt| match opt {
            Some(n) => char::from_digit(*n as u32, 10).unwrap_or('.'),
            None => '.',
        })
        .collect()
}

fn part_1(data: &str) -> usize {
    let bytes = data.trim_end().as_bytes();
    let total_size: usize = bytes.iter().map(|&b| (b - b'0') as usize).sum();
    let mut files: Vec<Option<usize>> = Vec::with_capacity(total_size); // too long

    for i in (0..bytes.len()).step_by(2) {
        let id = i / 2;
        let x = bytes[i] - b'0';
        #[rustfmt::skip]
        let y = if i + 1 < bytes.len() { bytes[i + 1] - b'0' } else { 0 };

        files.extend(repeat(Some(id)).take(x as usize));
        files.extend(repeat(None).take(y as usize));
    }

    let mut free_index = files.iter().position(|f| f.is_none()).unwrap();
    let mut last_index = files.iter().rposition(|f| f.is_some()).unwrap();
    while free_index != last_index + 1 {
        files.swap(free_index, last_index);

        while files[free_index].is_some() {
            free_index += 1;
        }

        while files[last_index].is_none() {
            last_index -= 1;
        }
    }

    files
        .iter()
        .flatten()
        .enumerate()
        .map(|(i, &v)| i * v)
        .sum()
}

#[allow(unused)]
fn part_2(data: &str) -> usize {
    0
}

// fn part_1_and_2(data: &str) -> (usize, usize) { let p1 = 0; let p2 = 0; (p1, p2) }

#[cfg(test)]
mod tests {
    use crate::{part_1, part_2};

    const INPUT: &str = "2333133121414131402";

    #[test]
    fn test_part_1() {
        let p1 = part_1(INPUT);
        assert_eq!(p1, 1928);
    }

    #[test]
    fn test_part_2() {
        let p2 = part_2(INPUT);
        assert_eq!(p2, 0);
    }
}
