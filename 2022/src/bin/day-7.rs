use std::{
    cell::RefCell,
    collections::HashMap,
    rc::{Rc, Weak},
};

const INPUT: &str = include_str!("day-7.txt");

#[cfg(test)]
const INPUT_EX: &str = "$ cd /
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
7214296 k";

aoc_2022::aoc! {
    struct Day7 {
        root: Rc<RefCell<Dir>>,
    }

    self(input) {
        let mut lines = input.lines().peekable();
        let root = Rc::new(RefCell::new(Dir::default()));
        let mut cwd = root.clone();
        while let Some(line) = lines.next() {
            let Some(line) = line.strip_prefix("$ ") else {
                return Err("expected command".into())
            };
            let mut c = line.split_ascii_whitespace();
            let Some(cmd) = c.next() else {
                return Err("missing command".into());
            };
            match cmd {
                "cd" => {
                    let Some(arg) = c.next() else {
                        return Err("missing argument for cd".into());
                    };
                    let d = match arg.trim() {
                        "/" => root.clone(),
                        ".." => cwd.borrow().up.upgrade().ok_or("missing parent")?,
                        arg => match cwd.borrow().entries.get(arg) {
                            Some(Entry::Dir(d)) => d.clone(),
                            Some(Entry::File(_)) => return Err("can't cd into file".into()),
                            None => return Err("dir not found".into()),
                        }
                    };
                    cwd = d;
                }
                "ls" => {
                    while let Some(line) = lines.peek() {
                        if line.starts_with('$') {
                            break;
                        }
                        let line = lines.next().unwrap();
                        let Some((s, name)) = line.split_once(' ') else {
                            return Err("invalid ls format".into());
                        };
                        let name = name.to_owned();
                        if s == "dir" {
                            let d = Dir {
                                entries: HashMap::new(),
                                up: Rc::downgrade(&cwd),
                                size: 0,
                            };
                            cwd.borrow_mut().entries.insert(name, Entry::Dir(Rc::new(RefCell::new(d))));
                        } else {
                            let size = s.parse::<usize>()?;
                            cwd.borrow_mut().entries.insert(name, Entry::File(size));
                        }
                    }
                }
                _ => return Err("unknown command".into())
            }
        }
        root.borrow_mut().calc_size();
        Ok(Self { root })
    }

    part1 usize {
        let mut sizes = Vec::new();
        self.root.borrow().get_sizes(&mut sizes, &|x| x <= 100_000);
        Ok(sizes.into_iter().sum())
    }

    part2 usize {
        let need = 30_000_000 - (70_000_000 - self.root.borrow().size);
        let mut sizes = Vec::new();
        self.root.borrow().get_sizes(&mut sizes, &|x| x >= need);
        sizes.sort_unstable();
        Ok(sizes[0])
    }

    input = INPUT;
    test day7_ex(INPUT_EX, 95437, 24933642);
    test day7(INPUT, 1778099, 1623571);
}

#[derive(Default)]
struct Dir {
    entries: HashMap<String, Entry>,
    up: Weak<RefCell<Dir>>,
    size: usize,
}

impl Dir {
    fn calc_size(&mut self) -> usize {
        let size = self
            .entries
            .values()
            .map(|e| match e {
                Entry::Dir(d) => d.borrow_mut().calc_size(),
                Entry::File(s) => *s,
            })
            .sum();
        self.size = size;
        size
    }

    fn get_sizes(&self, v: &mut Vec<usize>, p: &impl Fn(usize) -> bool) {
        if p(self.size) {
            v.push(self.size);
        }
        for e in self.entries.values() {
            if let Entry::Dir(d) = e {
                d.borrow().get_sizes(v, p)
            }
        }
    }
}

enum Entry {
    Dir(Rc<RefCell<Dir>>),
    File(usize),
}
