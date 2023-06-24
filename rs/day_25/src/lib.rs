use std::{
    fmt::{Display, Write},
    fs::read_to_string,
    path::Path,
};

lazy_static::lazy_static! {
    pub static ref PART_1: SNAFU =    SNAFU(vec![
    SNAFUDigit::Two,
    SNAFUDigit::DoubleMinus,
    SNAFUDigit::DoubleMinus,
    SNAFUDigit::Zero,
    SNAFUDigit::DoubleMinus,
    SNAFUDigit::Zero,
    SNAFUDigit::DoubleMinus,
    SNAFUDigit::DoubleMinus,
    SNAFUDigit::DoubleMinus,
    SNAFUDigit::Zero,
    SNAFUDigit::Two,
    SNAFUDigit::Minus,
    SNAFUDigit::Minus,
    SNAFUDigit::Two,
    SNAFUDigit::One,
    SNAFUDigit::Zero,
    SNAFUDigit::Minus,
    SNAFUDigit::Minus,
    SNAFUDigit::Minus,
    SNAFUDigit::One,
]);
}
pub const PART_2: usize = 0;

pub fn read_data(data_dir: &str) -> String {
    read_to_string(Path::new(data_dir).join("day_25.txt"))
        .unwrap()
        .trim()
        .to_string()
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Input {
    values: Vec<SNAFU>,
}

impl Input {
    pub fn from_data(data: &str) -> Self {
        let mut values = Vec::new();
        for line in data.lines() {
            let snafu = SNAFU(
                line.chars()
                    .map(|char| match char {
                        '=' => SNAFUDigit::DoubleMinus,
                        '-' => SNAFUDigit::Minus,
                        '0' => SNAFUDigit::Zero,
                        '1' => SNAFUDigit::One,
                        '2' => SNAFUDigit::Two,
                        _ => unreachable!(),
                    })
                    .collect::<Vec<_>>(),
            );
            values.push(snafu);
        }
        Self { values }
    }

    pub fn part_1(&self) -> SNAFU {
        self.values.iter().map(isize::from).sum::<isize>().into()
    }

    pub fn part_2(&self) -> usize {
        0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum SNAFUDigit {
    Two,
    One,
    Zero,
    Minus,
    DoubleMinus,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SNAFU(Vec<SNAFUDigit>);

impl Display for SNAFU {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for digit in &self.0 {
            f.write_char(match digit {
                SNAFUDigit::Two => '2',
                SNAFUDigit::One => '1',
                SNAFUDigit::Zero => '0',
                SNAFUDigit::Minus => '-',
                SNAFUDigit::DoubleMinus => '=',
            })?;
        }
        Ok(())
    }
}

impl From<SNAFU> for isize {
    fn from(value: SNAFU) -> Self {
        Self::from(&value)
    }
}

impl From<&SNAFU> for isize {
    fn from(value: &SNAFU) -> Self {
        value
            .0
            .iter()
            .rev()
            .enumerate()
            .fold(0isize, |acc, (idx, digit)| {
                let multiplier = 5isize.pow(idx as u32);
                match digit {
                    SNAFUDigit::Two => acc + (multiplier * 2),
                    SNAFUDigit::One => acc + multiplier,
                    SNAFUDigit::Zero => acc,
                    SNAFUDigit::Minus => acc - multiplier,
                    SNAFUDigit::DoubleMinus => acc - (2 * multiplier),
                }
            })
    }
}

impl From<isize> for SNAFU {
    fn from(input_value: isize) -> Self {
        let mut max_pow = 0;
        loop {
            if input_value <= (5isize.pow(max_pow + 1) / 2) {
                break;
            }
            max_pow += 1;
        }

        let mut out = Vec::new();

        let mut value = input_value;
        for pow in (0..=max_pow).rev() {
            if pow == 0 {
                out.push(match value {
                    -2 => SNAFUDigit::DoubleMinus,
                    -1 => SNAFUDigit::Minus,
                    0 => SNAFUDigit::Zero,
                    1 => SNAFUDigit::One,
                    2 => SNAFUDigit::Two,
                    _ => unreachable!("{out:?}\t{value}"),
                });
                continue;
            }

            let mul = 5isize.pow(pow);
            let max_tail = mul / 2;

            if ((mul + max_tail + 1)..=(mul + mul + max_tail)).contains(&value) {
                out.push(SNAFUDigit::Two);
                value -= 2 * mul;
                continue;
            }

            if ((max_tail + 1)..=(mul + max_tail)).contains(&value) {
                out.push(SNAFUDigit::One);
                value -= mul;
                continue;
            }

            if ((-max_tail)..=(max_tail)).contains(&value) {
                out.push(SNAFUDigit::Zero);
                continue;
            }

            if ((-mul - max_tail)..(-max_tail)).contains(&value) {
                out.push(SNAFUDigit::Minus);
                value += mul;
                continue;
            }

            if ((-mul - mul - max_tail)..(-mul - max_tail)).contains(&value) {
                out.push(SNAFUDigit::DoubleMinus);
                value += 2 * mul;
                continue;
            }

            unreachable!("{out:?}\t{value}");
        }

        Self(out)
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

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
            expected: SNAFU,
        }

        #[test]
        fn example() {
            run(&Case {
                data: super::example().1,
                expected: SNAFU(vec![
                    SNAFUDigit::Two,
                    SNAFUDigit::DoubleMinus,
                    SNAFUDigit::Minus,
                    SNAFUDigit::One,
                    SNAFUDigit::DoubleMinus,
                    SNAFUDigit::Zero,
                ]),
            })
        }

        #[test]
        fn actual() {
            run(&Case {
                data: Input::from_data(&read_data(DATA_DIR)),
                expected: PART_1.clone(),
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
                expected: 0,
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

    mod snafu {
        use super::*;

        #[test]
        fn example() {
            for input in 0..8192 {
                let val = SNAFU::from(input);
                let out = isize::from(val);
                assert_eq!(input, out);
            }
        }
    }

    fn example() -> (&'static str, Input) {
        (
            "1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122",
            Input {
                values: vec![
                    SNAFU(vec![
                        SNAFUDigit::One,
                        SNAFUDigit::DoubleMinus,
                        SNAFUDigit::Minus,
                        SNAFUDigit::Zero,
                        SNAFUDigit::Minus,
                        SNAFUDigit::Two,
                    ]),
                    SNAFU(vec![
                        SNAFUDigit::One,
                        SNAFUDigit::Two,
                        SNAFUDigit::One,
                        SNAFUDigit::One,
                        SNAFUDigit::One,
                    ]),
                    SNAFU(vec![
                        SNAFUDigit::Two,
                        SNAFUDigit::DoubleMinus,
                        SNAFUDigit::Zero,
                        SNAFUDigit::DoubleMinus,
                    ]),
                    SNAFU(vec![SNAFUDigit::Two, SNAFUDigit::One]),
                    SNAFU(vec![
                        SNAFUDigit::Two,
                        SNAFUDigit::DoubleMinus,
                        SNAFUDigit::Zero,
                        SNAFUDigit::One,
                    ]),
                    SNAFU(vec![SNAFUDigit::One, SNAFUDigit::One, SNAFUDigit::One]),
                    SNAFU(vec![
                        SNAFUDigit::Two,
                        SNAFUDigit::Zero,
                        SNAFUDigit::Zero,
                        SNAFUDigit::One,
                        SNAFUDigit::Two,
                    ]),
                    SNAFU(vec![SNAFUDigit::One, SNAFUDigit::One, SNAFUDigit::Two]),
                    SNAFU(vec![
                        SNAFUDigit::One,
                        SNAFUDigit::DoubleMinus,
                        SNAFUDigit::Minus,
                        SNAFUDigit::One,
                        SNAFUDigit::DoubleMinus,
                    ]),
                    SNAFU(vec![
                        SNAFUDigit::One,
                        SNAFUDigit::Minus,
                        SNAFUDigit::One,
                        SNAFUDigit::Two,
                    ]),
                    SNAFU(vec![SNAFUDigit::One, SNAFUDigit::Two]),
                    SNAFU(vec![SNAFUDigit::One, SNAFUDigit::DoubleMinus]),
                    SNAFU(vec![SNAFUDigit::One, SNAFUDigit::Two, SNAFUDigit::Two]),
                ],
            },
        )
    }
}
