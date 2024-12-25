use crate::Input;
use str_block::str_block;

pub fn inputs() -> Vec<Input> {
    vec![
        Input::Hashed("ea440c0ce2e7c82c64a413a900f3589029d6935384b67273221ab457e99f9b7a"),
        Input::Inline(
            "example",
            str_block! {"
                #####
                .####
                .####
                .####
                .#.#.
                .#...
                .....
                
                #####
                ##.##
                .#.##
                ...##
                ...#.
                ...#.
                .....
                
                .....
                #....
                #....
                #...#
                #.#.#
                #.###
                #####
                
                .....
                .....
                #.#..
                ###..
                ###.#
                ###.#
                #####
                
                .....
                .....
                .....
                #....
                #.#..
                #.#.#
                #####
            "},
            Some(3),
            None,
        ),
    ]
}

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct LockOrKey(u32);

impl LockOrKey {
    const MASK: u32 = (1 << 25) - 1;

    fn new(input: &mut &[u8]) -> Self {
        let is_key = input[0] == b'.';
        let mut value = if is_key { u32::MAX } else { 0 };
        for &i in input[6..36].iter().filter(|&&i| i != b'\n') {
            value = value << 1 | (i == b'#') as u32;
        }
        *input = &input[input.len().min(6 * 7 + 1)..];
        Self(value)
    }

    fn is_key(self) -> bool {
        (self.0 as i32) < 0
    }
}

pub fn part1(input: &str) -> u32 {
    let mut input = input.as_bytes();
    let mut locks = Vec::default();
    let mut keys = Vec::default();
    while !input.is_empty() {
        let lok = LockOrKey::new(&mut input);
        if lok.is_key() {
            keys.push(lok.0 & LockOrKey::MASK);
        } else {
            locks.push(lok.0);
        }
    }
    let mut matches = 0;
    for lock in locks.into_iter() {
        for key in keys.iter() {
            matches += (lock & key == 0) as u32
        }
    }
    matches
}

pub fn part2(_input: &str) -> u32 {
    0
}
