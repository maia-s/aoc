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
        #[derive(Clone)]
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
            use ::std::time::{Instant, Duration};
            type Res<T> = Result<T, Box<dyn ::std::error::Error>>;
            const TIME_LIMIT: Duration = Duration::new(0, 500_000_000);

            fn time<S, T>(s: impl Fn() -> S, f: impl Fn(&mut S) -> Res<T>) -> Res<(T, u32, Duration)> {
                let mut n = 0;
                let mut t = Duration::new(0, 0);
                let ts = Instant::now();
                loop {
                    n += 1;
                    let mut s = s();
                    let t0 = Instant::now();
                    let x = f(&mut s)?;
                    let t1 = Instant::now();
                    t += t1.duration_since(t0);
                    if t1.duration_since(ts) >= TIME_LIMIT || n == u32::MAX {
                        return Ok((x, n, t));
                    }
                }
            }

            let (day, p0n, p0t) = time(||(), |_| $Day::new($input))?;

            let (part1, p1n, p1t) = time(|| day.clone(), |day| day.part1())?;
            println!("part 1: {}", part1);

            $(
                let _: $p2ty;
                let (part2, p2n, p2t) = time(|| day.clone(), |day| day.part2())?;
                println!("part 2: {}", part2);
            )?

            print!("[ init: {:?}, part 1: {:?}", p0t / p0n, p1t / p1n);
            $(
                let _: $p2ty;
                print!(", part 2: {:?}", p2t / p2n);
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
                    $( assert_eq!($Day::new($tinput)?.part1()?, $tp1, "wrong result for part 1"); )?
                    $( assert_eq!($Day::new($tinput)?.part2()?, $tp2, "wrong result for part 2"); )?
                    Ok(())
                }
            )*
        }
    };
}
