use crate::{Conf, Input};

pub const INPUT: Conf<u64> = Conf::new(
    Input::FileHash("79434cdc88ac8fef1185321ccff895f5d32877e925cdad004b9f9bf3eefbdbe3"),
    6386640365805,
    6423258376982,
);

pub const EX: Conf<u64> = Conf::new(Input::Str("2333133121414131402\n"), 1928, 2858);

pub fn part1(input: &str) -> u64 {
    let input = &input.as_bytes()[..input.len() - 1];
    let mut checksum = 0;
    let mut it = input.iter().copied().enumerate();
    let mut rit = it.clone().rev();
    let mut i = (it.next().unwrap().1 - b'0') as u64;
    let mut mid = 0;
    let mut mn = 0;

    while let Some((_, space)) = it.next() {
        let mut space = space - b'0';
        while space != 0 {
            if mn == 0 {
                let Some((mid_, mb)) = rit.next() else {
                    unreachable!()
                };
                mid = mid_ as u64 / 2;
                mn = mb - b'0';
                rit.next();
            }
            checksum += i * mid;
            space -= 1;
            mn -= 1;
            i += 1;
        }
        let (id, len) = it.next().unwrap();
        let id = id as u64 / 2;
        if id == mid {
            while mn != 0 {
                checksum += i * id;
                mn -= 1;
                i += 1;
            }
            break;
        }
        for _ in 0..len - b'0' {
            checksum += i * id;
            i += 1;
        }
    }

    checksum
}

pub fn part2(input: &str) -> u64 {
    let input = &input.as_bytes()[..input.len() - 1];
    let mut it = input.iter().copied().enumerate();
    let mut rit = it.clone().rev();
    let mut spaces = Vec::with_capacity(input.len() / 2);
    let mut pos = vec![0; input.len() / 2 + 1];
    let mut disk = vec![0; (it.next().unwrap().1 - b'0') as usize];
    while let Some((_, len)) = it.next() {
        let len = len - b'0';
        if len != 0 {
            spaces.push((disk.len() as u32, len));
            disk.resize(disk.len() + len as usize, 0);
        }
        let (id, len) = it.next().unwrap();
        let (id, len) = (id / 2, len - b'0');
        pos[id] = disk.len() as u32;
        disk.resize(disk.len() + len as usize, id);
    }
    while let Some((id, len)) = rit.next() {
        let (id, len) = (id / 2, len - b'0');
        for s in spaces.iter_mut() {
            if s.1 >= len {
                let from = pos[id];
                if from < s.0 {
                    break;
                }
                disk.copy_within(from as usize..from as usize + len as usize, s.0 as usize);
                unsafe {
                    disk.as_mut_ptr()
                        .add(from as usize)
                        .write_bytes(0, len as usize)
                };
                s.0 += len as u32;
                s.1 -= len;
                break;
            }
        }
        rit.next();
    }
    disk.into_iter()
        .enumerate()
        .map(|(i, id)| i as u64 * id as u64)
        .sum()
}
