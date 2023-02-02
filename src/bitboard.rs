use crate::definitions::Square;

#[derive(Clone, Copy)]
pub struct Bitboard {
    bits: u64,
}

impl Default for Bitboard {
    fn default() -> Self {
        Self { bits: Default::default() }
    }
}

impl From<u64> for Bitboard {
    fn from(source: u64) -> Self {
        Self { bits: source }
    }
}

impl Bitboard {
    pub fn new_blank() -> Self {
        Self::from(0)
    }

    pub fn new_full() -> Self {
        Self::from(u64::MAX)
    }

    pub fn bits(&self) -> u64 {
        self.bits
    }

    pub fn print (&self) {
        println!();
        for rank in 0..8 {
            print!("{}  ", 8 - rank);  
    
            for file in 0..8 {
                print!( " {} ", if self.get_bit(rank*8 + file) { "X" } else { "-" } )
            }
            println!()
        }
        println!( "    a  b  c  d  e  f  g  h");
        println!( "    Bitboard: {}", self.bits)
    }
    
    pub fn get_bit(&self, square: u8) -> bool {
        self.bits & (1 << square) != 0
    }
    
    #[cfg(test)]
    pub fn get_bit_sq(&self, square: Square) -> bool {
        self.get_bit(square as u8)
    }
    
    pub fn set_bit(&mut self, square: u8) {
        self.bits |= 1 << square
    }

    pub fn set_bit_sq(&mut self, square: Square) {
        self.set_bit(square as u8)
    }

    pub fn unset_bit(&mut self, square: u8) {
        self.bits &= !(1 << square)
    }

    pub fn unset_bit_sq(&mut self, square: Square) {
        self.unset_bit(square as u8)
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.bits == 0
    }

    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        self.bits != 0
    }

    pub fn least_significant(&self) -> u8 {
        unsafe { core::arch::x86_64::_tzcnt_u64(self.bits) as u8}
    }

    /// Extract the least significant set bit. Modifies the bitboard and returns the position of the extracted bit
    pub fn extract_bit(&mut self) -> Option<u8> {
        if self.is_empty() { return None }

        let bit = self.bits.trailing_zeros();

        self.bits = unsafe { core::arch::x86_64::_blsr_u64(self.bits) };

        Some(bit as u8)
    }

    pub fn and(&self, other: Bitboard) -> Self {
        Self { bits: self.bits & other.bits }
    }

    pub fn and_u64(&self, other: u64) -> Self {
        Self { bits: self.bits & other }
    }

    pub fn or(&self, other: Bitboard) -> Self {
        Self { bits: self.bits | other.bits }
    }

    pub fn or_u64(&self, other: u64) -> Self {
        Self { bits: self.bits | other }
    }

    pub fn not(&self) -> Self {
        Self { bits: !self.bits }
    }

    pub fn count(&self) -> u32 {
        self.bits.count_ones()
    }

    pub fn xor(&self, other: Bitboard) -> Self {
        Self { bits: self.bits ^ other.bits }
    }
}