use crate::Input;
use std::collections::VecDeque;
//use str_block::str_block;

pub fn inputs() -> Vec<Input<u32, String>> {
    vec![
        Input::Hashed("1e7bc8a20b07a31a71a2a21fece1394e03db70b4c1d81406896a30131cba2235"),
        Input::Hashed("5d5b67dd9fd2f22c3377910932f3a8240d730a2c7593e990b025d20d05f04259"),
        Input::Hashed("08bbddf6ace9b636fbb877a41cb560cfd966e2ad097626083e8b853a63b7e4dc"),
        Input::Hashed("30ed8de297b590dc290eb985ce25142a88497520c54d6cdd025d5b0e68a95f7e"),
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
            Some("6,1".into()),
        ),*/
    ]
}

// example
//const SIZE: usize = 7;
//const FALL: usize = 12;

// actual
const SIZE: usize = 71;
const FALL: usize = 1024;

const DELTAS: [[i8; 2]; 4] = [[-1, 0], [0, -1], [1, 0], [0, 1]];

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
    map[0] = true;
    let mut queue = VecDeque::with_capacity(SIZE * SIZE);
    queue.push_back((0_i8, 0_i8, 1_u32));
    while let Some((x, y, steps)) = queue.pop_front() {
        for [dx, dy] in DELTAS {
            let (nx, ny) = (x + dx, y + dy);
            if (nx as u8) < SIZE as u8 && (ny as u8) < SIZE as u8 {
                let c = unsafe {
                    map.get_mut(ny as usize * SIZE + nx as usize)
                        .unwrap_unchecked()
                };
                if !*c {
                    *c = true;
                    if nx == (SIZE - 1) as _ && ny == (SIZE - 1) as _ {
                        return steps;
                    }
                    queue.push_back((nx, ny, steps + 1));
                }
            }
        }
    }
    unreachable!()
}

pub fn part2(input: &str) -> String {
    let input = input.as_bytes();
    let mut i = 0;
    let mut map = [0; SIZE * SIZE];
    let mut queue = Vec::with_capacity(SIZE * SIZE);
    let mut blocks = 0;
    for _ in 0..FALL {
        let Some((x, y)) = coord(input, &mut i) else {
            unreachable!()
        };
        map[y as usize * SIZE + x as usize] = u32::MAX;
    }
    'fall: loop {
        let Some((bx, by)) = coord(input, &mut i) else {
            unreachable!()
        };
        let c = &mut map[by as usize * SIZE + bx as usize];
        let prev = *c & !0x7fff;
        *c = u32::MAX;
        blocks += 0x10000;
        if prev == map[0] {
            map[0] = blocks;
            queue.clear();
            let mut x = 0;
            let mut y = 0;
            let mut rsteps = 0;
            'retrace: loop {
                queue.push((x, y, rsteps));
                rsteps += 1;
                for [dx, dy] in DELTAS {
                    let (nx, ny) = (x + dx, y + dy);
                    if (nx as u8) < SIZE as u8 && (ny as u8) < SIZE as u8 {
                        let c = unsafe {
                            map.get_mut(ny as usize * SIZE + nx as usize)
                                .unwrap_unchecked()
                        };
                        if *c == prev | rsteps {
                            *c = blocks | rsteps;
                            x = nx;
                            y = ny;
                            continue 'retrace;
                        }
                    }
                }
                break;
            }
            while let Some((x, y, steps)) = queue.pop() {
                let steps = steps + 1;
                for [dx, dy] in DELTAS {
                    let (nx, ny) = (x + dx, y + dy);
                    if (nx as u8) < SIZE as u8 && (ny as u8) < SIZE as u8 {
                        let c = unsafe {
                            map.get_mut(ny as usize * SIZE + nx as usize)
                                .unwrap_unchecked()
                        };
                        if *c < blocks {
                            if *c & !0x7fff == prev && *c & 0x7fff > rsteps {
                                let mut rx = nx;
                                let mut ry = ny;
                                let mut steps = steps;
                                let mut rsteps = *c & 0x7fff;
                                *c = blocks | steps | 0x8000;
                                'retrace_end: loop {
                                    steps += 1;
                                    rsteps += 1;
                                    for [dx, dy] in DELTAS {
                                        let (nx, ny) = (rx + dx, ry + dy);
                                        if (nx as u8) < SIZE as u8 && (ny as u8) < SIZE as u8 {
                                            let c = unsafe {
                                                map.get_mut(ny as usize * SIZE + nx as usize)
                                                    .unwrap_unchecked()
                                            };
                                            if *c == prev | rsteps {
                                                *c = blocks | steps | 0x8000;
                                                rx = nx;
                                                ry = ny;
                                                if nx == SIZE as i8 - 1 && ny == SIZE as i8 - 1 {
                                                    break 'retrace_end;
                                                }
                                                continue 'retrace_end;
                                            }
                                        }
                                    }
                                    unreachable!()
                                }
                            } else {
                                *c = blocks | steps;
                                if nx != (SIZE - 1) as _ || ny != (SIZE - 1) as _ {
                                    queue.push((nx, ny, steps));
                                    continue;
                                }
                            }
                            let mut btx = nx;
                            let mut bty = ny;
                            let c = unsafe {
                                map.get_mut(bty as usize * SIZE + btx as usize)
                                    .unwrap_unchecked()
                            };
                            *c |= 0x8000;
                            let mut bsteps = blocks | (steps - 1);
                            'backtrack: loop {
                                for [dx, dy] in DELTAS {
                                    let (nx, ny) = (btx + dx, bty + dy);
                                    if (nx as u8) < SIZE as u8 && (ny as u8) < SIZE as u8 {
                                        let c = unsafe {
                                            map.get_mut(ny as usize * SIZE + nx as usize)
                                                .unwrap_unchecked()
                                        };
                                        if *c == bsteps {
                                            *c |= 0x8000;
                                            if nx == 0 && ny == 0 {
                                                continue 'fall;
                                            };
                                            btx = nx;
                                            bty = ny;
                                            bsteps -= 1;
                                            continue 'backtrack;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            return format!("{bx},{by}");
        }
    }
}
