use std::{
    collections::{HashMap, HashSet, VecDeque},
    error::Error,
    fmt::{Display, Debug},
    str::FromStr,
};

const INPUT: &str = include_str!("day-16.txt");

const INPUT_EX: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

aoc_2022::aoc! {
    struct Day16 {
        valves: HashMap<ValveId, Valve>,
    }

    self(input) {
        let mut valves: HashMap<ValveId, Valve> = input.lines()
            .map(|line| line.parse::<Valve>().map(|v| (v.id, v)))
            .collect::<Result<_,_>>()?;
        let vlu = valves.clone();
        for valve in valves.values_mut() {
            valve.connect(&vlu);
        }
        Ok(Self { valves })
    }

    part1 isize {
        let mut max = 0;
        let mut q = VecDeque::new();
        q.push_back((30, 0, 0, 0, 0, ValveId::from_str("AA")?, HashSet::new(), String::from("AA")));
        'step: while let Some((mut time, mut delay, mut fr, mut fra, mut fro, id, seen, p)) = q.pop_front() {
            loop {
                fr += fra;
                fra += fro;
                fro = 0;
                if max < fr {
                    eprintln!("{p} {max}");
                    max = max.max(fr);
                }
                time -= 1;
                if time == 0 {
                    continue 'step;
                }
                if delay > 0 {
                    delay -= 1;
                } else {
                    break;
                }
            }
            if seen.contains(&id) {
                continue;
            }
            let mut seen = seen.clone();
            seen.insert(id);
            let fro = self.valves[&id].flow_rate;
            for (&tid, &td) in self.valves[&id].tunnels.iter() {
                let p = format!("{p} -> {tid}");
                q.push_back((time, td - 1 + (fro != 0) as isize, fr, fra+fro, 0, tid, seen.clone(), p));
            }
        }
        Ok(max)
    }

    part2 usize {
        todo!()
    }

    input = INPUT;
    test day16_ex(INPUT_EX, 1651);
}

#[derive(Clone)]
struct Valve {
    id: ValveId,
    flow_rate: isize,
    tunnels: HashMap<ValveId, isize>,
}

impl Valve {
    fn connect(&mut self, map: &HashMap<ValveId, Valve>) {
        let mut seen = HashSet::new();
        seen.insert(self.id);
        let mut q = VecDeque::new();
        for (&id, &dist) in self.tunnels.iter() {
            q.push_back((id, dist, seen.clone()));
        }
        while let Some((id, dist, seen)) = q.pop_front() {
            if seen.contains(&id) {
                continue;
            }
            self.tunnels.entry(id).or_insert(dist);
            let mut seen = seen.clone();
            seen.insert(id);
            for (&id, &td) in map[&id].tunnels.iter() {
                q.push_back((id, dist + td, seen.clone()))
            }
        }
        self.tunnels.retain(|id, _| map[id].flow_rate != 0);
    }
}

impl FromStr for Valve {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.strip_prefix("Valve ").ok_or("expected `Valve `")?;
        let (id, s) = s
            .split_once(" has flow rate=")
            .ok_or("expected flow rate")?;
        let id = id.parse()?;
        let (flow_rate, s) = s.split_once("; tunnel").ok_or("expected `; tunnel`")?;
        let flow_rate = flow_rate.parse()?;
        let s = if s.starts_with('s') {
            s.strip_prefix("s lead to valves ")
        } else {
            s.strip_prefix(" leads to valve ")
        }
        .ok_or("expected `lead(s) to...`")?;
        let tunnels = s
            .split(", ")
            .map(|s| s.parse().map(|id| (id, 1)))
            .collect::<Result<_, _>>()?;
        Ok(Self {
            id,
            flow_rate,
            tunnels,
        })
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct ValveId(u16);

impl FromStr for ValveId {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 2 {
            let s = s.as_bytes();
            Ok(Self(s[0] as u16 * 256 + s[1] as u16))
        } else {
            Err("invalid id".into())
        }
    }
}

impl Display for ValveId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let b0 = char::from((self.0 / 256) as u8);
        let b1 = char::from(self.0 as u8);
        write!(f, "{b0}{b1}")
    }
}

impl Debug for ValveId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as Display>::fmt(self, f)
    }
}
