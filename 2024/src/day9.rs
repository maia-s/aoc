use crate::Input;
use core::{array, cell::Cell};
use std::collections::VecDeque;

pub fn inputs() -> Vec<Input<u64>> {
    vec![
        Input::Hashed("79434cdc88ac8fef1185321ccff895f5d32877e925cdad004b9f9bf3eefbdbe3"),
        Input::Hashed("52a46a2520cac808c03c6145a6e7097f8f9442eea003826f69f9a6a92612945a"),
        Input::Hashed("200f28afa251e7ee4ea74cad59381c5663d294d0743f2aa7f4571c50a45e83f8"),
        Input::Hashed("8d9fc67cad414de50d29931d3df0cf45f82b5ec53afa1cb37bd51440a8c21704"),
        Input::Inline("example", "2333133121414131402\n", Some(1928), Some(2858)),
    ]
}

pub fn part1(input: &str) -> u64 {
    let input = &input.as_bytes()[..input.len() - 1];
    let mut checksum = 0;
    let mut it = input.iter().copied().enumerate();
    let mut rit = it.clone().rev();
    let mut i = (it.next().unwrap().1 - b'0') as u64;
    let mut mid = u64::MAX;
    let mut mlen = 0;

    while let Some((_, space)) = it.next() {
        let mut space = space - b'0';
        let (id, len) = it.next().unwrap();
        let (id, len) = (id as u64 / 2, len - b'0');
        while space != 0 {
            if mlen == 0 {
                let Some((mid_, mlen_)) = rit.next() else {
                    unreachable!()
                };
                (mid, mlen) = (mid_ as u64 / 2, mlen_ - b'0');
                if mid < id {
                    return checksum;
                }
                rit.next();
            }
            checksum += i * mid;
            space -= 1;
            mlen -= 1;
            i += 1;
        }
        if id == mid {
            while mlen != 0 {
                checksum += i * id;
                mlen -= 1;
                i += 1;
            }
            break;
        }
        for _ in 0..len {
            checksum += i * id;
            i += 1;
        }
    }

    checksum
}

pub fn part2(input: &str) -> u64 {
    #[derive(Clone, Copy)]
    struct Space(u32, u8);

    let input = &input.as_bytes()[..input.len() - 1];
    let mut it = input.iter().copied().enumerate();
    let mut rit = it.clone().rev();
    let mut spacestore = Vec::with_capacity(input.len() / 2);
    let mut spaces: [_; 9] = array::from_fn(|_| VecDeque::with_capacity(input.len() / 2));
    let mut pos = vec![0; input.len() / 2 + 1];
    let mut disk = vec![0; (it.next().unwrap().1 - b'0') as usize];
    while let Some((_, len)) = it.next() {
        let len = len - b'0';
        if len != 0 {
            let si = spacestore.len() as u32;
            spacestore.push(Cell::new(Space(disk.len() as u32, len)));
            for space in spaces[..len as usize].iter_mut() {
                space.push_back(si);
            }
            disk.resize(disk.len() + len as usize, 0);
        }
        let (id, len) = it.next().unwrap();
        let (id, len) = (id / 2, len - b'0');
        pos[id] = disk.len() as u32;
        disk.resize(disk.len() + len as usize, id);
    }
    while let Some((id, len)) = rit.next() {
        let (id, len) = (id / 2, len - b'0');
        let from = pos[id];
        let spaces = &mut spaces[len as usize - 1];
        let mut pop = 0;
        for &space in spaces.iter() {
            let sd = spacestore[space as usize].get();
            if from < sd.0 {
                break;
            } else if sd.1 < len {
                pop += 1;
                continue;
            }
            disk.copy_within(from as usize..from as usize + len as usize, sd.0 as usize);
            unsafe {
                disk.as_mut_ptr()
                    .add(from as usize)
                    .write_bytes(0, len as usize)
            };
            spacestore[space as usize].set(Space(sd.0 + len as u32, sd.1 - len));
            break;
        }
        for _ in 0..pop {
            spaces.pop_front();
        }
        rit.next();
    }
    disk.into_iter()
        .enumerate()
        .map(|(i, id)| i as u64 * id as u64)
        .sum()
}
