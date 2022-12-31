use std::{collections::HashMap, fs::read_to_string, path::Path};

pub const PART_1: usize = 1845;
pub const PART_2: usize = 2286;

pub fn read_data(data_dir: &str) -> String {
    read_to_string(Path::new(data_dir).join("day_16.txt"))
        .unwrap()
        .trim()
        .to_string()
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Input {
    valves: HashMap<usize, (usize, Vec<usize>)>,
}

impl Input {
    pub fn from_data(data: &str) -> Self {
        let line_count = data.lines().count();

        let mut valves = HashMap::with_capacity(line_count);
        let mut valve_idxs = Vec::with_capacity(line_count);

        valve_idxs.push(['A', 'A']);

        for line in data.lines() {
            let mut line_parts = line.split_ascii_whitespace();

            let mut label = [' '; 2];
            label
                .iter_mut()
                .zip(line_parts.nth(1).unwrap().chars())
                .for_each(|(l, c)| {
                    *l = c;
                });

            if ['A', 'A'].eq(&label) {
                continue;
            }
            valve_idxs.push(label);
        }

        for line in data.lines() {
            let mut line_parts = line.split_ascii_whitespace();

            let mut label = [' '; 2];
            label
                .iter_mut()
                .zip(line_parts.nth(1).unwrap().chars())
                .for_each(|(l, c)| {
                    *l = c;
                });

            let idx = valve_idxs.iter().position(|v| v == &label).unwrap();

            let flow_rate = line_parts
                .nth(2)
                .unwrap()
                .strip_prefix("rate=")
                .unwrap()
                .strip_suffix(';')
                .unwrap()
                .parse::<usize>()
                .unwrap();

            let neighbours = line_parts
                .skip(4)
                .map(|part| {
                    let mut label = [' '; 2];
                    label.iter_mut().zip(part.chars()).for_each(|(l, c)| *l = c);
                    valve_idxs.iter().position(|v| v == &label).unwrap()
                })
                .collect::<Vec<usize>>();

            valves.insert(idx, (flow_rate, neighbours));
        }

        Self { valves }
    }

    pub fn part_1(&self) -> usize {
        self.part_1_recursive::<30>(0, 0, 0, 0, 0)
    }

    pub fn part_2(&self) -> usize {
        self.part_2_recursive::<26>(0, 0, [0, 0], 0, [0, 0])
    }

    fn part_1_recursive<const MIN: u8>(
        &self,
        minute: u8,
        score: usize,
        current: usize,
        open_valves: u64,
        since_last_open: u64,
    ) -> usize {
        if minute == MIN {
            return score;
        }

        let (flow_rate, neighbours) = self.valves.get(&current).unwrap();
        let opened = open_valves & 1 << current > 0;

        let mut best = score;
        if *flow_rate > 0 && !opened {
            best = best.max(self.part_1_recursive::<MIN>(
                minute + 1,
                score + ((MIN - 1 - minute) as usize * flow_rate),
                current,
                open_valves | 1 << current,
                0,
            ));
        }

        for &neighbour in neighbours {
            if since_last_open & 1 << neighbour > 0 {
                continue;
            }
            best = best.max(self.part_1_recursive::<MIN>(
                minute + 1,
                score,
                neighbour,
                open_valves,
                since_last_open | 1 << current,
            ))
        }

        best
    }

    fn part_2_recursive<const MIN: u8>(
        &self,
        minute: u8,
        score: usize,
        current: [usize; 2],
        open_valves: u64,
        since_last_open: [u64; 2],
    ) -> usize {
        if minute == MIN {
            return score;
        }

        let in_early_threshold = minute < 3;

        let (flow_rate_0, neighbours_0) = self.valves.get(&current[0]).unwrap();
        let (flow_rate_1, neighbours_1) = self.valves.get(&current[1]).unwrap();
        let opened_0 = open_valves & 1 << current[0] > 0;
        let opened_1 = open_valves & 1 << current[1] > 0;

        let mut best = score;
        // open both
        if current[0] != current[1]
            && *flow_rate_0 > 0
            && *flow_rate_1 > 0
            && !opened_0
            && !opened_1
        {
            let new_score = self.part_2_recursive::<MIN>(
                minute + 1,
                score
                    + ((MIN - 1 - minute) as usize * flow_rate_0)
                    + ((MIN - 1 - minute) as usize * flow_rate_1),
                current,
                open_valves | 1 << current[0] | 1 << current[1],
                [0, 0],
            );
            best = best.max(new_score);
        }

        // open 0, move 1
        if *flow_rate_0 > 0 && !opened_0 {
            for &neighbour_1 in neighbours_1 {
                if since_last_open[1] & 1 << neighbour_1 > 0 {
                    continue;
                }
                let new_score = self.part_2_recursive::<MIN>(
                    minute + 1,
                    score + ((MIN - 1 - minute) as usize * flow_rate_0),
                    [current[0], neighbour_1],
                    open_valves | 1 << current[0],
                    [0, since_last_open[1] | 1 << current[1]],
                );
                best = best.max(new_score);
            }
        }

        // move 0, open 1
        if *flow_rate_1 > 0 && !opened_1 {
            for &neighbour_0 in neighbours_0 {
                if since_last_open[0] & 1 << neighbour_0 > 0 {
                    continue;
                }
                let new_score = self.part_2_recursive::<MIN>(
                    minute + 1,
                    score + ((MIN - 1 - minute) as usize * flow_rate_1),
                    [neighbour_0, current[1]],
                    open_valves | 1 << current[1],
                    [since_last_open[0] | 1 << current[0], 0],
                );
                best = best.max(new_score);
            }
        }

        // move both
        if in_early_threshold {
            let (tx, rx) = std::sync::mpsc::channel();

            for &neighbour_0 in neighbours_0 {
                if since_last_open[0] & 1 << neighbour_0 > 0 {
                    continue;
                }
                for &neighbour_1 in neighbours_1 {
                    if since_last_open[1] & 1 << neighbour_1 > 0 {
                        continue;
                    }

                    let t = tx.clone();
                    let s = Input {
                        valves: self.valves.clone(),
                    };
                    std::thread::spawn(move || {
                        t.send(s.part_2_recursive::<MIN>(
                            minute + 1,
                            score,
                            [neighbour_0, neighbour_1],
                            open_valves,
                            [
                                since_last_open[0] | 1 << current[0],
                                since_last_open[1] | 1 << current[1],
                            ],
                        ))
                        .unwrap();
                    });
                }
            }
            drop(tx);

            while let Ok(res) = rx.recv() {
                best = best.max(res);
            }

            return best;
        }

        for &neighbour_0 in neighbours_0 {
            if since_last_open[0] & 1 << neighbour_0 > 0 {
                continue;
            }
            for &neighbour_1 in neighbours_1 {
                if since_last_open[1] & 1 << neighbour_1 > 0 {
                    continue;
                }
                let new_score = self.part_2_recursive::<MIN>(
                    minute + 1,
                    score,
                    [neighbour_0, neighbour_1],
                    open_valves,
                    [
                        since_last_open[0] | 1 << current[0],
                        since_last_open[1] | 1 << current[1],
                    ],
                );
                best = best.max(new_score);
            }
        }

        best
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
                expected: 1651,
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
                expected: 1707,
            })
        }

        #[test]
        #[ignore = "takes hours to run"]
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
            "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II",
            Input {
                valves: HashMap::from([
                    (0, (0, vec![3, 8, 1])),
                    (1, (13, vec![2, 0])),
                    (2, (2, vec![3, 1])),
                    (3, (20, vec![2, 0, 4])),
                    (4, (3, vec![5, 3])),
                    (5, (0, vec![4, 6])),
                    (6, (0, vec![5, 7])),
                    (7, (22, vec![6])),
                    (8, (0, vec![0, 9])),
                    (9, (21, vec![8])),
                ]),
            },
        )
    }
}
