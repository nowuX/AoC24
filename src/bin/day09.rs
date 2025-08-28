use std::iter::repeat;

const DATA: &str = include_str!("../../input/09.in");

fn main() {
    let now = std::time::Instant::now();
    let (p1, p2) = part_1_and_2(DATA);
    let elapsed = now.elapsed();
    println!("Part 1: {p1:?}\nPart 2: {p2:?}\nTime: {elapsed:?}");
}

#[derive(Clone, PartialEq)]
struct Block {
    size: usize,
    index: usize,
}

fn part_1_and_2(data: &str) -> (usize, usize) {
    let bytes = data.trim_end().as_bytes();
    let total_size: usize = bytes.iter().map(|&b| (b - b'0') as usize).sum();
    let mut files: Vec<Option<usize>> = Vec::with_capacity(total_size); // too long

    for i in (0..bytes.len()).step_by(2) {
        let id = i / 2;
        let x = bytes[i] - b'0';
        let y = if i + 1 < bytes.len() {
            bytes[i + 1] - b'0'
        } else {
            0
        };

        files.extend(repeat(Some(id)).take(x as usize));
        files.extend(repeat(None).take(y as usize));
    }

    let p1 = part_1(files.clone());
    let p2 = part_2(files);

    (p1, p2)
}

fn part_1(mut files: Vec<Option<usize>>) -> usize {
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
        .enumerate()
        .fold(0, |acc, (i, v)| acc + i * v.unwrap_or(0))
}

fn part_2(mut files: Vec<Option<usize>>) -> usize {
    let (mut empty_blocks, full_blocks) = collect_blocks(&files);

    for full_block in full_blocks {
        if let Some(empty_block_index) = empty_blocks
            .iter()
            .position(|empty_block| empty_block.size >= full_block.size)
        {
            let empty_block = &empty_blocks[empty_block_index];

            if empty_block.index < full_block.index {
                for i in 0..full_block.size {
                    files.swap(empty_block.index + i, full_block.index + i);
                }

                let remaining_size = empty_blocks[empty_block_index].size - full_block.size;

                if remaining_size == 0 {
                    empty_blocks.remove(empty_block_index);
                } else {
                    empty_blocks[empty_block_index].size = remaining_size;
                    empty_blocks[empty_block_index].index += full_block.size;
                }
            }
        }
    }

    files
        .iter()
        .enumerate()
        .fold(0, |acc, (i, v)| acc + i * v.unwrap_or(0))
}

fn collect_blocks(files: &[Option<usize>]) -> (Vec<Block>, Vec<Block>) {
    let mut empty_blocks = Vec::new();
    let mut full_blocks = Vec::new();

    let mut i = 0;
    while i < files.len() {
        match files[i] {
            Some(value) => {
                let start = i;
                while i < files.len() && files[i] == Some(value) {
                    i += 1;
                }
                full_blocks.push(Block {
                    size: i - start,
                    index: start,
                });
            }
            None => {
                let start = i;
                while i < files.len() && files[i].is_none() {
                    i += 1;
                }
                empty_blocks.push(Block {
                    size: i - start,
                    index: start,
                });
            }
        }
    }
    full_blocks.reverse();

    (empty_blocks, full_blocks)
}

#[cfg(test)]
mod tests {
    use crate::part_1_and_2;

    const INPUT: &str = "2333133121414131402";

    #[test]
    fn test_part_1() {
        let (p1, _) = part_1_and_2(INPUT);
        assert_eq!(p1, 1928);
    }

    #[test]
    fn test_part_2() {
        let (_, p2) = part_1_and_2(INPUT);
        assert_eq!(p2, 2858);
    }
}
