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
#[derive(Copy, Clone, Debug, Hash)]
pub enum CharDataAddressPointer {
    P0, P1, P2, P3,
}

pub fn write_char(dbuf: &mut [u16; 4], cdap: CharDataAddressPointer, c: u16) {
    dbuf[cdap as usize] = c;
}

pub fn write_dot(dbuf: &mut [u16; 4], cdap: CharDataAddressPointer, dot: bool, dotmask: u16) {
    match dot {
        true =>  dbuf[cdap as usize] |=  dotmask,
        false => dbuf[cdap as usize] &= !dotmask,
    }
}

pub fn write_str<I: Iterator<Item = u16> + Sized>(dbuf: &mut [u16; 4], iter: I) {
    iter.enumerate().for_each(|(i, c)| dbuf[i] = c)
}

pub fn toggle_dot(dbuf: &mut [u16; 4], cdap: CharDataAddressPointer, dotmask: u16) {
    dbuf[cdap as usize] ^= dotmask
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
    font: fn(&u8) -> u16,
    ht16k33: HT16K33<I2C>,
}

impl<I2C, E> PHat<I2C, E>
where
    I2C: Read<Error = E> + Write<Error = E>,
{
    /// Creates a new instance.
    pub fn new(i2c: I2C, addr: u8, font: fn(&u8) -> u16) -> std::result::Result<PHat<I2C, E>, E> {
        let ht16k33 = HT16K33::new(i2c, addr);
        Ok(PHat {
            dbuf: [0; CHAR_TOTAL],
            font,
            ht16k33,
        })
    }

    /// Writes single character to display buffer 
    /// at Character Data Address pointer.
    pub fn write_char(&mut self, cdap: CharDataAddressPointer, c: &u8) {
        write_char(&mut self.dbuf, cdap, (self.font)(c))
    }

    /// Writes dot to display buffer 
    /// using `self.font` to map character '.' into a bitmask.
    pub fn write_dot(&mut self, cdap: CharDataAddressPointer, dot: bool) {
        write_dot(&mut self.dbuf, cdap, dot, (self.font)(&b'.'))
    }

    /// Writes string reference to display buffer using `self.font` as map.
    pub fn write_str(&mut self, s: &str) {
        write_str(&mut self.dbuf, s.as_bytes().iter().map(self.font))
    }

    /// Xors the bitmask of character '.' 
    /// at Character Data Address pointer to display buffer.
    pub fn toggle_dot(&mut self, cdap: CharDataAddressPointer) {
        toggle_dot(&mut self.dbuf, cdap, (self.font)(&b'.'))
    }

    /// Returns internal display buffer as a reference.
    pub fn dbuf(&self) -> &[u16; CHAR_TOTAL] {&self.dbuf}

    /// Returns internal display buffer as a mutable reference.
    pub fn mut_dbuf(&mut self) -> &mut [u16; CHAR_TOTAL] {&mut self.dbuf}

    /// Flushes display buffer, writing everything to i2c interface.
    pub fn flush(&mut self) -> Result<E> {flush(&mut self.ht16k33, &self.dbuf)}

    // Methods below are mostly reexported from ht16k33
    
    /// Turns system to normal mode, set's Row/Int to Row, turns on display, 
    /// turns dimming to max, and writes a blank dbuf.
    pub fn power_on(&mut self) -> Result<E> {self.ht16k33.power_on()}

    /// Writes blank buffer, turns off display and oscillator.
    pub fn shutdown(&mut self) -> Result<E> {self.ht16k33.shutdown()}

    /// Destroys self and returns internal ht16k33 interface.
    pub fn take_ht16k33(self) -> HT16K33<I2C> {self.ht16k33}

    /// Returns reference to internat system mode. 
    pub fn system_setup(&self) -> SystemSetup {self.ht16k33.system_setup()}

    /// Writes new System mode to controller
    /// and if successful store it's new state.
    pub fn write_system_setup(&mut self, sys: SystemSetup) -> Result<E> {
        self.ht16k33.write_system_setup(sys)
    }

    /// Returns reference to internal display state. 
    pub fn display_setup(&self) -> DisplaySetup {self.ht16k33.display_setup()}
    
    /// Writes a new display state to controller 
    /// and if successful store it's new state.
    pub fn write_dimming_setup(&mut self, dpy: DisplaySetup) -> Result<E> {
        self.ht16k33.write_display_setup(dpy)
    }

    /// Returns reference to internal rowint state.
    pub fn row_int_set(&self) -> RowIntSet {self.ht16k33.row_int_set()}

    /// Writes new Row/Int output to controller
    /// and if successful store it's new state.
    pub fn write_row_int_set(&mut self, rowint: RowIntSet) -> Result<E> {
        self.ht16k33.write_row_int_set(rowint)
    }

    /// Returns dimming level.
    pub fn dimming_set(&self) -> DimmingSet {self.ht16k33.dimming_set()}

    /// Writes a new dimming level to controller
    /// and if successful store it's new state.
    pub fn write_dimming_set(&mut self, dim: DimmingSet) -> Result<E> {
        self.ht16k33.write_dimming_set(dim)
    }

    /// Writes a slice of zeros to controller Display Ram.
    pub fn clear_dram(&mut self) -> Result<E> {self.ht16k33.clear_dram()}

    /// Reads Display Ram from controller into internal display buffer.
    pub fn read_dram(&mut self) -> Result<E> {
        self.ht16k33.read_dram(unsafe {
            &mut mem::transmute::<[u16; 4], [u8; 8]>(self.dbuf)
        })
    }
}
