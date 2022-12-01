use std::error::Error;

pub trait AoC: Sized {
    fn new(input: &str) -> Result<Self, Box<dyn Error>>;
    fn part_1(&self) -> usize;
    fn part_2(&self) -> usize;

    fn run(input: &str, p1: Option<usize>, p2: Option<usize>) -> Result<(), Box<dyn Error>> {
        let day = Self::new(input)?;
        let r1 = day.part_1();
        println!("part 1: {}", r1);
        if let Some(p1) = p1 {
            assert_eq!(r1, p1, "part 1 expected {p1}, got {r1}");
        }
        let r2 = day.part_2();
        println!("part 2: {}", r2);
        if let Some(p2) = p2 {
            assert_eq!(r2, p2, "part 2 expected {p2}, got {r2}");
        }
        Ok(())
    }
}
