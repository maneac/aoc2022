use std::{fs::read_to_string, path::Path};

pub const PART_1: usize = 3188;
pub const PART_2: usize = 1591977077342;

pub fn read_data(data_dir: &str) -> String {
    read_to_string(Path::new(data_dir).join("day_17.txt"))
        .unwrap()
        .trim()
        .to_string()
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Input {
    jets: Vec<u8>,
}

impl Input {
    pub fn from_data(data: &str) -> Self {
        Self {
            jets: data.as_bytes().to_vec(),
        }
    }

    pub fn part_1(&self) -> usize {
        self.size::<2022>()
    }

    pub fn part_2(&self) -> usize {
        self.size::<1_000_000_000_000>()
    }

    fn size<const ROCKS: usize>(&self) -> usize {
        let rocks = [
            [0b0_0011110, 0b0_0000000, 0b0_0000000, 0b0_0000000],
            [0b0_0001000, 0b0_0011100, 0b0_0001000, 0b0_0000000],
            [0b0_0011100, 0b0_0000100, 0b0_0000100, 0b0_0000000],
            [0b0_0010000, 0b0_0010000, 0b0_0010000, 0b0_0010000],
            [0b0_0011000, 0b0_0011000, 0b0_0000000, 0b0_0000000],
        ];

        let mut input = self.jets.iter().map(|b| b == &b'>').cycle();

        let mut cave = Vec::<u8>::new();

        let mut total_height = 0;

        for rock in rocks.iter().cycle().take(ROCKS) {
            let mut rock = *rock;

            let mut height = cave.len() + 3;
            'fall: loop {
                if input.next().unwrap()
                    && rock.iter().enumerate().all(|(idx, l)| {
                        !cave
                            .get(idx + height)
                            .map(|c| c & (l >> 1) > 0)
                            .unwrap_or_default()
                            && *l & 1 == 0
                    })
                {
                    rock.iter_mut().for_each(|l| {
                        *l >>= 1;
                    });
                } else if rock.iter().enumerate().all(|(idx, l)| {
                    !cave
                        .get(idx + height)
                        .map(|c| c & (l << 1) > 0)
                        .unwrap_or_default()
                        && *l & 0b0_1000000 == 0
                }) {
                    rock.iter_mut().for_each(|l| {
                        *l <<= 1;
                    });
                }

                for (idx, line) in rock.iter().enumerate() {
                    if height + idx == 0 {
                        break 'fall;
                    }
                    if cave
                        .get(idx + height - 1)
                        .map(|l| l & line > 0)
                        .unwrap_or_default()
                    {
                        break 'fall;
                    }
                }

                height -= 1;
            }

            for (idx, line) in rock.iter().enumerate().take_while(|(_, l)| **l > 0) {
                if cave.get(idx + height).is_some() {
                    cave[idx + height] |= line;

                    if cave[idx + height] == 0b0_1111111 {
                        total_height += idx + height;

                        let h = cave.len() - (idx + height);
                        for i in 0..h {
                            cave[i] = cave[i + idx + height];
                        }
                        cave.truncate(h);
                        height = 0;
                    }
                    continue;
                }
                cave.push(*line);
            }
        }

        total_height + cave.len()
    }
}

#[allow(dead_code)]
fn print_cave(cave: &[u8]) {
    for line in cave.iter().rev() {
        let l: String = (0..7).rev().fold(String::with_capacity(7), |mut acc, b| {
            if line & (1 << b) > 0 {
                acc.push('#');
            } else {
                acc.push('.');
            }
            acc
        });
        println!("|{l}|");
    }
    println!("+-------+")
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
        #[ignore = "this will take too long to run"]
        fn example() {
            run(&Case {
                data: super::example().1,
                expected: 1514285714288,
            })
        }

        #[test]
        #[ignore = "this will take too long to run"]
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
                jets: ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"
                    .as_bytes()
                    .to_vec(),
            },
        )
    }
}
