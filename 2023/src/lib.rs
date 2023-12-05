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
macro_rules! str_block {
    ($s:literal) => {{
        // const way to do &$s[1..]
        let add = if $s.is_empty() { 0 } else { 1 };
        let ptr = unsafe { $s.as_ptr().add(add) };
        let bytes = unsafe { ::std::slice::from_raw_parts(ptr, $s.len() - add) };
        if let Ok(s) = ::std::str::from_utf8(bytes) {
            s
        } else {
            panic!("string didn't start with newline")
        }
    }};
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
            use ::std::time::Instant;

            let p0t = Instant::now();
            let mut day = $Day::new($input)?;
            let p0t = Instant::now().duration_since(p0t);

            let p1t = Instant::now();
            let part1 = day.part1()?;
            let p1t = Instant::now().duration_since(p1t);
            println!("part 1: {}", part1);

            $(
                let p2t = Instant::now();
                let part2: $p2ty = day.part2()?;
                let p2t = Instant::now().duration_since(p2t);
                println!("part 2: {}", part2);
            )?

            print!("[ init: {p0t:?}, part 1: {p1t:?}");
            $(
                let _: $p2ty;
                print!(", part 2: {p2t:?}");
            )?
            println!(" ]");

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
