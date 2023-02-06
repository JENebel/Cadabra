use std::{fmt::Display, ops::{BitAnd, BitOr, BitXor, BitAndAssign, BitOrAssign, BitXorAssign, Not}};

pub trait BitUtils {
    fn get_bit(&self, square: u8) -> bool;
    fn set_bit(&mut self, square: u8);
    fn unset_bit(&mut self, square: u8);
}

#[derive(Clone, Copy)]
pub struct Bitboard {
    pub bits: u64,
}

macro_rules! implement_bitboard_operation {
    ($trait_name: ident, $fn_name: ident, $operator: tt) => {
        impl $trait_name<Bitboard> for Bitboard {
            type Output = Bitboard;
            fn $fn_name(self, rhs: Bitboard) -> Self::Output {
                Bitboard { bits: self.bits $operator rhs.bits }
            }
        }
        
        impl $trait_name<u64> for Bitboard {
            type Output = Bitboard;
            fn $fn_name(self, rhs: u64) -> Self::Output {
                Bitboard { bits: self.bits $operator rhs}
            }
        }

        impl $trait_name<Bitboard> for u64 {
            type Output = Bitboard;
        
            fn $fn_name(self, rhs: Bitboard) -> Self::Output {
                Bitboard { bits: self $operator rhs.bits }
            }
        }
    };
}
macro_rules! implement_bitboard_assign_operation {
    ($trait_name: ident, $fn_name: ident, $operator: tt) => {
        impl $trait_name<Bitboard> for Bitboard {
            fn $fn_name(&mut self, rhs: Bitboard) {
                self.bits $operator rhs.bits
            }
        }
        
        impl $trait_name<u64> for Bitboard {
            fn $fn_name(&mut self, rhs: u64) {
                self.bits &= rhs
            }
        }

        impl $trait_name<Bitboard> for u64 {
            fn $fn_name(&mut self, rhs: Bitboard) {
                *self $operator rhs.bits
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
        self.bits & (1 << square) != 0
    }

    fn set_bit(&mut self, square: u8) {
        self.bits |= 1 << square
    }

    fn unset_bit(&mut self, square: u8) {
        self.bits &= !(1 << square)
    }
}

impl Not for Bitboard {
    type Output = Bitboard;

    fn not(self) -> Self::Output {
        Bitboard { bits: !self.bits }
    }
}

impl Default for Bitboard {
    fn default() -> Self {
        Self { bits: Default::default() }
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
        str = format!("{str} Bitboard: {}\n", self.bits);
        write!(f, "{str}")
    }
}

impl Bitboard {
    pub const EMPTY: Bitboard = Bitboard { bits: u64::MAX };
    pub const FULL: Bitboard = Bitboard { bits: u64::MAX };

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.bits == 0
    }

    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        self.bits != 0
    }

    #[inline(always)]
    pub fn least_significant(&self) -> u8 {
        unsafe { core::arch::x86_64::_tzcnt_u64(self.bits) as u8 }
    }

    /// Extract the least significant set bit. Modifies the bitboard and returns the position of the extracted bit
    pub fn extract_bit(&mut self) -> Option<u8> {
        if self.is_empty() { return None }

        let bit = self.least_significant();

        self.bits = unsafe { core::arch::x86_64::_blsr_u64(self.bits) };

        Some(bit as u8)
    }

    pub fn count(&self) -> u32 {
        self.bits.count_ones()
    }
}