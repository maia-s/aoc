#[macro_export]
macro_rules! aoc {
    (
        struct $Day:ident { $($fields:tt)* }
        $self:ident($in:ident) { $($new:tt)* }
        part1 { $($part1:tt)* }
        part2 { $($part2:tt)* }
        input = $input:expr;
        $(test $tname:ident($tinput:expr, $tp1:expr $(, $tp2:expr)?);)*
    ) => {
        struct $Day { $($fields)* }

        impl $Day {
            fn new($in: &str) -> Result<$Day, Box<dyn ::std::error::Error>> {
                $($new)*
            }
    
            fn part1(&$self) -> Result<usize, Box<dyn ::std::error::Error>> {
                $($part1)*            
            }
    
            fn part2(&$self) -> Result<usize, Box<dyn ::std::error::Error>> {
                $($part2)*            
            }
        }

        fn main() -> Result<(), Box<dyn ::std::error::Error>> {
            let day = $Day::new($input)?;
            println!("part 1: {}", day.part1()?);
            println!("part 2: {}", day.part2()?);
            Ok(())
        }

        #[cfg(test)]
        mod tests {
            use super::*;

            $(
                #[test]
                fn $tname() -> Result<(), Box<dyn ::std::error::Error>> {
                    let test = $Day::new($tinput)?;
                    assert_eq!($tp1, test.part1()?, "wrong result for part 1");
                    $( assert_eq!($tp2, test.part2()?, "wrong result for part 2"); )*
                    Ok(())
                }
            )*
        }
    };
}
