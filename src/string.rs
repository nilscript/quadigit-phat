use crate::char::Char;
use crate::constant::STRING_SIZE;
use ::core::array;
use ::core::convert::{From, Into};
use ::core::iter::FromIterator;
use ::core::ops::Index;
use ::core::ops::IndexMut;

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

// Singles
impl<T> From<T> for String
where
    T: Into<Char>,
{
    fn from(c: T) -> Self {
        let mut string = Self::default();
        string[0] = c.into();
        string
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

// String
/// Internally calls `.chars()`
impl From<&str> for String {
    fn from(string: &str) -> Self {
        Self::from_iter(string.chars())
    }
}

// char iter. Maps and calls from_iter<Char>
impl FromIterator<char> for String {
    fn from_iter<I: IntoIterator<Item = char>>(chars: I) -> Self {
        Self::from_iter(chars.into_iter().map(Char::from))
    }
}

impl FromIterator<Char> for String {
    fn from_iter<I: IntoIterator<Item = Char>>(iter: I) -> Self {
        let mut string = Self::default();

        iter.into_iter()
            .zip(0..STRING_SIZE) // Throttle
            .for_each(|(c, i)| string[i] = c);

        string
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
mod tests {
    use super::*;

    #[test]
    fn test_from() {
        // Only the first 4 characters fits
        let test = String::from("Testing string");

        assert_eq!(
            test,
            String::from(&['T', 'e', 's', 't', 'i', 'n', 'g', ' ', 's', 'l', 'i', 'c', 'e'][..])
        );
        assert_eq!(test[0], String::from('T')[0]);
        assert_eq!(test[0], String::from(Char::from('T'))[0]);
        assert_eq!(test, String::from_iter("Testing ".chars()));

        println!("{:?}", test);
    }
}
