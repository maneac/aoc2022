use std::{fs::read_to_string, path::Path};

pub const PART_1: isize = 1087;
pub const PART_2: isize = 13084440324666;

pub fn read_data(data_dir: &str) -> String {
    read_to_string(Path::new(data_dir).join("day_20.txt"))
        .unwrap()
        .trim()
        .to_string()
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Input {
    numbers: Vec<(usize, isize)>,
}

impl Input {
    pub fn from_data(data: &str) -> Self {
        let mut numbers = Vec::new();
        for (idx, line) in data.trim().lines().enumerate() {
            numbers.push((idx, line.trim().parse().unwrap()));
        }
        Self { numbers }
    }

    pub fn part_1(&self) -> isize {
        let mut output = self.numbers.clone();

        for original_idx in 0..output.len() {
            mix(&mut output, original_idx);
        }

        let zero_val_idx = output.iter().position(|(_, v)| *v == 0).unwrap();
        let thousandth = output[(zero_val_idx + 1000) % output.len()].1;
        let two_thousandth = output[(zero_val_idx + 2000) % output.len()].1;
        let three_thousandth = output[(zero_val_idx + 3000) % output.len()].1;
        thousandth + two_thousandth + three_thousandth
    }

    pub fn part_2(&self) -> isize {
        const DECRYPTION_KEY: isize = 811589153;

        let mut output = self.numbers.clone();
        output.iter_mut().for_each(|(_, v)| {
            *v *= DECRYPTION_KEY;
        });

        for _round in 0..10 {
            for original_idx in 0..output.len() {
                mix(&mut output, original_idx);
            }
        }

        let zero_val_idx = output.iter().position(|(_, v)| *v == 0).unwrap();
        let thousandth = output[(zero_val_idx + 1000) % output.len()].1;
        let two_thousandth = output[(zero_val_idx + 2000) % output.len()].1;
        let three_thousandth = output[(zero_val_idx + 3000) % output.len()].1;
        thousandth + two_thousandth + three_thousandth
    }
}

fn mix(output: &mut Vec<(usize, isize)>, original_idx: usize) {
    let idx = output.iter().position(|(i, _)| *i == original_idx).unwrap();
    let val = output[idx];

    if val.1 == 0 {
        return;
    }

    let target_idx = idx as isize + val.1;
    let target_idx = target_idx % (output.len() - 1) as isize;
    let target_idx = match target_idx {
        0 => output.len() as isize - 1,
        ..=-1 => target_idx + output.len() as isize - 1,
        _ => target_idx,
    };

    output.remove(idx);
    output.insert(target_idx as usize, val);
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA_DIR: &str = "../../data";

    mod mix {
        use super::*;

        struct Case {
            input: Vec<(usize, isize)>,
            expected: Vec<(usize, isize)>,
        }

        #[test]
        fn no_change() {
            run(&Case {
                input: vec![(0, 0), (1, 0), (2, 0)],
                expected: vec![(0, 0), (1, 0), (2, 0)],
            })
        }

        #[test]
        fn add_one() {
            run(&Case {
                input: vec![(0, 0), (1, 1), (2, 0)],
                expected: vec![(0, 0), (2, 0), (1, 1)],
            })
        }

        #[test]
        fn sub_one() {
            run(&Case {
                input: vec![(0, 0), (1, 0), (2, -1)],
                expected: vec![(0, 0), (2, -1), (1, 0)],
            })
        }

        #[test]
        fn wrapping_add() {
            run(&Case {
                input: vec![(0, 0), (1, 0), (2, 2), (3, 0)],
                expected: vec![(0, 0), (2, 2), (1, 0), (3, 0)],
            })
        }

        #[test]
        fn wrapping_sub() {
            run(&Case {
                input: vec![(0, 0), (1, -2), (2, 0), (3, 0)],
                expected: vec![(0, 0), (2, 0), (1, -2), (3, 0)],
            })
        }

        #[test]
        fn double_wrapping_add() {
            run(&Case {
                input: vec![(0, 0), (1, 0), (2, 5), (3, 0)],
                expected: vec![(0, 0), (2, 5), (1, 0), (3, 0)],
            })
        }

        #[test]
        fn double_wrapping_sub() {
            run(&Case {
                input: vec![(0, 0), (1, -5), (2, 0), (3, 0)],
                expected: vec![(0, 0), (2, 0), (1, -5), (3, 0)],
            })
        }

        fn run(test: &Case) {
            let mut output = test.input.clone();
            for i in 0..output.len() {
                mix(&mut output, i);
            }
            assert_eq!(test.expected, output);
        }
    }

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
            expected: isize,
        }

        #[test]
        fn example() {
            run(&Case {
                data: super::example().1,
                expected: 3,
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
            expected: isize,
        }

        #[test]
        fn example() {
            run(&Case {
                data: super::example().1,
                expected: 1623178306,
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
            "1
        2
        -3
        3
        -2
        0
        4",
            Input {
                numbers: vec![(0, 1), (1, 2), (2, -3), (3, 3), (4, -2), (5, 0), (6, 4)],
            },
        )
    }
}
