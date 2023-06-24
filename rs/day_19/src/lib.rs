use std::{fs::read_to_string, path::Path};

pub const PART_1: usize = 1395;
pub const PART_2: usize = 2700;

pub fn read_data(data_dir: &str) -> String {
    read_to_string(Path::new(data_dir).join("day_19.txt"))
        .unwrap()
        .trim()
        .to_string()
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Input {
    blueprints: Vec<Blueprint>,
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
struct Blueprint {
    ore_robot_ore_cost: u8,
    clay_robot_ore_cost: u8,
    obisidian_robot_ore_cost: u8,
    obisidian_robot_clay_cost: u8,
    geode_robot_ore_cost: u8,
    geode_robot_obsidian_cost: u8,
}

impl Input {
    pub fn from_data(data: &str) -> Self {
        let mut blueprints = Vec::new();
        for line in data.trim().lines() {
            let parts = line.trim().split('.').collect::<Vec<_>>();
            let blueprint = Blueprint {
                ore_robot_ore_cost: parts[0]
                    .split_ascii_whitespace()
                    .nth_back(1)
                    .unwrap()
                    .parse()
                    .unwrap(),
                clay_robot_ore_cost: parts[1]
                    .split_ascii_whitespace()
                    .nth_back(1)
                    .unwrap()
                    .parse()
                    .unwrap(),
                obisidian_robot_ore_cost: parts[2]
                    .split_ascii_whitespace()
                    .nth_back(4)
                    .unwrap()
                    .parse()
                    .unwrap(),
                obisidian_robot_clay_cost: parts[2]
                    .split_ascii_whitespace()
                    .nth_back(1)
                    .unwrap()
                    .parse()
                    .unwrap(),
                geode_robot_ore_cost: parts[3]
                    .split_ascii_whitespace()
                    .nth_back(4)
                    .unwrap()
                    .parse()
                    .unwrap(),
                geode_robot_obsidian_cost: parts[3]
                    .split_ascii_whitespace()
                    .nth_back(1)
                    .unwrap()
                    .parse()
                    .unwrap(),
            };
            blueprints.push(blueprint);
        }
        Self { blueprints }
    }

    pub fn part_1(&self) -> usize {
        let mut scores = Vec::new();
        for blueprint in &self.blueprints {
            let mut best = 0;
            let mut possibilities = vec![State {
                ore_robots: 1,
                ..Default::default()
            }];

            let max_ore = blueprint
                .clay_robot_ore_cost
                .max(blueprint.ore_robot_ore_cost)
                .max(blueprint.obisidian_robot_ore_cost)
                .max(blueprint.geode_robot_ore_cost);

            while let Some(mut state) = possibilities.pop() {
                if state.ore > (max_ore + state.ore_robots) {
                    continue;
                }
                state.minute += 1;

                if state.minute > 24 {
                    best = best.max(state.geodes);
                    continue;
                }

                if state.obsidian >= blueprint.geode_robot_obsidian_cost
                    && state.ore >= blueprint.geode_robot_ore_cost
                {
                    state.ore -= blueprint.geode_robot_ore_cost;
                    state.obsidian -= blueprint.geode_robot_obsidian_cost;
                    state.collect();
                    state.geode_robots += 1;
                    possibilities.push(state);
                    continue;
                }

                if state.clay >= blueprint.obisidian_robot_clay_cost
                    && state.ore >= blueprint.obisidian_robot_ore_cost
                {
                    let mut branch = state;
                    branch.ore -= blueprint.obisidian_robot_ore_cost;
                    branch.clay -= blueprint.obisidian_robot_clay_cost;
                    branch.collect();
                    branch.obsidian_robots += 1;
                    possibilities.push(branch);
                }

                if state.ore >= blueprint.clay_robot_ore_cost {
                    let mut branch = state;
                    branch.ore -= blueprint.clay_robot_ore_cost;
                    branch.collect();
                    branch.clay_robots += 1;
                    possibilities.push(branch);
                }

                if state.ore >= blueprint.ore_robot_ore_cost {
                    let mut branch = state;
                    branch.ore -= blueprint.ore_robot_ore_cost;
                    branch.collect();
                    branch.ore_robots += 1;
                    possibilities.push(branch);
                }

                state.collect();
                if state.ore <= (max_ore + state.ore_robots) {
                    possibilities.push(state);
                }
            }
            scores.push(best);
        }

        scores
            .iter()
            .enumerate()
            .fold(0usize, |acc, (idx, &score)| {
                acc + ((idx + 1) * score as usize)
            })
    }

    pub fn part_2(&self) -> usize {
        let mut scores = Vec::new();
        for blueprint in self.blueprints.iter().take(3) {
            let mut best = 0;
            let mut possibilities = vec![State {
                ore_robots: 1,
                ..Default::default()
            }];

            let max_ore = blueprint
                .clay_robot_ore_cost
                .max(blueprint.ore_robot_ore_cost)
                .max(blueprint.obisidian_robot_ore_cost)
                .max(blueprint.geode_robot_ore_cost);

            while let Some(mut state) = possibilities.pop() {
                if state.ore > 2 * (max_ore + state.ore_robots) {
                    continue;
                }

                state.minute += 1;

                if state.minute > 32 {
                    best = best.max(state.geodes);
                    continue;
                }

                if state.obsidian >= blueprint.geode_robot_obsidian_cost
                    && state.ore >= blueprint.geode_robot_ore_cost
                {
                    state.ore -= blueprint.geode_robot_ore_cost;
                    state.obsidian -= blueprint.geode_robot_obsidian_cost;
                    state.collect();
                    state.geode_robots += 1;
                    possibilities.push(state);
                    continue;
                }

                if state.clay >= blueprint.obisidian_robot_clay_cost
                    && state.ore >= blueprint.obisidian_robot_ore_cost
                {
                    let mut branch = state;
                    branch.ore -= blueprint.obisidian_robot_ore_cost;
                    branch.clay -= blueprint.obisidian_robot_clay_cost;
                    branch.collect();
                    branch.obsidian_robots += 1;
                    possibilities.push(branch);
                }

                if state.ore >= blueprint.clay_robot_ore_cost {
                    let mut branch = state;
                    branch.ore -= blueprint.clay_robot_ore_cost;
                    branch.collect();
                    branch.clay_robots += 1;
                    possibilities.push(branch);
                }

                if state.ore >= blueprint.ore_robot_ore_cost {
                    let mut branch = state;
                    branch.ore -= blueprint.ore_robot_ore_cost;
                    branch.collect();
                    branch.ore_robots += 1;
                    possibilities.push(branch);
                }

                state.collect();
                if state.ore <= (max_ore + state.ore_robots) {
                    possibilities.push(state);
                }
            }
            scores.push(best);
        }

        scores.iter().map(|&s| s as usize).product()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
struct State {
    minute: u8,
    ore_robots: u8,
    ore: u8,
    clay_robots: u8,
    clay: u8,
    obsidian_robots: u8,
    obsidian: u8,
    geode_robots: u8,
    geodes: u8,
}

impl State {
    fn collect(&mut self) {
        self.ore += self.ore_robots;
        self.clay += self.clay_robots;
        self.obsidian += self.obsidian_robots;
        self.geodes += self.geode_robots;
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
                expected: 33,
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
                expected: 56 * 62,
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
            "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.",
            Input {
                blueprints: vec![
                    Blueprint {
                        ore_robot_ore_cost: 4,
                        clay_robot_ore_cost: 2,
                        obisidian_robot_ore_cost: 3,
                        obisidian_robot_clay_cost: 14,
                        geode_robot_ore_cost: 2,
                        geode_robot_obsidian_cost: 7,
                    },
                    Blueprint {
                        ore_robot_ore_cost: 2,
                        clay_robot_ore_cost: 3,
                        obisidian_robot_ore_cost: 3,
                        obisidian_robot_clay_cost: 8,
                        geode_robot_ore_cost: 3,
                        geode_robot_obsidian_cost: 12,
                    },
                ],
            },
        )
    }
}
