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
        write!(f, "{}", self.0)
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
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
        $($pno:literal $part:ident $pty:ty { $($part_body:tt)* })*
        $($tinput:ident { $($tno:literal $tpart:ident = $tresult:expr),* $(,)? } )*
    ) => {
        $(#[$attr])*
        #[derive(Clone)]
        struct $Day $(<$lt>)? { $($fields)* }

        impl $(<$lt>)? $Day $(<$lt>)? {
            fn new($in: & $($lt)? str) -> Result<$Day, Box<dyn ::std::error::Error>> {
                $($new)*
            }

            $(
                fn $part(&mut $self) -> Result<$pty, Box<dyn ::std::error::Error>> {
                    $($part_body)*
                }
            )*
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

            let mut timings = Vec::new();
            $(
                let (part, pn, pt) = time(|| day.clone(), |day| day.$part())?;
                timings.push((pn, pt));
                println!("part {}: {}", $pno, part);
            )*

            print!("[ init: {:?}", p0t / p0n);
            let ti = 0;
            $(
                print!(", part {}: {:?}", $pno, timings[ti].1 / timings[ti].0);
                let ti = ti + 1;
            )*
            println!(" ]");

            Ok(())
        }

        #[cfg(test)]
        mod tests {
            use super::*;

            $(
                #[allow(non_snake_case)]
                mod $tinput {
                    use super::*;

                    $(
                        #[test]
                        fn $tpart() -> Result<(), Box<dyn ::std::error::Error>> {
                            assert_eq!($Day::new($tinput)?.$tpart()?, $tresult, "wrong result for part {}", $tno);
                            Ok(())
                        }
                    )*
                }
            )*
        }
    };
}
