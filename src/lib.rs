#![cfg_attr(not(feature = "std"), no_std)]
#![feature(array_value_iter)] // Used in Digit::IntoIter
#![allow(clippy::manual_range_contains)]
#![feature(step_trait)]
#![feature(step_trait_ext)]
#![feature(peekable_next_if)]

pub mod font;

use crate::font::ASCII;
use bounded_integer::bounded_integer;
use core::str::Chars;
use core::ops::RangeInclusive;
pub use ht16k33_lite::*;

pub const CHAR_TOTAL: usize = 4;
pub const CHAR_SIZE:  usize = 2;

pub const DOT_MASK:   u8    = 0b0100_0000;

bounded_integer! {
/// Each digit addresses 2 buffer bytes who collectivly form a set of leds, 
/// used for displaying a character.
#[repr(u8)]
pub enum Digit { 0..4 }
}

impl Digit {
    /// Creates an address that are pointing to the first memory cell of a digit.
    pub fn start(&self) -> usize {
        *self as usize * 2
    }

    /// Creates an address that are pointing to the second memory cell of a digit.
    pub fn end(&self) -> usize {
        *self as usize * 2 + 1
    }

    /// Creates 2 addresses that are pointing to both cells of a digit.
    pub fn to_address(&self) -> [usize; CHAR_SIZE] {
        [self.start(), self.end()]
    }

    /// Creates an inclusive address range from self.start() to other.end()
    pub fn to_range(&self, other: &Self) -> RangeInclusive<usize> {
        self.start()..=other.end()
    }

    pub fn full_range() -> RangeInclusive<usize> {
        Digit::MIN.to_range(&Digit::MAX)
    }
}

impl IntoIterator for Digit {
    type Item = usize;
    type IntoIter = core::array::IntoIter<Self::Item, CHAR_SIZE>;

    /// Creates an iterator over the 2 addresses of an digit.
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self.to_address())
    }
}

pub trait PHat {
    fn set_char(&mut self, digit: Digit, c: Char);

    fn set_dot(&mut self, digit: Digit, doot: bool);

    fn set_text(&mut self, chars: Chars);
}

/// Bitmask character type
pub type Char = [u8; CHAR_SIZE];

impl<I2C> PHat for HT16K33<I2C> {

    fn set_char(&mut self, digit: Digit, c: Char) {
        self.dbuf[digit.start()] = c[0];
        self.dbuf[digit.end()  ] = c[1];
    }

    /// Set's the dot led for one digit. 
    /// Fourletter phat libary called this decimal, 
    /// but to avoid confusion it's now a dot.
    /// 
    /// Example:
    /// 
    /// ```
    /// # use quadigit_phat::*;
    /// # use embedded_hal_mock::i2c::{Mock as I2c, Transaction};
    /// # let expectations = [
    /// # Transaction::write(0, vec![0, 0, 0, 0, 0b00100_0000, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
    /// # ];
    /// # let mut phat = PHat::new(I2c::new(&expectations), 0u8);
    /// phat.set_dot(Digit::P1, true);
    /// phat.write_buffer().unwrap();
    /// ```
    /// 
    /// For the curious the dot mask is: 0000_0000_0100_0000 (for each digit)
    fn set_dot(&mut self, digit: Digit, dot: bool) {
        let addr = digit.end();

        match dot {
            true =>  self.dbuf[addr] |=  DOT_MASK,
            false => self.dbuf[addr] &= !DOT_MASK,
        }
    }

    /// Set's 4 characters into buffer.
    /// 
    /// Example:
    ///
    /// ```
    /// # use quadigit_phat::*;
    /// # use embedded_hal_mock::i2c::{Mock as I2c, Transaction};
    /// # let expectations = [
    /// # Transaction::write(0, vec![0, 0xFF, 0, 0xFF, 0, 0xFF, 0, 0xFF, 0, 0, 0, 0, 0, 0, 0, 0, 0])
    /// # ];
    /// # let mut phat = PHat::new(I2c::new(&expectations), 0u8);
    /// phat.set_text("8888".chars());
    /// phat.write_buffer().unwrap();
    /// ```
    /// 
    /// This will probably work with many projects but
    /// if you need more fine control take a look at the source code.
    fn set_text(&mut self, chars: Chars) {
        let mapper: fn(&char) -> Char = 
            |c| *ASCII.get(c).unwrap_or_else(|| ASCII.get(&'?').unwrap());

        compile_dot(chars, mapper, &mut self.dbuf[Digit::full_range()]);
    }
}

