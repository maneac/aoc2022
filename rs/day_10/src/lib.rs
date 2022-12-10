use std::{
    fmt::{Display, Write},
    fs::read_to_string,
    path::Path,
};

pub const PART_1: usize = 14220;
pub const PART_2: Crt = Crt([
    [
        '#', '#', '#', '#', '.', '#', '#', '#', '.', '.', '.', '#', '#', '.', '.', '#', '#', '#',
        '.', '.', '#', '.', '.', '.', '.', '#', '#', '#', '#', '.', '#', '#', '#', '#', '.', '#',
        '.', '.', '#', '.',
    ],
    [
        '.', '.', '.', '#', '.', '#', '.', '.', '#', '.', '#', '.', '.', '#', '.', '#', '.', '.',
        '#', '.', '#', '.', '.', '.', '.', '#', '.', '.', '.', '.', '.', '.', '.', '#', '.', '#',
        '.', '.', '#', '.',
    ],
    [
        '.', '.', '#', '.', '.', '#', '.', '.', '#', '.', '#', '.', '.', '#', '.', '#', '.', '.',
        '#', '.', '#', '.', '.', '.', '.', '#', '#', '#', '.', '.', '.', '.', '#', '.', '.', '#',
        '.', '.', '#', '.',
    ],
    [
        '.', '#', '.', '.', '.', '#', '#', '#', '.', '.', '#', '#', '#', '#', '.', '#', '#', '#',
        '.', '.', '#', '.', '.', '.', '.', '#', '.', '.', '.', '.', '.', '#', '.', '.', '.', '#',
        '.', '.', '#', '.',
    ],
    [
        '#', '.', '.', '.', '.', '#', '.', '#', '.', '.', '#', '.', '.', '#', '.', '#', '.', '#',
        '.', '.', '#', '.', '.', '.', '.', '#', '.', '.', '.', '.', '#', '.', '.', '.', '.', '#',
        '.', '.', '#', '.',
    ],
    [
        '#', '#', '#', '#', '.', '#', '.', '.', '#', '.', '#', '.', '.', '#', '.', '#', '.', '.',
        '#', '.', '#', '#', '#', '#', '.', '#', '.', '.', '.', '.', '#', '#', '#', '#', '.', '.',
        '#', '#', '.', '.',
    ],
]);

