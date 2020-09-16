use ::core::{array, fmt};
use ::enum_iterator::IntoEnumIterator;
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
    type Item = usize;
    type IntoIter = array::IntoIter<Self::Item, 2>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self.to_addr())
    }
}

impl Digit {
    /// Creates 2 Display Data Addresses
    /// These addresses are used as backends for the Digit interface.
    pub fn to_addr(&self) -> [usize; 2] {
        [*self as usize * 2, *self as usize * 2 + 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::vec::Vec;

    #[test]
    fn test_addresses() {
        let rows = (0..8).collect::<Vec<_>>();
        let digit = Digit::into_enum_iter().flatten().collect::<Vec<_>>();

        assert_eq!(rows, digit);
    }
}
