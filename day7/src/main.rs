use std::{convert::From, vec};

const TOTAL_AVAILABLE: u32 = 70000000;
const TARGET_AVAILABLE: u32 = 30000000;

#[derive(Debug, Clone)]
enum Cmd {
    CD { dir: String },
    LS,
}

#[derive(Debug, Clone)]
struct Exec<'a> {
    cmd: Cmd,
    output: Vec<&'a str>,
}

struct File {
    size: u32,
}

impl From<&str> for File {
    fn from(s: &str) -> Self {
        let words: Vec<&str> = s.split_whitespace().collect();
        let size = words[0].parse::<u32>().unwrap();
        return Self { size };
    }
}

struct Dir {
    files: Vec<File>,
    children: Vec<Dir>,
}

impl Dir {
    fn new() -> Self {
        Self {
            files: vec![],
            children: vec![],
        }
    }

    fn size(&self) -> u32 {
        let file_sum: u32 = self.files.iter().map(|file| file.size).sum();
        let dir_sum: u32 = self.children.iter().map(|child| child.size()).sum();
        file_sum + dir_sum
    }
}

fn flatten_fs(root: &Dir) -> Vec<&Dir> {
    let mut v: Vec<&Dir> = vec![root];
    for child in root.children.iter() {
        v.append(&mut flatten_fs(child))
    }
    return v;
}

fn walk_rec<'a>(cmds: &mut impl Iterator<Item = Exec<'a>>) -> Dir {
    let mut cwd = Dir::new();
    while let Some(Exec { cmd, output }) = cmds.next() {
        match cmd {
            Cmd::CD { dir } => {
                if dir == ".." {
                    return cwd;
                } else {
                    let child = walk_rec(cmds);
                    cwd.children.push(child);
                }
            }
            Cmd::LS => {
                let files: Vec<File> = output
                    .iter()
                    .filter(|line| !line.starts_with("dir"))
                    .map(|line| File::from(*line))
                    .collect();
                cwd.files = files;
                println!("{:#?}", output);
            }
        }
    }
    return cwd;
}

impl From<&str> for Cmd {
    fn from(s: &str) -> Self {
        let mut words = s.split_whitespace().into_iter();
        let cmd = words.next().unwrap();
        let args = words.collect::<Vec<&str>>();

        match cmd {
            "cd" => Cmd::CD {
                dir: args[0].to_string(),
            },
            "ls" => Cmd::LS,
            cmd => panic!("unknown command {}", cmd),
        }
    }
}

fn main() {
    let input: String = std::fs::read_to_string("input.txt").unwrap();
    let execs: Vec<Exec> = input
        .split("$")
        .filter(|split| !split.is_empty())
        .map(|split| {
            println!("{:#?}", split);
            let lines = split.lines().collect::<Vec<&str>>();
            let cmd = Cmd::from(lines[0]);
            return (cmd, lines[1..].to_vec());
        })
        .map(|(cmd, output)| Exec { cmd, output })
        .collect();

    let root = walk_rec(&mut execs.into_iter());
    let sum: u32 = flatten_fs(&root)
        .iter()
        .map(|dir| dir.size())
        .filter(|size| size <= &100000u32)
        .sum();
    println!("Part1: {sum}");

    let root_size = root.size();

    let min_delete_requirement = TARGET_AVAILABLE - (TOTAL_AVAILABLE - root_size);
    let mut available_targets: Vec<&Dir> = flatten_fs(&root)
        .into_iter()
        .filter(|dir| dir.size() >= min_delete_requirement)
        .collect();
    available_targets.sort_by_key(|dir| dir.size());
    let deletion_target = available_targets[0];
    println!("Part2: {}", deletion_target.size())
}
