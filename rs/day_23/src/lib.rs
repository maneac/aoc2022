use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
    path::Path,
};

pub const PART_1: usize = 3766;
pub const PART_2: usize = 954;

pub fn read_data(data_dir: &str) -> String {
    read_to_string(Path::new(data_dir).join("day_23.txt"))
        .unwrap()
        .trim()
        .to_string()
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Input {
    elves: HashMap<usize, (isize, isize)>,
    locations: HashSet<(isize, isize)>,
}

impl Input {
    pub fn from_data(data: &str) -> Self {
        let mut elves = HashMap::new();
        let mut locations = HashSet::new();
        for (y, line) in data.lines().enumerate() {
            let y = y as isize;
            for (x, char) in line.char_indices() {
                let x = x as isize;
                if char != '#' {
                    continue;
                }
                elves.insert(elves.len(), (x, y));
                locations.insert((x, y));
            }
        }
        Self { elves, locations }
    }

    pub fn part_1(&self) -> usize {
        let mut elves = self.elves.clone();
        let mut locations = self.locations.clone();

        let mut considerations = [
            // North
            ([(-1, -1), (0, -1), (1, -1)], (0, -1)),
            // South
            ([(-1, 1), (0, 1), (1, 1)], (0, 1)),
            // West
            ([(-1, -1), (-1, 0), (-1, 1)], (-1, 0)),
            // East
            ([(1, -1), (1, 0), (1, 1)], (1, 0)),
        ];

        for _ in 0..10 {
            let mut proposals = HashMap::<(isize, isize), Vec<usize>>::new();

            'elves: for (&elf_id, &location) in &elves {
                let (e_x, e_y) = location;

                // No neighbours
                if ![
                    (e_x, e_y - 1),
                    (e_x + 1, e_y - 1),
                    (e_x + 1, e_y),
                    (e_x + 1, e_y + 1),
                    (e_x, e_y + 1),
                    (e_x - 1, e_y + 1),
                    (e_x - 1, e_y),
                    (e_x - 1, e_y - 1),
                ]
                .iter()
                .any(|p| locations.contains(p))
                {
                    continue;
                }

                for (loc_mods, (m_x, m_y)) in considerations {
                    if loc_mods
                        .iter()
                        .all(|(x, y)| !locations.contains(&(e_x + x, e_y + y)))
                    {
                        if let Some(ids) = proposals.get_mut(&(e_x + m_x, e_y + m_y)) {
                            ids.push(elf_id);
                        } else {
                            proposals.insert((e_x + m_x, e_y + m_y), vec![elf_id]);
                        }
                        continue 'elves;
                    }
                }
            }

            for (proposal, elf_ids) in proposals {
                if elf_ids.len() != 1 {
                    continue;
                }
                let elf_id = elf_ids[0];
                let cur_loc = elves.insert(elf_id, proposal).unwrap();
                locations.remove(&cur_loc);
                locations.insert(proposal);
            }

            considerations.rotate_left(1);
        }

        let ((min_x, max_x), (min_y, max_y)) = locations.iter().fold(
            ((isize::MAX, 0isize), (isize::MAX, 0isize)),
            |((min_x, max_x), (min_y, max_y)), &(x, y)| {
                ((min_x.min(x), max_x.max(x)), (min_y.min(y), max_y.max(y)))
            },
        );

        let total = (max_x - min_x + 1) * (max_y - min_y + 1);

        total as usize - locations.len()
    }

    pub fn part_2(&self) -> usize {
        let mut elves = self.elves.clone();
        let mut locations = self.locations.clone();

        let mut considerations = [
            // North
            ([(-1, -1), (0, -1), (1, -1)], (0, -1)),
            // South
            ([(-1, 1), (0, 1), (1, 1)], (0, 1)),
            // West
            ([(-1, -1), (-1, 0), (-1, 1)], (-1, 0)),
            // East
            ([(1, -1), (1, 0), (1, 1)], (1, 0)),
        ];

        for round in 1.. {
            let mut proposals = HashMap::<(isize, isize), Vec<usize>>::new();

            'elves: for (&elf_id, &location) in &elves {
                let (e_x, e_y) = location;

                // No neighbours
                if ![
                    (e_x, e_y - 1),
                    (e_x + 1, e_y - 1),
                    (e_x + 1, e_y),
                    (e_x + 1, e_y + 1),
                    (e_x, e_y + 1),
                    (e_x - 1, e_y + 1),
                    (e_x - 1, e_y),
                    (e_x - 1, e_y - 1),
                ]
                .iter()
                .any(|p| locations.contains(p))
                {
                    continue;
                }

                for (loc_mods, (m_x, m_y)) in considerations {
                    if loc_mods
                        .iter()
                        .all(|(x, y)| !locations.contains(&(e_x + x, e_y + y)))
                    {
                        if let Some(ids) = proposals.get_mut(&(e_x + m_x, e_y + m_y)) {
                            ids.push(elf_id);
                        } else {
                            proposals.insert((e_x + m_x, e_y + m_y), vec![elf_id]);
                        }
                        continue 'elves;
                    }
                }
            }

            let mut moved = false;
            for (proposal, elf_ids) in proposals {
                if elf_ids.len() != 1 {
                    continue;
                }
                let elf_id = elf_ids[0];
                let cur_loc = elves.insert(elf_id, proposal).unwrap();
                locations.remove(&cur_loc);
                locations.insert(proposal);
                moved = true;
            }
            if !moved {
                return round;
            }

            considerations.rotate_left(1);
        }

        let ((min_x, max_x), (min_y, max_y)) = locations.iter().fold(
            ((isize::MAX, 0isize), (isize::MAX, 0isize)),
            |((min_x, max_x), (min_y, max_y)), &(x, y)| {
                ((min_x.min(x), max_x.max(x)), (min_y.min(y), max_y.max(y)))
            },
        );

        let total = (max_x - min_x + 1) * (max_y - min_y + 1);

        total as usize - locations.len()
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
                expected: 110,
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
                expected: 20,
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
            "....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..",
            Input {
                elves: HashMap::from([
                    (0, (4, 0)),
                    (1, (2, 1)),
                    (2, (3, 1)),
                    (3, (4, 1)),
                    (4, (6, 1)),
                    (5, (0, 2)),
                    (6, (4, 2)),
                    (7, (6, 2)),
                    (8, (1, 3)),
                    (9, (5, 3)),
                    (10, (6, 3)),
                    (11, (0, 4)),
                    (12, (2, 4)),
                    (13, (3, 4)),
                    (14, (4, 4)),
                    (15, (0, 5)),
                    (16, (1, 5)),
                    (17, (3, 5)),
                    (18, (5, 5)),
                    (19, (6, 5)),
                    (20, (1, 6)),
                    (21, (4, 6)),
                ]),
                locations: HashSet::from([
                    (4, 0),
                    (2, 1),
                    (3, 1),
                    (4, 1),
                    (6, 1),
                    (0, 2),
                    (4, 2),
                    (6, 2),
                    (1, 3),
                    (5, 3),
                    (6, 3),
                    (0, 4),
                    (2, 4),
                    (3, 4),
                    (4, 4),
                    (0, 5),
                    (1, 5),
                    (3, 5),
                    (5, 5),
                    (6, 5),
                    (1, 6),
                    (4, 6),
                ]),
            },
        )
    }
}
