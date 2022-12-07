use std::collections::HashMap;

fn main() {
    let input = include_str!("./data.txt");
    let cmds = Cmd::parse_output(input);
    let root = build_from_cmd(cmds);

    part1and2(&root);
}

fn part1and2(root: &Dir) {
    let mut sizes: HashMap<String, usize> = HashMap::new();

    fn traverse(parent: &str, dir: &Dir, sizes: &mut HashMap<String, usize>) -> usize {
        let my_path = {
            let mut p = parent.to_owned();
            p.push('/');
            p.push_str(&dir.name);
            p
        };

        let file_size: usize = dir.files.iter().map(|f| f.size).sum();
        let dir_size: usize = dir.dirs.iter().map(|d| traverse(&my_path, d, sizes)).sum();
        let total = file_size + dir_size;

        sizes.insert(my_path, total);
        total
    }

    for child in &root.dirs {
        traverse("", child, &mut sizes);
    }

    let answer: usize = sizes.values().filter(|size| **size < 100000).sum();

    println!("Part 1 result: {answer}");

    let disk_size = 70000000;
    let required_unused_size = 30000000;

    let current_size: usize = root
        .dirs
        .iter()
        .map(|d| sizes[&format!("/{}", d.name)])
        .sum::<usize>()
        + root.files.iter().map(|f| f.size).sum::<usize>();

    let difference = required_unused_size - (disk_size - current_size);

    let mut possible_dirs = sizes
        .iter()
        .filter(|(_, size)| **size >= difference)
        .collect::<Vec<_>>();

    possible_dirs.sort_by_key(|(_, size)| *size);
    let size = possible_dirs.first().unwrap().1.clone();

    println!("Part 2 result: {size}");
}

fn build_from_cmd(cmds: Vec<Cmd>) -> Dir {
    let mut root = Dir::new("".into());

    let mut current_path: String = "".into();

    for cmd in &cmds[1..] {
        match cmd {
            Cmd::Cd(path) => {
                if path == ".." {
                    let parts = current_path.split("/").collect::<Vec<_>>();
                    current_path = parts[0..(parts.len() - 1)].join("/");
                } else {
                    current_path.push('/');
                    current_path.push_str(&path);
                }
            }
            Cmd::Ls(items) => {
                let dir = root.traverse(&current_path);

                for item in items {
                    let parts = item.split(" ").collect::<Vec<_>>();

                    if parts[0] == "dir" {
                        dir.dirs.push(Dir::new(parts[1].into()))
                    } else {
                        let size = parts[0].parse::<usize>().unwrap();
                        dir.files.push(File {
                            name: parts[1].into(),
                            size,
                        });
                    }
                }
            }
        }
    }

    root
}

#[derive(Debug)]
struct Dir {
    name: String,
    dirs: Vec<Dir>,
    files: Vec<File>,
}

impl Dir {
    pub fn new(name: String) -> Dir {
        Dir {
            name,
            dirs: vec![],
            files: vec![],
        }
    }

    pub fn traverse(&mut self, path: &str) -> &mut Dir {
        let parts = path
            .split("/")
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>();

        let mut current = self;

        for part in parts {
            match current.dirs.iter().position(|d| &d.name == &part) {
                Some(idx) => current = &mut current.dirs[idx],
                None => {
                    let dir = Dir::new(part.into());
                    current.dirs.push(dir);
                    current = current.dirs.last_mut().unwrap()
                }
            }
        }

        current
    }
}

#[derive(Debug)]
struct File {
    name: String,
    size: usize,
}

#[derive(Debug)]
enum Cmd {
    Cd(String),
    Ls(Vec<String>),
}

impl Cmd {
    fn parse_output(output: &str) -> Vec<Cmd> {
        let mut cmds = vec![];

        let mut iter = output.split("\n").filter(|s| !s.is_empty()).peekable();

        loop {
            match iter.next() {
                Some(line) => {
                    let parts = line.split(" ").collect::<Vec<_>>();
                    let cmd = parts[1];

                    match cmd {
                        "ls" => {
                            let mut output = vec![];

                            while matches!(iter.peek(), Some(item) if !item.starts_with("$")) {
                                output.push(iter.next().unwrap().into());
                            }

                            cmds.push(Cmd::Ls(output));
                        }
                        "cd" => cmds.push(Cmd::Cd(parts[2].into())),
                        _ => panic!("Unknown command encountered: {}", cmd),
                    }
                }
                None => break,
            }
        }

        cmds
    }
}
