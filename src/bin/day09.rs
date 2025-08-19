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

    let mut files_p1 = files.clone();
    let mut free_index = files_p1.iter().position(|f| f.is_none()).unwrap();
    let mut last_index = files_p1.iter().rposition(|f| f.is_some()).unwrap();
    while free_index != last_index + 1 {
        files_p1.swap(free_index, last_index);

        while files_p1[free_index].is_some() {
            free_index += 1;
        }

        while files_p1[last_index].is_none() {
            last_index -= 1;
        }
    }

    let (mut empty_blocks, full_blocks) = collect_blocks(&files);

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

    let p1 = files_p1
        .iter()
        .flatten()
        .enumerate()
        .map(|(i, &v)| i * v)
        .sum();
    let p2 = files
        .iter()
        .enumerate()
        .map(|(i, v)| match v {
            Some(v) => i * v,
            None => 0,
        })
        .sum();

    (p1, p2)
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

// TODO instead of element to element jump index for large blocks
fn collect_blocks(files: &[Option<usize>]) -> (Vec<Block>, Vec<Block>) {
    let mut empty_blocks = Vec::new();
    let mut full_blocks = Vec::new();

    let mut empty_buffer: Option<(usize, usize)> = None;
    let mut full_buffer: Option<(usize, usize, usize)> = None;

    for (i, file) in files.iter().enumerate() {
        match file {
            Some(value) => {
                if let Some((start, size)) = empty_buffer.take() {
                    empty_blocks.push(Block {
                        size: size,
                        index: start,
                    });
                }

                full_buffer = Some(match full_buffer {
                    None => (i, 1, *value),
                    Some((start, size, prev_value)) if prev_value == *value => {
                        (start, size + 1, *value)
                    }
                    Some((start, size, _)) => {
                        full_blocks.push(Block { size, index: start });
                        (i, 1, *value)
                    }
                });
            }
            None => {
                if let Some((start, size, _)) = full_buffer.take() {
                    full_blocks.push(Block { size, index: start });
                }

                empty_buffer = Some(match empty_buffer {
                    None => (i, 1),
                    Some((start, size)) => (start, size + 1),
                });
            }
        }
    }

    if let Some((start, size)) = empty_buffer {
        empty_blocks.push(Block { size, index: start });
    }
    if let Some((start, size, _)) = full_buffer {
        full_blocks.push(Block { size, index: start });
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
