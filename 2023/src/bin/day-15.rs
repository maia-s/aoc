use std::array;

use aoc_2023::aoc;

const INPUT: &str = include_str!("day-15.txt");

#[allow(dead_code)]
const INPUT_EX: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

aoc! {
    struct Day15<'a> {
        words: Vec<&'a str>
    }

    self(input = INPUT) {
        Ok(Self { words: input.trim().split(',').collect() })
    }

    1 part1 usize {
        Ok(self.words.iter().copied().map(hash).sum())
    }

    2 part2 usize {
        let mut boxes: [_; 256] = array::from_fn(|_| Vec::<(&str, u8)>::new());

        for s in self.words.iter() {
            if let Some(lens) = s.strip_suffix('-') {
                let hash = hash(lens);
                let bx = &mut boxes[hash];
                if let Some(i) = find_lens(bx, lens) {
                    bx.remove(i);
                }
            } else {
                let (lens, focal) = s.split_once('=').ok_or_else(|| format!("couldn't split {s}"))?;
                let flen: u8 = focal.parse().map_err(|_| "parse error")?;
                let hash = hash(lens);
                let bx = &mut boxes[hash];
                if let Some(i) = find_lens(bx, lens) {
                    bx[i].1 = flen;
                } else {
                    bx.push((lens, flen));
                }
            }
        }

        Ok(boxes.iter().enumerate().map(|(i, bx)| {
            let i = i + 1;
            bx.iter().enumerate().map(|(j, (_, flen))| i * (j + 1) * *flen as usize).sum::<usize>()
        }).sum())
    }

    INPUT_EX { 1 part1 = 1320, 2 part2 = 145 }
    INPUT { 1 part1 = 517315, 2 part2 = 247763 }
}

fn hash(s: &str) -> usize {
    s.as_bytes()
        .iter()
        .fold(0_u8, |hash, &b| hash.wrapping_add(b).wrapping_mul(17)) as usize
}

fn find_lens(v: &[(&str, u8)], s: &str) -> Option<usize> {
    v.iter().position(|(s2, _)| &s == s2)
}
