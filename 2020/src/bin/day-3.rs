const INPUT: &[u8] = include_bytes!("day-3.input");
const WIDTH: usize = 31;

fn main() {
    let map: Vec<u8> = INPUT
        .iter()
        .copied()
        .filter_map(|b| match b {
            b'.' => Some(0),
            b'#' => Some(1),
            _ => None,
        })
        .collect();
    part_1(&map);
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
