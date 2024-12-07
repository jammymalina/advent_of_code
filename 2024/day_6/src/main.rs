use rayon::prelude::*;
use rustc_hash::FxHashSet as HashSet;

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
enum GuardDirection {
    North,
    East,
    South,
    West,
}

#[derive(Copy, Clone)]
struct Guard {
    x: i32,
    y: i32,
    direction: GuardDirection,
}

impl Guard {
    const fn new(x: i32, y: i32, direction_symbol: char) -> Self {
        let direction = match direction_symbol {
            '^' => GuardDirection::North,
            '>' => GuardDirection::East,
            'v' => GuardDirection::South,
            '<' => GuardDirection::West,
            _ => panic!("Unexpected direction symbol"),
        };
        Self { x, y, direction }
    }

    fn turn_right(&mut self) {
        match self.direction {
            GuardDirection::North => self.direction = GuardDirection::East,
            GuardDirection::East => self.direction = GuardDirection::South,
            GuardDirection::South => self.direction = GuardDirection::West,
            GuardDirection::West => self.direction = GuardDirection::North,
        }
    }

    fn move_forward(&mut self) {
        let (x, y) = self.get_next_forward_position();
        self.x = x;
        self.y = y;
    }

    const fn get_next_forward_position(&self) -> (i32, i32) {
        match self.direction {
            GuardDirection::North => (self.x, self.y - 1),
            GuardDirection::East => (self.x + 1, self.y),
            GuardDirection::South => (self.x, self.y + 1),
            GuardDirection::West => (self.x - 1, self.y),
        }
    }
}

struct TraverseInfo {
    visited: HashSet<(i32, i32)>,
    is_loop: bool,
}

#[derive(Clone)]
struct Grid {
    width: i32,
    height: i32,
    guard: Guard,
    obstacles: HashSet<(i32, i32)>,
}

impl Grid {
    const fn new(width: i32, height: i32, guard: Guard, obstacles: HashSet<(i32, i32)>) -> Self {
        Self {
            width,
            height,
            guard,
            obstacles,
        }
    }

    fn guard_traverse(&self) -> TraverseInfo {
        let mut guard = self.guard;

        let mut visited = HashSet::default();
        let mut loop_check = HashSet::default();

        while guard.x >= 0 && guard.y >= 0 && guard.x < self.width && guard.y < self.height {
            visited.insert((guard.x, guard.y));
            if loop_check.contains(&(guard.x, guard.y, guard.direction)) {
                return TraverseInfo {
                    visited,
                    is_loop: true,
                };
            }
            loop_check.insert((guard.x, guard.y, guard.direction));

            let (next_x, next_y) = guard.get_next_forward_position();

            if self.obstacles.contains(&(next_x, next_y)) {
                guard.turn_right();
            } else {
                guard.move_forward();
            }
        }

        TraverseInfo {
            visited,
            is_loop: false,
        }
    }

    fn find_obstruction_count(&self) -> i32 {
        let traverse_info = self.guard_traverse();

        if traverse_info.is_loop {
            return 0;
        }

        let mut path = traverse_info.visited;
        path.remove(&(self.guard.x, self.guard.y));

        path.par_iter()
            .map(|(x, y)| {
                let mut grid = self.clone();
                grid.add_obstacle(*x, *y);
                let traverse_info = grid.guard_traverse();

                i32::from(traverse_info.is_loop)
            })
            .sum()
    }

    fn add_obstacle(&mut self, x: i32, y: i32) {
        self.obstacles.insert((x, y));
    }
}

fn main() {
    let input = include_str!("input.txt");
    let lines: Vec<_> = input.lines().collect();

    assert!(!lines.is_empty(), "Input seems to be empty");

    let grid_width = lines[0].len();
    let grid_height = lines.len();

    assert!(grid_width > 0, "Input is invalid, line seems to be empty");

    let mut guards = vec![];
    let mut obstacles = HashSet::default();

    for (y, line) in lines.into_iter().enumerate() {
        assert!(
            line.len() == grid_width,
            "Each line must have the same width"
        );

        for (x, field) in line.chars().enumerate() {
            match field {
                '#' => {
                    obstacles.insert((x.try_into().unwrap(), y.try_into().unwrap()));
                }
                '.' => {}
                '^' | '>' | 'v' | '<' => {
                    guards.push(Guard::new(
                        x.try_into().unwrap(),
                        y.try_into().unwrap(),
                        field,
                    ));
                }
                _ => {
                    panic!("Unknown symbol in the input");
                }
            };
        }
    }

    assert!(guards.len() == 1, "There must be exactly one guard");
    let guard = guards.into_iter().next().unwrap();

    let grid = Grid::new(
        grid_width.try_into().unwrap(),
        grid_height.try_into().unwrap(),
        guard,
        obstacles,
    );

    let guard_visited_fields = grid.guard_traverse().visited.len();
    println!("Guard visited {guard_visited_fields} fields before leaving the grid");

    let loop_obstacle_count = grid.find_obstruction_count();
    println!("There are {loop_obstacle_count} options where to place obstruction to get guard stuck in the loop");
}
