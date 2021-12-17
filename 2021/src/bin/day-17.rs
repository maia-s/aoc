use std::ops::RangeInclusive;

const INPUT: &str = include_str!("day-17.input");

fn main() {
    let (xr, yr) = INPUT.trim().split_once(", ").unwrap();
    let xr = parse_range(xr);
    let yr = parse_range(yr);

    let dxs = ((0.25 + 2.0 * *xr.start() as f64).sqrt() - 0.5).floor() as isize;
    let mut max_y = 0;
    let mut hits: usize = 0;
    for dx in dxs..*xr.end() + 1 {
        for dy in *yr.start()..-*yr.start() {
            if let Some(my) = throw((dx, dy), (xr.clone(), yr.clone())) {
                max_y = max_y.max(my);
                hits += 1;
            }
        }
    }

    println!("part 1: {}", max_y);
    println!("part 2: {}", hits);
}

fn parse_range(r: &str) -> RangeInclusive<isize> {
    let (_, r) = r.split_once('=').unwrap();
    let (s, e) = r.split_once("..").unwrap();
    let (s, e) = (s.parse::<isize>().unwrap(), e.parse::<isize>().unwrap());
    s..=e
}

fn throw(
    (mut dx, mut dy): (isize, isize),
    (xr, yr): (RangeInclusive<isize>, RangeInclusive<isize>),
) -> Option<isize> {
    let mut x = 0;
    let mut y = 0;
    let mut max_y = 0;
    while y >= *yr.start() {
        x += dx;
        y += dy;
        max_y = max_y.max(y);
        dx -= dx.signum();
        dy -= 1;
        if xr.contains(&x) && yr.contains(&y) {
            return Some(max_y);
        }
    }
    None
}
