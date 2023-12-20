use std::collections::{HashMap, HashSet, VecDeque};

use aoc_2023::{aoc, str_block, Error};

const INPUT: &str = include_str!("day-20.txt");

#[allow(dead_code)]
const INPUT_EX: &str = str_block! {"
broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a
"};

#[allow(dead_code)]
const INPUT_EX2: &str = str_block! {"
broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output
"};

aoc! {
    struct Day20<'a> {
        system: System<'a>,
    }

    self(input = INPUT) {
        let mut modules = HashMap::new();
        let targets = input.lines().map(|line| {
            let (module, targets) = line.split_once(" -> ").ok_or("missing ` -> `")?;
            let (module, m) = if module == "broadcaster" {
                (module, Module::new(module, Broadcaster))
            } else if let Some(module) = module.strip_prefix('%') {
                (module, Module::new(module, FlipFlop::new()))
            } else if let Some(module) = module.strip_prefix('&') {
                (module, Module::new(module, Conjunction::new()))
            } else {
                (module, Module::new(module, Observer))
            };
            modules.insert(module, m);
            Ok((module, targets.split(", ")))
        }).collect::<Result<Vec<_>, Error>>()?;
        for (module, targets) in targets.into_iter() {
            for target in targets {
                let port = modules.entry(target).or_insert_with(|| Module::new(target, Observer)).alloc_port();
                modules.get_mut(module).unwrap().add_target(target, port);
            }
        }
        Ok(Self {
            system: System {
                modules,
                queue: VecDeque::new(),
            }
        })
    }

    1 part1 usize {
        let mut lows = 0;
        let mut highs = 0;
        for _ in 0..1000 {
            let (l, h, _) = self.system.push_button();
            lows += l;
            highs += h;
        }
        Ok(lows * highs)
    }

    2 part2 usize {
        for i in 1_usize.. {
            if self.system.push_button().2 {
                return Ok(i)
            }
        }
        unreachable!()
    }

    # graph {
        let mut seen = HashSet::new();
        let mut queue = Vec::new();
        queue.push((0, "broadcaster"));
        while let Some((i, module)) = queue.pop() {
            for _ in 0..i { eprint!("  "); }
            let m = self.system.modules.get(module).unwrap();
            eprint!("{}{module}", m.kind());
            if seen.contains(module) {
                eprintln!(" => ...");
                continue;
            }
            eprintln!();
            seen.insert(module);
            for (t, _) in m.targets.iter().rev() {
                queue.push((i + 1, t));
            }
        }
    }

    INPUT_EX { 1 part1 = 32000000; graph }
    INPUT_EX2 { 1 part1 = 11687500; graph }
    INPUT { 1 part1 = 777666211; graph }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Port(usize);

#[derive(Clone, Copy, PartialEq, Eq)]
enum Pulse {
    Low,
    High,
}

#[derive(Clone)]
struct System<'a> {
    modules: HashMap<&'a str, Module<'a>>,
    queue: VecDeque<(&'a str, Port, Pulse)>,
}

impl<'a> System<'a> {
    fn push_button(&mut self) -> (usize, usize, bool) {
        let mut lows = 0;
        let mut highs = 0;
        self.queue.push_back(("broadcaster", Port(0), Pulse::Low));
        while let Some((target, port, pulse)) = self.queue.pop_front() {
            if target == "rx" && pulse == Pulse::Low {
                return (0, 0, true);
            }
            match pulse {
                Pulse::Low => lows += 1,
                Pulse::High => highs += 1,
            }
            let module = self.modules.get_mut(target).unwrap();
            if let Some(pulse) = module.pulse(port, pulse) {
                for (target, port) in module.targets.iter() {
                    self.queue.push_back((target, *port, pulse));
                }
            }
        }
        (lows, highs, false)
    }
}

struct Module<'a> {
    name: &'a str,
    targets: Vec<(&'a str, Port)>,
    implementation: Box<dyn ModuleImpl + 'a>,
}

impl<'a> Clone for Module<'a> {
    fn clone(&self) -> Self {
        Self {
            name: self.name,
            targets: self.targets.clone(),
            implementation: self.implementation.clone(),
        }
    }
}

impl<'a> Module<'a> {
    fn new(name: &'a str, implementation: impl ModuleImpl + 'a) -> Self {
        Self {
            name,
            targets: Vec::new(),
            implementation: Box::new(implementation),
        }
    }

    fn add_target(&mut self, target: &'a str, port: Port) {
        self.targets.push((target, port));
    }

    fn kind(&self) -> char {
        self.implementation.kind()
    }

    fn alloc_port(&mut self) -> Port {
        self.implementation.alloc_port()
    }

    fn pulse(&mut self, port: Port, pulse: Pulse) -> Option<Pulse> {
        self.implementation.pulse(port, pulse)
    }
}

trait ModuleImpl {
    fn clone(&self) -> Box<dyn ModuleImpl>;

    fn kind(&self) -> char {
        '_'
    }

    fn alloc_port(&mut self) -> Port {
        Port(0)
    }

    fn pulse(&mut self, port: Port, pulse: Pulse) -> Option<Pulse>;
}

struct Broadcaster;

impl ModuleImpl for Broadcaster {
    fn clone(&self) -> Box<dyn ModuleImpl> {
        Box::new(Self)
    }

    fn pulse(&mut self, _: Port, pulse: Pulse) -> Option<Pulse> {
        Some(pulse)
    }
}

struct Observer;

impl ModuleImpl for Observer {
    fn clone(&self) -> Box<dyn ModuleImpl> {
        Box::new(Self)
    }

    fn pulse(&mut self, _: Port, _: Pulse) -> Option<Pulse> {
        None
    }
}

struct FlipFlop(bool);

impl FlipFlop {
    fn new() -> Self {
        Self(false)
    }
}

impl ModuleImpl for FlipFlop {
    fn clone(&self) -> Box<dyn ModuleImpl> {
        Box::new(Self(self.0))
    }

    fn kind(&self) -> char {
        '%'
    }

    fn pulse(&mut self, _: Port, pulse: Pulse) -> Option<Pulse> {
        if pulse == Pulse::Low {
            self.0 = !self.0;
            if self.0 {
                Some(Pulse::High)
            } else {
                Some(Pulse::Low)
            }
        } else {
            None
        }
    }
}

struct Conjunction(Vec<Pulse>);

impl Conjunction {
    fn new() -> Self {
        Self(Vec::new())
    }
}

impl ModuleImpl for Conjunction {
    fn clone(&self) -> Box<dyn ModuleImpl> {
        Box::new(Self(self.0.clone()))
    }

    fn kind(&self) -> char {
        '&'
    }

    fn alloc_port(&mut self) -> Port {
        let port = Port(self.0.len());
        self.0.push(Pulse::Low);
        port
    }

    fn pulse(&mut self, port: Port, pulse: Pulse) -> Option<Pulse> {
        self.0[port.0] = pulse;
        if self.0.iter().all(|&x| x == Pulse::High) {
            Some(Pulse::Low)
        } else {
            Some(Pulse::High)
        }
    }
}
