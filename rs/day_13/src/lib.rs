use std::{fs::read_to_string, path::Path};

pub const PART_1: usize = 5330;
pub const PART_2: usize = 27648;

pub fn read_data(data_dir: &str) -> String {
    read_to_string(Path::new(data_dir).join("day_13.txt"))
        .unwrap()
        .trim()
        .to_string()
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd)]
pub struct Input {
    pairs: Vec<[Data; 2]>,
}

impl Input {
    pub fn from_data(data: &str) -> Self {
        let mut pairs = Vec::new();
        for pair in data.split("\n\n") {
            let parts = pair.split_once('\n').unwrap();
            pairs.push([Data::from(parts.0), Data::from(parts.1)]);
        }
        Self { pairs }
    }

    pub fn part_1(&self) -> usize {
        self.pairs
            .iter()
            .enumerate()
            .filter_map(|(idx, pair)| {
                if matches!(
                    pair[0].partial_cmp(&pair[1]),
                    Some(std::cmp::Ordering::Less)
                ) {
                    Some(idx + 1)
                } else {
                    None
                }
            })
            .sum()
    }

    pub fn part_2(&self) -> usize {
        let divider = [
            Data::List(vec![Data::List(vec![Data::Integer(2)])]),
            Data::List(vec![Data::List(vec![Data::Integer(6)])]),
        ];

        let mut packets = self
            .pairs
            .iter()
            .flatten()
            .chain(&divider)
            .collect::<Vec<&Data>>();

        packets.sort();

        divider
            .iter()
            .map(|packet| packets.iter().position(|&p| p.eq(packet)).unwrap() + 1)
            .product()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Data {
    Integer(u8),
    List(Vec<Data>),
}

impl From<&str> for Data {
    fn from(input: &str) -> Self {
        fn parse_part(input: &str) -> (Data, &str) {
            let mut inp = input.strip_prefix('[').unwrap_or(input);
            let mut output = Vec::new();
            while !inp.is_empty() {
                if let Some(remainder) = inp.strip_prefix(']').map(|p| p.trim_start_matches(',')) {
                    return (Data::List(output), remainder);
                }
                if inp.starts_with('[') {
                    let res = parse_part(inp);
                    output.push(res.0);
                    inp = res.1;
                    continue;
                }

                match inp.find(&[',', ']'][..]) {
                    Some(delim_idx) => {
                        output.push(Data::Integer(inp[..delim_idx].parse().unwrap()));
                        inp = &inp[delim_idx..];
                    }
                    None => {
                        output.push(Data::Integer(inp.parse().unwrap()));
                        inp = "";
                    }
                }
                inp = inp.trim_start_matches(',');
            }

            (Data::List(output), "")
        }

        parse_part(input).0
    }
}

impl PartialOrd for Data {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Data::Integer(left), Data::Integer(right)) => match left.cmp(right) {
                std::cmp::Ordering::Less => Some(std::cmp::Ordering::Less),
                std::cmp::Ordering::Equal => None,
                std::cmp::Ordering::Greater => Some(std::cmp::Ordering::Greater),
            },
            (&Data::Integer(left), Data::List(_)) => {
                Data::List(vec![Data::Integer(left)]).partial_cmp(other)
            }
            (Data::List(_), &Data::Integer(right)) => {
                self.partial_cmp(&Data::List(vec![Data::Integer(right)]))
            }
            (Data::List(left), Data::List(right)) => {
                for (lhs, rhs) in left.iter().zip(right) {
                    if let Some(comparison) = lhs.partial_cmp(rhs) {
                        return Some(comparison);
                    }
                }
                Data::Integer(left.len() as u8).partial_cmp(&Data::Integer(right.len() as u8))
            }
        }
    }
}

impl Ord for Data {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap_or(std::cmp::Ordering::Equal)
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
                expected: 140,
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
            "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]",
            Input {
                pairs: vec![
                    [
                        Data::List(vec![
                            Data::Integer(1),
                            Data::Integer(1),
                            Data::Integer(3),
                            Data::Integer(1),
                            Data::Integer(1),
                        ]),
                        Data::List(vec![
                            Data::Integer(1),
                            Data::Integer(1),
                            Data::Integer(5),
                            Data::Integer(1),
                            Data::Integer(1),
                        ]),
                    ],
                    [
                        Data::List(vec![
                            Data::List(vec![Data::Integer(1)]),
                            Data::List(vec![Data::Integer(2), Data::Integer(3), Data::Integer(4)]),
                        ]),
                        Data::List(vec![Data::List(vec![Data::Integer(1)]), Data::Integer(4)]),
                    ],
                    [
                        Data::List(vec![Data::Integer(9)]),
                        Data::List(vec![Data::List(vec![
                            Data::Integer(8),
                            Data::Integer(7),
                            Data::Integer(6),
                        ])]),
                    ],
                    [
                        Data::List(vec![
                            Data::List(vec![Data::Integer(4), Data::Integer(4)]),
                            Data::Integer(4),
                            Data::Integer(4),
                        ]),
                        Data::List(vec![
                            Data::List(vec![Data::Integer(4), Data::Integer(4)]),
                            Data::Integer(4),
                            Data::Integer(4),
                            Data::Integer(4),
                        ]),
                    ],
                    [
                        Data::List(vec![
                            Data::Integer(7),
                            Data::Integer(7),
                            Data::Integer(7),
                            Data::Integer(7),
                        ]),
                        Data::List(vec![Data::Integer(7), Data::Integer(7), Data::Integer(7)]),
                    ],
                    [Data::List(vec![]), Data::List(vec![Data::Integer(3)])],
                    [
                        Data::List(vec![Data::List(vec![Data::List(vec![])])]),
                        Data::List(vec![Data::List(vec![])]),
                    ],
                    [
                        Data::List(vec![
                            Data::Integer(1),
                            Data::List(vec![
                                Data::Integer(2),
                                Data::List(vec![
                                    Data::Integer(3),
                                    Data::List(vec![
                                        Data::Integer(4),
                                        Data::List(vec![
                                            Data::Integer(5),
                                            Data::Integer(6),
                                            Data::Integer(7),
                                        ]),
                                    ]),
                                ]),
                            ]),
                            Data::Integer(8),
                            Data::Integer(9),
                        ]),
                        Data::List(vec![
                            Data::Integer(1),
                            Data::List(vec![
                                Data::Integer(2),
                                Data::List(vec![
                                    Data::Integer(3),
                                    Data::List(vec![
                                        Data::Integer(4),
                                        Data::List(vec![
                                            Data::Integer(5),
                                            Data::Integer(6),
                                            Data::Integer(0),
                                        ]),
                                    ]),
                                ]),
                            ]),
                            Data::Integer(8),
                            Data::Integer(9),
                        ]),
                    ],
                ],
            },
        )
    }
}
