use std::{fs::read_to_string, path::Path};

pub const PART_1: usize = 1807;
pub const PART_2: usize = 480000;

pub fn read_data(data_dir: &str) -> String {
    read_to_string(Path::new(data_dir).join("day_08.txt"))
        .unwrap()
        .trim()
        .to_string()
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Input {
    rows: Vec<Vec<u8>>,
    cols: Vec<Vec<u8>>,
}

impl Input {
    pub fn from_data(data: &str) -> Self {
        let mut rows = Vec::new();
        let mut cols = Vec::new();
        for line in data.lines() {
            if cols.is_empty() {
                cols.resize(line.len(), Vec::new());
            }
            let row = line.bytes().enumerate().fold(
                Vec::with_capacity(line.len()),
                |mut acc, (idx, byte)| {
                    let height = byte - b'0';
                    cols.get_mut(idx).unwrap().push(height);
                    acc.push(height);
                    acc
                },
            );
            rows.push(row);
        }
        Input { rows, cols }
    }

    pub fn part_1(&self) -> usize {
        let mut grid = vec![vec![false; self.rows[0].len()]; self.rows.len()];

        // Horizontal visibility.
        for (row, vis) in self.rows.iter().zip(grid.iter_mut()) {
            // Left -> Right.
            let mut max = None;
            for (idx, &height) in row.iter().enumerate() {
                if match max {
                    None => true,
                    Some(max_height) => height > max_height,
                } {
                    *vis.get_mut(idx).unwrap() = true;
                }
                max = Some(max.unwrap_or_default().max(height))
            }

            // Right -> Left.
            let mut max = None;
            for (idx, &height) in row.iter().enumerate().rev() {
                if match max {
                    None => true,
                    Some(max_height) => height > max_height,
                } {
                    *vis.get_mut(idx).unwrap() = true;
                }
                max = Some(max.unwrap_or_default().max(height))
            }
        }

        // Vertical visibility.
        for (col_idx, col) in self.cols.iter().enumerate() {
            // Top -> Bottom.
            let mut max = None;
            for (idx, &height) in col.iter().enumerate() {
                if match max {
                    None => true,
                    Some(max_height) => height > max_height,
                } {
                    *grid.get_mut(idx).unwrap().get_mut(col_idx).unwrap() = true;
                }

                max = Some(max.unwrap_or_default().max(height))
            }

            // Bottom -> Top.
            let mut max = None;
            for (idx, &height) in col.iter().enumerate().rev() {
                if match max {
                    None => true,
                    Some(max_height) => height > max_height,
                } {
                    *grid.get_mut(idx).unwrap().get_mut(col_idx).unwrap() = true;
                }
                max = Some(max.unwrap_or_default().max(height))
            }
        }

        grid.iter().flatten().filter(|&&v| v).count()
    }

    pub fn part_2(&self) -> usize {
        let mut max_visibility = 0;
        for (x, row) in self
            .rows
            .iter()
            .enumerate()
            .skip(1)
            .take(self.rows.len() - 2)
        {
            for (y, col) in self
                .cols
                .iter()
                .enumerate()
                .skip(1)
                .take(self.cols.len() - 2)
            {
                let house_height = row.get(y).copied().unwrap();

                let l_to_r = row
                    .iter()
                    .skip(y + 1)
                    .position(|&height| height >= house_height)
                    .map(|pos| pos + 1)
                    .unwrap_or(row.len() - y - 1);

                let r_to_l = row
                    .iter()
                    .take(y)
                    .rev()
                    .position(|&height| height >= house_height)
                    .map(|pos| pos + 1)
                    .unwrap_or(y);

                let t_to_b = col
                    .iter()
                    .skip(x + 1)
                    .position(|&height| height >= house_height)
                    .map(|pos| pos + 1)
                    .unwrap_or(col.len() - x - 1);

                let b_to_t = col
                    .iter()
                    .take(x)
                    .rev()
                    .position(|&height| height >= house_height)
                    .map(|pos| pos + 1)
                    .unwrap_or(x);

                max_visibility = max_visibility.max(l_to_r * r_to_l * t_to_b * b_to_t);
            }
        }

        max_visibility
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
                expected: 21,
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
                expected: 8,
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
            "30373
25512
65332
33549
35390",
            Input {
                rows: vec![
                    vec![3, 0, 3, 7, 3],
                    vec![2, 5, 5, 1, 2],
                    vec![6, 5, 3, 3, 2],
                    vec![3, 3, 5, 4, 9],
                    vec![3, 5, 3, 9, 0],
                ],
                cols: vec![
                    vec![3, 2, 6, 3, 3],
                    vec![0, 5, 5, 3, 5],
                    vec![3, 5, 3, 5, 3],
                    vec![7, 1, 3, 4, 9],
                    vec![3, 2, 2, 9, 0],
                ],
            },
        )
    }
}
