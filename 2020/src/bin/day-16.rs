use std::{collections::HashMap, str::FromStr};

const INPUT: &str = include_str!("day-16.input");

fn main() {
    let mut parts = INPUT.split("\n\n");

    let fields = Fields(
        parts
            .next()
            .unwrap()
            .lines()
            .map(|s| {
                let mut it = s.split(": ");
                let name = it.next().unwrap();
                let mut ranges = [(0, 0); 2];
                for (i, s) in it.next().unwrap().split(" or ").enumerate() {
                    let mut it = s.split('-');
                    let from = it.next().unwrap().parse().unwrap();
                    let to = it.next().unwrap().parse().unwrap();
                    ranges[i] = (from, to);
                }
                (name, ranges)
            })
            .collect(),
    );

    let _my_ticket: Ticket = parts
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .next()
        .unwrap()
        .parse()
        .unwrap();

    let nearby_tickets: Vec<Ticket> = parts
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();

    println!("part 1: {}", part_1(&fields, &nearby_tickets));
}

fn part_1(fields: &Fields, nearby_tickets: &[Ticket]) -> usize {
    nearby_tickets
        .iter()
        .flat_map(|ticket| ticket.invalids(&fields))
        .sum()
}

struct Fields<'a>(HashMap<&'a str, [(usize, usize); 2]>);

impl Fields<'_> {
    fn contains(&self, n: &usize) -> bool {
        self.0
            .iter()
            .any(|(_, ranges)| ranges.iter().any(|&(from, to)| (from..=to).contains(n)))
    }
}

struct Ticket(Vec<usize>);

impl FromStr for Ticket {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.split(',').map(|s| s.parse().unwrap()).collect()))
    }
}

impl Ticket {
    fn invalids(&self, fields: &Fields) -> Vec<usize> {
        self.0
            .iter()
            .filter_map(|n| if fields.contains(n) { None } else { Some(*n) })
            .collect()
    }
}
