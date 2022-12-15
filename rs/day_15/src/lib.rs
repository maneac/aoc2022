use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
    path::Path,
};

pub const PART_1: usize = 5073496;
pub const PART_2: usize = 13081194638237;

pub fn read_data(data_dir: &str) -> String {
    read_to_string(Path::new(data_dir).join("day_15.txt"))
        .unwrap()
        .trim()
        .to_string()
}

#[derive(Debug, Default)]
pub struct Input {
    sensors_to_manhattan: HashMap<(isize, isize), isize>,
    beacons: HashSet<(isize, isize)>,
    part_1_row: isize,
}

impl PartialEq for Input {
    fn eq(&self, other: &Self) -> bool {
        self.sensors_to_manhattan == other.sensors_to_manhattan && self.beacons == other.beacons
    }
}

impl Input {
    pub fn from_data(data: &str) -> Self {
        let mut sensors_to_manhattan = HashMap::new();
        let mut beacons = HashSet::new();
        for line in data.lines() {
            let (lhs, rhs) = line.split_once(": closest beacon is at x=").unwrap();
            let lhs_parts = lhs
                .strip_prefix("Sensor at x=")
                .unwrap()
                .split_once(", y=")
                .unwrap();
            let sensor: (isize, isize) =
                (lhs_parts.0.parse().unwrap(), lhs_parts.1.parse().unwrap());

            let rhs_parts = rhs.split_once(", y=").unwrap();
            let beacon = (rhs_parts.0.parse().unwrap(), rhs_parts.1.parse().unwrap());

            let manhattan = (sensor.0.abs_diff(beacon.0) + sensor.1.abs_diff(beacon.1)) as isize;

            sensors_to_manhattan.insert(sensor, manhattan);
            beacons.insert(beacon);
        }

        Self {
            sensors_to_manhattan,
            beacons,
            part_1_row: 2_000_000,
        }
    }

    pub fn part_1(&self) -> usize {
        let mut covered_ranges = Vec::new();

        for (&(sensor_x, sensor_y), manhattan) in &self.sensors_to_manhattan {
            if (sensor_y - manhattan..=sensor_y + manhattan).contains(&self.part_1_row) {
                let x = manhattan - (sensor_y.abs_diff(self.part_1_row)) as isize;
                covered_ranges.push((sensor_x - x, sensor_x + x));
            }
        }

        covered_ranges.sort();

        let (covered, _) =
            covered_ranges
                .into_iter()
                .fold((0, isize::MIN), |(count, last_max), (min, max)| {
                    if last_max + 1 >= max {
                        return (count, last_max);
                    }

                    let range = min.max(last_max + 1)..=max;

                    let beacons_in_range = self
                        .beacons
                        .iter()
                        .filter(|&&(x, y)| y == self.part_1_row && range.contains(&x))
                        .count();
                    (count + range.count() - beacons_in_range, max)
                });

        covered
    }

    pub fn part_2(&self) -> usize {
        let search_range = 0..=(self.part_1_row * 2);

        let mut covered_ranges = Vec::new();
        for y in search_range {
            covered_ranges.clear();

            for (&(sensor_x, sensor_y), &manhattan) in &self.sensors_to_manhattan {
                if (sensor_y - manhattan..=sensor_y + manhattan).contains(&y) {
                    let x = manhattan - (sensor_y.abs_diff(y)) as isize;
                    covered_ranges.push((sensor_x - x, sensor_x + x));
                }
            }

            covered_ranges.sort();

            let mut last_max = 0;
            for &(min, max) in &covered_ranges {
                if last_max + 1 >= max {
                    continue;
                }

                if last_max == min - 2 {
                    return (((min - 1) * 4_000_000) + y) as usize;
                }

                last_max = max;
            }
        }

        unreachable!();
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
                expected: 26,
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
                expected: 56000011,
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
            "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3",
            Input {
                sensors_to_manhattan: HashMap::from([
                    ((2, 18), 7),
                    ((9, 16), 1),
                    ((13, 2), 3),
                    ((12, 14), 4),
                    ((10, 20), 4),
                    ((14, 17), 5),
                    ((8, 7), 9),
                    ((2, 0), 10),
                    ((0, 11), 3),
                    ((20, 14), 8),
                    ((17, 20), 6),
                    ((16, 7), 5),
                    ((14, 3), 1),
                    ((20, 1), 7),
                ]),
                beacons: HashSet::from([(-2, 15), (10, 16), (15, 3), (2, 10), (25, 17), (21, 22)]),
                part_1_row: 10,
            },
        )
    }
}