pub fn read_data(data_dir: &str) -> String {
    read_to_string(Path::new(data_dir).join("day_10.txt"))
        .unwrap()
        .trim()
        .to_string()
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Input {
    instructions: Vec<Instruction>,
}

impl Input {
    pub fn from_data(data: &str) -> Self {
        let instructions = data.lines().map(Instruction::from).collect();

        Self { instructions }
    }

    pub fn part_1(&self) -> usize {
        let mut x = 1;

        let mut strengths = [0; 6];

        let mut cycle = 0;
        for instruction in &self.instructions {
            let (cycle_add, x_add) = match instruction {
                Instruction::Noop => (1, 0),
                Instruction::Addx(val) => (2, *val),
            };
            for _ in 0..cycle_add {
                cycle += 1;
                let offset_cycle = cycle - 20;
                if offset_cycle >= 0 && offset_cycle % 40 == 0 {
                    strengths[(offset_cycle / 40) as usize] = x * cycle;
                }
            }
            x += x_add
        }

        strengths.iter().sum::<i32>() as usize
    }

    pub fn part_2(&self) -> Crt {
        let mut crt = [['.'; 40]; 6];

        let mut x = 1;
        let mut cycle = 0;
        for instruction in &self.instructions {
            let (cycle_add, x_add) = match instruction {
                Instruction::Noop => (1, 0),
                Instruction::Addx(val) => (2, *val),
            };
            for _ in 0..cycle_add {
                let x_pos: i32 = cycle % 40;
                if x_pos.abs_diff(x) <= 1 {
                    crt[(cycle / 40) as usize][x_pos as usize] = '#';
                }
                cycle += 1;
            }
            x += x_add
        }

        Crt(crt)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Instruction {
    Noop,
    Addx(i32),
}

impl From<&str> for Instruction {
    fn from(line: &str) -> Self {
        if line == "noop" {
            return Self::Noop;
        }

        Self::Addx(
            line.split_ascii_whitespace()
                .nth(1)
                .unwrap()
                .parse::<i32>()
                .unwrap(),
        )
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Crt([[char; 40]; 6]);

impl Display for Crt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.0 {
            for chr in row {
                f.write_char(chr)?;
            }
            f.write_char('\n')?;
        }
        Ok(())
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
        fn small_example() {
            run(&Case {
                input: "noop
addx 3
addx -5",
                expected: Input {
                    instructions: vec![
                        Instruction::Noop,
                        Instruction::Addx(3),
                        Instruction::Addx(-5),
                    ],
                },
            })
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
                expected: 13140,
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
            expected: Crt,
        }

        #[test]
        fn example() {
            run(&Case {
                data: super::example().1,
                expected: Crt([
                    [
                        '#', '#', '.', '.', '#', '#', '.', '.', '#', '#', '.', '.', '#', '#', '.',
                        '.', '#', '#', '.', '.', '#', '#', '.', '.', '#', '#', '.', '.', '#', '#',
                        '.', '.', '#', '#', '.', '.', '#', '#', '.', '.',
                    ],
                    [
                        '#', '#', '#', '.', '.', '.', '#', '#', '#', '.', '.', '.', '#', '#', '#',
                        '.', '.', '.', '#', '#', '#', '.', '.', '.', '#', '#', '#', '.', '.', '.',
                        '#', '#', '#', '.', '.', '.', '#', '#', '#', '.',
                    ],
                    [
                        '#', '#', '#', '#', '.', '.', '.', '.', '#', '#', '#', '#', '.', '.', '.',
                        '.', '#', '#', '#', '#', '.', '.', '.', '.', '#', '#', '#', '#', '.', '.',
                        '.', '.', '#', '#', '#', '#', '.', '.', '.', '.',
                    ],
                    [
                        '#', '#', '#', '#', '#', '.', '.', '.', '.', '.', '#', '#', '#', '#', '#',
                        '.', '.', '.', '.', '.', '#', '#', '#', '#', '#', '.', '.', '.', '.', '.',
                        '#', '#', '#', '#', '#', '.', '.', '.', '.', '.',
                    ],
                    [
                        '#', '#', '#', '#', '#', '#', '.', '.', '.', '.', '.', '.', '#', '#', '#',
                        '#', '#', '#', '.', '.', '.', '.', '.', '.', '#', '#', '#', '#', '#', '#',
                        '.', '.', '.', '.', '.', '.', '#', '#', '#', '#',
                    ],
                    [
                        '#', '#', '#', '#', '#', '#', '#', '.', '.', '.', '.', '.', '.', '.', '#',
                        '#', '#', '#', '#', '#', '#', '.', '.', '.', '.', '.', '.', '.', '#', '#',
                        '#', '#', '#', '#', '#', '.', '.', '.', '.', '.',
                    ],
                ]),
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
            let actual = test.data.part_2();
            assert_eq!(
                test.expected, actual,
                "\nExpected:\n{}\n\nActual:\n{}",
                test.expected, actual
            )
        }
    }

    fn example() -> (&'static str, Input) {
        (
            "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop",
            Input {
                instructions: vec![
                    Instruction::Addx(15),
                    Instruction::Addx(-11),
                    Instruction::Addx(6),
                    Instruction::Addx(-3),
                    Instruction::Addx(5),
                    Instruction::Addx(-1),
                    Instruction::Addx(-8),
                    Instruction::Addx(13),
                    Instruction::Addx(4),
                    Instruction::Noop,
                    Instruction::Addx(-1),
                    Instruction::Addx(5),
                    Instruction::Addx(-1),
                    Instruction::Addx(5),
                    Instruction::Addx(-1),
                    Instruction::Addx(5),
                    Instruction::Addx(-1),
                    Instruction::Addx(5),
                    Instruction::Addx(-1),
                    Instruction::Addx(-35),
                    Instruction::Addx(1),
                    Instruction::Addx(24),
                    Instruction::Addx(-19),
                    Instruction::Addx(1),
                    Instruction::Addx(16),
                    Instruction::Addx(-11),
                    Instruction::Noop,
                    Instruction::Noop,
                    Instruction::Addx(21),
                    Instruction::Addx(-15),
                    Instruction::Noop,
                    Instruction::Noop,
                    Instruction::Addx(-3),
                    Instruction::Addx(9),
                    Instruction::Addx(1),
                    Instruction::Addx(-3),
                    Instruction::Addx(8),
                    Instruction::Addx(1),
                    Instruction::Addx(5),
                    Instruction::Noop,
                    Instruction::Noop,
                    Instruction::Noop,
                    Instruction::Noop,
                    Instruction::Noop,
                    Instruction::Addx(-36),
                    Instruction::Noop,
                    Instruction::Addx(1),
                    Instruction::Addx(7),
                    Instruction::Noop,
                    Instruction::Noop,
                    Instruction::Noop,
                    Instruction::Addx(2),
                    Instruction::Addx(6),
                    Instruction::Noop,
                    Instruction::Noop,
                    Instruction::Noop,
                    Instruction::Noop,
                    Instruction::Noop,
                    Instruction::Addx(1),
                    Instruction::Noop,
                    Instruction::Noop,
                    Instruction::Addx(7),
                    Instruction::Addx(1),
                    Instruction::Noop,
                    Instruction::Addx(-13),
                    Instruction::Addx(13),
                    Instruction::Addx(7),
                    Instruction::Noop,
                    Instruction::Addx(1),
                    Instruction::Addx(-33),
                    Instruction::Noop,
                    Instruction::Noop,
                    Instruction::Noop,
                    Instruction::Addx(2),
                    Instruction::Noop,
                    Instruction::Noop,
                    Instruction::Noop,
                    Instruction::Addx(8),
                    Instruction::Noop,
                    Instruction::Addx(-1),
                    Instruction::Addx(2),
                    Instruction::Addx(1),
                    Instruction::Noop,
                    Instruction::Addx(17),
                    Instruction::Addx(-9),
                    Instruction::Addx(1),
                    Instruction::Addx(1),
                    Instruction::Addx(-3),
                    Instruction::Addx(11),
                    Instruction::Noop,
                    Instruction::Noop,
                    Instruction::Addx(1),
                    Instruction::Noop,
                    Instruction::Addx(1),
                    Instruction::Noop,
                    Instruction::Noop,
                    Instruction::Addx(-13),
                    Instruction::Addx(-19),
                    Instruction::Addx(1),
                    Instruction::Addx(3),
                    Instruction::Addx(26),
                    Instruction::Addx(-30),
                    Instruction::Addx(12),
                    Instruction::Addx(-1),
                    Instruction::Addx(3),
                    Instruction::Addx(1),
                    Instruction::Noop,
                    Instruction::Noop,
                    Instruction::Noop,
                    Instruction::Addx(-9),
                    Instruction::Addx(18),
                    Instruction::Addx(1),
                    Instruction::Addx(2),
                    Instruction::Noop,
                    Instruction::Noop,
                    Instruction::Addx(9),
                    Instruction::Noop,
                    Instruction::Noop,
                    Instruction::Noop,
                    Instruction::Addx(-1),
                    Instruction::Addx(2),
                    Instruction::Addx(-37),
                    Instruction::Addx(1),
                    Instruction::Addx(3),
                    Instruction::Noop,
                    Instruction::Addx(15),
                    Instruction::Addx(-21),
                    Instruction::Addx(22),
                    Instruction::Addx(-6),
                    Instruction::Addx(1),
                    Instruction::Noop,
                    Instruction::Addx(2),
                    Instruction::Addx(1),
                    Instruction::Noop,
                    Instruction::Addx(-10),
                    Instruction::Noop,
                    Instruction::Noop,
                    Instruction::Addx(20),
                    Instruction::Addx(1),
                    Instruction::Addx(2),
                    Instruction::Addx(2),
                    Instruction::Addx(-6),
                    Instruction::Addx(-11),
                    Instruction::Noop,
                    Instruction::Noop,
                    Instruction::Noop,
                ],
            },
        )
    }
}
