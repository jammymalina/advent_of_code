use rayon::prelude::*;
use rustc_hash::FxHashSet as HashSet;

struct Grid {
    terrain: Vec<u32>,
    width: i32,
    height: i32,
}

impl Grid {
    fn new(terrain: Vec<u32>, width: usize, height: usize) -> Self {
        Self {
            terrain,
            width: i32::try_from(width).unwrap(),
            height: i32::try_from(height).unwrap(),
        }
    }

    fn find_trail_heads(&self) -> Vec<(i32, i32)> {
        (0..self.height)
            .flat_map(|row| (0..self.width).map(move |col| (row, col)))
            .filter(|(row, col)| {
                self.get_terrain_height(*row, *col)
                    .map_or_else(|| unreachable!(), |terrain_height| terrain_height == 0)
            })
            .collect()
    }

    fn find_reachable_targets(&self, target_height: u32) -> u32 {
        let trail_heads: Vec<(i32, i32)> = self.find_trail_heads();
        if trail_heads.is_empty() {
            return 0;
        }

        trail_heads
            .into_par_iter()
            .map(|(row, col)| {
                u32::try_from(
                    self.find_reachable_target_from_coords(row, col, target_height)
                        .len(),
                )
                .unwrap()
            })
            .sum()
    }

    fn find_reachable_target_from_coords(
        &self,
        row: i32,
        col: i32,
        target_height: u32,
    ) -> HashSet<(i32, i32)> {
        let terrain_height = self.get_terrain_height(row, col).unwrap();
        if terrain_height == target_height {
            return vec![(row, col)].into_iter().collect();
        }

        vec![
            (row - 1, col),
            (row + 1, col),
            (row, col - 1),
            (row, col + 1),
        ]
        .into_par_iter()
        .map(|(row, col)| (row, col, self.get_terrain_height(row, col)))
        .filter(|(_, _, surr_terrain_height)| {
            surr_terrain_height.map_or(false, |surr_terrain_height| {
                surr_terrain_height == terrain_height + 1
            })
        })
        .flat_map(|(row, col, _)| self.find_reachable_target_from_coords(row, col, target_height))
        .collect()
    }

    fn find_all_routes(&self, target_height: u32) -> u32 {
        let trail_heads: Vec<(i32, i32)> = self.find_trail_heads();
        if trail_heads.is_empty() {
            return 0;
        }

        trail_heads
            .into_par_iter()
            .map(|(row, col)| self.find_route_from_coords(row, col, target_height))
            .sum()
    }

    fn find_route_from_coords(&self, row: i32, col: i32, target_height: u32) -> u32 {
        let terrain_height = self.get_terrain_height(row, col).unwrap();
        if terrain_height == target_height {
            return 1;
        }

        vec![
            (row - 1, col),
            (row + 1, col),
            (row, col - 1),
            (row, col + 1),
        ]
        .into_par_iter()
        .map(|(row, col)| (row, col, self.get_terrain_height(row, col)))
        .filter(|(_, _, surr_terrain_height)| {
            surr_terrain_height.map_or(false, |surr_terrain_height| {
                surr_terrain_height == terrain_height + 1
            })
        })
        .map(|(row, col, _)| self.find_route_from_coords(row, col, target_height))
        .sum()
    }

    fn get_terrain_height(&self, row: i32, col: i32) -> Option<u32> {
        if row < 0 || col < 0 || col >= self.width || row >= self.height {
            None
        } else {
            Some(self.terrain[usize::try_from(col + row * self.height).unwrap()])
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

    let terrain: Vec<u32> = lines
        .iter()
        .flat_map(|line| {
            assert!(
                line.len() == grid_width,
                "Each line must have the same width"
            );
            line.chars().map(|c| c.to_digit(10).unwrap())
        })
        .collect();

    let grid = Grid::new(terrain, grid_width, grid_height);

    let reachable_targets = grid.find_reachable_targets(9);
    println!("Found {reachable_targets} reachable targets");

    let all_routes = grid.find_all_routes(9);
    println!("Found {all_routes} unique routes to target");
}
