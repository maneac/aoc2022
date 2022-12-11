use std::{collections::VecDeque, fs::read_to_string, path::Path};

pub const PART_1: usize = 64032;
pub const PART_2: usize = 12729522272;

pub fn read_data(data_dir: &str) -> String {
    read_to_string(Path::new(data_dir).join("day_11.txt"))
        .unwrap()
        .trim()
        .to_string()
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Input {
    monkeys: Vec<Monkey>,
}

impl Input {
    pub fn from_data(data: &str) -> Self {
        let monkeys = data.split("\n\n").map(Monkey::from).collect();
        Input { monkeys }
    }

    pub fn part_1(&self) -> usize {
        let mut inspection_counts = vec![0; self.monkeys.len()];

        let mut items: Vec<(usize, usize)> = self
            .monkeys
            .iter()
            .enumerate()
            .flat_map(|(idx, monkey)| monkey.items.iter().map(move |&item| (item, idx)))
            .collect();

        for _round in 0..20 {
            for (turn, (monkey, count)) in self
                .monkeys
                .iter()
                .zip(inspection_counts.iter_mut())
                .enumerate()
            {
                for (item, owner) in items.iter_mut() {
                    if *owner != turn {
                        continue;
                    }
                    *count += 1;

                    *item = match monkey.operation {
                        (Operator::Add, None) => *item + *item,
                        (Operator::Add, Some(val)) => *item + val,
                        (Operator::Mul, None) => *item * *item,
                        (Operator::Mul, Some(val)) => *item * val,
                    } / 3;

                    *owner = if *item % monkey.test[0] == 0 {
                        monkey.test[1]
                    } else {
                        monkey.test[2]
                    };
                }
            }
        }

        inspection_counts.sort();
        inspection_counts.iter().rev().take(2).product()
    }

    pub fn part_2(&self) -> usize {
        let modulo: usize = self.monkeys.iter().map(|monkey| monkey.test[0]).product();

        let mut inspection_counts = vec![0; self.monkeys.len()];

        let mut items: Vec<(usize, usize)> = self
            .monkeys
            .iter()
            .enumerate()
            .flat_map(|(idx, monkey)| monkey.items.iter().map(move |&item| (item, idx)))
            .collect();

        for _round in 0..10_000 {
            for (turn, (monkey, count)) in self
                .monkeys
                .iter()
                .zip(inspection_counts.iter_mut())
                .enumerate()
            {
                for (item, owner) in items.iter_mut() {
                    if *owner != turn {
                        continue;
                    }
                    *count += 1;

                    *item = match monkey.operation {
                        (Operator::Add, None) => *item + *item,
                        (Operator::Add, Some(val)) => *item + val,
                        (Operator::Mul, None) => *item * *item,
                        (Operator::Mul, Some(val)) => *item * val,
                    } % modulo;

                    *owner = if *item % monkey.test[0] == 0 {
                        monkey.test[1]
                    } else {
                        monkey.test[2]
                    };
                }
            }
        }

        inspection_counts.sort();
        inspection_counts.iter().rev().take(2).product()
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Monkey {
    items: VecDeque<usize>,
    operation: (Operator, Option<usize>),
    test: [usize; 3],
}

impl From<&str> for Monkey {
    fn from(input: &str) -> Self {
        let mut lines = input.lines().skip(1);

        let starting_items = lines
            .next()
            .unwrap()
            .split(": ")
            .nth(1)
            .unwrap()
            .split(", ")
            .map(|item| item.parse().unwrap())
            .collect();

        let (operator, modifier) = lines
            .next()
            .unwrap()
            .split("new = old ")
            .nth(1)
            .unwrap()
            .split_once(' ')
            .unwrap();

        let operator = match operator {
            "+" => Operator::Add,
            "*" => Operator::Mul,
            op => panic!("unexpected op: '{op}'"),
        };

        let rhs = match modifier {
            "old" => None,
            num => Some(num.parse().unwrap()),
        };

        let operation = (operator, rhs);

        let divisible_by = lines
            .next()
            .unwrap()
            .rsplit(' ')
            .next()
            .unwrap()
            .parse()
            .unwrap();
        let true_monkey = lines
            .next()
            .unwrap()
            .rsplit(' ')
            .next()
            .unwrap()
            .parse()
            .unwrap();
        let false_monkey = lines
            .next()
            .unwrap()
            .rsplit(' ')
            .next()
            .unwrap()
            .parse()
            .unwrap();

        let test = [divisible_by, true_monkey, false_monkey];

        Monkey {
            items: starting_items,
            operation,
            test,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Operator {
    Add,
    Mul,
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
                expected: 10605,
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
                expected: 2713310158,
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
            "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1",
            Input {
                monkeys: vec![
                    Monkey {
                        items: VecDeque::from([79, 98]),
                        operation: (Operator::Mul, Some(19)),
                        test: [23, 2, 3],
                    },
                    Monkey {
                        items: VecDeque::from([54, 65, 75, 74]),
                        operation: (Operator::Add, Some(6)),
                        test: [19, 2, 0],
                    },
                    Monkey {
                        items: VecDeque::from([79, 60, 97]),
                        operation: (Operator::Mul, None),
                        test: [13, 1, 3],
                    },
                    Monkey {
                        items: VecDeque::from([74]),
                        operation: (Operator::Add, Some(3)),
                        test: [17, 0, 1],
                    },
                ],
            },
        )
    }
}
