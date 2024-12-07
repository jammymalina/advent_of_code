use std::collections::HashSet;

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
enum GuardDirection {
    North,
    East,
    South,
    West,
}

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

    fn reset(&mut self, x: i32, y: i32, direction: GuardDirection) {
        self.x = x;
        self.y = y;
        self.direction = direction;
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

    fn guard_traverse(&mut self) -> Option<usize> {
        let mut visited = HashSet::new();
        let mut loop_check = HashSet::new();

        while self.guard.x >= 0
            && self.guard.y >= 0
            && self.guard.x < self.width
            && self.guard.y < self.height
        {
            visited.insert((self.guard.x, self.guard.y));
            if loop_check.contains(&(self.guard.x, self.guard.y, self.guard.direction)) {
                return None;
            }
            loop_check.insert((self.guard.x, self.guard.y, self.guard.direction));

            let (next_x, next_y) = self.guard.get_next_forward_position();

            if self.obstacles.contains(&(next_x, next_y)) {
                self.guard.turn_right();
            } else {
                self.guard.move_forward();
            }
        }

        Some(visited.len())
    }

    fn find_obstruction_count(&mut self) -> usize {
        let mut count = 0;

        let guard_x = self.guard.x;
        let guard_y = self.guard.y;
        let guard_dir = self.guard.direction;

        for x in 0..self.width {
            for y in 0..self.height {
                if (x == guard_x && y == guard_y) || self.obstacles.contains(&(x, y)) {
                    continue;
                }

                self.obstacles.insert((x, y));
                let traverse_count = self.guard_traverse();
                if traverse_count.is_none() {
                    count += 1;
                }
                self.obstacles.remove(&(x, y));
                self.guard.reset(guard_x, guard_y, guard_dir);
            }
        }

        count
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
    let mut obstacles = HashSet::new();

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
                _ => {
                    guards.push(Guard::new(
                        x.try_into().unwrap(),
                        y.try_into().unwrap(),
                        field,
                    ));
                }
            };
        }
    }

    assert!(guards.len() == 1, "There must be exactly one guard");
    let guard = guards.into_iter().next().unwrap();

    let mut grid = Grid::new(
        grid_width.try_into().unwrap(),
        grid_height.try_into().unwrap(),
        guard,
        obstacles,
    );

    // let guard_visited_fields = grid.guard_traverse().unwrap();
    // println!("Guard visited {guard_visited_fields} fields before leaving the grid");

    let loop_obstacle_count = grid.find_obstruction_count();
    println!("There are {loop_obstacle_count} options where to place obstruction to get guard stuck in the loop");
}
