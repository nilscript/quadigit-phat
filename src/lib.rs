pub mod fonts;

use core::mem;
use dyn_iter::DynIter;
use embedded_hal::blocking::i2c::{Read, Write};
pub use ht16k33_lite::prelude::*;
use std::iter::once;

/// Number of characters that can be displayed at once.
pub const CHAR_TOTAL: usize = 4;
/// Number of bytes needed to represent a character.
pub const CHAR_SIZE: usize = 2;

pub type QuadBuf = [u16; CHAR_TOTAL];
pub type HT16K33Buf = [u8; COMMONS_SIZE];

/// Writes a single character bitmask to address in buffer
pub fn write_char(dbuf: &mut QuadBuf, addr: usize, c: u16) {
    dbuf[addr] = c;
}

/// 
pub fn write_dot(dbuf: &mut QuadBuf, addr: usize, dot: bool, dotmask: u16) {
    match dot {
        true  => dbuf[addr] |=  dotmask,
        false => dbuf[addr] &= !dotmask,
    }
}

pub fn write_str(dbuf: &mut QuadBuf, iter: &mut dyn Iterator<Item = u16>) {
    iter.take(CHAR_TOTAL).enumerate().for_each(|(i, c)| dbuf[i] = c)
}

pub fn toggle_dot(dbuf: &mut QuadBuf, addr: usize, dotmask: u16) {
    dbuf[addr] ^= dotmask
}

pub fn flush<I2C, E>(ht16k33: &mut HT16K33<I2C>, dbuf: &QuadBuf) -> Result<E>
where
    I2C: Read<Error = E> + Write<Error = E>,
{
    unsafe {
        ht16k33.write_dram(
            &DisplayDataAddressPointer::P0,
            &mem::transmute::<QuadBuf, HT16K33Buf>(*dbuf),
        )
    }
}

pub struct PHat<I2C, E>
where
    I2C: Read<Error = E> + Write<Error = E>,
{
    dbuf: QuadBuf, // Display buffer
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
        Ok(PHat {
            dbuf: [0; CHAR_TOTAL],
            font,
            ht16k33,
        })
    }

    /// Writes single character at address in display buffer.
    pub fn write_char(&mut self, addr: usize, c: u8) {
        write_char(
            &mut self.dbuf, 
            addr, 
            (self.font)(DynIter::new(once(c))).next().unwrap()
        )
    }

    /// Writes dot to display buffer 
    /// using `self.font` to map character '.' into a bitmask.
    pub fn write_dot(&mut self, addr: usize, dot: bool) {
        write_dot(
            &mut self.dbuf, 
            addr, 
            dot, 
            (self.font)(DynIter::new(once(b'.'))).next().unwrap()
        )
    }

    /// Writes string reference to display buffer using `self.font` as map.
    pub fn write_str(&mut self, s: &str) {
        write_str(&mut self.dbuf, &mut (self.font)(DynIter::new(s.bytes())))
    }

    /// Xors the bitmask of character '.' 
    /// at address in display buffer.
    pub fn toggle_dot(&mut self, addr: usize) {
        toggle_dot(
            &mut self.dbuf, 
            addr, 
            (self.font)(DynIter::new(once(b'.'))).next().unwrap()
        )
    }

    /// Returns internal display buffer as a reference.
    pub fn dbuf(&self) -> &QuadBuf {&self.dbuf}

    /// Returns internal display buffer as a mutable reference.
    pub fn mut_dbuf(&mut self) -> &mut QuadBuf {&mut self.dbuf}

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
            &mut mem::transmute::<QuadBuf, HT16K33Buf>(self.dbuf)
        })
    }
}
