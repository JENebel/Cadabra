use std::{fmt::Display, ops::{BitAnd, BitOr, BitXor, BitAndAssign, BitOrAssign, BitXorAssign, Not}};

use bitintr::Popcnt;

pub trait BitUtils {
    fn get_bit(&self, square: u8) -> bool;
    fn set_bit(&mut self, square: u8);
    fn unset_bit(&mut self, square: u8);
}

#[derive(Clone, Copy)]
pub struct Bitboard(pub u64);

macro_rules! implement_bitboard_operation {
    ($trait_name: ident, $fn_name: ident, $operator: tt) => {
        impl $trait_name<Bitboard> for Bitboard {
            type Output = Bitboard;
            fn $fn_name(self, rhs: Bitboard) -> Self::Output {
                Bitboard(self.0 $operator rhs.0 )
            }
        }
        
        impl $trait_name<u64> for Bitboard {
            type Output = Bitboard;
            fn $fn_name(self, rhs: u64) -> Self::Output {
                Bitboard(self.0 $operator rhs)
            }
        }

        impl $trait_name<Bitboard> for u64 {
            type Output = Bitboard;
            fn $fn_name(self, rhs: Bitboard) -> Self::Output {
                Bitboard(self $operator rhs.0)
            }
        }
    };
}
macro_rules! implement_bitboard_assign_operation {
    ($trait_name: ident, $fn_name: ident, $operator: tt) => {
        impl $trait_name<Bitboard> for Bitboard {
            fn $fn_name(&mut self, rhs: Bitboard) {
                self.0 $operator rhs.0
            }
        }
        
        impl $trait_name<u64> for Bitboard {
            fn $fn_name(&mut self, rhs: u64) {
                self.0 $operator rhs
            }
        }

        impl $trait_name<Bitboard> for u64 {
            fn $fn_name(&mut self, rhs: Bitboard) {
                *self $operator rhs.0
            }
        }
    };
}

implement_bitboard_operation!(BitAnd, bitand, &);
implement_bitboard_operation!(BitOr, bitor, |);
implement_bitboard_operation!(BitXor, bitxor, ^);

implement_bitboard_assign_operation!(BitAndAssign, bitand_assign, &=);
implement_bitboard_assign_operation!(BitOrAssign, bitor_assign, |=);
implement_bitboard_assign_operation!(BitXorAssign, bitxor_assign, ^=);

impl BitUtils for u64 {
    fn get_bit(&self, square: u8) -> bool {
        self & (1 << square) != 0
    }

    fn set_bit(&mut self, square: u8) {
        *self |= 1 << square
    }

    fn unset_bit(&mut self, square: u8) {
        *self &= !(1 << square)
    }
}

impl BitUtils for Bitboard {
    fn get_bit(&self, square: u8) -> bool {
        self.0 & (1 << square) != 0
    }

    fn set_bit(&mut self, square: u8) {
        self.0 |= 1 << square
    }

    fn unset_bit(&mut self, square: u8) {
        self.0 &= !(1 << square)
    }
}

impl Not for Bitboard {
    type Output = Bitboard;

    fn not(self) -> Self::Output {
        Bitboard(!self.0)
    }
}

impl Default for Bitboard {
    fn default() -> Self {
        Bitboard::EMPTY
    }
}

impl Display for Bitboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut str = "".to_string();
        for rank in 0..8 {
            str = format!("{str}{} ", 8 - rank);  
            //println!("{str}");
    
            for file in 0..8 {
                str = format!("{str} {} ", if self.get_bit(rank*8 + file) { "X" } else { "-" })
            }
            str = format!("{str}\n")
        }
        str = format!("{str}   a  b  c  d  e  f  g  h\n");
        str = format!("{str} Bitboard: {}\n", self.0);
        write!(f, "{str}")
    }
}

impl Bitboard {
    pub const EMPTY: Bitboard = Bitboard(0);
    pub const FULL: Bitboard = Bitboard(u64::MAX);

    pub fn as_u64(&self) -> u64 {
        self.0
    }

    pub fn least_significant(&self) -> u8 {
        self.0.trailing_zeros() as u8
    }

    pub fn count_bits(&self) -> u64 {
        self.0.popcnt()
    }

    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }
}

/// Iterate over the set bits on the bitboard. TODO: maybe add ExactIterator impl for performance?
impl Iterator for Bitboard {
    type Item = u8;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        match self.is_empty() {
            true => None,
            false => {
                let bit = self.least_significant();
                self.0 = bitintr::Blsr::blsr(self.0);
                Some(bit)
            },
        }        
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.count_bits() as usize))
    }
}