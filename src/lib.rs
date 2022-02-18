#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::manual_range_contains)]
#![feature(step_trait)]
#![feature(generators, generator_trait)]

pub mod fonts;

use core::str::Chars;
pub use ht16k33_lite::prelude::*;
use strum::IntoEnumIterator;
use strum::EnumIter;
use std::ops::RangeInclusive;
use std::convert::TryFrom;

/// Number of characters that can be displayed at once.
pub const CHAR_TOTAL: usize = 4;
/// Number of bytes needed to represent a character.
pub const CHAR_SIZE:  usize = 2;

/// Each digit addresses 2 buffer bytes who collectively form a set of LEDs, 
/// used for displaying a single character.
#[derive(Copy, Clone, Debug, EnumIter, PartialEq, Eq, Hash)]
pub enum CharDataAddressPointer { 
    P0, P1, P2, P3 
}

impl CharDataAddressPointer {
    /// Creates the first pointer to the first memory cell of a specific digit.
    pub fn first(&self) -> usize {*self as usize * 2}

    /// Creates the second pointer to the last memory cell of a specific digit.
    pub fn second(&self) -> usize {*self as usize * 2 + 1}

    /// Creates 2 addresses that are pointing to both cells of a digit.
    pub fn to_address(&self) -> [usize; CHAR_SIZE] {
        [self.first(), self.second()]
    }

    /// Creates a usize range of all valid character addresses.
    pub fn to_full_address_range() -> RangeInclusive<usize> {
        Self::P0 as usize..=Self::P3 as usize
    }
}

impl TryFrom<usize> for CharDataAddressPointer {
    type Error =  &'static str;

    fn try_from(u: usize) -> Result<Self, Self::Error> {
        match u {
           u if u == Self::P0 as usize => Ok(Self::P0),
           u if u == Self::P1 as usize => Ok(Self::P1),
           u if u == Self::P2 as usize => Ok(Self::P2),
           u if u == Self::P3 as usize => Ok(Self::P3),
           u => Err(format!("CharAddress only accepts values ranging from 0 to 3! Value was: {}.", u)),
        }
    }
}

pub trait PHat {
    /// Set's one character for one digit.
    /// 
    /// If 
    /// ```
    /// # use quadigit_phat::*;
    /// # use embedded_hal_mock::i2c::{Mock as I2c, Transaction};
    /// # let expectations = [
    /// # Transaction::write(0, vec![0, 0x00, 0x00, 0x3F, 0x00, 0x70, 0x24, 0x00, 0x00, 0x00, 0x00, 0, 0, 0, 0, 0, 0])
    /// # ];
    /// # let mut phat = HT16K33::new(I2c::new(&expectations), 0u8);
    /// (Digit::P1..=Digit::P2)
    /// .zip("OK".chars().map(|c| fonts::ascii(&c)))
    /// .for_each(|(d, c)| phat.set_char(d, c));
    /// 
    /// phat.write_dbuf().unwrap();
    /// ```
    fn set_char(&mut self, caddr: CharDataAddressPointer, c: Char);

    /// Iterates over `chars` mapping them with `mapper` 
    /// and set's the internal buffer.
    /// 
    /// Unlike `PHat::set_text()` 
    /// this method does not compile dots.
    fn set_chars(&mut self, mapper: fn(&char) -> Char, chars: Chars);

    /// Set's the dot led for one digit. 
    /// Fourletter phat library called this decimal, 
    /// but to avoid confusion it's now a dot.
    /// 
    /// Example:
    /// 
    /// ```
    /// # use quadigit_phat::*;
    /// # use embedded_hal_mock::i2c::{Mock as I2c, Transaction};
    /// # let expectations = [
    /// # Transaction::write(0, vec![0, 0, 0, 0, 0b0100_0000, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
    /// # ];
    /// # let mut phat = HT16K33::new(I2c::new(&expectations), 0u8);
    /// phat.set_dot(Digit::P1, true);
    /// phat.write_dbuf().unwrap();
    /// ```
    /// 
    /// For the curious the dot mask is: 0000_0000_0100_0000 (for each digit)
    fn set_dot(&mut self, caddr: CharDataAddressPointer, dot: bool);

