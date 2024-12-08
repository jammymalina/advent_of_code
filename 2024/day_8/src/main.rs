use itertools::Itertools;
use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};

struct Grid {
    antennas: HashMap<char, Vec<(i32, i32)>>,
    width: i32,
    height: i32,
}

impl Grid {
    fn new(antennas: HashMap<char, Vec<(i32, i32)>>, width: usize, height: usize) -> Self {
        Self {
            antennas,
            width: i32::try_from(width).unwrap(),
            height: i32::try_from(height).unwrap(),
        }
    }

    fn find_antinodes_within_distance(&self) -> u32 {
        let mut antinodes = HashSet::default();

        for positions in self.antennas.values() {
            let antenna_pairs = positions.iter().combinations(2);
            for pair in antenna_pairs {
                let pos1 = *pair[0];
                let pos2 = *pair[1];

                let distance = (pos2.0 - pos1.0, pos2.1 - pos1.1);
                let antinode_1 = (pos1.0 - distance.0, pos1.1 - distance.1);
                let antinode_2 = (pos2.0 + distance.0, pos2.1 + distance.1);

                if self.validate_antinode_position(antinode_1.0, antinode_1.1) {
                    antinodes.insert(antinode_1);
                }

                if self.validate_antinode_position(antinode_2.0, antinode_2.1) {
                    antinodes.insert(antinode_2);
                }
            }
        }

        antinodes.len().try_into().unwrap()
    }

    fn find_antinodes_any_distance(&self) -> u32 {
        let mut antinodes = HashSet::default();

        for positions in self.antennas.values() {
            let antenna_pairs = positions.iter().combinations(2);
            for pair in antenna_pairs {
                let pos1 = *pair[0];
                let pos2 = *pair[1];

                let distance = (pos2.0 - pos1.0, pos2.1 - pos1.1);

                let divisor = Self::gcd(distance.0, distance.1);
                let delta_row = distance.0 / divisor;
                let delta_col = distance.1 / divisor;

                for (start_n, increment) in [(0, 1), (-1, -1)] {
                    let mut n = start_n;
                    while self
                        .validate_antinode_position(pos1.0 + n * delta_row, pos1.1 + n * delta_col)
                    {
                        antinodes.insert((pos1.0 + n * delta_row, pos1.1 + n * delta_col));
                        n += increment;
                    }
                }
            }
        }

        antinodes.len().try_into().unwrap()
    }

    const fn validate_antinode_position(&self, row: i32, col: i32) -> bool {
        row >= 0 && row < self.width && col >= 0 && col < self.height
    }

    const fn gcd(a: i32, b: i32) -> i32 {
        if b == 0 {
            return a;
        }

        Self::gcd(b, a % b)
    }
}

fn main() {
    let input = include_str!("input.txt");
    let lines: Vec<_> = input.lines().collect();

    assert!(!lines.is_empty(), "Input seems to be empty");

    let grid_width = lines[0].len();
    let grid_height = lines.len();

    assert!(grid_width > 0, "Input is invalid, line seems to be empty");

    let mut antennas = HashMap::default();

    lines
        .iter()
        .enumerate()
        .flat_map(|(row, line)| {
            assert!(
                line.len() == grid_width,
                "Each line must have the same width"
            );
            line.chars().enumerate().filter_map(move |(col, c)| {
                let row = i32::try_from(row).unwrap();
                let col = i32::try_from(col).unwrap();
                if c == '.' {
                    None
                } else {
                    Some((c, (row, col)))
                }
            })
        })
        .for_each(|(c, pos)| antennas.entry(c).or_insert_with(Vec::new).push(pos));

    let grid = Grid::new(antennas, grid_width, grid_height);

    let antinode_count = grid.find_antinodes_within_distance();
    println!("Antinode count with distance restrictions: {antinode_count}");

    let antinode_count = grid.find_antinodes_any_distance();
    println!("Antinode count without any distance restrictions {antinode_count}");
}
