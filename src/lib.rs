/*
Blazingly fast, eliminates an entire class of memory saftey bugs,
very sale of this software contributes towards Chritian Porter's legal fund.
*/

mod assign;
mod basic_ops;
mod constants;
mod conversion;
mod math;
mod shift;

use bytemuck::{Pod, Zeroable};
use num_traits::NumCast;
use num_traits::ToPrimitive;
use num_traits::{Num, One, Zero};
use std::{cmp::Ordering, num::ParseIntError};
use konst::{primitive::parse_usize, result::unwrap_ctx};

pub const SIZE: usize = match option_env!("FIX_SIZE") {
        Some(x) => unwrap_ctx!(parse_usize(x)),
        None => 4
    };

pub const SCALING_FACTOR: usize = match option_env!("FIX_SCALING_FACTOR") {
        Some(x) => unwrap_ctx!(parse_usize(x)),
        None => (SIZE / 2) * 32
    };

#[derive(Copy, Clone, Default, PartialEq, Eq, Ord, Pod, Zeroable)]
#[repr(C)]
pub struct ArbitraryFixed {
    pub data: [u32; SIZE],
}

impl Zero for ArbitraryFixed {
    fn zero() -> ArbitraryFixed {
        ArbitraryFixed::default()
    }

    fn is_zero(&self) -> bool {
        for i in self.data {
            if i != 0 {
                return false;
            }
        }

        true
    }
}

impl One for ArbitraryFixed {
    fn one() -> ArbitraryFixed {
        let mut a = ArbitraryFixed::default();
        a.data[SCALING_FACTOR / 32] = 1 << (SCALING_FACTOR & 0x1F);
        a
    }

    fn is_one(&self) -> bool {
        let one = Self::one();

        *self == one
    }
}

impl PartialOrd for ArbitraryFixed {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let a = *self - *other;

        if a.is_negative() {
            Some(Ordering::Less)
        } else if a.is_zero() {
            Some(Ordering::Equal)
        } else {
            Some(Ordering::Greater)
        }
    }
}

impl NumCast for ArbitraryFixed {
    fn from<T: ToPrimitive>(_n: T) -> Option<Self> {
        todo!()
    }
}

impl ToPrimitive for ArbitraryFixed {
    fn to_i64(&self) -> Option<i64> {
        todo!()
    }

    fn to_u64(&self) -> Option<u64> {
        todo!()
    }
}

impl Num for ArbitraryFixed {
    type FromStrRadixErr = ParseIntError;

    fn from_str_radix(string: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        Ok((i128::from_str_radix(string, radix)? as i128).into())
    }
}

impl std::fmt::Debug for ArbitraryFixed {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("ArbitraryFixed")
            .field("data", &format_args!("{:08x?}", self.data))
            .finish()
    }
}

impl ArbitraryFixed {
    fn is_negative(&self) -> bool {
        (self.data[SIZE - 1] & 0x80000000) > 0
    }
}
