// This file was ported from https://github.com/pimoroni/fourletter-phat.git
// which was originally borrowed from https://github.com/adafruit/Adafruit_Python_LED_Backpack
// Original copyright included below:
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.

use crate::error::Error;
use ::core::array;
use ::core::convert::{From, Into};
use ::core::ops::Index;
use ::phf::phf_map;

/// TODO Insert GOOD DOCUMENTATION here
pub static ASCII: phf::Map<char, [u8; 2]> = phf_map! {
    ' ' =>  [0b0000_0000, 0b0000_0000],
    '!' =>  [0b0000_0110, 0b0000_0000],
    '"' =>  [0b0010_0000, 0b0000_0010],
    '#' =>  [0b1100_1110, 0b0001_0010],
    '$' =>  [0b1110_1101, 0b0001_0010],
    '%' =>  [0b0010_0100, 0b0000_1100],
    '&' =>  [0b0101_1101, 0b0010_0011],
    '\'' => [0b0000_0000, 0b0000_0100],
    '(' =>  [0b0000_0000, 0b0010_0100],
    ')' =>  [0b0000_0000, 0b0000_1001],
    '*' =>  [0b1100_0000, 0b0011_1111],
    '+' =>  [0b1100_0000, 0b0001_0010],
    ',' =>  [0b0000_0000, 0b0000_1000],
    '-' =>  [0b1100_0000, 0b0000_0000],
    '.' =>  [0b0000_0000, 0b0000_0000],
    '/' =>  [0b0000_0000, 0b0000_1100],
    '0' =>  [0b0011_1111, 0b0000_1100],
    '1' =>  [0b0000_0110, 0b0000_0000],
    '2' =>  [0b1101_1011, 0b0000_0000],
    '3' =>  [0b1000_1111, 0b0000_0000],
    '4' =>  [0b1110_0110, 0b0000_0000],
    '5' =>  [0b0110_1001, 0b0010_0000],
    '6' =>  [0b1111_1101, 0b0000_0000],
    '7' =>  [0b0000_0111, 0b0000_0000],
    '8' =>  [0b1111_1111, 0b0000_0000],
    '9' =>  [0b1110_1111, 0b0000_0000],
    ':' =>  [0b0000_0000, 0b0001_0010],
    ';' =>  [0b0000_0000, 0b0000_1010],
    '<' =>  [0b0000_0000, 0b0010_0100],
    '=' =>  [0b1100_1000, 0b0000_0000],
    '>' =>  [0b0000_0000, 0b0000_1001],
    '?' =>  [0b1000_0011, 0b0001_0000],
    '@' =>  [0b1011_1011, 0b0000_0010],
    'A' =>  [0b1111_0111, 0b0000_0000],
    'B' =>  [0b1000_1111, 0b0001_0010],
    'C' =>  [0b0011_1001, 0b0000_0000],
    'D' =>  [0b0000_1111, 0b0001_0010],
    'E' =>  [0b1111_1001, 0b0000_0000],
    'F' =>  [0b0111_0001, 0b0000_0000],
    'G' =>  [0b1011_1101, 0b0000_0000],
    'H' =>  [0b1111_0110, 0b0000_0000],
    'I' =>  [0b0000_0000, 0b0001_0010],
    'J' =>  [0b0001_1110, 0b0000_0000],
    'K' =>  [0b0111_0000, 0b0010_0100],
    'L' =>  [0b0011_1000, 0b0000_0000],
    'M' =>  [0b0011_0110, 0b0000_0101],
    'N' =>  [0b0011_0110, 0b0010_0001],
    'O' =>  [0b0011_1111, 0b0000_0000],
    'P' =>  [0b1111_0011, 0b0000_0000],
    'Q' =>  [0b0011_1111, 0b0010_0000],
    'R' =>  [0b1111_0011, 0b0010_0000],
    'S' =>  [0b1110_1101, 0b0000_0000],
    'T' =>  [0b0000_0001, 0b0001_0010],
    'U' =>  [0b0011_1110, 0b0000_0000],
    'V' =>  [0b0011_0000, 0b0000_1100],
    'W' =>  [0b0011_0110, 0b0010_1000],
    'X' =>  [0b0000_0000, 0b0010_1101],
    'Y' =>  [0b0000_0000, 0b0001_0101],
    'Z' =>  [0b0000_1001, 0b0000_1100],
    '[' =>  [0b0011_1001, 0b0000_0000],
    '\\' => [0b0000_0000, 0b0010_0001],
    ']' =>  [0b0000_1111, 0b0000_0000],
    '^' =>  [0b0000_0011, 0b0000_1100],
    '_' =>  [0b0000_1000, 0b0000_0000],
    '`' =>  [0b0000_0000, 0b0000_0001],
    'a' =>  [0b0101_1000, 0b0001_0000],
    'b' =>  [0b0111_1000, 0b0010_0000],
    'c' =>  [0b1101_1000, 0b0000_0000],
    'd' =>  [0b1000_1110, 0b0000_1000],
    'e' =>  [0b0101_1000, 0b0000_1000],
    'f' =>  [0b0111_0001, 0b0000_0000],
    'g' =>  [0b1000_1110, 0b0000_0100],
    'h' =>  [0b0111_0000, 0b0001_0000],
    'i' =>  [0b0000_0000, 0b0001_0000],
    'j' =>  [0b0000_1110, 0b0000_0000],
    'k' =>  [0b0000_0000, 0b0011_0110],
    'l' =>  [0b0011_0000, 0b0000_0000],
    'm' =>  [0b1101_0100, 0b0001_0000],
    'n' =>  [0b0101_0000, 0b0001_0000],
    'o' =>  [0b1101_1100, 0b0000_0000],
    'p' =>  [0b0111_0000, 0b0000_0001],
    'q' =>  [0b1000_0110, 0b0000_0100],
    'r' =>  [0b0101_0000, 0b0000_0000],
    's' =>  [0b1000_1000, 0b0010_0000],
    't' =>  [0b0111_1000, 0b0000_0000],
    'u' =>  [0b0001_1100, 0b0000_0000],
    'v' =>  [0b0000_0100, 0b0010_0000],
    'w' =>  [0b0001_0100, 0b0010_1000],
    'x' =>  [0b1100_0000, 0b0010_1000],
    'y' =>  [0b0000_1100, 0b0010_0000],
    'z' =>  [0b0100_1000, 0b0000_1000],
    '{' =>  [0b0100_1001, 0b0000_1001],
    '|' =>  [0b0000_0000, 0b0001_0010],
    '}' =>  [0b1000_1001, 0b0010_0100],
    '~' =>  [0b0010_0000, 0b0000_0101],
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Char([u8; 2]);

impl Default for Char {
    fn default() -> Char {
        Char::from([0, 0])
    }
}

impl Index<usize> for Char {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl From<[u8; 2]> for Char {
    fn from(bits: [u8; 2]) -> Self {
        Char(bits)
    }
}

impl From<char> for Char {
    /// Note that this function is unchecked and will use the default character
    /// if no mapping exist. Use `.checked_from()` if you are paranoid.
    fn from(c: char) -> Self {
        ASCII.get(&c).map(|&c| Self::from(c)).unwrap_or_default()
    }
}

impl Into<[u8; 2]> for Char {
    fn into(self) -> [u8; 2] {
        [self[0], self[1]]
    }
}

impl Char {
    fn checked_from(c: char) -> Result<Self, Error> {
        ASCII
            .get(&c)
            .map(|&c| Self::from(c))
            .ok_or(Error::NoMapping { character: c })
    }
}

impl IntoIterator for Char {
    type Item = u8;
    type IntoIter = array::IntoIter<Self::Item, 2>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self.0)
    }
}
