use std::{collections::HashSet, fs::read_to_string, path::Path};

pub const PART_1: usize = 373;
pub const PART_2: usize = 997;

pub fn read_data(data_dir: &str) -> String {
    read_to_string(Path::new(data_dir).join("day_24.txt"))
        .unwrap()
        .trim()
        .to_string()
}

#[derive(Debug, PartialEq, Eq)]
pub struct Input {
    maze: Maze,
}

impl Input {
    pub fn from_data(data: &str) -> Self {
        let mut maze = Maze {
            max_x: 0,
            max_y: 0,
            blizzards: Vec::new(),
        };
        for (y, line) in data.lines().skip(1).enumerate() {
            let y = y as u8 + 1;
            for (x, char) in line.chars().skip(1).enumerate() {
                let x = x as u8;
                if char == '#' {
                    if x == 0 {
                        return Self { maze };
                    }
                    break;
                }
                maze.max_x = maze.max_x.max(x);
                match char {
                    '>' => maze.blizzards.push(([x, y], Blizzard::Right)),
                    '<' => maze.blizzards.push(([x, y], Blizzard::Left)),
                    'v' => maze.blizzards.push(([x, y], Blizzard::Down)),
                    '^' => maze.blizzards.push(([x, y], Blizzard::Up)),
                    '.' => {}
                    _ => unreachable!(),
                }
            }
            maze.max_y = y;
        }
        Self { maze }
    }

    pub fn part_1(&self) -> usize {
        let mut maze = self.maze.clone();

        let mut options = HashSet::from([[0, 0]]);
        for time in 0..1024 {
            println!("Time: {time}\tPossibilities: {}", options.len());
            maze.step();
            let mut new_options = HashSet::new();
            for [x, y] in options.into_iter() {
                if x == maze.max_x && y == maze.max_y + 1 {
                    return time;
                }
                if x == maze.max_x && y == maze.max_y {
                    new_options.insert([x, y + 1]);
                    continue;
                }
                if y == 0 {
                    assert_eq!(x, 0);
                    if !maze.contains(&[0, 1]) {
                        new_options.insert([0, 1]);
                    }
                    new_options.insert([0, 0]);
                    continue;
                }

                if x > 0 {
                    let left = [x - 1, y];
                    if !maze.contains(&left) {
                        new_options.insert(left);
                    }
                }
                if x < maze.max_x {
                    let right = [x + 1, y];
                    if !maze.contains(&right) {
                        new_options.insert(right);
                    }
                }
                if y > 1 {
                    let up = [x, y - 1];
                    if !maze.contains(&up) {
                        new_options.insert(up);
                    }
                }
                if y < maze.max_y {
                    let down = [x, y + 1];
                    if !maze.contains(&down) {
                        new_options.insert(down);
                    }
                }
                if !maze.contains(&[x, y]) {
                    new_options.insert([x, y]);
                }
            }
            options = new_options;
        }
        unreachable!()
    }

