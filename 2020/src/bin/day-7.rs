use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("day-7.input");

fn main() {
    let mut contains = HashMap::new();
    let mut contained_by = HashMap::new();
    for line in INPUT.lines() {
        let mut it = line.splitn(2, " bags contain ");
        let key = it.next().unwrap();
        let contents = it.next().unwrap();
        let bags = if contents == "no other bags." {
            vec![]
        } else {
            contents
                .split(", ")
                .map(|bag| {
                    let mut it = bag.splitn(2, ' ');
                    let n = it.next().unwrap().parse().unwrap();
                    let bag = it.next().unwrap().split(" bag").next().unwrap();
                    contained_by.entry(bag).or_insert(vec![]).push(key);
                    (n, bag)
                })
                .collect()
        };
        contains.insert(key, bags);
    }

    println!("part 1: {}", part_1(&contained_by));
    println!("part 2: {}", part_2(&contains));
}

fn containers<'a>(
    outer: &mut HashSet<&'a str>,
    map: &HashMap<&'a str, Vec<&'a str>>,
    bag: &'a str,
) {
    outer.insert(bag);
    if let Some(v) = map.get(bag) {
        for &i in v.iter() {
            containers(outer, map, i);
        }
    }
}

fn part_1(map: &HashMap<&str, Vec<&str>>) -> usize {
    let mut outer = HashSet::new();
    containers(&mut outer, map, "shiny gold");
    outer.len() - 1
}

fn contained(map: &HashMap<&str, Vec<(usize, &str)>>, bag: &str) -> usize {
    let mut total = 0;
    for &(n, sub) in map[bag].iter() {
        total += n * (1 + contained(map, sub));
    }
    total
}

fn part_2(map: &HashMap<&str, Vec<(usize, &str)>>) -> usize {
    contained(map, "shiny gold")
}
