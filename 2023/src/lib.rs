use std::borrow::Cow;

pub struct Error(Cow<'static, str>);

impl From<&'static str> for Error {
    fn from(value: &'static str) -> Self {
        Self(Cow::Borrowed(value))
    }
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        Self(Cow::Owned(value))
    }
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[macro_export]
macro_rules! aoc {
    (
        $(#[$attr:meta])* struct $Day:ident $(<$lt:lifetime>)? { $($fields:tt)* }
        $self:ident($in:ident = $input:expr) { $($new:tt)* }
        part1 $p1ty:ty { $($part1:tt)* }
        $( part2 $p2ty:ty { $($part2:tt)* } )?
        $(test $tname:ident($tinput:expr, $($tp1:expr)? $(, $tp2:expr)?);)*
    ) => {
        $(#[$attr])*
        struct $Day $(<$lt>)? { $($fields)* }

        impl $(<$lt>)? $Day $(<$lt>)? {
            fn new($in: & $($lt)? str) -> Result<$Day, Box<dyn ::std::error::Error>> {
                $($new)*
            }

            fn part1(&mut $self) -> Result<$p1ty, Box<dyn ::std::error::Error>> {
                $($part1)*
            }

            $(
                fn part2(&mut $self) -> Result<$p2ty, Box<dyn ::std::error::Error>> {
                    $($part2)*
                }
            )?
        }

        fn main() -> Result<(), Box<dyn ::std::error::Error>> {
            let mut day = $Day::new($input)?;
            println!("part 1: {}", day.part1()?);
            $( println!("part 2: {}", day.part2()? as $p2ty); )?
            Ok(())
        }

        #[cfg(test)]
        mod tests {
            use super::*;

            $(
                #[test]
                fn $tname() -> Result<(), Box<dyn ::std::error::Error>> {
                    let mut test = $Day::new($tinput)?;
                    $( assert_eq!(test.part1()?, $tp1, "wrong result for part 1"); )?
                    $( assert_eq!(test.part2()?, $tp2, "wrong result for part 2"); )?
                    Ok(())
                }
            )*
        }
    };
}
