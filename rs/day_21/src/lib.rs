use std::{collections::HashMap, fs::read_to_string, path::Path};

pub const PART_1: usize = 72664227897438;
pub const PART_2: usize = 3916491093817;

pub fn read_data(data_dir: &str) -> String {
    read_to_string(Path::new(data_dir).join("day_21.txt"))
        .unwrap()
        .trim()
        .to_string()
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Input {
    monkeys: HashMap<String, Monkey>,
}

impl Input {
    pub fn from_data(data: &str) -> Self {
        let mut monkeys = HashMap::new();
        for line in data.trim().lines() {
            let (label, sum) = line.trim().split_once(": ").unwrap();
            monkeys.insert(
                label.to_string(),
                Monkey {
                    label: label.to_string(),
                    sum: Sum::from(sum),
                },
            );
        }
        Self { monkeys }
    }

    pub fn part_1(&self) -> usize {
        self.monkeys.get("root").unwrap().value(&self.monkeys)
    }

    pub fn part_2(&self) -> usize {
        let mut monkeys = self.monkeys.clone();
        {
            let r = monkeys.get_mut("root").unwrap();
            if let Sum::Sum((_, op, _)) = &mut r.sum {
                *op = Op::Eq;
            }
        }

        let eqn = monkeys.get("root").unwrap().as_equation(&monkeys);
        if let Equation::Equation { rhs, .. } = eqn.balance_to_human() {
            return rhs.solve(0);
        }
        unreachable!()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Equation {
    Human,
    Num(usize),
    Equation {
        contains_human: bool,
        lhs: Box<Equation>,
        op: Op,
        rhs: Box<Equation>,
    },
}

impl Equation {
    fn balance_to_human(self) -> Self {
        match self {
            Equation::Equation {
                contains_human,
                lhs: mut parent_lhs,
                op: parent_op,
                rhs: mut parent_rhs,
            } if contains_human => {
                while !matches!(parent_lhs.as_ref(), Equation::Human) {
                    if parent_lhs.contains_human() {
                        match parent_lhs.as_ref() {
                            Equation::Equation { lhs, op, rhs, .. } => {
                                if lhs.contains_human() {
                                    parent_rhs = Box::new(Self::Equation {
                                        contains_human: false,
                                        lhs: parent_rhs,
                                        op: op.negate(),
                                        rhs: rhs.clone(),
                                    });
                                    parent_lhs = lhs.clone();
                                    continue;
                                }

                                if op.commutative() {
                                    parent_rhs = Box::new(Self::Equation {
                                        contains_human: false,
                                        lhs: parent_rhs,
                                        op: op.negate(),
                                        rhs: lhs.clone(),
                                    });
                                    parent_lhs = rhs.clone();
                                    continue;
                                }

                                parent_rhs = Box::new(Self::Equation {
                                    contains_human: true,
                                    lhs: parent_rhs,
                                    op: op.negate(),
                                    rhs: rhs.clone(),
                                });
                                parent_lhs = lhs.clone();
                                continue;
                            }
                            Equation::Human => unreachable!(),
                            Equation::Num(_) => unreachable!(),
                        }
                    }

                    match parent_rhs.as_ref() {
                        Equation::Equation { op, .. } => {
                            if op.commutative() {
                                (parent_lhs, parent_rhs) = (parent_rhs, parent_lhs);
                                continue;
                            }
                            unimplemented!()
                        }
                        Equation::Human => unreachable!(),
                        Equation::Num(_) => unreachable!(),
                    }
                }
                Self::Equation {
                    contains_human,
                    lhs: parent_lhs,
                    op: parent_op,
                    rhs: parent_rhs,
                }
            }
            _ => self,
        }
    }

    fn solve(&self, human_value: usize) -> usize {
        match &self {
            Equation::Human => human_value,
            Equation::Num(v) => *v,
            Equation::Equation {
                contains_human: _,
                lhs,
                op,
                rhs,
            } => match op {
                Op::Add => lhs.solve(human_value) + rhs.solve(human_value),
                Op::Sub => lhs.solve(human_value) - rhs.solve(human_value),
                Op::Mul => lhs.solve(human_value) * rhs.solve(human_value),
                Op::Div => lhs.solve(human_value) / rhs.solve(human_value),
                Op::Eq => (lhs.solve(human_value) == rhs.solve(human_value)) as usize,
            },
        }
    }

    fn contains_human(&self) -> bool {
        match &self {
            Equation::Human => true,
            Equation::Num(_) => false,
            Equation::Equation { contains_human, .. } => *contains_human,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Monkey {
    label: String,
    sum: Sum,
}

impl Monkey {
    fn value(&self, monkeys: &HashMap<String, Monkey>) -> usize {
        match &self.sum {
            Sum::Num(v) => *v,
            Sum::Sum((lhs_label, op, rhs_label)) => {
                let lhs = monkeys.get(lhs_label).unwrap().value(monkeys);
                let rhs = monkeys.get(rhs_label).unwrap().value(monkeys);
                match op {
                    Op::Add => lhs + rhs,
                    Op::Sub => lhs - rhs,
                    Op::Mul => lhs * rhs,
                    Op::Div => lhs / rhs,
                    Op::Eq => lhs.abs_diff(rhs),
                }
            }
        }
    }

    fn as_equation(&self, monkeys: &HashMap<String, Monkey>) -> Equation {
        if self.label == "humn" {
            return Equation::Human;
        }
        match &self.sum {
            Sum::Num(v) => Equation::Num(*v),
            Sum::Sum((lhs, op, rhs)) => {
                let lhs = Box::new(monkeys.get(lhs).unwrap().as_equation(monkeys));
                let rhs = Box::new(monkeys.get(rhs).unwrap().as_equation(monkeys));
                Equation::Equation {
                    contains_human: lhs.contains_human() || rhs.contains_human(),
                    lhs,
                    op: *op,
                    rhs,
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Sum {
    Num(usize),
    Sum((String, Op, String)),
}

impl From<&str> for Sum {
    fn from(value: &str) -> Self {
        if let Ok(v) = value.parse() {
            return Self::Num(v);
        }

        let mut parts = value.split_ascii_whitespace();
        let lhs_label = parts.next().unwrap();
        let operator = match parts.next().unwrap() {
            "+" => Op::Add,
            "-" => Op::Sub,
            "*" => Op::Mul,
            "/" => Op::Div,
            "=" => Op::Eq,
            op => panic!("unknown operator: {op:?}"),
        };
        let rhs_label = parts.next().unwrap();

        Self::Sum((lhs_label.into(), operator, rhs_label.into()))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
}

impl Op {
    fn negate(&self) -> Self {
        match &self {
            Op::Add => Op::Sub,
            Op::Sub => Op::Add,
            Op::Mul => Op::Div,
            Op::Div => Op::Mul,
            Op::Eq => Op::Eq,
        }
    }

    fn commutative(&self) -> bool {
        matches!(self, Op::Add | Op::Mul | Op::Eq)
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
                expected: 152,
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
                expected: 301,
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
            "root: pppw + sjmn
        dbpl: 5
        cczh: sllz + lgvd
        zczc: 2
        ptdq: humn - dvpt
        dvpt: 3
        lfqf: 4
        humn: 5
        ljgn: 2
        sjmn: drzm * dbpl
        sllz: 4
        pppw: cczh / lfqf
        lgvd: ljgn * ptdq
        drzm: hmdt - zczc
        hmdt: 32",
            Input {
                monkeys: HashMap::from([
                    (
                        "root".into(),
                        Monkey {
                            label: "root".into(),
                            sum: "pppw + sjmn".into(),
                        },
                    ),
                    (
                        "dbpl".into(),
                        Monkey {
                            label: "dbpl".into(),
                            sum: "5".into(),
                        },
                    ),
                    (
                        "cczh".into(),
                        Monkey {
                            label: "cczh".into(),
                            sum: "sllz + lgvd".into(),
                        },
                    ),
                    (
                        "zczc".into(),
                        Monkey {
                            label: "zczc".into(),
                            sum: "2".into(),
                        },
                    ),
                    (
                        "ptdq".into(),
                        Monkey {
                            label: "ptdq".into(),
                            sum: "humn - dvpt".into(),
                        },
                    ),
                    (
                        "dvpt".into(),
                        Monkey {
                            label: "dvpt".into(),
                            sum: "3".into(),
                        },
                    ),
                    (
                        "lfqf".into(),
                        Monkey {
                            label: "lfqf".into(),
                            sum: "4".into(),
                        },
                    ),
                    (
                        "humn".into(),
                        Monkey {
                            label: "humn".into(),
                            sum: "5".into(),
                        },
                    ),
                    (
                        "ljgn".into(),
                        Monkey {
                            label: "ljgn".into(),
                            sum: "2".into(),
                        },
                    ),
                    (
                        "sjmn".into(),
                        Monkey {
                            label: "sjmn".into(),
                            sum: "drzm * dbpl".into(),
                        },
                    ),
                    (
                        "sllz".into(),
                        Monkey {
                            label: "sllz".into(),
                            sum: "4".into(),
                        },
                    ),
                    (
                        "pppw".into(),
                        Monkey {
                            label: "pppw".into(),
                            sum: "cczh / lfqf".into(),
                        },
                    ),
                    (
                        "lgvd".into(),
                        Monkey {
                            label: "lgvd".into(),
                            sum: "ljgn * ptdq".into(),
                        },
                    ),
                    (
                        "drzm".into(),
                        Monkey {
                            label: "drzm".into(),
                            sum: "hmdt - zczc".into(),
                        },
                    ),
                    (
                        "hmdt".into(),
                        Monkey {
                            label: "hmdt".into(),
                            sum: "32".into(),
                        },
                    ),
                ]),
            },
        )
    }
}
