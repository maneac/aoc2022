use std::{collections::HashSet, fs::read_to_string, path::Path};

pub const PART_1: usize = 6494;
pub const PART_2: usize = 2691;

pub fn read_data(data_dir: &str) -> String {
    read_to_string(Path::new(data_dir).join("day_09.txt"))
        .unwrap()
        .trim()
        .to_string()
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Input {
    directions: Vec<(char, u8)>,
}

impl Input {
    pub fn from_data(data: &str) -> Self {
        let directions = data.lines().fold(Vec::new(), |mut acc, line| {
            let mut line_parts = line.split_ascii_whitespace();
            acc.push((
                line_parts.next().unwrap().chars().next().unwrap(),
                line_parts.next().unwrap().parse::<u8>().unwrap(),
            ));
            acc
        });
        Self { directions }
    }

    pub fn part_1(&self) -> usize {
        let mut visited = HashSet::new();

        let (mut head_x, mut head_y) = (0i32, 0i32);
        let (mut tail_x, mut tail_y) = (0i32, 0i32);

        for &(direction, amount) in &self.directions {
            for _ in 0..amount {
                match direction {
                    'U' => {
                        head_y += 1;
                    }
                    'D' => {
                        head_y -= 1;
                    }
                    'L' => {
                        head_x -= 1;
                    }
                    'R' => {
                        head_x += 1;
                    }
                    _ => unreachable!(),
                }

                if tail_x.abs_diff(head_x) == 2 {
                    tail_x += (head_x - tail_x).signum();
                    tail_y = head_y;
                }
                if tail_y.abs_diff(head_y) == 2 {
                    tail_y += (head_y - tail_y).signum();
                    tail_x = head_x;
                }

                visited.insert((tail_x, tail_y));
            }
        }

        visited.len()
    }

    pub fn part_2(&self) -> usize {
        let mut visited = HashSet::new();

        let mut rope = [(0i32, 0i32); 10];

        for &(direction, amount) in &self.directions {
            for _ in 0..amount {
                match direction {
                    'U' => {
                        rope[0].1 += 1;
                    }
                    'D' => {
                        rope[0].1 -= 1;
                    }
                    'L' => {
                        rope[0].0 -= 1;
                    }
                    'R' => {
                        rope[0].0 += 1;
                    }
                    _ => unreachable!(),
                }

                for idx in 1..rope.len() {
                    let (head_x, head_y) = rope[idx - 1];
                    let (mut tail_x, mut tail_y) = rope[idx];

                    if tail_x.abs_diff(head_x) < 2 && tail_y.abs_diff(head_y) < 2 {
                        continue;
                    }

                    if tail_x.abs_diff(head_x) == 2 && tail_y == head_y {
                        tail_x += (head_x - tail_x).signum();
                        rope[idx] = (tail_x, tail_y);
                        continue;
                    }

                    if tail_y.abs_diff(head_y) == 2 && tail_x == head_x {
                        tail_y += (head_y - tail_y).signum();
                        rope[idx] = (tail_x, tail_y);
                        continue;
                    }

                    tail_x += (head_x - tail_x).signum();
                    tail_y += (head_y - tail_y).signum();

                    rope[idx] = (tail_x, tail_y);
                }
                visited.insert(rope[9]);
            }
        }

        visited.len()
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
                expected: 13,
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
                expected: 1,
            })
        }

        #[test]
        fn large_example() {
            run(&Case {
                data: Input::from_data(
                    "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20",
                ),
                expected: 36,
            });
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
            "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2",
            Input {
                directions: vec![
                    ('R', 4),
                    ('U', 4),
                    ('L', 3),
                    ('D', 1),
                    ('R', 4),
                    ('D', 1),
                    ('L', 5),
                    ('R', 2),
                ],
            },
        )
    }
}
