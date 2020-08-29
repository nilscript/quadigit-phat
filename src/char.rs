use crate::bitmap::ASCII;
use crate::error::Error;
use ::core::array;
use ::core::convert::{From, Into};
use ::core::ops::Index;
use ::ht16k33::DisplayData as Data;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Char([Data; 2]);

impl Default for Char {
    fn default() -> Char {
        Char::from([0, 0])
    }
}

impl Index<usize> for Char {
    type Output = Data;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl From<[u8; 2]> for Char {
    fn from(bits: [u8; 2]) -> Self {
        Char([Data::from_byte(bits[0]), Data::from_byte(bits[1])])
    }
}

impl From<char> for Char {
    fn from(c: char) -> Self {
        Self::from(*ASCII.get(&c).unwrap_or(&[0, 0]))
    }
}

impl Into<[u8; 2]> for Char {
    fn into(self) -> [u8; 2] {
        [self[0].bits(), self[1].bits()]
    }
}

impl Char {
    fn checked_from(c: char) -> Result<Self, Error> {
        Ok(Self::from(
            *ASCII.get(&c).ok_or(Error::NoMapping { character: c })?,
        ))
    }
}

impl IntoIterator for Char {
    type Item = Data;
    type IntoIter = array::IntoIter<Self::Item, 2>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self.0)
    }
}
