struct FileSystem {
    disk_map: Vec<u32>,
}

impl FileSystem {
    fn new(disk_map: &str) -> Self {
        Self {
            disk_map: disk_map.chars().filter_map(|c| c.to_digit(10)).collect(),
        }
    }

    fn move_files_to_left(&mut self) -> usize {
        let get_file_id = |map_index| map_index / 2;
        let add_to_checksum = |acc, disk_position, file_id, block_count| {
            let mut result = acc;
            let mut iter = disk_position;
            for _ in 0..block_count {
                result += iter * file_id;
                iter += 1;
            }
            (iter, result)
        };

        let mut checksum = 0;

        let mut disk_position = 0;
        let mut left_map_index = 0;
        let mut right_map_index = if self.disk_map.len() % 2 == 0 {
            self.disk_map.len() - 2
        } else {
            self.disk_map.len() - 1
        };

        let mut last_remaining = None;
        while left_map_index < right_map_index {
            let block_count = self.disk_map[left_map_index];
            (disk_position, checksum) = add_to_checksum(
                checksum,
                disk_position,
                get_file_id(left_map_index),
                block_count,
            );

            left_map_index += 1;
            if left_map_index >= right_map_index {
                break;
            }

            let mut free_space = self.disk_map[left_map_index];
            if let Some((block_count, map_index)) = last_remaining {
                if free_space < block_count {
                    last_remaining = Some((block_count - free_space, map_index));
                    (disk_position, checksum) = add_to_checksum(
                        checksum,
                        disk_position,
                        get_file_id(map_index),
                        block_count - free_space,
                    );
                    self.disk_map[left_map_index] = 0;
                    continue;
                }

                (disk_position, checksum) =
                    add_to_checksum(checksum, disk_position, get_file_id(map_index), block_count);
                last_remaining = None;
                free_space -= block_count;
                self.disk_map[left_map_index] = free_space;

                right_map_index = map_index - 2;
            }

            while free_space > 0 && left_map_index < right_map_index {
                let block_count = self.disk_map[right_map_index];
                if block_count > free_space {
                    last_remaining = Some((block_count - free_space, right_map_index));
                    (disk_position, checksum) = add_to_checksum(
                        checksum,
                        disk_position,
                        get_file_id(right_map_index),
                        free_space,
                    );
                    self.disk_map[left_map_index] = 0;
                    break;
                }

                (disk_position, checksum) = add_to_checksum(
                    checksum,
                    disk_position,
                    get_file_id(right_map_index),
                    block_count,
                );
                free_space -= block_count;
                self.disk_map[left_map_index] = free_space;

                right_map_index -= 2;
            }
            left_map_index += 1;
        }

        if let Some((block_count, map_index)) = last_remaining {
            (disk_position, checksum) =
                add_to_checksum(checksum, disk_position, get_file_id(map_index), block_count);
            right_map_index = map_index - 2;
        }

        for map_index in (left_map_index..right_map_index).step_by(2) {
            (disk_position, checksum) = add_to_checksum(
                checksum,
                disk_position,
                get_file_id(map_index),
                self.disk_map[map_index],
            );
        }

        checksum
    }
}

fn main() {
    let input = include_str!("input.txt").trim();

    let mut fs = FileSystem::new(input);
    let checksum = fs.move_files_to_left();
    println!("Checksum after moving files to the left: {checksum}");
}
