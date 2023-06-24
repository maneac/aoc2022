use std::{collections::HashSet, fs::read_to_string, path::Path};

pub const PART_1: usize = 4320;
pub const PART_2: usize = 2456;

pub fn read_data(data_dir: &str) -> String {
    read_to_string(Path::new(data_dir).join("day_18.txt"))
        .unwrap()
        .trim()
        .to_string()
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Input {
    cubes: HashSet<[u8; 3]>,
}

impl Input {
    pub fn from_data(data: &str) -> Self {
        let mut cubes = HashSet::new();
        for line in data.trim().lines() {
            let [x,y,z] = line
            .trim()
                .split_terminator(',')
                .map(|n| n.parse::<u8>().unwrap())
                .take(3)
                .collect::<Vec<u8>>()[..3] else {
                panic!("incorrect num parts");    
            };
            cubes.insert([x + 1, y + 1, z + 1]);
        }

        Self { cubes }
    }

    pub fn part_1(&self) -> usize {
        let mut total = 0;
        for &[x, y, z] in &self.cubes {
            for neighbour in [
                [x - 1, y, z],
                [x + 1, y, z],
                [x, y - 1, z],
                [x, y + 1, z],
                [x, y, z - 1],
                [x, y, z + 1],
            ] {
                if !self.cubes.contains(&neighbour) {
                    total += 1;
                }
            }
        }

        total
    }

    pub fn part_2(&self) -> usize {
        let max = self
            .cubes
            .iter()
            .fold([0u8; 3], |[a_x, a_y, a_z], &[c_x, c_y, c_z]| {
                [a_x.max(c_x), a_y.max(c_y), a_z.max(c_z)]
            });

        let mut total = 0;
        let mut visited = HashSet::<[u8; 3]>::new();
        let mut to_visit = vec![[0u8; 3]];

        while let Some([x, y, z]) = to_visit.pop() {
            if !visited.insert([x, y, z]) {
                continue;
            }
            for neighbour in [
                [x.saturating_sub(1), y, z],
                [(x + 1).clamp(0, max[0] + 1), y, z],
                [x, y.saturating_sub(1), z],
                [x, (y + 1).clamp(0, max[1] + 1), z],
                [x, y, z.saturating_sub(1)],
                [x, y, (z + 1).clamp(0, max[2] + 1)],
            ] {
                if neighbour == [x, y, z] {
                    continue;
                }
                if self.cubes.contains(&neighbour) {
                    total += 1;
                    continue;
                }
                if !visited.contains(&neighbour) {
                    to_visit.push(neighbour);
                }
            }
        }
        total
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
        fn small_example() {
            run(&Case {
                input: super::small_example().0,
                expected: super::small_example().1,
            })
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
        fn small_example() {
            run(&Case {
                data: super::small_example().1,
                expected: 10,
            })
        }

        #[test]
        fn example() {
            run(&Case {
                data: super::example().1,
                expected: 64,
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
                expected: 58,
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

    fn small_example() -> (&'static str, Input) {
        (
            "1,1,1
2,1,1",
            Input {
                cubes: HashSet::from([[2, 2, 2], [3, 2, 2]]),
            },
        )
    }

    fn example() -> (&'static str, Input) {
        (
            "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5
",
            Input {
                cubes: HashSet::from([
                    [3, 3, 3],
                    [2, 3, 3],
                    [4, 3, 3],
                    [3, 2, 3],
                    [3, 4, 3],
                    [3, 3, 2],
                    [3, 3, 4],
                    [3, 3, 5],
                    [3, 3, 7],
                    [2, 3, 6],
                    [4, 3, 6],
                    [3, 2, 6],
                    [3, 4, 6],
                ]),
            },
        )
    }
}
