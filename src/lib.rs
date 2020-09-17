#![cfg_attr(not(feature = "std"), no_std)]
#![feature(array_value_iter)] // Used in Digit::IntoIter
#![feature(generators, generator_trait)]
#![feature(type_ascription)]
//#![warn(missing_docs)]
#![allow(dead_code)]

mod char;
mod constant;
mod digit;
mod error;
pub mod prelude;
mod string;

pub use crate::char::Char;
pub use crate::constant::*;
pub use crate::digit::Digit;
pub use crate::error::Error;
pub use crate::string::String;
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

    fn update_decimal(&mut self, digit: Digit, decimal: bool) {
        let addr = digit.to_addr()[1];
        let decimal_mask = 0b0100_0000;

        match decimal {
            true => self.buffer[addr] |= decimal_mask,
            false => self.buffer[addr] &= !decimal_mask
        }
    }

    fn set_decimal(&mut self, digit: Digit, decimal: bool)  -> Result<E> {
        self.update_decimal(digit, decimal);
        self.set_row_mask(digit.to_addr()[1], self.buffer[digit.to_addr()[1]])
    }

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
    /// assert_eq!(phat.buffer[0..2], [0, 0b1111_1111]);
    /// ```
    fn update_digit<C>(&mut self, digit: Digit, c: C)
    where
        C: Into<Char>,
    {
        c.into()
            .into_iter()
            .zip(digit)
            .for_each(|(c, d)| self.buffer[d] = c);
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
    /// assert_eq!(phat.buffer[6..8], [0b1100_0000, 0b0011_1111]);
    ///
    /// phat.set_digit(Digit::DIGIT_3, [0b0011_1111, 0b1100_0000]);
    /// assert_eq!(phat.buffer[6..8], [0b0011_1111, 0b1100_0000]);
    ///
    /// phat.set_digit(Digit::DIGIT_3, '*');
    /// assert_eq!(phat.buffer[6..8], [0b1100_0000, 0b0011_1111]);
    /// ```
    fn set_digit<T>(&mut self, digit: Digit, t: T) -> Result<E>
    where
        T: Into<Char>,
    {
        self.update_digit(digit, t);
        self.write_display_buffer()
    }

    fn print_offset<T>(&mut self, offset: Digit, string: T) -> Result<E>
    where
        T: Into<String>,
    {
        (offset.to_addr()[0]..BITMAP_SIZE)
            // Flattens string to iterator of chars, chars to iterator of bytes
            .zip(string.into().into_iter().flatten())
            .try_for_each(|(d, c)| self.set_row_mask(d, c))
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
    /// phat.print("89AB");
    /// ```
    fn print<T>(&mut self, string: T) -> Result<E>
    where
        T: Into<String>,
    {
        self.print_offset(Digit::default(), string)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ht16k33::i2c_mock::{I2cMock as I2c, I2cMockError as Error};

    const TEST: [u8; 8] = [0b0000_0001, 0b0001_0010, 0b1111_1001, 0b0000_0000, 0b1110_1101, 0b0000_0000, 0b0000_0001, 0b0001_0010];

    #[test]
    fn test_print() -> Result<Error> {
        let mut phat = PHat::new(I2c::new(), 0u8);
        phat.print("TEST")?;
        assert_eq!(phat.buffer[..8], TEST);

        phat.print("TESTING")?;
        assert_eq!(phat.buffer[..8], TEST);

        phat.print(" ")?;
        assert_eq!(phat.buffer[0], 0b0000_0000);
        
        phat.print(' ')?;
        assert_eq!(phat.buffer[0], 0b0000_0000);

        phat.print(Char::from('*'))?;
        assert_eq!(phat.buffer[..2], [0b1100_0000, 0b0011_1111]);

        Ok(())
    }
}
