const INPUT: &str = include_str!("day-4.txt");

#[cfg(test)]
const INPUT_EX: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

aoc_2022::aoc! {
    struct Day4 {
        pairs: Vec<(ElfRange, ElfRange)>,
    }

    self(input) {
        let mut pairs = Vec::new();
        for line in input.lines() {
            let Some((a, b)) = line.split_once(',') else { return Err("missing comma".into()) };
            let (Some((a1, a2)), Some((b1, b2))) = (a.split_once('-'), b.split_once('-')) else {
                return Err("missing `-`".into())
            };
            let (a1, a2, b1, b2) = (a1.parse()?, a2.parse()?, b1.parse()?, b2.parse()?);
            pairs.push((ElfRange { start: a1, end: a2 }, ElfRange { start: b1, end: b2 }))
        }
        Ok(Self { pairs })
    }

    part1 usize {
        Ok(self.pairs.iter().map(|pair| (pair.0.contains(&pair.1) || pair.1.contains(&pair.0)) as usize).sum())
    }

    part2 usize {
        Ok(self.pairs.iter().map(|pair| pair.0.overlaps(&pair.1) as usize).sum())
    }

    input = INPUT;
    test day4_ex(INPUT_EX, 2, 4);
    test day4(INPUT, 518, 909);
}

struct ElfRange {
    start: usize,
    end: usize,
}

impl ElfRange {
    fn contains(&self, other: &ElfRange) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn overlaps(&self, other: &ElfRange) -> bool {
        self.start <= other.end && self.end >= other.start
    }
}
