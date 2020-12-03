const INPUT: &[u8] = include_bytes!("day-3.input");
const WIDTH: usize = 31;

fn main() {
    let map: Vec<u8> = INPUT
        .iter()
        .filter_map(|&b| match b {
            b'.' => Some(0),
            b'#' => Some(1),
            _ => None,
        })
        .collect();

    part_1(&map);
    part_2(&map);
}

fn slide(map: &[u8], right: usize, down: usize) -> usize {
    let mut x = 0;
    let mut y = 0;
    let mut trees = 0;
    while let Some(&tree) = map.get(x + y * WIDTH) {
        trees += tree as usize;
        x = (x + right) % WIDTH;
        y += down;
    }
    trees
}

fn part_1(map: &[u8]) {
    println!("=[ part 1 ]=");
    println!("{} trees", slide(map, 3, 1));
}

fn part_2(map: &[u8]) {
    println!("\n=[ part 2 ]=");
    let r1d1 = slide(map, 1, 1);
    let r3d1 = slide(map, 3, 1);
    let r5d1 = slide(map, 5, 1);
    let r7d1 = slide(map, 7, 1);
    let r1d2 = slide(map, 1, 2);
    println!("{}", r1d1 * r3d1 * r5d1 * r7d1 * r1d2);
}
