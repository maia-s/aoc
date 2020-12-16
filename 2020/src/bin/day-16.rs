use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

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
                let mut range = FieldRange::default();
                for (i, s) in it.next().unwrap().split(" or ").enumerate() {
                    let mut it = s.split('-');
                    let from = it.next().unwrap().parse().unwrap();
                    let to = it.next().unwrap().parse().unwrap();
                    range.0[i] = (from, to);
                }
                (name, range)
            })
            .collect(),
    );

    let my_ticket: Ticket = parts
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
    println!("part 2: {}", part_2(&fields, &my_ticket, &nearby_tickets));
}

fn part_1(fields: &Fields, nearby_tickets: &[Ticket]) -> usize {
    nearby_tickets
        .iter()
        .flat_map(|ticket| ticket.invalids(fields))
        .sum()
}

fn part_2(fields: &Fields, my_ticket: &Ticket, nearby_tickets: &[Ticket]) -> usize {
    let tickets = std::iter::once(my_ticket).chain(
        nearby_tickets
            .iter()
            .filter(|ticket| ticket.is_valid(fields)),
    );

    let all: HashSet<_> = (0..fields.0.len()).collect();

    let mut possibilities: HashMap<_, _> = fields
        .0
        .iter()
        .map(|(&name, ranges)| {
            let mut set = all.clone();
            for ticket in tickets.clone() {
                for (i, n) in ticket.0.iter().enumerate() {
                    if !ranges.contains(n) {
                        set.remove(&i);
                    }
                }
            }
            (name, set)
        })
        .collect();

    let mut product = 1;
    let mut remove = vec![];
    loop {
        for (&name, set) in possibilities.iter_mut().filter(|(_, s)| s.len() == 1) {
            let n = set.drain().next().unwrap();
            if name.starts_with("departure") {
                product *= my_ticket.0[n];
            }
            remove.push(n);
        }
        if remove.is_empty() {
            break;
        }
        for i in remove.drain(..) {
            for set in possibilities.values_mut() {
                set.remove(&i);
            }
        }
    }

    product
}

struct Fields<'a>(HashMap<&'a str, FieldRange>);

impl Fields<'_> {
    fn contains(&self, n: &usize) -> bool {
        self.0.values().any(|range| range.contains(n))
    }
}

#[derive(Debug, Default)]
struct FieldRange([(usize, usize); 2]);

impl FieldRange {
    fn contains(&self, n: &usize) -> bool {
        self.0.iter().any(|&(from, to)| (from..=to).contains(n))
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
    fn is_valid(&self, fields: &Fields) -> bool {
        self.0.iter().all(|n| fields.contains(n))
    }

    fn invalids(&self, fields: &Fields) -> Vec<usize> {
        self.0
            .iter()
            .filter_map(|n| if fields.contains(n) { None } else { Some(*n) })
            .collect()
    }
}
