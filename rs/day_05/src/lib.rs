use std::{fs::read_to_string, path::Path};

pub const PART_1: &str = "SPFMVDTZT";
pub const PART_2: &str = "ZFSJBPRFP";

pub fn read_data(data_dir: &str) -> String {
    read_to_string(Path::new(data_dir).join("day_05.txt")).unwrap()
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Input {
    crate_stacks: Vec<Vec<u8>>,
    instructions: Vec<[usize; 3]>,
}

impl Input {
    pub fn from_data(data: &str) -> Self {
        let mut crate_stacks = Vec::new();
        let mut line_iter = data.lines();
        for line in &mut line_iter {
            if crate_stacks.is_empty() {
                crate_stacks.resize((line.len() + 1) / 4, Vec::new());
            }
            if line.starts_with(" 1") {
                continue;
            }
            if line.is_empty() {
                break;
            }

            for (idx, byte) in line.bytes().skip(1).step_by(4).enumerate() {
                if byte == b' ' {
                    continue;
                }
                crate_stacks.get_mut(idx).unwrap().insert(0, byte);
            }
        }

        let mut instructions = Vec::new();
        for instruction in line_iter {
            let mut parts = [0; 3];
            parts
                .iter_mut()
                .zip(
                    instruction
                        .split_ascii_whitespace()
                        .filter_map(|chunk| chunk.parse::<usize>().ok()),
                )
                .for_each(|(val, num)| {
                    *val = num;
                });
            instructions.push(parts);
        }

        Input {
            crate_stacks,
            instructions,
        }
    }

    pub fn part_1(&self) -> String {
        let mut stacks = self.crate_stacks.clone();

        for &[qty, from, to] in &self.instructions {
            for _ in 0..qty {
                let mover = stacks[from - 1].pop().unwrap();
                stacks[to - 1].push(mover);
            }
        }

        stacks
            .iter()
            .fold(String::with_capacity(stacks.len()), |mut acc, stack| {
                acc.push(stack.last().copied().unwrap() as char);
                acc
            })
    }

    pub fn part_2(&self) -> String {
        let mut stacks = self.crate_stacks.clone();

        for &[qty, from, to] in &self.instructions {
            let drain_idx = stacks[from - 1].len() - qty;
            let mut drained = stacks[from - 1].drain(drain_idx..).collect::<Vec<u8>>();
            stacks[to - 1].append(&mut drained);
        }

        stacks
            .iter()
            .fold(String::with_capacity(stacks.len()), |mut acc, stack| {
                acc.push(stack.last().copied().unwrap() as char);
                acc
            })
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

        struct Case<'c> {
            data: Input,
            expected: &'c str,
        }

        #[test]
        fn example() {
            run(&Case {
                data: super::example().1,
                expected: "CMZ",
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
            assert_eq!(test.expected, test.data.part_1().as_str())
        }
    }

    mod part_2 {
        use super::*;

        struct Case<'c> {
            data: Input,
            expected: &'c str,
        }

        #[test]
        fn example() {
            run(&Case {
                data: super::example().1,
                expected: "MCD",
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
            "    [D]    
[N] [C]    
[Z] [M] [P]
    1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2",
            Input {
                crate_stacks: vec![vec![b'Z', b'N'], vec![b'M', b'C', b'D'], vec![b'P']],
                instructions: vec![[1, 2, 1], [3, 1, 3], [2, 2, 1], [1, 1, 2]],
            },
        )
    }
}
