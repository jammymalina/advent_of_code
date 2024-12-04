use std::collections::HashSet;

struct Grid {
    chars: Vec<char>,
    width: i32,
    height: i32,
}

impl Grid {
    fn new(chars: Vec<char>, width: usize, height: usize) -> Self {
        Self {
            chars,
            width: i32::try_from(width).unwrap(),
            height: i32::try_from(height).unwrap(),
        }
    }

    fn find_x_shaped_xmas(&self) -> u32 {
        let mut result = 0;

        let expected_chars: HashSet<char> = "MS".chars().collect();

        for row in 0..self.height {
            for col in 0..self.width {
                if let Some(c) = self.get_char(row, col) {
                    if c == 'A' {
                        let x1: HashSet<char> = vec![
                            self.get_char(row - 1, col - 1),
                            self.get_char(row + 1, col + 1),
                        ]
                        .into_iter()
                        .flatten()
                        .collect();

                        let x2: HashSet<char> = vec![
                            self.get_char(row - 1, col + 1),
                            self.get_char(row + 1, col - 1),
                        ]
                        .into_iter()
                        .flatten()
                        .collect();

                        if x1 == expected_chars && x2 == expected_chars {
                            result += 1;
                        }
                    }
                }
            }
        }

        result
    }

    fn find_string(&self, text: &str) -> u32 {
        let mut result = 0;

        let directions = [
            (0, 1),
            (0, -1),
            (1, 0),
            (-1, 0),
            (1, 1),
            (-1, 1),
            (1, -1),
            (-1, -1),
        ];

        for row in 0..self.height {
            for col in 0..self.width {
                if let Some(c) = self.get_char(row, col) {
                    if text.starts_with(c) {
                        for direction in directions {
                            if self.find_word_dir(text, row, col, direction) {
                                result += 1;
                            }
                        }
                    }
                }
            }
        }

        result
    }

    fn find_word_dir(&self, text: &str, row: i32, col: i32, direction: (i32, i32)) -> bool {
        if text.is_empty() {
            return true;
        }

        if let Some(c) = self.get_char(row, col) {
            if text.starts_with(c) {
                return self.find_word_dir(
                    &text[1..],
                    row + direction.0,
                    col + direction.1,
                    direction,
                );
            }
        }

        false
    }

    fn get_char(&self, row: i32, col: i32) -> Option<char> {
        if row < 0 || col < 0 || col >= self.width || row >= self.height {
            None
        } else {
            Some(self.chars[usize::try_from(col + row * self.height).unwrap()])
        }
    }
}

fn main() {
    let input = include_str!("input.txt");
    let lines: Vec<_> = input.lines().collect();

    assert!(!lines.is_empty(), "Input seems to be empty");

    let grid_width = lines[0].len();
    let grid_height = lines.len();

    assert!(grid_width > 0, "Input is invalid, line seems to be empty");

    let grid_chars: Vec<char> = lines
        .iter()
        .flat_map(|line| {
            assert!(
                line.len() == grid_width,
                "Each line must have the same width"
            );
            line.chars()
        })
        .collect();

    let grid = Grid::new(grid_chars, grid_width, grid_height);

    let xmas_count = grid.find_string("XMAS");
    println!("XMAS count: {xmas_count}");

    let xmas_count = grid.find_x_shaped_xmas();
    println!("X shaped XMAS count {xmas_count}");
}
