use std::{collections::HashMap, fs::read_to_string, path::Path};

pub const PART_1: usize = 1315285;
pub const PART_2: usize = 9847279;

pub fn read_data(data_dir: &str) -> String {
    read_to_string(Path::new(data_dir).join("day_07.txt"))
        .unwrap()
        .trim()
        .to_string()
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Input {
    dir_tree: HashMap<String, usize>,
}

impl Input {
    pub fn from_data(data: &str) -> Self {
        let mut dir_tree = HashMap::new();

        let cur_path = &mut Path::new("").to_path_buf();

        for command in data.split("$ ").skip(1) {
            let (cmd, remainder) = command.split_once(&[' ', '\n'][..]).unwrap();
            match cmd {
                "ls" => {
                    let dir_size = remainder
                        .trim()
                        .split_terminator('\n')
                        .fold(0, |acc, args| {
                            let lhs = args.split_ascii_whitespace().next().unwrap();
                            if lhs != "dir" {
                                acc + lhs.parse::<usize>().unwrap()
                            } else {
                                acc
                            }
                        });

                    dir_tree
                        .entry(cur_path.to_str().unwrap().to_string())
                        .and_modify(|e| *e += dir_size)
                        .or_insert(dir_size);

                    let mut path = cur_path.as_path();
                    loop {
                        path = match path.parent() {
                            Some(path) => path,
                            None => break,
                        };

                        dir_tree
                            .entry(path.to_str().unwrap().to_string())
                            .and_modify(|e| *e += dir_size)
                            .or_insert(dir_size);
                    }
                }
                _ => {
                    let args = remainder.trim();
                    match args {
                        "/" => *cur_path = Path::new("/").to_path_buf(),
                        ".." => {
                            cur_path.pop();
                        }
                        into => {
                            cur_path.push(into);
                        }
                    }
                }
            };
        }

        Self { dir_tree }
    }

    pub fn part_1(&self) -> usize {
        let size_limit = 100_000;

        self.dir_tree
            .values()
            .filter(|&&size| size < size_limit)
            .sum()
    }

    pub fn part_2(&self) -> usize {
        let target_total_space = 40_000_000;

        let current_total_space = self.dir_tree.get("/").unwrap();

        let target = current_total_space - target_total_space;

        // smallest dir larger than target
        self.dir_tree
            .values()
            .filter(|&&size| size >= target)
            .min()
            .copied()
            .unwrap()
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
                expected: 95437,
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
                expected: 24933642,
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
        let dir_tree = HashMap::from_iter(
            vec![
                ("/", 48_381_165),
                ("/a", 94_853),
                ("/a/e", 584),
                ("/d", 24_933_642),
            ]
            .iter()
            .map(|(path, size)| (path.to_string(), *size as usize)),
        );

        (
            "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k",
            Input { dir_tree },
        )
    }
}
