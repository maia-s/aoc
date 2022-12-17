use std::{
    collections::{HashMap, HashSet, VecDeque, BinaryHeap},
    error::Error,
    fmt::{Debug, Display},
    str::FromStr, rc::Rc, hash::Hash,
};

const INPUT: &str = include_str!("day-16.txt");

#[cfg(test)]
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
        q.push_back((30, 0, 0, 0, ValveId::from_str("AA")?, HashSet::new()));
        'step: while let Some((mut time, mut delay, mut fr, fra, id, seen)) = q.pop_front() {
            loop {
                fr += fra;
                max = max.max(fr);
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
                q.push_back((time, td - 1 + (fro != 0) as isize, fr, fra+fro, tid, seen.clone()));
            }
        }
        Ok(max)
    }

    part2 isize {
        let mut max = 0;
        let mut q = BinaryHeap::new();
        let aa = ValveId::from_str("AA")?;
        q.push(Node(26, 0, 0, 0, 0, aa, aa, Rc::new(HashSet::new())));
        'step: while let Some(Node(mut time, mut d1, mut d2, mut fr, fra, id1, id2, seen)) = q.pop() {
            let (do1, do2) = loop {
                fr += fra;
                max = max.max(fr);
                time -= 1;
                if time == 0 {
                    continue 'step;
                }
                if d1 > 0 && d2 > 0 {
                    d1 -= 1;
                    d2 -= 1;
                } else if d1 > 0 {
                    d1 -= 1;
                    break (false, true);
                } else if d2 > 0 {
                    d2 -= 1;
                    break (true, false);
                } else {
                    break (true, true);
                }
            };

            let (do1, do2) = (
                do1 && !seen.contains(&id1),
                do2 && !seen.contains(&id2)
            );

            if !do1 && !do2 {
                max = max.max(fr + fra * time);
                continue;
            }

            let mut seen = (*seen).clone();
            let mut fro1 = 0;
            let mut fro2 = 0;

            if do1 {
                seen.insert(id1);
                fro1 = self.valves[&id1].flow_rate;
            }

            if do2 {
                seen.insert(id2);
                fro2 = self.valves[&id2].flow_rate;
            }

            let seen = Rc::new(seen);

            if do1 && !do2 {
                for (&tid, &td) in self.valves[&id1].tunnels.iter() {
                    let d1 = td - 1 + (fro1 != 0) as isize;
                    if tid != id2 || d1 != d2 {
                        q.push(Node(time, d1, d2, fr, fra + fro1, tid, id2, Rc::clone(&seen)));
                    }
                }
            } else if do2 && !do1 {
                for (&tid, &td) in self.valves[&id2].tunnels.iter() {
                    let d2 = td - 1 + (fro2 != 0) as isize;
                    if tid != id1 || d1 != d2 {
                        q.push(Node(time, d1, d2, fr, fra + fro2, id1, tid, Rc::clone(&seen)));
                    }
                }
            } else {
                for (&tid1, &td1) in self.valves[&id1].tunnels.iter() {
                    for (&tid2, &td2) in self.valves[&id2].tunnels.iter() {
                        let d1 = td1 - 1 + (fro1 != 0) as isize;
                        let d2 = td2 - 1 + (fro2 != 0) as isize;
                        if tid1 != tid2 || d1 != d2 {
                            q.push(Node(time, d1, d2,
                                fr, fra + fro1 + fro2, tid1, tid2, Rc::clone(&seen))
                            );
                        }
                    }
                }
            }
        }
        Ok(max)
    }

    input = INPUT;
    test day16_ex(INPUT_EX, 1651, 1707);
    test day16(INPUT, 2087 /* ,2591 */);
}

#[derive(PartialEq, Eq)]
struct Node(isize, isize, isize, isize, isize, ValveId, ValveId, Rc<HashSet<ValveId>>);

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.0.cmp(&self.0)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
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
