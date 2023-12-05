use aoc_2023::{aoc, str_block, Error};

const INPUT: &str = include_str!("day-5.txt");

#[allow(dead_code)]
const INPUT_EX: &str = str_block! {"
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
"};

aoc! {
    struct Day5 {
        seeds: Vec<usize>,
        maps: [Map; 7],
    }

    self(input = INPUT) {
        let mut sections = input.split("\n\n");

        let seeds = sections.next().ok_or("missing seeds section")?
            .strip_prefix("seeds: ").ok_or("missing seeds header")?
            .split_ascii_whitespace().map(str::parse).collect::<Result<_, _>>()?;

        let maps = [
            Map::parse(&mut sections, "seed-to-soil")?,
            Map::parse(&mut sections, "soil-to-fertilizer")?,
            Map::parse(&mut sections, "fertilizer-to-water")?,
            Map::parse(&mut sections, "water-to-light")?,
            Map::parse(&mut sections, "light-to-temperature")?,
            Map::parse(&mut sections, "temperature-to-humidity")?,
            Map::parse(&mut sections, "humidity-to-location")?,
        ];

        Ok(Self { seeds, maps })
    }

    part1 usize {
        Ok(self.seeds.iter().map(|&seed| self.seed_location(seed)).min().unwrap())
    }

    part2 usize {
        Ok(self.seeds.chunks(2).map(|c|
            (c[0]..c[0] + c[1]).map(|seed| self.seed_location(seed)).min().unwrap()
        ).min().unwrap())
    }

    test day5_example(INPUT_EX, 35, 46);
    test day5(INPUT, 825516882, 136096660);
}

impl Day5 {
    fn seed_location(&self, seed: usize) -> usize {
        let mut index = seed;
        for map in self.maps.iter() {
            index = map.get(index);
        }
        index
    }
}

struct Map(Vec<(usize, usize, usize)>);

impl Map {
    fn get(&self, index: usize) -> usize {
        for i in self.0.iter() {
            if (i.1..i.1 + i.2).contains(&index) {
                return index - i.1 + i.0;
            }
        }
        index
    }

    fn parse<'a>(
        sections: &mut impl Iterator<Item = &'a str>,
        section: &str,
    ) -> Result<Self, Error> {
        let mut lines = sections
            .next()
            .ok_or_else(|| format!("missing {section} section"))?
            .lines();
        if lines.next() != Some(&format!("{section} map:")) {
            return Err(Error::from(format!("missing {section} header")));
        }

        let mut map = Vec::new();

        for line in lines {
            let mut line = line.split_ascii_whitespace().map(str::parse);
            let map_to = line
                .next()
                .ok_or_else(|| format!("{section} missing map to"))?
                .map_err(|_| format!("{section} map to parse error"))?;
            let map_from = line
                .next()
                .ok_or_else(|| format!("{section} missing map from"))?
                .map_err(|_| format!("{section} map from parse error"))?;
            let map_len = line
                .next()
                .ok_or_else(|| format!("{section} missing map length"))?
                .map_err(|_| format!("{section} map length parse error"))?;
            map.push((map_to, map_from, map_len));
        }

        Ok(Self(map))
    }
}
