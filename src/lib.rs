pub mod fonts;

use core::mem;
use embedded_hal::blocking::i2c::{Read, Write};
pub use ht16k33_lite::prelude::*;

/// Number of characters that can be displayed at once.
pub const CHAR_TOTAL: usize = 4;
/// Number of bytes needed to represent a character.
pub const CHAR_SIZE: usize = 2;

/// Each digit addresses 2 buffer bytes who collectively form a set of LEDs,
/// used for displaying a single character.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum CharDataAddressPointer {
    P0, P1, P2, P3,
}

pub fn flush<I2C, E>(ht16k33: &mut HT16K33<I2C>, dbuf: &[u16; 4]) -> Result<E>
where
    I2C: Read<Error = E> + Write<Error = E>,
{
    unsafe {
        ht16k33.write_dram(
            &DisplayDataAddressPointer::P0,
            &mem::transmute::<[u16; 4], [u8; 8]>(*dbuf),
        )
    }
}

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
pub fn write_char(dbuf: &mut [u16; 4], cdap: CharDataAddressPointer, c: u16) {
    dbuf[cdap as usize] = c;
}

pub fn write_dot(dbuf: &mut [u16; 4], cdap: CharDataAddressPointer, dot: bool) {
    match dot {
        true =>  dbuf[cdap as usize] |=  fonts::DOT_MASK,
        false => dbuf[cdap as usize] &= !fonts::DOT_MASK,
    }
}

pub fn write_str<I: Iterator<Item = u16> + Sized>(dbuf: &mut [u16; 4], iter: I) {
    iter.enumerate().for_each(|(i, c)| dbuf[i] = c);
}

/*
pub trait PHat {
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
 */

pub struct PHat<I2C, E>
where
    I2C: Read<Error = E> + Write<Error = E>,
{
    dbuf: [u16; CHAR_TOTAL], // Display buffer
    ht16k33: HT16K33<I2C>,
}

impl<I2C, E> PHat<I2C, E>
where
    I2C: Read<Error = E> + Write<Error = E>,
{
    pub fn new(mut ht16k33: HT16K33<I2C>) -> std::result::Result<PHat<I2C, E>, E> {
        ht16k33.power_on()?;
        Ok(PHat {
            dbuf: [0; CHAR_TOTAL],
            ht16k33,
        })
    }

    pub fn ht16k33_ref(&self) -> &HT16K33<I2C> {
        &self.ht16k33
    }

    pub fn ht16k33_mut(&mut self) -> &mut HT16K33<I2C> {
        &mut self.ht16k33
    }

    pub fn write_char(&mut self, cdap: CharDataAddressPointer, c: u16) {
        write_char(&mut self.dbuf, cdap, c)
    }

    pub fn write_dot(&mut self, cdap: CharDataAddressPointer, dot: bool) {
        write_dot(&mut self.dbuf, cdap, dot)
    }

    pub fn write_str<I: Iterator<Item = u16> + Sized>(&mut self, iter: I) {
        write_str(&mut self.dbuf, iter)
    }

    pub fn flush(&mut self) -> Result<E> {
        unsafe {
            self.ht16k33.write_dram(
                &DisplayDataAddressPointer::P0,
                &mem::transmute::<[u16; 4], [u8; 8]>(self.dbuf),
            )
        }
    }
}

impl<I2C, E> Drop for PHat<I2C, E>
where
    I2C: Read<Error = E> + Write<Error = E>,
{
    fn drop(&mut self) {
        #![allow(unused_must_use)]
        self.ht16k33.shutdown();
    }
}
