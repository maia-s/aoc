use std::{
    fmt::{Debug, Display},
    ops::{BitAnd, BitOr, Not},
    str::FromStr,
};

use crate::Error;

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Dir(u8);

impl Display for Dir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match *self {
            Self::N => '^',
            Self::W => '<',
            Self::S => 'v',
            Self::E => '>',
            Self::SE => '┌',
            Self::SW => '┐',
            Self::NW => '┘',
            Self::NE => '└',
            _ => unreachable!(),
        };
        write!(f, "{c}")
    }
}

impl Debug for Dir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Dir({:#04b})", self.0)
    }
}

impl TryFrom<char> for Dir {
    type Error = Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'N' | 'U' => Ok(Self::N),
            'W' | 'L' => Ok(Self::W),
            'S' | 'D' => Ok(Self::S),
            'E' | 'R' => Ok(Self::E),
            _ => Err("invalid direction".into()),
        }
    }
}

impl FromStr for Dir {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 1 {
            s.chars().next().unwrap().try_into()
        } else {
            Err("invalid direction".into())
        }
    }
}

impl Not for Dir {
    type Output = Self;
    fn not(self) -> Self::Output {
        Self(self.0 ^ 0xf)
    }
}

impl BitAnd for Dir {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitOr for Dir {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl Dir {
    pub const NONE: Self = Self(0);

    pub const N: Self = Self(0b1000);
    pub const W: Self = Self(0b0100);
    pub const S: Self = Self(0b0010);
    pub const E: Self = Self(0b0001);

    pub const NW: Self = Self(0b1100);
    pub const NE: Self = Self(0b1001);
    pub const SW: Self = Self(0b0110);
    pub const SE: Self = Self(0b0011);
    pub const NS: Self = Self(0b1010);
    pub const WE: Self = Self(0b0101);

    pub fn bits(self) -> u8 {
        self.0
    }

    pub fn cardinal_index(self) -> usize {
        match self {
            Self::N => 0,
            Self::W => 1,
            Self::S => 2,
            Self::E => 3,
            _ => panic!("not a cardinal direction"),
        }
    }

    pub fn rev(self) -> Self {
        Self(((self.0 | (self.0 << 4)) >> 2) & 0xf)
    }

    pub fn delta(self) -> (isize, isize) {
        (self.dx(), self.dy())
    }

    pub fn dx(&self) -> isize {
        match self.0 & 0b0101 {
            0b0001 => 1,
            0b0100 => -1,
            _ => 0,
        }
    }

    pub fn dy(&self) -> isize {
        match self.0 & 0b1010 {
            0b1000 => -1,
            0b0010 => 1,
            _ => 0,
        }
    }

    pub fn mov(&self, x: usize, y: usize, w: usize, h: usize) -> Option<(usize, usize)> {
        let x = (x as isize + self.dx()) as usize;
        let y = (y as isize + self.dy()) as usize;
        if (0..w).contains(&x) && (0..h).contains(&y) {
            Some((x, y))
        } else {
            None
        }
    }

    pub fn count_dirs(self) -> u32 {
        self.0.count_ones()
    }

    pub fn has_n(self) -> bool {
        self & Self::N != Self::NONE
    }

    pub fn has_s(self) -> bool {
        self & Self::S != Self::NONE
    }

    pub fn has_e(self) -> bool {
        self & Self::E != Self::NONE
    }

    pub fn has_w(self) -> bool {
        self & Self::W != Self::NONE
    }
}