/// Iterates over characters mapping them to buffer.
/// Periods or dots (.). are inlined to it's predecessor 
/// unless escaped by another dot.
/// 
/// Example: 
/// ```
/// # use quadigit_phat::*;
/// # use embedded_hal_mock::i2c::{Mock as I2c, Transaction};
/// # let expectations = [
/// # Transaction::write(0, vec![0, 0xFF, 0x40, 0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00, 0, 0, 0, 0, 0, 0, 0, 0]),
/// # Transaction::write(0, vec![0, 0xFF, 0x00, 0x00, 0x40, 0xFF, 0x00, 0xFF, 0x00, 0, 0, 0, 0, 0, 0, 0, 0]),
/// # Transaction::write(0, vec![0, 0x00, 0x40, 0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00, 0, 0, 0, 0, 0, 0, 0, 0]),
/// # Transaction::write(0, vec![0, 0x00, 0x40, 0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00, 0, 0, 0, 0, 0, 0, 0, 0])
/// # ];
/// # let mut phat = PHat::new(I2c::new(&expectations), 0u8);
/// phat.set_text("8.888".chars()); // Would be displayed as        ["8.", "8", "8", "8"]
/// phat.write_buffer().unwrap();
/// phat.clear_buffer();
/// 
/// phat.set_text("8..88".chars()); // Would be displayed as        ["8", ".", "8", "8"]
/// phat.write_buffer().unwrap();
/// phat.clear_buffer();
/// 
/// phat.set_text(".888".chars()); // Would be displayed as         [".", "8", "8", "8"]
/// phat.write_buffer().unwrap();
/// phat.clear_buffer();
/// 
/// phat.set_text("..888".chars()); // Would still be displayed as  [".", "8", "8", "8"]
/// phat.write_buffer().unwrap();
/// phat.clear_buffer();
/// ```
pub fn compile_dot(chars: Chars, mapper: fn(&char) -> Char, buffer: &mut [u8]) {
    let mut chars = chars.peekable();
    let mut index = 
        (Digit::MIN..=Digit::new_saturating(buffer.len() as u8 / 2)).peekable();

    // chars and digit are not synced
    while let Some((c, i)) = chars.next().zip(index.peek()) {
        // Ordering of checks matters.
        if c == '.' && chars.next_if_eq(&'.').is_none() && *i != Digit::Z {
            buffer[i.start() -1] |= DOT_MASK;
        } else {
            let c = mapper(&c);
            buffer[i.start()] = c[0];
            buffer[i.end()  ] = c[1];
            index.next(); // Advances index
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn tesing() {
        let mapper: fn(&char) -> Char = 
            |c| *ASCII.get(c).unwrap_or_else(|| ASCII.get(&'?').unwrap());

        let mut buffer = [0u8; 8];
        compile_dot("8888".chars(), mapper, &mut buffer);
        assert_eq!([0xFF, 0, 0xFF, 0, 0xFF, 0, 0xFF, 0], buffer);

        let mut buffer = [0u8; 8];
        compile_dot("8.888".chars(), mapper, &mut buffer);
        assert_eq!([0xFF, 64, 0xFF, 0, 0xFF, 0, 0xFF, 0], buffer);

        let mut buffer = [0u8; 8];
        compile_dot("8..88".chars(), mapper, &mut buffer);
        assert_eq!([0xFF, 0, 0, 64, 0xFF, 0, 0xFF, 0], buffer);

        let mut buffer = [0u8; 8];
        compile_dot("8...88".chars(), mapper, &mut buffer);
        assert_eq!([0xFF, 0, 0, 64, 0xFF, 0, 0xFF, 0], buffer);

        let mut buffer = [0u8; 8];
        compile_dot("8....8".chars(), mapper, &mut buffer);
        assert_eq!([0xFF, 0, 0, 64, 0, 64, 0xFF, 0], buffer);
    }
}
