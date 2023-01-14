pub mod fonts;

use core::iter::once;
use ambassador::{Delegate};
use dyn_iter::DynIter;
use embedded_hal::blocking::i2c::{Read, Write};
use ht16k33_lite::*;

/// Number of characters that can be displayed at once.
pub const CHAR_TOTAL: usize = 4;
/// Number of bytes needed to represent a character.
pub const CHAR_SIZE: usize = 2;

#[derive(Delegate)]
#[delegate(HT16K33Trait<I2C, E>, target = "ht16k33")]
pub struct PHat<I2C, E>
where
    I2C: Read<Error = E> + Write<Error = E>,
{
    font: fn(DynIter<u8>) -> DynIter<u16>,
    ht16k33: HT16K33<I2C>,
}

impl<I2C, E> PHat<I2C, E>
where
    I2C: Read<Error = E> + Write<Error = E>,
{
    /// Creates a new instance.
    pub fn new(i2c: I2C, addr: u8, font: fn(DynIter<u8>) -> DynIter<u16>) 
        -> std::result::Result<PHat<I2C, E>, E> 
    {
        let ht16k33 = HT16K33::new(i2c, addr);
        Ok(PHat { font, ht16k33 })
    }
    
    pub fn cbuf(&self) -> &[u16] {
        unsafe { self.dbuf().align_to().1 }
    }

    pub fn cbuf_mut(&mut self) -> &mut [u16] {
        unsafe { self.dbuf_mut().align_to_mut().1 }
    }

    pub fn set_cbuf(&mut self, src: &[u16; CHAR_TOTAL]) {
        self.cbuf_mut().copy_from_slice(src);
    }

    /// Writes single character at address in display buffer.
    pub fn set_char(&mut self, addr: usize, c: u8) {
        self.cbuf_mut()[addr] = (self.font)(DynIter::new(once(c)))
            .next()
            .unwrap();
        
    }

    /// Writes dot to display buffer 
    /// using `self.font` to map character '.' into a bitmask.
    pub fn set_dot(&mut self, addr: usize, dot: bool) {
        let dotmask = self.font()(DynIter::new(once(b'.'))).next().unwrap();
        let cbuf = self.cbuf_mut();

        match dot {
            false => cbuf[addr] |=  dotmask, 
            true  => cbuf[addr] &= !dotmask,
        }
    }

    /// Writes string reference to display buffer using `self.font` as map.
    pub fn set_str(&mut self, s: &str) {
        self.font()(DynIter::new(s.bytes()))
            .zip(self.cbuf_mut().iter_mut())
            .for_each(|(f, c)| *c = f);
    }

    pub fn font(&self) -> fn(DynIter<u8>) -> DynIter<u16> {self.font}

    pub fn set_font(&mut self, font: fn(DynIter<u8>) -> DynIter<u16>) {
        self.font = font
    }

    /// Xors the bitmask of character '.' 
    /// at address in display buffer.
    pub fn toggle_dot(&mut self, addr: usize) {
        self.cbuf_mut()[addr] ^= (self.font)(DynIter::new(once(b'.'))).next().unwrap();
    }

    /// Flushes display buffer, writing everything to i2c interface.
    pub fn flush(&mut self) -> Result<E> {
        self.ht16k33.write_dbuf()
    }

    /// Destroys self and returns internal ht16k33 interface.
    pub fn take_ht16k33(self) -> HT16K33<I2C> {self.ht16k33}
}
