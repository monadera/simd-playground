#![cfg_attr(feature = "simd", feature(portable_simd))]

use itertools::{EitherOrBoth, Itertools};
use std::ops::BitOr;

#[derive(Clone)]
pub struct BitSet<B> {
    data: Vec<B>,
}

#[cfg(target_pointer_width = "32")]
const POINTER_WIDTH: usize = 32;
#[cfg(target_pointer_width = "64")]
const POINTER_WIDTH: usize = 64;

pub trait Block: Clone + Sized + BitOr<Output = Self> {
    const SIZE: usize;
    fn zeros() -> Self;
    fn ones() -> Self;
    fn set(&mut self, idx: usize);
    fn get(&self, idx: usize) -> bool;
}

impl Block for usize {
    const SIZE: usize = POINTER_WIDTH;

    fn zeros() -> Self {
        0
    }

    fn ones() -> Self {
        usize::MAX
    }

    fn set(&mut self, idx: usize) {
        *self |= 1 << idx;
    }

    fn get(&self, idx: usize) -> bool {
        self & (1 << idx) != 0
    }
}

#[cfg(feature = "simd")]
impl Block for std::simd::u64x4 {
    const SIZE: usize = 64 * 4;

    fn zeros() -> Self {
        std::simd::u64x4::splat(0)
    }

    fn ones() -> Self {
        std::simd::u64x4::splat(u64::MAX)
    }

    #[inline(always)]
    fn set(&mut self, idx: usize) {
        let (lane, bit) = div_rem(idx, 64);
        self[lane] |= 1 << bit;
    }

    #[inline(always)]
    fn get(&self, idx: usize) -> bool {
        let (lane, bit) = div_rem(idx, 64);
        let value = self[lane] & (1 << bit);

        value != 0
    }
}

#[cfg(feature = "wide")]
impl Block for wide::u64x4 {
    const SIZE: usize = 64 * 4;

    fn zeros() -> Self {
        Self::MIN
    }

    fn ones() -> Self {
        Self::MAX
    }

    #[inline(always)]
    fn set(&mut self, idx: usize) {
        let (lane, bit) = div_rem(idx, 64);
        let mut array = self.to_array();
        array[lane] |= 1 << bit;
        *self = Self::from(array)
    }

    #[inline(always)]
    fn get(&self, idx: usize) -> bool {
        let (lane, bit) = div_rem(idx, 64);
        let value = self.to_array()[lane] & (1 << bit);

        value != 0
    }
}

#[inline(always)]
fn div_rem(idx: usize, block_size: usize) -> (usize, usize) {
    (idx / block_size, idx % block_size)
}

impl<B: Block> BitSet<B> {
    #[inline(always)]
    pub fn required_blocks(n: usize) -> usize {
        let (mut blocks, rem) = div_rem(n, B::SIZE);
        blocks += (rem > 0) as usize;

        blocks
    }

    pub fn ones(n: usize) -> Self {
        Self {
            data: vec![B::ones(); Self::required_blocks(n)],
        }
    }

    pub fn zeros(n: usize) -> Self {
        Self {
            data: vec![B::zeros(); Self::required_blocks(n)],
        }
    }

    pub fn set(&mut self, idx: usize) {
        let (block_idx, bit_idx) = div_rem(idx, B::SIZE);
        if let Some(block) = self.data.get_mut(block_idx) {
            block.set(bit_idx);
        } else {
            panic!("setting {idx}, which is out of range");
        }
    }

    pub fn get(&self, idx: usize) -> bool {
        let (block_idx, bit_idx) = div_rem(idx, B::SIZE);
        if let Some(block) = self.data.get(block_idx) {
            block.get(bit_idx)
        } else {
            panic!("getting {idx}, which is out of range");
        }
    }

    pub fn union(self, other: Self) -> Self {
        let zipped = self.data.into_iter().zip_longest(other.data);
        let blocks = zipped.into_iter().map(|pair| match pair {
            EitherOrBoth::Both(lhs, rhs) => lhs | rhs,
            EitherOrBoth::Left(lhs) => lhs,
            EitherOrBoth::Right(rhs) => rhs,
        });

        blocks.collect::<Vec<B>>().into()
    }
}

impl<B: Block> BitOr for BitSet<B> {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.union(rhs)
    }
}

impl<B: Clone + Block> From<Vec<B>> for BitSet<B> {
    fn from(data: Vec<B>) -> Self {
        Self { data }
    }
}

#[cfg(test)]
mod tests {
    mod usize_tests {
        use crate::BitSet;
        #[test]
        fn test_union_of_ones_and_zeros() {
            let size = 1_000;
            let p: BitSet<usize> = BitSet::zeros(size);
            let r: BitSet<usize> = BitSet::ones(size);
            let s = p | r;

            for i in 0..size {
                assert_eq!(s.get(i), true);
            }
        }

        #[test]
        fn test_union_random_bits() {
            let size = 1_000;
            let mut p: BitSet<usize> = BitSet::zeros(size);
            let mut r: BitSet<usize> = BitSet::zeros(size);
            p.set(20);
            r.set(600);
            let s = p | r;

            for i in 0..size {
                assert_eq!(s.get(i), i == 20 || i == 600);
            }
        }
    }

    #[cfg(feature = "simd")]
    mod simd_tests {
        use crate::BitSet;
        use std::simd::u64x4;

        #[test]
        fn test_union_of_ones_and_zeros() {
            let size = 1_000;
            let p: BitSet<u64x4> = BitSet::zeros(size);
            let r: BitSet<u64x4> = BitSet::ones(size);
            let s = p | r;

            for i in 0..size {
                assert_eq!(s.get(i), true);
            }
        }

        #[test]
        fn test_union_random_bits() {
            let size = 1_000;
            let mut p: BitSet<u64x4> = BitSet::zeros(size);
            let mut r: BitSet<u64x4> = BitSet::zeros(size);
            p.set(20);
            r.set(600);
            let s = p | r;

            for i in 0..size {
                assert_eq!(s.get(i), i == 20 || i == 600);
            }
        }
    }
}