    /// Iterates over `chars` mapping them with `mapper` 
    /// and set's the internal buffer.
    /// Periods or dots (.) are inlined to the previous character
    /// unless escaped by another dot.
    /// 
    /// This will probably work with many projects but
    /// if you need more fine control take a look at the source code.
    /// 
    /// /// Example: 
    /// ```
    /// # use quadigit_phat::*;
    /// # use embedded_hal_mock::i2c::{Mock as I2c, Transaction};
    /// # let expectations = [
    /// # Transaction::write(0, vec![0, 0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00, 0, 0, 0, 0, 0, 0, 0, 0]),
    /// # Transaction::write(0, vec![0, 0xFF, 0x40, 0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00, 0, 0, 0, 0, 0, 0, 0, 0]),
    /// # Transaction::write(0, vec![0, 0xFF, 0x00, 0x00, 0x40, 0xFF, 0x00, 0xFF, 0x00, 0, 0, 0, 0, 0, 0, 0, 0]),
    /// # Transaction::write(0, vec![0, 0x00, 0x40, 0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00, 0, 0, 0, 0, 0, 0, 0, 0]),
    /// # Transaction::write(0, vec![0, 0x00, 0x40, 0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00, 0, 0, 0, 0, 0, 0, 0, 0]),
    /// # Transaction::write(0, vec![0, 0x00, 0x40, 0x00, 0x40, 0x00, 0x40, 0x00, 0x40, 0, 0, 0, 0, 0, 0, 0, 0]),
    /// # ];
    /// # let mut phat = HT16K33::new(I2c::new(&expectations), 0u8);
    /// // Would be displayed as  ["8", "8", "8", "8"]
    /// phat.set_text(fonts::ascii, "8888".chars());
    /// phat.write_dbuf().unwrap();
    /// phat.clear_dbuf();
    /// 
    /// // Would be displayed as  ["8.", "8", "8", "8"]
    /// phat.set_text(fonts::ascii, "8.888".chars()); 
    /// phat.write_dbuf().unwrap();
    /// phat.clear_dbuf();
    /// 
    /// // Would be displayed as ["8", ".", "8", "8"]
    /// phat.set_text(fonts::ascii, "8..88".chars()); 
    /// phat.write_dbuf().unwrap();
    /// phat.clear_dbuf();
    /// 
    /// // Would be displayed as [".", "8", "8", "8"]
    /// phat.set_text(fonts::ascii, ".888".chars()); 
    /// phat.write_dbuf().unwrap();
    /// phat.clear_dbuf();
    /// 
    /// // Would still be displayed [".", "8", "8", "8"]
    /// phat.set_text(fonts::ascii, "..888".chars()); 
    /// phat.write_dbuf().unwrap();
    /// phat.clear_dbuf();
    /// 
    /// // Would be displayed [".", ".", ".", "."]
    /// phat.set_text(fonts::ascii, "........".chars()); 
    /// phat.write_dbuf().unwrap();
    /// phat.clear_dbuf();
    /// ```
    fn set_text(&mut self, mapper: fn(&char) -> Char, chars: Chars);
}

/// Bitmask character type
pub type Char = [u8; CHAR_SIZE];

impl<I2C> PHat for HT16K33<I2C> {
    fn set_char(&mut self, caddr: CharDataAddressPointer, c: Char) {
        self.dbuf[caddr.first() ] = c[0];
        self.dbuf[caddr.second()] = c[1];
    }
    
    fn set_chars(&mut self, mapper: fn(&char) -> Char, chars: Chars) {
        CharDataAddressPointer::iter()
        .zip(chars.map(|c| mapper(&c)))
        .for_each(|(d, c)| {
            self.dbuf[d.first() ] = c[0];
            self.dbuf[d.second()] = c[1];
        });
    }

    fn set_dot(&mut self, cdap: CharDataAddressPointer, dot: bool) {
        let addr = cdap.second();
        match dot {
            true =>  self.dbuf[addr] |=  fonts::DOT_MASK,
            false => self.dbuf[addr] &= !fonts::DOT_MASK,
        }
    }

    fn set_text(&mut self, mapper: fn(&char) -> Char, chars: Chars) {
        compile_dot(&mut self.dbuf[CharDataAddressPointer::to_full_address_range()], mapper, chars);
    }
}

/// Iterates over characters and maps them to buffer.
/// Periods or dots (.) are inlined to the previous character
/// unless escaped by another dot.
/// 
/// For examples look at `PHat::set_text()` method
pub fn compile_dot(buf: &mut [u8], mapper: fn(&char) -> Char, chars: Chars) {
    // chars and digit are not synced with iterators by design.
    let mut chars = chars.peekable();
    let mut index = CharDataAddressPointer::iter().peekable();

    while let Some((c, i)) = chars.next().zip(index.peek()) {
        // Ordering of checks matters.
        // Edge cases are covered like general cases.
        if c == '.' && chars.next_if_eq(&'.').is_none() && *i != CharDataAddressPointer::P0 {
            buf[i.first() -1] |= fonts::DOT_MASK;

        // Character is not an escaped dot or a dot.
        } else {
            let c = mapper(&c);
            buf[i.first() ] = c[0];
            buf[i.second()] = c[1];
            index.next();
        }
    }
}
