use std::{str::FromStr, ops::RangeInclusive};

const INPUT: &str = include_str!("day-2.input");

trait FindFrom<T> {
    fn find_from(&self, index: usize, pat: T) -> Option<usize>;
}

impl FindFrom<char> for &str {
    fn find_from(&self, index: usize, pat: char) -> Option<usize> {
        self[index..].find(pat).map(|i| i + index)
    }
}

struct Password {
    text: String,
    range: RangeInclusive<usize>,
    ch: char,
}

impl FromStr for Password {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let range_sep = s.find('-').ok_or("missing '-'")?;
        let range_end = s.find_from(range_sep, ' ').ok_or("missing ' '")?;
        let colon = s.find_from(range_end, ':').ok_or("missing ':'")?;
        let min = s[..range_sep].parse().map_err(|_| "min range parse error")?;
        let max = s[(range_sep+1)..range_end].parse().map_err(|_| "max range parse error")?;
        let ch = s[(range_end+1)..].chars().next().ok_or("missing char")?;
        let text = s[(colon+1)..].trim().to_string();
        Ok(Self {
            text,
            range: min..=max,
            ch
        })
    }
}

fn main() -> Result<(), &'static str> {
    part_1()?;
    Ok(())
}

fn part_1() -> Result<(), &'static str> {
    println!("=[ part 1 ]=");
    let mut valid = 0;
    for line in INPUT.lines() {
        let pw = line.parse::<Password>()?;
        let mut ch_count = 0;
        for ch in pw.text.chars() {
            if ch == pw.ch {
                ch_count += 1;
            }
        }
        if pw.range.contains(&ch_count) {
            valid += 1;
        }
    }
    println!("valid passwords: {}", valid);
    Ok(())
}
