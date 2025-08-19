use std::iter::repeat;

use aoc_24::utils::collections::LinkedList;

const DATA: &str = include_str!("../../input/09.in");

fn main() {
    // let (p1, p2) = part_1_and_2(&DATA);
    let now = std::time::Instant::now();
    let p1 = part_1(DATA);
    let p2 = part_2(DATA);
    let elapsed = now.elapsed();
    println!("Part 1: {p1:?}\nPart 2: {p2:?}\nTime: {elapsed:?}");
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

#[derive(PartialEq)]
struct Block {
    size: usize,
    index: usize,
    value: Option<usize>,
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

    let mut empty_list = collect_empty_blocks(&files);
    let full_list = collect_full_blocks(&files);
    for full_block in full_list {
        if let Some(empty_block) = empty_list.match_gt(|s| s.size >= full_block.size) {
            if empty_block.index < full_block.index {
                for i in 0..full_block.size {
                    files.swap(empty_block.index + i, full_block.index + i);
                }
                empty_list = collect_empty_blocks(&files);
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

fn collect_empty_blocks(files: &[Option<usize>]) -> LinkedList<Block> {
    let mut empty_list = LinkedList::new();
    let mut current_buffer = None;

    for (i, file) in files.iter().enumerate() {
        match (file, &mut current_buffer) {
            (None, None) => {
                current_buffer = Some((i, 1));
            }
            (None, Some((_, size))) => *size += 1,
            (Some(_), Some((start_index, size))) => {
                empty_list.push_front(Block {
                    size: *size,
                    index: *start_index,
                    value: None,
                });
                current_buffer = None;
            }
            (Some(_), None) => {}
        }
    }

    if let Some((start_index, size)) = current_buffer {
        empty_list.push_front(Block {
            size,
            index: start_index,
            value: None,
        });
    }
    empty_list.reverse();
    empty_list
}

fn collect_full_blocks(files: &[Option<usize>]) -> Vec<Block> {
    let mut full_list = vec![];
    let mut current_buffer = None;

    for (i, file) in files.iter().enumerate() {
        match (file, &mut current_buffer) {
            (None, None) => {}
            (None, Some((start_index, size, value))) => {
                full_list.push(Block {
                    size: *size,
                    index: *start_index,
                    value: Some(*value),
                });
                current_buffer = None;
            }
            (Some(value), Some(buffer)) => {
                if *value == buffer.2 {
                    buffer.1 += 1
                } else {
                    full_list.push(Block {
                        size: buffer.1,
                        index: buffer.0,
                        value: Some(buffer.2),
                    });
                    current_buffer = Some((i, 1, *value))
                }
            }
            (Some(value), None) => current_buffer = Some((i, 1, *value)),
        }
    }

    if let Some((start_index, size, value)) = current_buffer {
        full_list.push(Block {
            size,
            index: start_index,
            value: Some(value),
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
