use std::{collections::HashMap, fs::read_to_string, path::Path};

pub const PART_1: usize = 109094;
pub const PART_2: usize = 53324;

pub fn read_data(data_dir: &str) -> String {
    read_to_string(Path::new(data_dir).join("day_22.txt"))
        .unwrap()
        .trim_end()
        .to_string()
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Input {
    map: HashMap<(usize, usize), bool>,
    instructions: Vec<Instruction>,
    start: (usize, usize),
    max: (usize, usize),
}

impl Input {
    pub fn from_data(data: &str) -> Self {
        let (map_str, instruction_str) = data.trim_end().split_once("\n\n").unwrap();

        let mut map = HashMap::new();
        let mut max = (0, 0);
        let mut start = None;

        for (row_idx, row) in map_str.trim_end().lines().enumerate() {
            let row_idx = row_idx + 1;
            for (col_idx, wall_char) in row.chars().enumerate() {
                let col_idx = col_idx + 1;
                match wall_char {
                    ' ' => {
                        continue;
                    }
                    '.' => {
                        map.insert((col_idx, row_idx), false);
                    }
                    '#' => {
                        map.insert((col_idx, row_idx), true);
                    }
                    _ => unreachable!(),
                };
                max.0 = max.0.max(col_idx);
                max.1 = max.1.max(row_idx);
                if start.is_none() {
                    start = Some((col_idx, row_idx));
                }
            }
        }

        let mut instructions = Vec::new();
        for instruction in instruction_str.trim().split_inclusive(['R', 'L']) {
            if !instruction.ends_with(['R', 'L']) {
                instructions.push(Instruction::Forward(instruction.parse().unwrap()));
                continue;
            }
            let (forward, rotate) = instruction.split_at(instruction.len() - 1);
            instructions.push(Instruction::Forward(forward.parse().unwrap()));
            instructions.push(match rotate {
                "R" => Instruction::RotateRight,
                "L" => Instruction::RotateLeft,
                _ => unreachable!(),
            });
        }

        Self {
            map,
            instructions,
            start: start.unwrap(),
            max,
        }
    }

    pub fn part_1(&self) -> usize {
        let mut pos = self.start;
        let mut facing = Facing::Right;

        'instructions: for instruction in &self.instructions {
            match instruction {
                Instruction::RotateLeft => facing = facing.next_back().unwrap(),
                Instruction::RotateRight => facing = facing.next().unwrap(),
                Instruction::Forward(steps) => {
                    for _ in 0..*steps {
                        let next = pos + facing;
                        match self.map.get(&next) {
                            Some(&true) => continue 'instructions,
                            Some(&false) => {
                                pos = next;
                            }
                            None => {
                                let mut prev = pos;
                                while self.map.get(&(prev - facing)).is_some() {
                                    prev = prev - facing;
                                }
                                if *self.map.get(&prev).unwrap() {
                                    continue 'instructions;
                                }
                                pos = prev;
                            }
                        }
                    }
                }
            }
        }

        (1000 * pos.1) + (4 * pos.0) + facing as usize
    }

    pub fn part_2(&self) -> usize {
        let mut pos = self.start;
        let mut facing = Facing::Right;

        'instructions: for instruction in &self.instructions {
            match instruction {
                Instruction::RotateLeft => facing = facing.next_back().unwrap(),
                Instruction::RotateRight => facing = facing.next().unwrap(),
                Instruction::Forward(steps) => {
                    for _ in 0..*steps {
                        let next = pos + facing;
                        match self.map.get(&next) {
                            Some(&true) => continue 'instructions,
                            Some(&false) => {
                                pos = next;
                            }
                            None => {
                                let (prev, prev_facing) = self.cube_next(pos, facing);
                                if *self.map.get(&prev).unwrap() {
                                    continue 'instructions;
                                }
                                pos = prev;
                                facing = prev_facing;
                            }
                        }
                    }
                }
            }
        }

        (1000 * pos.1) + (4 * pos.0) + facing as usize
    }

    fn cube_next(&self, cur_pos: (usize, usize), cur_facing: Facing) -> ((usize, usize), Facing) {
        // Hard-coded for the example and real data :(

        if self.max.0 < 50 {
            // test cube
            return match (cur_pos, cur_facing) {
                // 1 => 2
                ((x, 1), Facing::Up) if (9..=12).contains(&x) => ((13 - x, 5), Facing::Down),
                // 1 => 6
                ((12, y), Facing::Right) if (1..=4).contains(&y) => ((16, 13 - y), Facing::Left),
                // 4 => 6
                ((12, y), Facing::Right) if (5..=8).contains(&y) => ((21 - y, 9), Facing::Down),
                // 6 => 4
                ((x, 9), Facing::Up) if (13..=16).contains(&x) => ((12, 21 - x), Facing::Left),
                // 6 => 1
                ((16, y), Facing::Right) if (9..=12).contains(&y) => ((12, 13 - y), Facing::Left),
                // 6 => 2
                ((x, 12), Facing::Down) if (13..=16).contains(&x) => ((1, 21 - x), Facing::Right),
                // 5 => 2
                ((x, 12), Facing::Down) if (9..=12).contains(&x) => ((13 - x, 8), Facing::Up),
                // 5 => 3
                ((9, y), Facing::Left) if (9..=12).contains(&y) => ((17 - y, 8), Facing::Up),
                // 3 => 5
                ((x, 8), Facing::Down) if (5..=8).contains(&x) => ((9, 17 - x), Facing::Right),
                // 2 => 5
                ((x, 8), Facing::Down) if (1..=4).contains(&x) => ((13 - x, 12), Facing::Up),
                // 2 => 6
                ((1, y), Facing::Left) if (5..=8).contains(&y) => ((21 - y, 12), Facing::Up),
                // 2 => 1
                ((x, 5), Facing::Up) if (1..=4).contains(&x) => ((13 - x, 1), Facing::Down),
                // 3 => 1
                ((x, 5), Facing::Up) if (5..=8).contains(&x) => ((9, x - 4), Facing::Right),
                // 1 => 3
                ((9, y), Facing::Left) if (1..=4).contains(&y) => ((4 + y, 5), Facing::Down),
                _ => {
                    unreachable!()
                }
            };
        }

        //   1 2
        //   3
        // 4 5
        // 6

        match (cur_pos, cur_facing) {
            // 1 => 6
            ((x, 1), Facing::Up) if (51..=100).contains(&x) => ((1, x + 100), Facing::Right),
            // 2 => 6
            ((x, 1), Facing::Up) if (101..=150).contains(&x) => ((x - 100, 200), Facing::Up),
            // 2 => 5
            ((150, y), Facing::Right) if (1..=50).contains(&y) => ((100, 151 - y), Facing::Left),
            // 2 => 3
            ((x, 50), Facing::Down) if (101..=150).contains(&x) => ((100, x - 50), Facing::Left),
            // 3 => 2
            ((100, y), Facing::Right) if (51..=100).contains(&y) => ((y + 50, 50), Facing::Up),
            // 5 => 2
            ((100, y), Facing::Right) if (101..=150).contains(&y) => ((150, 151 - y), Facing::Left),
            // 5 => 6
            ((x, 150), Facing::Down) if (51..=100).contains(&x) => ((50, x + 100), Facing::Left),
            // 6 => 5
            ((50, y), Facing::Right) if (151..=200).contains(&y) => ((y - 100, 150), Facing::Up),
            // 6 => 2
            ((x, 200), Facing::Down) if (1..=50).contains(&x) => ((x + 100, 1), Facing::Down),
            // 6 => 1
            ((1, y), Facing::Left) if (151..=200).contains(&y) => ((y - 100, 1), Facing::Down),
            // 4 => 1
            ((1, y), Facing::Left) if (101..=150).contains(&y) => ((51, 151 - y), Facing::Right),
            // 4 => 3
            ((x, 101), Facing::Up) if (1..=50).contains(&x) => ((51, x + 50), Facing::Right),
            // 3 => 4
            ((51, y), Facing::Left) if (51..=100).contains(&y) => ((y - 50, 101), Facing::Down),
            // 1 => 4
            ((51, y), Facing::Left) if (1..=50).contains(&y) => ((1, 151 - y), Facing::Right),
            _ => {
                unreachable!()
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Instruction {
    Forward(usize),
    RotateLeft,
    RotateRight,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Facing {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}

impl std::ops::Sub<Facing> for (usize, usize) {
    type Output = (usize, usize);

    fn sub(self, rhs: Facing) -> Self::Output {
        let (col, row) = self;
        match rhs {
            Facing::Right => (col - 1, row),
            Facing::Down => (col, row - 1),
            Facing::Left => (col + 1, row),
            Facing::Up => (col, row + 1),
        }
    }
}

impl std::ops::Add<Facing> for (usize, usize) {
    type Output = (usize, usize);

    fn add(self, rhs: Facing) -> Self::Output {
        let (col, row) = self;
        match rhs {
            Facing::Right => (col + 1, row),
            Facing::Down => (col, row + 1),
            Facing::Left => (col - 1, row),
            Facing::Up => (col, row - 1),
        }
    }
}

impl Iterator for Facing {
    type Item = Facing;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Facing::Right => Some(Facing::Down),
            Facing::Down => Some(Facing::Left),
            Facing::Left => Some(Facing::Up),
            Facing::Up => Some(Facing::Right),
        }
    }
}

impl DoubleEndedIterator for Facing {
    fn next_back(&mut self) -> Option<Self::Item> {
        match self {
            Facing::Right => Some(Facing::Up),
            Facing::Down => Some(Facing::Right),
            Facing::Left => Some(Facing::Down),
            Facing::Up => Some(Facing::Left),
        }
    }
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
                expected: 6032,
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
                expected: 5031,
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

    mod cube_next {
        use super::*;

        struct Case {
            initial: (usize, usize),
            expected: (usize, usize),
            initial_facing: Facing,
            expected_facing: Facing,
        }

        mod example {
            use super::*;

            #[test]
            fn one_up() {
                // 1 up => 2 down
                let initial_facing = Facing::Up;
                let expected_facing = Facing::Down;
                run(&Case {
                    initial: (9, 1),
                    expected: (4, 5),
                    initial_facing,
                    expected_facing,
                });
                run(&Case {
                    initial: (12, 1),
                    expected: (1, 5),
                    initial_facing,
                    expected_facing,
                });
            }

            #[test]
            fn one_right() {
                // 1 right => 6 left
                let initial_facing = Facing::Right;
                let expected_facing = Facing::Left;
                run(&Case {
                    initial: (12, 1),
                    expected: (16, 12),
                    initial_facing,
                    expected_facing,
                });
                run(&Case {
                    initial: (12, 4),
                    expected: (16, 9),
                    initial_facing,
                    expected_facing,
                });
            }

            #[test]
            fn four_right() {
                // 4 right => 6 down
                let initial_facing = Facing::Right;
                let expected_facing = Facing::Down;
                run(&Case {
                    initial: (12, 5),
                    expected: (16, 9),
                    initial_facing,
                    expected_facing,
                });
                run(&Case {
                    initial: (12, 8),
                    expected: (13, 9),
                    initial_facing,
                    expected_facing,
                });
            }

            #[test]
            fn six_up() {
                // 6 up => 4 left
                let initial_facing = Facing::Up;
                let expected_facing = Facing::Left;
                run(&Case {
                    initial: (13, 9),
                    expected: (12, 8),
                    initial_facing,
                    expected_facing,
                });
                run(&Case {
                    initial: (16, 9),
                    expected: (12, 5),
                    initial_facing,
                    expected_facing,
                });
            }

            #[test]
            fn six_right() {
                // 6 right => 1 left
                let initial_facing = Facing::Right;
                let expected_facing = Facing::Left;
                run(&Case {
                    initial: (16, 9),
                    expected: (12, 4),
                    initial_facing,
                    expected_facing,
                });
                run(&Case {
                    initial: (16, 12),
                    expected: (12, 1),
                    initial_facing,
                    expected_facing,
                });
            }

            #[test]
            fn six_down() {
                // 6 down => 2 right
                let initial_facing = Facing::Down;
                let expected_facing = Facing::Right;
                run(&Case {
                    initial: (13, 12),
                    expected: (1, 8),
                    initial_facing,
                    expected_facing,
                });
                run(&Case {
                    initial: (16, 12),
                    expected: (1, 5),
                    initial_facing,
                    expected_facing,
                });
            }

            #[test]
            fn five_down() {
                // 5 down => 2 up
                let initial_facing = Facing::Down;
                let expected_facing = Facing::Up;
                run(&Case {
                    initial: (9, 12),
                    expected: (4, 8),
                    initial_facing,
                    expected_facing,
                });
                run(&Case {
                    initial: (12, 12),
                    expected: (1, 8),
                    initial_facing,
                    expected_facing,
                });
            }

            #[test]
            fn five_left() {
                // 5 left => 3 up
                let initial_facing = Facing::Left;
                let expected_facing = Facing::Up;
                run(&Case {
                    initial: (9, 9),
                    expected: (8, 8),
                    initial_facing,
                    expected_facing,
                });
                run(&Case {
                    initial: (9, 12),
                    expected: (5, 8),
                    initial_facing,
                    expected_facing,
                });
            }

            #[test]
            fn three_down() {
                // 3 down => 5 right
                let initial_facing = Facing::Down;
                let expected_facing = Facing::Right;
                run(&Case {
                    initial: (5, 8),
                    expected: (9, 12),
                    initial_facing,
                    expected_facing,
                });
                run(&Case {
                    initial: (8, 8),
                    expected: (9, 9),
                    initial_facing,
                    expected_facing,
                });
            }

            #[test]
            fn two_down() {
                // 2 down => 5 up
                let initial_facing = Facing::Down;
                let expected_facing = Facing::Up;
                run(&Case {
                    initial: (1, 8),
                    expected: (12, 12),
                    initial_facing,
                    expected_facing,
                });
                run(&Case {
                    initial: (4, 8),
                    expected: (9, 12),
                    initial_facing,
                    expected_facing,
                });
            }

            #[test]
            fn two_left() {
                // 2 left => 6 up
                let initial_facing = Facing::Left;
                let expected_facing = Facing::Up;
                run(&Case {
                    initial: (1, 5),
                    expected: (16, 12),
                    initial_facing,
                    expected_facing,
                });
                run(&Case {
                    initial: (1, 8),
                    expected: (13, 12),
                    initial_facing,
                    expected_facing,
                });
            }

            #[test]
            fn two_up() {
                // 2 up => 1 down
                let initial_facing = Facing::Up;
                let expected_facing = Facing::Down;
                run(&Case {
                    initial: (1, 5),
                    expected: (12, 1),
                    initial_facing,
                    expected_facing,
                });
                run(&Case {
                    initial: (4, 5),
                    expected: (9, 1),
                    initial_facing,
                    expected_facing,
                });
            }

            #[test]
            fn three_up() {
                // 3 up => 1 right
                let initial_facing = Facing::Up;
                let expected_facing = Facing::Right;
                run(&Case {
                    initial: (5, 5),
                    expected: (9, 1),
                    initial_facing,
                    expected_facing,
                });
                run(&Case {
                    initial: (8, 5),
                    expected: (9, 4),
                    initial_facing,
                    expected_facing,
                });
            }

            #[test]
            fn one_left() {
                // 1 left => 3 down
                let initial_facing = Facing::Left;
                let expected_facing = Facing::Down;
                run(&Case {
                    initial: (9, 1),
                    expected: (5, 5),
                    initial_facing,
                    expected_facing,
                });
                run(&Case {
                    initial: (9, 4),
                    expected: (8, 5),
                    initial_facing,
                    expected_facing,
                });
            }

            fn run(test: &Case) {
                let input = example().1;

                let actual = input.cube_next(test.initial, test.initial_facing);
                assert_eq!(test.expected, actual.0);
                assert_eq!(test.expected_facing, actual.1);

                let mut rev = test.expected_facing;
                rev = rev.next().unwrap().next().unwrap();

                let reversed = input.cube_next(actual.0, rev);
                assert_eq!(test.initial, reversed.0);

                let mut final_facing = reversed.1;
                final_facing = final_facing.next().unwrap().next().unwrap();
                assert_eq!(test.initial_facing, final_facing);
            }
        }

        mod actual {
            use super::*;

            //   1 2
            //   3
            // 4 5
            // 6

            #[test]
            fn one_up() {
                // 1 up => 6 right
                let initial_facing = Facing::Up;
                let expected_facing = Facing::Right;
                run(&Case {
                    initial: (51, 1),
                    expected: (1, 151),
                    initial_facing,
                    expected_facing,
                });
                run(&Case {
                    initial: (100, 1),
                    expected: (1, 200),
                    initial_facing,
                    expected_facing,
                });
            }

            #[test]
            fn two_up() {
                // 2 up => 6 up
                let initial_facing = Facing::Up;
                let expected_facing = Facing::Up;
                run(&Case {
                    initial: (101, 1),
                    expected: (1, 200),
                    initial_facing,
                    expected_facing,
                });
                run(&Case {
                    initial: (150, 1),
                    expected: (50, 200),
                    initial_facing,
                    expected_facing,
                });
            }

            #[test]
            fn two_right() {
                // 2 right => 5 left
                let initial_facing = Facing::Right;
                let expected_facing = Facing::Left;
                run(&Case {
                    initial: (150, 1),
                    expected: (100, 150),
                    initial_facing,
                    expected_facing,
                });
                run(&Case {
                    initial: (150, 50),
                    expected: (100, 101),
                    initial_facing,
                    expected_facing,
                });
            }

            #[test]
            fn two_down() {
                // 2 down => 3 left
                let initial_facing = Facing::Down;
                let expected_facing = Facing::Left;
                run(&Case {
                    initial: (101, 50),
                    expected: (100, 51),
                    initial_facing,
                    expected_facing,
                });
                run(&Case {
                    initial: (150, 50),
                    expected: (100, 100),
                    initial_facing,
                    expected_facing,
                });
            }

            #[test]
            fn three_right() {
                // 3 right => 2 up
                let initial_facing = Facing::Right;
                let expected_facing = Facing::Up;
                run(&Case {
                    initial: (100, 51),
                    expected: (101, 50),
                    initial_facing,
                    expected_facing,
                });
                run(&Case {
                    initial: (100, 100),
                    expected: (150, 50),
                    initial_facing,
                    expected_facing,
                });
            }

            #[test]
            fn five_right() {
                // 5 right => 2 left
                let initial_facing = Facing::Right;
                let expected_facing = Facing::Left;
                run(&Case {
                    initial: (100, 101),
                    expected: (150, 50),
                    initial_facing,
                    expected_facing,
                });
                run(&Case {
                    initial: (100, 150),
                    expected: (150, 1),
                    initial_facing,
                    expected_facing,
                });
            }

            #[test]
            fn five_down() {
                // 5 down => 6 left
                let initial_facing = Facing::Down;
                let expected_facing = Facing::Left;
                run(&Case {
                    initial: (51, 150),
                    expected: (50, 151),
                    initial_facing,
                    expected_facing,
                });
                run(&Case {
                    initial: (100, 150),
                    expected: (50, 200),
                    initial_facing,
                    expected_facing,
                });
            }

            #[test]
            fn six_right() {
                // 6 right => 5 up
                let initial_facing = Facing::Right;
                let expected_facing = Facing::Up;
                run(&Case {
                    initial: (50, 151),
                    expected: (51, 150),
                    initial_facing,
                    expected_facing,
                });
                run(&Case {
                    initial: (50, 200),
                    expected: (100, 150),
                    initial_facing,
                    expected_facing,
                });
            }

            #[test]
            fn six_down() {
                // 6 down => 2 down
                let initial_facing = Facing::Down;
                let expected_facing = Facing::Down;
                run(&Case {
                    initial: (1, 200),
                    expected: (101, 1),
                    initial_facing,
                    expected_facing,
                });
                run(&Case {
                    initial: (50, 200),
                    expected: (150, 1),
                    initial_facing,
                    expected_facing,
                });
            }

            #[test]
            fn six_left() {
                // 6 left => 1 down
                let initial_facing = Facing::Left;
                let expected_facing = Facing::Down;
                run(&Case {
                    initial: (1, 151),
                    expected: (51, 1),
                    initial_facing,
                    expected_facing,
                });
                run(&Case {
                    initial: (1, 200),
                    expected: (100, 1),
                    initial_facing,
                    expected_facing,
                });
            }

            #[test]
            fn four_left() {
                // 4 left => 1 right
                let initial_facing = Facing::Left;
                let expected_facing = Facing::Right;
                run(&Case {
                    initial: (1, 101),
                    expected: (51, 50),
                    initial_facing,
                    expected_facing,
                });
                run(&Case {
                    initial: (1, 150),
                    expected: (51, 1),
                    initial_facing,
                    expected_facing,
                });
            }

            #[test]
            fn four_up() {
                // 4 up => 3 right
                let initial_facing = Facing::Up;
                let expected_facing = Facing::Right;
                run(&Case {
                    initial: (1, 101),
                    expected: (51, 51),
                    initial_facing,
                    expected_facing,
                });
                run(&Case {
                    initial: (50, 101),
                    expected: (51, 100),
                    initial_facing,
                    expected_facing,
                });
            }

            #[test]
            fn three_left() {
                // 3 left => 4 down
                let initial_facing = Facing::Left;
                let expected_facing = Facing::Down;
                run(&Case {
                    initial: (51, 51),
                    expected: (1, 101),
                    initial_facing,
                    expected_facing,
                });
                run(&Case {
                    initial: (51, 100),
                    expected: (50, 101),
                    initial_facing,
                    expected_facing,
                });
            }

            #[test]
            fn one_left() {
                // 1 left => 4 right
                let initial_facing = Facing::Left;
                let expected_facing = Facing::Right;
                run(&Case {
                    initial: (51, 1),
                    expected: (1, 150),
                    initial_facing,
                    expected_facing,
                });
                run(&Case {
                    initial: (51, 50),
                    expected: (1, 101),
                    initial_facing,
                    expected_facing,
                });
            }

            fn run(test: &Case) {
                let input = Input::from_data(&read_data(DATA_DIR));

                let actual = input.cube_next(test.initial, test.initial_facing);
                assert_eq!(test.expected, actual.0);
                assert_eq!(test.expected_facing, actual.1);

                let mut rev = test.expected_facing;
                rev = rev.next().unwrap().next().unwrap();

                let reversed = input.cube_next(actual.0, rev);
                assert_eq!(test.initial, reversed.0);

                let mut final_facing = reversed.1;
                final_facing = final_facing.next().unwrap().next().unwrap();
                assert_eq!(test.initial_facing, final_facing);
            }
        }
    }

    fn example() -> (&'static str, Input) {
        (
            "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5
",
            Input {
                map: HashMap::from([
                    ((9, 1), false),
                    ((10, 1), false),
                    ((11, 1), false),
                    ((12, 1), true),
                    ((9, 2), false),
                    ((10, 2), true),
                    ((11, 2), false),
                    ((12, 2), false),
                    ((9, 3), true),
                    ((10, 3), false),
                    ((11, 3), false),
                    ((12, 3), false),
                    ((9, 4), false),
                    ((10, 4), false),
                    ((11, 4), false),
                    ((12, 4), false),
                    ((1, 5), false),
                    ((2, 5), false),
                    ((3, 5), false),
                    ((4, 5), true),
                    ((5, 5), false),
                    ((6, 5), false),
                    ((7, 5), false),
                    ((8, 5), false),
                    ((9, 5), false),
                    ((10, 5), false),
                    ((11, 5), false),
                    ((12, 5), true),
                    ((1, 6), false),
                    ((2, 6), false),
                    ((3, 6), false),
                    ((4, 6), false),
                    ((5, 6), false),
                    ((6, 6), false),
                    ((7, 6), false),
                    ((8, 6), false),
                    ((9, 6), true),
                    ((10, 6), false),
                    ((11, 6), false),
                    ((12, 6), false),
                    ((1, 7), false),
                    ((2, 7), false),
                    ((3, 7), true),
                    ((4, 7), false),
                    ((5, 7), false),
                    ((6, 7), false),
                    ((7, 7), false),
                    ((8, 7), true),
                    ((9, 7), false),
                    ((10, 7), false),
                    ((11, 7), false),
                    ((12, 7), false),
                    ((1, 8), false),
                    ((2, 8), false),
                    ((3, 8), false),
                    ((4, 8), false),
                    ((5, 8), false),
                    ((6, 8), false),
                    ((7, 8), false),
                    ((8, 8), false),
                    ((9, 8), false),
                    ((10, 8), false),
                    ((11, 8), true),
                    ((12, 8), false),
                    ((9, 9), false),
                    ((10, 9), false),
                    ((11, 9), false),
                    ((12, 9), true),
                    ((13, 9), false),
                    ((14, 9), false),
                    ((15, 9), false),
                    ((16, 9), false),
                    ((9, 10), false),
                    ((10, 10), false),
                    ((11, 10), false),
                    ((12, 10), false),
                    ((13, 10), false),
                    ((14, 10), true),
                    ((15, 10), false),
                    ((16, 10), false),
                    ((9, 11), false),
                    ((10, 11), true),
                    ((11, 11), false),
                    ((12, 11), false),
                    ((13, 11), false),
                    ((14, 11), false),
                    ((15, 11), false),
                    ((16, 11), false),
                    ((9, 12), false),
                    ((10, 12), false),
                    ((11, 12), false),
                    ((12, 12), false),
                    ((13, 12), false),
                    ((14, 12), false),
                    ((15, 12), true),
                    ((16, 12), false),
                ]),
                start: (9, 1),
                max: (16, 12),
                instructions: vec![
                    Instruction::Forward(10),
                    Instruction::RotateRight,
                    Instruction::Forward(5),
                    Instruction::RotateLeft,
                    Instruction::Forward(5),
                    Instruction::RotateRight,
                    Instruction::Forward(10),
                    Instruction::RotateLeft,
                    Instruction::Forward(4),
                    Instruction::RotateRight,
                    Instruction::Forward(5),
                    Instruction::RotateLeft,
                    Instruction::Forward(5),
                ],
            },
        )
    }
}
