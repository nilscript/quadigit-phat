use crate::char::Char;
use crate::constant::STRING_SIZE;
use ::core::array;
use ::core::convert::{From, Into};
use ::core::iter::FromIterator;
use ::core::ops::Index;
use ::core::ops::IndexMut;
use ::core::str::Chars;
use ::ht16k33::COMMONS_SIZE;

/// A constant sized collection of chars
///
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct String([Char; STRING_SIZE]);

impl Default for String {
    fn default() -> Self {
        String([Char::default(); STRING_SIZE])
    }
}

impl Index<usize> for String {
    type Output = Char;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for String {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl From<char> for String {
    fn from(c: char) -> Self {
        let mut string = Self::default();
        string[0] = Char::from(c);
        string
    }
}

impl From<Char> for String {
    fn from(c: Char) -> Self {
        let mut string = Self::default();
        string[0] = c;
        string
    }
}

impl From<Chars<'_>> for String {
    fn from(chars: Chars<'_>) -> Self {
        Self::from_iter(chars)
    }
}

// Slices
impl<T> From<&[T]> for String
where
    T: Copy + Into<Char>,
{
    fn from(slice: &[T]) -> Self {
        let mut string = Self::default();

        (0..STRING_SIZE).for_each(|i| string[i] = slice[i].into());

        string
    }
}

// &str to Chars
impl From<&str> for String {
    fn from(s: &str) -> Self {
        Self::from_iter(s.chars())
    }
}

// Chars iter. Maps and calls from_iter<Char>
impl FromIterator<char> for String {
    fn from_iter<I: IntoIterator<Item = char>>(chars: I) -> Self {
        Self::from_iter(chars.into_iter().map(Char::from))
    }
}

// Performs the conversion
impl FromIterator<Char> for String {
    fn from_iter<I: IntoIterator<Item = Char>>(iter: I) -> Self {
        let mut string = Self::default();

        iter.into_iter()
            .zip(0..STRING_SIZE) // Throttle
            .for_each(|(c, i)| string[i] = c);

        string
    }
}

impl Into<[u8; COMMONS_SIZE]> for String {
    fn into(self) -> [u8; COMMONS_SIZE] {
        let mut array = [0; COMMONS_SIZE];
        self.into_iter()
            .flatten()
            .enumerate()
            .for_each(|(i, c)| array[i] = c);
        array
    }
}

impl IntoIterator for String {
    type Item = Char;
    type IntoIter = array::IntoIter<Self::Item, STRING_SIZE>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self.0)
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::*;

    #[test]
    fn test_from() {
        let TEST = [0b0000_0001,0b0001_0010,0b1111_1001,0b0000_0000,0b1110_1101,0b0000_0000,0b0000_0001,0b0001_0010];

        // Only the first 4 characters fits
        assert_eq!(
            String::from(&['T', 'E', 'S', 'T', 'I', 'N', 'G', ' ', 'S', 'L', 'I', 'C', 'E'][..]),
            String::from(&['T', 'E', 'S', 'T'][..])
        );
        assert_eq!(String::from('T')[0], Char::from('T'));
        assert_eq!((String::from("TEST").into(): [u8; COMMONS_SIZE]), TEST)
    }
}
