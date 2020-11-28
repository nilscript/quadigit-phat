#![cfg_attr(not(feature = "std"), no_std)]
#![feature(array_value_iter)] // Used in Digit::IntoIter
#![allow(clippy::manual_range_contains)]
#![feature(step_trait)]
#![feature(step_trait_ext)]

pub mod font;

use crate::font::ASCII;
use bounded_integer::bounded_integer;
use core::str::Chars;
use embedded_hal::blocking::i2c::{Write, WriteRead};
use extend::ext;
pub use ht16k33_diet::*;

pub const CHAR_SIZE: usize = 2;

bounded_integer! {
/// Each digit addresses 2 buffer bytes who collectivly form a set of leds, 
/// used for displaying a character.
#[repr(u8)]
pub enum Digit { 0..4 }
}

impl Digit {
    /// Creates 2 Display Data Addresses
    /// These addresses are used as backends for the Digit interface.
    pub fn to_addr(&self) -> [usize; CHAR_SIZE] {
        [*self as usize * 2, *self as usize * 2 + 1]
    }
}

impl IntoIterator for Digit {
    type Item = usize;
    type IntoIter = core::array::IntoIter<Self::Item, CHAR_SIZE>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self.to_addr())
    }
}

/// Set an fourletter-phat driver
pub type PHat<I2C> = HT16K33<I2C>;
/// Bitmask character type
pub type Char = [u8; CHAR_SIZE];

#[ext(pub)]
impl<I2C, E> PHat<I2C>
where
    I2C: Write<Error = E> + WriteRead<Error = E>
{
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
        let addr = digit.to_addr()[1];
        let decimal_mask = 0b0100_0000;

        match dot {
            true =>  self.buffer[addr] |= decimal_mask,
            false => self.buffer[addr] &= !decimal_mask,
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
        let mapper = |c| ASCII.get(&c)
            .unwrap_or_else(|| ASCII.get(&'?').unwrap());

        (Digit::Z..=Digit::P3) // Z stands for zero, P3 for positive 3
            .flatten()          // Flattens into u8 buffer addresses
            .zip(chars.map(mapper).flatten())
            .for_each(|(d, &c)| self.buffer[d] = c);
    }
}
