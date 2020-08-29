#![cfg_attr(not(feature = "std"), no_std)]
#![feature(array_value_iter)]
//#![warn(missing_docs)]
#![allow(dead_code)]

pub mod bitmap;
mod char;
mod digit;
mod error;

pub use crate::{char::Char, digit::Digit, error::Error};

use ::embedded_hal::blocking::i2c::{Write, WriteRead};
use ::extend::ext;
pub use ::ht16k33::*; // TODO Replace with proper prelude

pub type PHat<I2C> = HT16K33<I2C>;
pub type Result<E> = core::result::Result<(), E>;

/// Set an fourletter-phat driver
///
/// # Example:
/// ```
/// # use ht16k33::i2c_mock::I2cMock as I2c;
/// # use quadigit_phat::PHat;
///
/// let phat = PHat::new(I2c::new(), 0u8);
///
/// ```
#[ext(pub)]
impl<I2C, E> PHat<I2C>
where
    I2C: Write<Error = E> + WriteRead<Error = E>,
{
    /// Update digit buffer
    ///
    /// # Arguments
    /// * `digit`   - Digit to update
    /// * `c`       - Char to set buffer with
    ///
    /// # Examples
    /// ```
    /// # use ht16k33::i2c_mock::{I2cMock as I2c, I2cMockError as Error};
    /// # use ht16k33::DisplayData;
    /// # use quadigit_phat::*;
    ///
    /// # let mut phat = PHat::new(I2c::new(), 0u8);
    /// # phat.initialize();
    /// phat.update_digit(Digit::DIGIT_2, Char::from([0, 0b0000_0111]));
    ///
    /// assert_eq!(phat.display_buffer()[4..6], [DisplayData::from_byte(0), DisplayData::from_byte(0b0000_0111)]);
    /// ```
 
    /// Update digit buffer
    ///
    /// # Arguments
    /// * `digit`   - Digit to update
    /// * `raw`     - Raw pair of bytes to set buffer with
    ///
    /// # Examples
    /// ```
    /// # use ht16k33::i2c_mock::{I2cMock as I2c, I2cMockError as Error};
    /// # use ht16k33::DisplayData;
    /// # use quadigit_phat::*;
    ///
    /// # let mut phat = PHat::new(I2c::new(), 0u8);
    /// # phat.initialize();
    /// phat.update_digit(Digit::DIGIT_0, [0, 0b1111_1111]);
    ///
    /// assert_eq!(phat.display_buffer()[0..2], [DisplayData::from_byte(0), DisplayData::from_byte(0b1111_1111)]);
    /// ```
    fn update_digit<T>(&mut self, digit: Digit, t: T)
    where
        T: Into<Char>,
    {
        let raw: [u8; 2] = t.into().into();
        raw.iter()
            .copied()
            .map(DisplayData::from_byte)
            .zip(digit)
            .for_each(|(c, d)| self.update_row_mask(d, c));
    }

    /// Write to digit
    ///
    /// # Arguments
    /// * `digit`   - Digit to write to
    /// * `c`       - Char to write with
    ///
    /// # Examples
    /// ```
    /// # use ht16k33::i2c_mock::{I2cMock as I2c, I2cMockError as Error};
    /// # use ht16k33::DisplayData;
    /// # use quadigit_phat::*;
    /// # use core::convert::TryFrom;
    ///
    /// # let mut phat = PHat::new(I2c::new(), 0u8);
    /// # phat.initialize();
    /// phat.set_digit(Digit::DIGIT_3, Char::from('*'));
    /// assert_eq!(phat.display_buffer()[6..8], [DisplayData::from_byte(0b0011_1111), DisplayData::from_byte(0b1100_0000)]);
    ///
    /// phat.set_digit(Digit::DIGIT_3, [0b0011_1111, 0b1100_0000]);
    /// assert_eq!(phat.display_buffer()[6..8], [DisplayData::from_byte(0b0011_1111), DisplayData::from_byte(0b1100_0000)]);
    ///
    /// phat.set_digit(Digit::DIGIT_3, '*');
    /// assert_eq!(phat.display_buffer()[6..8], [DisplayData::from_byte(0b0011_1111), DisplayData::from_byte(0b1100_0000)]);
    /// ```
    fn set_digit<T>(&mut self, digit: Digit, t: T) -> Result<E>
    where
        T: Into<Char>,
    {
        let raw: [u8; 2] = t.into().into();
        raw.iter()
            .copied()
            .map(DisplayData::from_byte)
            .zip(digit)
            .try_for_each(|(c, d)| self.set_row_mask(d, c))
    }

    /// Print a 
    /// 
    /// ```
    ///
    /// ```
    fn print_offset<T, I>(&mut self, offset: Digit, iter: I) -> Result<E>
    where
        T: Into<Char>,
        I: IntoIterator<Item = T>,
    {
        iter.into_iter()
            .flat_map(T::into)
            .zip(offset)
            .try_for_each(|(c, d)| self.set_row_mask(d, c))
    }

    /// Print a maximum of 4 values to respective digit on display
    ///
    /// Values not fitting on display will be ignored
    ///
    /// ```
    /// # use ht16k33::i2c_mock::{I2cMock as I2c, I2cMockError as Error};
    /// # use ht16k33::DisplayData;
    /// # use quadigit_phat::*;
    /// # use core::convert::TryFrom;
    ///
    /// # let mut phat = PHat::new(I2c::new(), 0u8);
    /// # phat.initialize();
    /// phat.print("TEST".chars());
    /// ```
    fn print<T, I>(&mut self, iter: I) -> Result<E>
    where
        T: Into<Char>,
        I: IntoIterator<Item = T>,
    {
        self.print_offset(Digit::default(), iter)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::iter::once;
    use ht16k33::i2c_mock::{I2cMock as I2c, I2cMockError as Error};

    #[test]
    fn test_print() -> Result<Error> {
        let mut phat0 = PHat::new(I2c::new(), 0);
        phat0.set_digit(Digit::DIGIT_0, 'T')?;
        phat0.set_digit(Digit::DIGIT_1, 'E')?;
        phat0.set_digit(Digit::DIGIT_2, 'S')?;
        phat0.set_digit(Digit::DIGIT_3, 'T')?;
        
        let mut phat1 = PHat::new(I2c::new(), 0u8);
        phat1.print("TEST".chars())?;

        assert_eq!(phat0.display_buffer(), phat1.display_buffer());

        Ok(())
    }
}