    pub fn part_2(&self) -> usize {
        let mut maze = self.maze.clone();

        let mut options = HashSet::from([([0, 0], 0)]);
        for time in 0..1024 {
            println!("Time: {time}\tPossibilities: {}", options.len());
            maze.step();
            let mut new_options = HashSet::new();
            for ([x, y], stage) in options.into_iter() {
                match stage {
                    0 => {
                        if x == maze.max_x && y == maze.max_y {
                            new_options.insert(([x, y + 1], stage + 1));
                            continue;
                        }
                        if y == 0 {
                            assert_eq!(x, 0);
                            if !maze.contains(&[0, 1]) {
                                new_options.insert(([0, 1], stage));
                            }
                            new_options.insert(([0, 0], stage));
                            continue;
                        }
                    }
                    1 => {
                        if x == 0 && y == 1 {
                            new_options.insert(([x, y + 1], stage + 1));
                            continue;
                        }
                        if y > maze.max_y {
                            assert_eq!(x, maze.max_x);
                            if !maze.contains(&[maze.max_x, maze.max_y]) {
                                new_options.insert(([maze.max_x, maze.max_y], stage));
                            }
                            new_options.insert(([maze.max_x, maze.max_y + 1], stage));
                            continue;
                        }
                    }
                    2 => {
                        if x == maze.max_x && y == maze.max_y + 1 {
                            return time;
                        }
                        if x == maze.max_x && y == maze.max_y {
                            new_options.insert(([x, y + 1], stage));
                            continue;
                        }
                        if y == 0 {
                            assert_eq!(x, 0);
                            if !maze.contains(&[0, 1]) {
                                new_options.insert(([0, 1], stage));
                            }
                            new_options.insert(([0, 0], stage));
                            continue;
                        }
                    }
                    _ => unreachable!(),
                }

                if x > 0 {
                    let left = [x - 1, y];
                    if !maze.contains(&left) {
                        new_options.insert((left, stage));
                    }
                }
                if x < maze.max_x {
                    let right = [x + 1, y];
                    if !maze.contains(&right) {
                        new_options.insert((right, stage));
                    }
                }
                if y > 1 {
                    let up = [x, y - 1];
                    if !maze.contains(&up) {
                        new_options.insert((up, stage));
                    }
                }
                if y < maze.max_y {
                    let down = [x, y + 1];
                    if !maze.contains(&down) {
                        new_options.insert((down, stage));
                    }
                }
                if !maze.contains(&[x, y]) {
                    new_options.insert(([x, y], stage));
                }
            }
            options = new_options;
        }
        unreachable!()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Maze {
    max_x: u8,
    max_y: u8,
    blizzards: Vec<([u8; 2], Blizzard)>,
}

impl Maze {
    fn contains(&self, to_find: &[u8; 2]) -> bool {
        self.blizzards.iter().any(|(point, _)| to_find.eq(point))
    }

    fn step(&mut self) {
        for ([x, y], blizzard) in self.blizzards.iter_mut() {
            match blizzard {
                Blizzard::Left => {
                    if *x == 0 {
                        *x = self.max_x;
                        continue;
                    }
                    *x -= 1;
                }
                Blizzard::Right => {
                    if *x == self.max_x {
                        *x = 0;
                        continue;
                    }
                    *x += 1;
                }
                Blizzard::Up => {
                    if *y == 1 {
                        *y = self.max_y;
                        continue;
                    }
                    *y -= 1;
                }
                Blizzard::Down => {
                    if *y == self.max_y {
                        *y = 1;
                        continue;
                    }
                    *y += 1;
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Blizzard {
    Left,
    Right,
    Up,
    Down,
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA_DIR: &str = "../../data";

    mod from_data {
        use super::*;

        struct Case<'c> {
            input: &'c str,
            expected: Input,
        }

        #[test]
        fn example() {
            run(&Case {
                input: super::example().0,
                expected: super::example().1,
            })
        }

        fn run(test: &Case) {
            assert_eq!(test.expected, Input::from_data(test.input))
        }
    }

    mod part_1 {
        use super::*;

        struct Case {
            data: Input,
            expected: usize,
        }

        #[test]
        fn example() {
            run(&Case {
                data: super::example().1,
                expected: 18,
            })
        }

        #[test]
        fn actual() {
            run(&Case {
                data: Input::from_data(&read_data(DATA_DIR)),
                expected: PART_1,
            })
        }

        fn run(test: &Case) {
            assert_eq!(test.expected, test.data.part_1())
        }
    }

    mod part_2 {
        use super::*;

        struct Case {
            data: Input,
            expected: usize,
        }

        #[test]
        fn example() {
            run(&Case {
                data: super::example().1,
                expected: 54,
            })
        }

        #[test]
        fn actual() {
            run(&Case {
                data: Input::from_data(&read_data(DATA_DIR)),
                expected: PART_2,
            })
        }

        fn run(test: &Case) {
            assert_eq!(test.expected, test.data.part_2())
        }
    }

    fn example() -> (&'static str, Input) {
        (
            "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#",
            Input {
                maze: Maze {
                    max_x: 5,
                    max_y: 4,
                    blizzards: vec![
                        ([0, 1], Blizzard::Right),
                        ([1, 1], Blizzard::Right),
                        ([3, 1], Blizzard::Left),
                        ([4, 1], Blizzard::Up),
                        ([5, 1], Blizzard::Left),
                        ([1, 2], Blizzard::Left),
                        ([4, 2], Blizzard::Left),
                        ([5, 2], Blizzard::Left),
                        ([0, 3], Blizzard::Right),
                        ([1, 3], Blizzard::Down),
                        ([3, 3], Blizzard::Right),
                        ([4, 3], Blizzard::Left),
                        ([5, 3], Blizzard::Right),
                        ([0, 4], Blizzard::Left),
                        ([1, 4], Blizzard::Up),
                        ([2, 4], Blizzard::Down),
                        ([3, 4], Blizzard::Up),
                        ([4, 4], Blizzard::Up),
                        ([5, 4], Blizzard::Right),
                    ],
                },
            },
        )
    }
}
