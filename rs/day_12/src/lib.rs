use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    fs::read_to_string,
    path::Path,
};

pub const PART_1: usize = 504;
pub const PART_2: usize = 500;

pub fn read_data(data_dir: &str) -> String {
    read_to_string(Path::new(data_dir).join("day_12.txt"))
        .unwrap()
        .trim()
        .to_string()
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Input {
    start: (usize, usize),
    end: (usize, usize),
    grid: Vec<Vec<u8>>,
    nodes_to_neighbours: HashMap<(usize, usize), Vec<(usize, usize)>>,
}

impl Input {
    pub fn from_data(data: &str) -> Self {
        let mut output = Self {
            start: (0, 0),
            end: (0, 0),
            grid: Vec::with_capacity(data.len()),
            nodes_to_neighbours: HashMap::with_capacity(data.len()),
        };

        for (y, line) in data.lines().enumerate() {
            let mut row = Vec::with_capacity(line.len());
            for (x, chr) in line.chars().enumerate() {
                let chr = match chr {
                    'S' => {
                        output.start = (x, y);
                        'a'
                    }
                    'E' => {
                        output.end = (x, y);
                        'z'
                    }
                    c => c,
                };
                let height = chr as u8 - b'a';

                if !output.grid.is_empty() {
                    let above = *output.grid.last().unwrap().get(x).unwrap();
                    if height >= above || above - height < 2 {
                        output
                            .nodes_to_neighbours
                            .entry((x, y))
                            .and_modify(|edge_list| edge_list.push((x, y - 1)))
                            .or_insert_with(|| vec![(x, y - 1)]);
                    }
                    if above >= height || height - above < 2 {
                        output
                            .nodes_to_neighbours
                            .entry((x, y - 1))
                            .and_modify(|edge_list| edge_list.push((x, y)))
                            .or_insert_with(|| vec![(x, y)]);
                    }
                }

                if !row.is_empty() {
                    let left = *row.last().unwrap();
                    if height >= left || left - height < 2 {
                        output
                            .nodes_to_neighbours
                            .entry((x, y))
                            .and_modify(|edge_list| edge_list.push((x - 1, y)))
                            .or_insert_with(|| vec![(x - 1, y)]);
                    }
                    if left >= height || height - left < 2 {
                        output
                            .nodes_to_neighbours
                            .entry((x - 1, y))
                            .and_modify(|edge_list| edge_list.push((x, y)))
                            .or_insert_with(|| vec![(x, y)]);
                    }
                }

                row.push(height);
            }
            output.grid.push(row);
        }

        output
    }

    pub fn part_1(&self) -> usize {
        self.shortest_path_to_destination(self.start).unwrap()
    }

    pub fn part_2(&self) -> usize {
        self.grid
            .iter()
            .enumerate()
            .flat_map(|(y, v)| {
                v.iter()
                    .enumerate()
                    .filter(|(_, v)| v.eq(&&0))
                    .map(move |(x, _)| {
                        self.shortest_path_to_destination((x, y))
                            .unwrap_or(usize::MAX)
                    })
            })
            .min()
            .unwrap()
    }

    fn shortest_path_to_destination(&self, start: (usize, usize)) -> Option<usize> {
        let mut distances = HashMap::with_capacity(self.nodes_to_neighbours.len());
        let mut shortest_distance_queue = BinaryHeap::with_capacity(self.nodes_to_neighbours.len());

        distances.insert(start, 0);

        for &neighbour in self.nodes_to_neighbours.get(&start).unwrap() {
            distances.insert(neighbour, 1);
            shortest_distance_queue.push(Reverse((1, neighbour)))
        }

        while let Some(Reverse((distance_from_start, node))) = shortest_distance_queue.pop() {
            if distance_from_start.ne(distances.get(&node).unwrap()) {
                continue;
            }

            if node == self.end {
                return Some(distance_from_start);
            }

            let new_distance = distance_from_start + 1;

            for &neighbour in self.nodes_to_neighbours.get(&node).unwrap() {
                let existing_distance = distances.get(&neighbour).copied().unwrap_or(usize::MAX);

                if existing_distance <= new_distance {
                    continue;
                }

                distances.insert(neighbour, new_distance);
                shortest_distance_queue.push(Reverse((new_distance, neighbour)))
            }
        }

        None
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
                expected: 31,
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
                expected: 29,
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
            "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi",
            #[allow(clippy::eq_op)]
            Input {
                start: (0, 0),
                end: (5, 2),
                grid: vec![
                    vec![
                        b'a' - b'a', // S
                        b'a' - b'a',
                        b'b' - b'a',
                        b'q' - b'a',
                        b'p' - b'a',
                        b'o' - b'a',
                        b'n' - b'a',
                        b'm' - b'a',
                    ],
                    vec![
                        b'a' - b'a',
                        b'b' - b'a',
                        b'c' - b'a',
                        b'r' - b'a',
                        b'y' - b'a',
                        b'x' - b'a',
                        b'x' - b'a',
                        b'l' - b'a',
                    ],
                    vec![
                        b'a' - b'a',
                        b'c' - b'a',
                        b'c' - b'a',
                        b's' - b'a',
                        b'z' - b'a',
                        b'z' - b'a', // E
                        b'x' - b'a',
                        b'k' - b'a',
                    ],
                    vec![
                        b'a' - b'a',
                        b'c' - b'a',
                        b'c' - b'a',
                        b't' - b'a',
                        b'u' - b'a',
                        b'v' - b'a',
                        b'w' - b'a',
                        b'j' - b'a',
                    ],
                    vec![
                        b'a' - b'a',
                        b'b' - b'a',
                        b'd' - b'a',
                        b'e' - b'a',
                        b'f' - b'a',
                        b'g' - b'a',
                        b'h' - b'a',
                        b'i' - b'a',
                    ],
                ],
                nodes_to_neighbours: HashMap::from([
                    ((0, 0), vec![(1, 0), (0, 1)]),
                    ((1, 0), vec![(0, 0), (2, 0), (1, 1)]),
                    ((2, 0), vec![(1, 0), (2, 1)]),
                    ((3, 0), vec![(2, 0), (4, 0), (3, 1)]),
                    ((4, 0), vec![(3, 0), (5, 0)]),
                    ((5, 0), vec![(4, 0), (6, 0)]),
                    ((6, 0), vec![(5, 0), (7, 0)]),
                    ((7, 0), vec![(6, 0), (7, 1)]),
                    ((0, 1), vec![(0, 0), (1, 1), (0, 2)]),
                    ((1, 1), vec![(1, 0), (0, 1), (2, 1), (1, 2)]),
                    ((2, 1), vec![(2, 0), (1, 1), (2, 2)]),
                    ((3, 1), vec![(3, 0), (2, 1), (3, 2)]),
                    ((4, 1), vec![(4, 0), (3, 1), (5, 1), (4, 2)]),
                    ((5, 1), vec![(5, 0), (4, 1), (6, 1)]),
                    ((6, 1), vec![(6, 0), (5, 1), (7, 1), (6, 2)]),
                    ((7, 1), vec![(7, 0), (7, 2)]),
                    ((0, 2), vec![(0, 1), (0, 3)]),
                    ((1, 2), vec![(1, 1), (0, 2), (2, 2), (1, 3)]),
                    ((2, 2), vec![(2, 1), (1, 2), (2, 3)]),
                    ((3, 2), vec![(3, 1), (2, 2), (3, 3)]),
                    ((4, 2), vec![(4, 1), (3, 2), (5, 2), (4, 3)]),
                    ((5, 2), vec![(5, 1), (4, 2), (6, 2), (5, 3)]),
                    ((6, 2), vec![(6, 1), (7, 2), (6, 3)]),
                    ((7, 2), vec![(7, 1), (7, 3)]),
                    ((0, 3), vec![(0, 2), (0, 4)]),
                    ((1, 3), vec![(1, 2), (0, 3), (2, 3), (1, 4)]),
                    ((2, 3), vec![(2, 2), (1, 3), (2, 4)]),
                    ((3, 3), vec![(3, 2), (2, 3), (4, 3), (3, 4)]),
                    ((4, 3), vec![(3, 3), (5, 3), (4, 4)]),
                    ((5, 3), vec![(4, 3), (6, 3), (5, 4)]),
                    ((6, 3), vec![(6, 2), (5, 3), (7, 3), (6, 4)]),
                    ((7, 3), vec![(7, 2), (7, 4)]),
                    ((0, 4), vec![(0, 3), (1, 4)]),
                    ((1, 4), vec![(1, 3), (0, 4)]),
                    ((2, 4), vec![(2, 3), (1, 4), (3, 4)]),
                    ((3, 4), vec![(2, 4), (4, 4)]),
                    ((4, 4), vec![(3, 4), (5, 4)]),
                    ((5, 4), vec![(4, 4), (6, 4)]),
                    ((6, 4), vec![(5, 4), (7, 4)]),
                    ((7, 4), vec![(7, 3), (6, 4)]),
                ]),
            },
        )
    }
}
