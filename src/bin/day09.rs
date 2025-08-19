use std::iter::repeat;

const DATA: &str = include_str!("../../input/09.in");

fn main() {
    // let (p1, p2) = part_1_and_2(&DATA);
    let now = std::time::Instant::now();
    let p1 = part_1(DATA);
    let elapsed = now.elapsed();
    println!("Part 1: {p1:?}\nTime: {elapsed:?}");
    let now = std::time::Instant::now();
    let p2 = part_2(DATA);
    let elapsed = now.elapsed();
    println!("Part 2: {p2:?}\nTime: {elapsed:?}");
}

fn part_1(data: &str) -> usize {
    let bytes = data.trim_end().as_bytes();
    let total_size: usize = bytes.iter().map(|&b| (b - b'0') as usize).sum();
    let mut files: Vec<Option<usize>> = Vec::with_capacity(total_size); // too long

    // populate file system
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

#[derive(Clone, PartialEq)]
struct Block {
    size: usize,
    index: usize,
}

fn part_2(data: &str) -> usize {
    let bytes = data.trim_end().as_bytes();
    let total_size: usize = bytes.iter().map(|&b| (b - b'0') as usize).sum();
    let mut files: Vec<Option<usize>> = Vec::with_capacity(total_size);

    for i in (0..bytes.len()).step_by(2) {
        let id = i / 2;
        let x = bytes[i] - b'0';
        #[rustfmt::skip]
        let y = if i + 1 < bytes.len() { bytes[i + 1] - b'0' } else { 0 };

        files.extend(repeat(Some(id)).take(x as usize));
        files.extend(repeat(None).take(y as usize));
    }

    let mut empty_blocks = collect_empty_blocks(&files);
    let full_blocks = collect_full_blocks(&files);

    for full_block in full_blocks {
        if let Some(empty_block_index) = find_empty_block_index(&empty_blocks, &full_block) {
            let empty_block = &empty_blocks[empty_block_index];

            if empty_block.index < full_block.index {
                for i in 0..full_block.size {
                    files.swap(empty_block.index + i, full_block.index + i);
                }

                update_empty_blocks(&mut empty_blocks, empty_block_index, full_block.size)
            }
        }
    }

    files
        .iter()
        .enumerate()
        .map(|(i, v)| match v {
            Some(v) => i * v,
            None => 0,
        })
        .sum()
}

fn find_empty_block_index(empty_blocks: &[Block], full_block: &Block) -> Option<usize> {
    empty_blocks
        .iter()
        .position(|empty_block| empty_block.size >= full_block.size)
}

fn update_empty_blocks(
    empty_blocks: &mut Vec<Block>,
    empty_block_idx: usize,
    full_block_size: usize,
) {
    let remaining_size = empty_blocks[empty_block_idx].size - full_block_size;

    if remaining_size == 0 {
        empty_blocks.remove(empty_block_idx);
    } else {
        empty_blocks[empty_block_idx].size = remaining_size;
        empty_blocks[empty_block_idx].index += full_block_size;
    }
}

fn collect_empty_blocks(files: &[Option<usize>]) -> Vec<Block> {
    let mut empty_list = Vec::new();
    let mut current_buffer = None;

    for (i, file) in files.iter().enumerate() {
        match (file, &mut current_buffer) {
            (None, None) => {
                current_buffer = Some((i, 1));
            }
            (None, Some((_, size))) => *size += 1,
            (Some(_), Some((start_index, size))) => {
                empty_list.push(Block {
                    size: *size,
                    index: *start_index,
                });
                current_buffer = None;
            }
            (Some(_), None) => {}
        }
    }

    if let Some((start_index, size)) = current_buffer {
        empty_list.push(Block {
            size,
            index: start_index,
        });
    }
    empty_list
}

fn collect_full_blocks(files: &[Option<usize>]) -> Vec<Block> {
    let mut full_list = Vec::new();
    let mut current_buffer = None;

    for (i, file) in files.iter().enumerate() {
        match (file, &mut current_buffer) {
            (None, None) => {}
            (None, Some((start_index, size, _))) => {
                full_list.push(Block {
                    size: *size,
                    index: *start_index,
                });
                current_buffer = None;
            }
            (Some(next_value), Some((start_index, size, value))) => {
                if next_value == value {
                    *size += 1
                } else {
                    full_list.push(Block {
                        size: *size,
                        index: *start_index,
                    });
                    current_buffer = Some((i, 1, *next_value))
                }
            }
            (Some(value), None) => current_buffer = Some((i, 1, *value)),
        }
    }

    if let Some((start_index, size, _)) = current_buffer {
        full_list.push(Block {
            size,
            index: start_index,
        });
    }
    full_list.reverse();
    full_list
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
        assert_eq!(p2, 2858);
    }
}
