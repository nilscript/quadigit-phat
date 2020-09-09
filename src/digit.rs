use ::core::{array, convert::TryFrom, fmt};
use ::enum_iterator::IntoEnumIterator;
use ::ht16k33::DisplayDataAddress as DataAddr;
use ::num_enum::{IntoPrimitive, TryFromPrimitive};

#[allow(non_camel_case_types)]
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    IntoPrimitive,
    TryFromPrimitive,
    IntoEnumIterator,
)]
#[repr(u8)]
pub enum Digit {
    DIGIT_0,
    DIGIT_1,
    DIGIT_2,
    DIGIT_3,
}

impl Default for Digit {
    fn default() -> Digit {
        Digit::DIGIT_0
    }
}

impl fmt::Display for Digit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Digit::{:?}", *self)
    }
}

impl IntoIterator for Digit {
    type Item = DataAddr;
    type IntoIter = array::IntoIter<Self::Item, 2>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self.to_addr())
    }
}

impl Digit {
    pub fn to_display_data_addresses(&self) -> [DataAddr; 2] {
        let row = [*self as u8 * 2, *self as u8 * 2 + 1];

        match (DataAddr::try_from(row[0]), DataAddr::try_from(row[1])) {
            (Ok(row0), Ok(row1)) => [row0, row1],
            _ => unreachable!(),
            // The greatest address for DataAddr is 0x07,
            // which is within limits for the last digit:
            // DIGIT_3 = 0x03; 3 * 2 + 1 == 7
        }
    }

    /// Creates 2 Display Data Addresses
    ///
    /// These addresses are used as backends for the Digit interface.
    pub fn to_addr(&self) -> [DataAddr; 2] {
        self.to_display_data_addresses()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::vec::Vec;

    #[test]
    fn test_addresses() {
        let rows = (0..8)
            .map(|n| DataAddr::try_from(n).unwrap())
            .collect::<Vec<DataAddr>>();
        let digit = Digit::into_enum_iter().flatten().collect::<Vec<DataAddr>>();

        assert_eq!(rows, digit);
    }
}
