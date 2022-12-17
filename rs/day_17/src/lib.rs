use std::{fs::read_to_string, path::Path};

pub const PART_1: usize = 3188;
pub const PART_2: usize = 0;

pub fn read_data(data_dir: &str) -> String {
    read_to_string(Path::new(data_dir).join("day_17.txt"))
        .unwrap()
        .trim()
        .to_string()
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Input {
    jets: Vec<bool>,
}

impl Input {
    pub fn from_data(data: &str) -> Self {
        Self {
            jets: data.bytes().map(|b| b == b'>').collect(),
        }
    }

    pub fn part_1(&self) -> usize {
        self.fill_cavern::<2022>()
    }

    pub fn part_2(&self) -> usize {
        // self.fill_cavern::<1_000_000_000_000>()
        self.fill_cavern::<1_000_000_000>()
    }

    fn fill_cavern<const LIMIT: usize>(&self) -> usize {
        let shapes = vec![
            [0b001_1110, 0b000_0000, 0b000_0000, 0b000_0000],
            [0b000_1000, 0b001_1100, 0b000_1000, 0b000_0000],
            [0b001_1100, 0b000_0100, 0b000_0100, 0b000_0000],
            [0b001_0000, 0b001_0000, 0b001_0000, 0b001_0000],
            [0b001_1000, 0b001_1000, 0b000_0000, 0b000_0000],
        ];

        let jet_len = self.jets.len();

        let mut cavern = Vec::<u8>::new();
        let mut jets = self.jets.iter().enumerate().cycle();
        for shape in shapes.iter().cycle().take(LIMIT) {
            let mut shape = *shape;
            let mut height = cavern.len() + 3;

            for (idx, &push_right) in jets.by_ref() {
                if push_right
                    && !shape.iter().any(|r| r & 1 > 0)
                    && !shape
                        .iter()
                        .zip(cavern.iter().skip(height))
                        .any(|(r, c)| (r >> 1) & c > 0)
                {
                    shape.iter_mut().for_each(|s| {
                        *s >>= 1;
                    })
                }

                if !push_right
                    && !shape.iter().any(|r| r & 0b100_0000 > 0)
                    && !shape
                        .iter()
                        .zip(cavern.iter().skip(height))
                        .any(|(r, c)| (r << 1) & c > 0)
                {
                    shape.iter_mut().for_each(|s| {
                        *s <<= 1;
                    })
                }

                if height == 0
                    || cavern
                        .iter()
                        .skip(height - 1)
                        .zip(shape)
                        .any(|(c, r)| c & r > 0)
                {
                    let mut shape_iter = shape.iter().take_while(|&&s| s > 0);
                    for row in cavern.iter_mut().skip(height) {
                        if let Some(s) = shape_iter.next() {
                            *row |= *s;
                        } else {
                            break;
                        }
                    }

                    cavern.extend(shape_iter);
                    break;
                }

                height -= 1;
            }
        }

        cavern.len()
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
                expected: 3068,
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
                expected: 1514285714288,
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
            ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>",
            Input {
                jets: vec![
                    true, true, true, false, false, true, false, true, true, false, false, false,
                    true, true, false, true, true, true, false, false, false, true, true, true,
                    false, false, false, true, false, false, false, true, true, false, true, true,
                    false, false, true, true,
                ],
            },
        )
    }
}
