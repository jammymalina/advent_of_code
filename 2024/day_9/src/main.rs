use std::cmp::Ordering;

#[derive(Clone, Copy, Debug)]
enum FileBlock {
    File(u32, u32),
    Empty(u32),
}

struct FileSystem {
    blocks: Vec<FileBlock>,
}

impl FileSystem {
    fn new(disk_map: &str) -> Self {
        let mut blocks = vec![];

        let mut is_file = true;
        let mut file_id = 0;
        for c in disk_map.chars() {
            let block_count = c.to_digit(10).unwrap();
            if is_file {
                assert!(block_count > 0, "File must have at least one block");
                blocks.push(FileBlock::File(file_id, block_count));
                file_id += 1;
            } else {
                blocks.push(FileBlock::Empty(block_count));
            }

            is_file = !is_file;
        }

        Self { blocks }
    }

    fn compress_fragmented(&mut self) -> u64 {
        let mut left_index = 0;
        let mut right_index = self.blocks.len() - 1;

        while left_index < right_index {
            while left_index < right_index
                && (matches!(self.blocks[left_index], FileBlock::File(_, _))
                    || matches!(self.blocks[left_index], FileBlock::Empty(0)))
            {
                left_index += 1;
            }

            while left_index < right_index
                && matches!(self.blocks[right_index], FileBlock::Empty(_))
            {
                right_index -= 1;
            }

            if left_index >= right_index {
                break;
            }

            let empty_block_count = match self.blocks.get(left_index) {
                Some(FileBlock::Empty(block_count)) => *block_count,
                _ => unreachable!(),
            };

            let (file_id, file_block_count) = match self.blocks.get(right_index) {
                Some(FileBlock::File(file_id, block_count)) => (*file_id, *block_count),
                _ => unreachable!(),
            };

            match file_block_count.cmp(&empty_block_count) {
                Ordering::Less | Ordering::Equal => {
                    self.blocks[left_index] = FileBlock::File(file_id, file_block_count);
                    self.blocks[right_index] = FileBlock::Empty(file_block_count);

                    let free_space = empty_block_count - file_block_count;
                    if free_space > 0 {
                        self.blocks
                            .insert(left_index + 1, FileBlock::Empty(free_space));
                    }
                }
                Ordering::Greater => {
                    self.blocks[left_index] = FileBlock::File(file_id, empty_block_count);

                    let leftover_file_block_count = file_block_count - empty_block_count;

                    self.blocks[right_index] = FileBlock::File(file_id, leftover_file_block_count);
                    self.blocks
                        .insert(right_index + 1, FileBlock::Empty(empty_block_count));
                }
            }
        }

        self.checksum()
    }

    fn compress_non_fragmented(&mut self) -> u64 {
        self.checksum()
    }

    fn checksum(&self) -> u64 {
        let mut position = 0;
        self.blocks
            .iter()
            .map(|block| match block {
                FileBlock::Empty(block_size) => {
                    position += block_size;
                    0
                }
                FileBlock::File(file_id, block_size) => {
                    let mut result = 0;
                    for _ in 0..*block_size {
                        result += u64::from(position) * u64::from(*file_id);
                        position += 1;
                    }
                    result
                }
            })
            .sum()
    }
}

fn main() {
    let input = include_str!("input.txt").trim();

    let mut fs = FileSystem::new(input);
    let checksum = fs.compress_fragmented();
    println!("Checksum after moving files to the left: {checksum}");
}
