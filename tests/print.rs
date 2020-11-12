use quadigit_phat::*;
use ht16k33::i2c_mock::{I2cMock as I2c, I2cMockError as Error};

#[test]
fn print() -> Result<(), Error> {
    let mut phat = PHat::new(I2c::new(), 0u8);
    phat.print("Test")
}
