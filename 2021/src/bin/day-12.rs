use std::collections::{hash_map::Entry, HashMap, HashSet};

const INPUT: &str = include_str!("day-12.input");

#[derive(Default)]
struct System(HashMap<String, Cave>);

impl System {
    fn add_path(&mut self, from: &str, to: &str) {
        self.add_cave(from);
        self.add_cave(to);
        self.0.get_mut(from).unwrap().paths.insert(to.to_owned());
        self.0.get_mut(to).unwrap().paths.insert(from.to_owned());
    }

    fn add_cave(&mut self, name: &str) {
        let name = name.to_owned();
        match self.0.entry(name.clone()) {
            Entry::Occupied(_) => (),
            Entry::Vacant(e) => {
                let is_big = name != name.to_lowercase();
                e.insert(Cave {
                    name,
                    paths: HashSet::new(),
                    is_big,
                });
            }
        }
    }

    fn walks(&self, allow_twice: bool) -> usize {
        fn walks(
            map: &HashMap<String, Cave>,
            name: &str,
            mut seen: HashSet<String>,
            mut allow_twice: bool,
        ) -> usize {
            if name == "end" {
                return 1;
            }
            let cave = map.get(name).unwrap();
            if !cave.is_big && !seen.insert(cave.name.clone()) {
                if !allow_twice || name == "start" {
                    return 0;
                } else {
                    allow_twice = false;
                }
            }
            cave.paths
                .iter()
                .map(|path| walks(map, path, seen.clone(), allow_twice))
                .sum()
        }
        walks(&self.0, "start", HashSet::new(), allow_twice)
    }
}

struct Cave {
    name: String,
    paths: HashSet<String>,
    is_big: bool,
}

fn main() {
    let mut system = System::default();

    for (from, to) in INPUT.lines().map(|line| line.split_once('-').unwrap()) {
        system.add_path(from.trim(), to.trim());
    }

    println!("part 1: {}", system.walks(false));
    println!("part 2: {}", system.walks(true));
}
