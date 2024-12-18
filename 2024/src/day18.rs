use crate::Input;
use std::collections::VecDeque;
//use str_block::str_block;

pub fn inputs() -> Vec<Input> {
    vec![
        Input::Hashed("1e7bc8a20b07a31a71a2a21fece1394e03db70b4c1d81406896a30131cba2235"),
        /*Input::Inline(
            "example",
            str_block! {"
                5,4
                4,2
                4,5
                3,0
                2,1
                6,3
                2,4
                1,5
                0,6
                3,3
                2,6
                5,1
                1,2
                5,5
                2,5
                6,5
                1,4
                0,4
                6,4
                1,1
                6,1
                1,0
                0,5
                1,6
                2,0
            "},
            Some(22),
            None,
        ),*/
    ]
}

// example
//const SIZE: usize = 7;
//const FALL: usize = 12;

// actual
const SIZE: usize = 71;
const FALL: usize = 1024;

const DELTAS: [[i8; 2]; 4] = [[0, -1], [1, 0], [0, 1], [-1, 0]];

fn num(input: &[u8], i: &mut usize) -> u8 {
    let mut j = *i;
    let mut num = input[j] - b'0';
    loop {
        j += 1;
        let digit = input[j];
        if digit >= b'0' {
            num = num * 10 + (digit - b'0');
        } else {
            break;
        }
    }
    *i = j + 1;
    num
}

fn coord(input: &[u8], i: &mut usize) -> Option<(u8, u8)> {
    (*i < input.len()).then(|| (num(input, i), num(input, i)))
}

pub fn part1(input: &str) -> u32 {
    let input = input.as_bytes();
    let mut i = 0;
    let mut map = [false; SIZE * SIZE];
    for _ in 0..FALL {
        let Some((x, y)) = coord(input, &mut i) else {
            unreachable!()
        };
        map[y as usize * SIZE + x as usize] = true;
    }
    let mut queue = VecDeque::new();
    queue.push_back((0_i8, 0_i8, 0_u32));
    while let Some((x, y, steps)) = queue.pop_front() {
        if x == (SIZE - 1) as _ && y == (SIZE - 1) as _ {
            return steps;
        }
        let c = unsafe {
            map.get_mut(y as usize * SIZE + x as usize)
                .unwrap_unchecked()
        };
        if !*c {
            *c = true;
            for [dx, dy] in DELTAS {
                let (nx, ny) = (x + dx, y + dy);
                if (nx as u8) < SIZE as u8 && (ny as u8) < SIZE as u8 {
                    queue.push_back((nx, ny, steps + 1));
                }
            }
        }
    }
    unreachable!()
}

pub fn part2(_input: &str) -> u32 {
    0
}
