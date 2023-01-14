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

use std::{iter::Peekable, ops::BitOr};

use dyn_iter::DynIter;
use itertools::Itertools;
use phf::phf_map;

use self::mask::{QUESTION_MARK, PERIOD};

macro_rules! font {
    [$(($char:expr => $const:ident : $bits:expr)),* $(,)*] => {
        /// TODO: Insert documentation
        pub mod mask {
            $(pub const $const: u16 = $bits;)*
        }

        /// TODO: Insert good documentation
        pub static ASCII: phf::Map<u8, u16> = phf_map! {
            $($char => mask::$const,)*
        };
    };
}

font! {
    (b' ' => SPACE:                 0b0000_0000_0000_0000),
    (b'!' => EXCLAMATION_POINT:     0b0000_0000_0000_0110),
    (b'"' => QUOTATION_MARK:        0b0000_0010_0010_0000),
    (b'#' => NUMBER_SIGN:           0b0001_0010_1100_1110),
    (b'$' => DOLLAR_SIGN:           0b0001_0010_1110_1101),
    (b'%' => PERCENT_SIGN:          0b0000_1100_0010_0100),
    (b'&' => AMPERSAND:             0b0010_0011_0101_1101),
    (b'\''=> APOSTROPHE:            0b0000_0100_0000_0000),
    (b'(' => LEFT_PARENTHESIS:      0b0010_0100_0000_0000),
    (b')' => RIGHT_PARENTHESIS:     0b0000_1001_0000_0000),
    (b'*' => ASTERISK:              0b0011_1111_1100_0000),
    (b'+' => PLUS_SIGN:             0b0001_0010_1100_0000),
    (b',' => COMMA:                 0b0000_1000_0000_0000),
    (b'-' => HYPHEN_MINUS:          0b0000_0000_1100_0000),
    (b'.' => PERIOD:                0b0100_0000_0000_0000),
    (b'/' => SLASH:                 0b0000_1100_0000_0000),
    (b'0' => DIGIT_ZERO:            0b0000_1100_0011_1111),
    (b'1' => DIGIT_ONE:             0b0000_0000_0000_0110),
    (b'2' => DIGIT_TWO:             0b0000_0000_1101_1011),
    (b'3' => DIGIT_THREE:           0b0000_0000_1000_1111),
    (b'4' => DIGIT_FOUR:            0b0000_0000_1110_0110),
    (b'5' => DIGIT_FIVE:            0b0010_0000_0110_1001),
    (b'6' => DIGIT_SIX:             0b0000_0000_1111_1101),
    (b'7' => DIGIT_SEVEN:           0b0000_0000_0000_0111),
    (b'8' => DIGIT_EIGHT:           0b0000_0000_1111_1111),
    (b'9' => DIGIT_NINE:            0b0000_0000_1110_1111),
    (b':' => COLON:                 0b0001_0010_0000_0000),
    (b';' => SEMI_COLON:            0b0000_1010_0000_0000),
    (b'<' => LESS_THAN_SIGN:        0b0010_0100_0000_0000),
    (b'=' => EQUALS_SIGN:           0b0000_0000_1100_1000),
    (b'>' => GREATER_THAN_SIGN:     0b0000_1001_0000_0000),
    (b'?' => QUESTION_MARK:         0b0001_0000_1000_0011),
    (b'@' => AT_SIGN:               0b0000_0010_1011_1011),
    (b'A' => LETTER_A:              0b0000_0000_1111_0111),
    (b'B' => LETTER_B:              0b0001_0010_1000_1111),
    (b'C' => LETTER_C:              0b0000_0000_0011_1001),
    (b'D' => LETTER_D:              0b0001_0010_0000_1111),
    (b'E' => LETTER_E:              0b0000_0000_1111_1001),
    (b'F' => LETTER_F:              0b0000_0000_0111_0001),
    (b'G' => LETTER_G:              0b0000_0000_1011_1101),
    (b'H' => LETTER_H:              0b0000_0000_1111_0110),
    (b'I' => LETTER_I:              0b0001_0010_0000_0000),
    (b'J' => LETTER_J:              0b0000_0000_0001_1110),
    (b'K' => LETTER_K:              0b0010_0100_0111_0000),
    (b'L' => LETTER_L:              0b0000_0000_0011_1000),
    (b'M' => LETTER_M:              0b0000_0101_0011_0110),
    (b'N' => LETTER_N:              0b0010_0001_0011_0110),
    (b'O' => LETTER_O:              0b0000_0000_0011_1111),
    (b'P' => LETTER_P:              0b0000_0000_1111_0011),
    (b'Q' => LETTER_Q:              0b0010_0000_0011_1111),
    (b'R' => LETTER_R:              0b0010_0000_1111_0011),
    (b'S' => LETTER_S:              0b0000_0000_1110_1101),
    (b'T' => LETTER_T:              0b0001_0010_0000_0001),
    (b'U' => LETTER_U:              0b0000_0000_0011_1110),
    (b'V' => LETTER_V:              0b0000_1100_0011_0000),
    (b'W' => LETTER_W:              0b0010_1000_0011_0110),
    (b'X' => LETTER_X:              0b0010_1101_0000_0000),
    (b'Y' => LETTER_Y:              0b0010_1101_0000_0000),
    (b'Z' => LETTER_Z:              0b0000_1100_0000_1001),
    (b'[' => LEFT_SQUARE_BRACKET:   0b0000_0000_0011_1001),
    (b'\\'=> BACKSLASH:             0b0010_0001_0000_0000),
    (b']' => RIGHT_SQUARE_BRACKET:  0b0000_0000_0000_1111),
    (b'^' => CARET:                 0b0000_1100_0000_0011),
    (b'_' => UNDERSCORE:            0b0000_0000_0000_1000),
    (b'`' => GRAVE_ACCENT:          0b0000_0001_0000_0000),
    (b'a' => SMALL_LETTER_A:        0b0001_0000_0101_1000),
    (b'b' => SMALL_LETTER_B:        0b0010_0000_0111_1000),
    (b'c' => SMALL_LETTER_C:        0b0000_0000_1101_1000),
    (b'd' => SMALL_LETTER_D:        0b0000_1000_1000_1110),
    (b'e' => SMALL_LETTER_E:        0b0000_1000_0101_1000),
    (b'f' => SMALL_LETTER_F:        0b0000_0000_0111_0001),
    (b'g' => SMALL_LETTER_G:        0b0000_0100_1000_1110),
    (b'h' => SMALL_LETTER_H:        0b0001_0000_0111_0000),
    (b'i' => SMALL_LETTER_I:        0b0001_0000_0000_0000),
    (b'j' => SMALL_LETTER_J:        0b0000_0000_0000_1110),
    (b'k' => SMALL_LETTER_K:        0b0011_0110_0000_0000),
    (b'l' => SMALL_LETTER_L:        0b0000_0000_0011_0000),
    (b'm' => SMALL_LETTER_M:        0b0001_0000_1101_0100),
    (b'n' => SMALL_LETTER_N:        0b0001_0000_0101_0000),
    (b'o' => SMALL_LETTER_O:        0b0000_0000_1101_1100),
    (b'p' => SMALL_LETTER_P:        0b0000_0001_0111_0000),
    (b'q' => SMALL_LETTER_Q:        0b0000_0100_1000_0110),
    (b'r' => SMALL_LETTER_R:        0b0000_0000_0101_0000),
    (b's' => SMALL_LETTER_S:        0b0010_0000_1000_1000),
    (b't' => SMALL_LETTER_T:        0b0000_0000_0111_1000),
    (b'u' => SMALL_LETTER_U:        0b0000_0000_0001_1100),
    (b'v' => SMALL_LETTER_V:        0b0010_0000_0000_0100),
    (b'w' => SMALL_LETTER_W:        0b0010_1000_0001_0100),
    (b'x' => SMALL_LETTER_X:        0b0010_1000_1100_0000),
    (b'y' => SMALL_LETTER_Y:        0b0010_0000_0000_1100),
    (b'z' => SMALL_LETTER_Z:        0b0000_1000_0100_1000),
    (b'{' => LEFT_CURLY_BRACKET:    0b0000_1001_0100_1001),
    (b'|' => VERTICAL_LINE:         0b0001_0010_0000_0000),
    (b'}' => RIGHT_CURLY_BRACKET:   0b0010_0100_1000_1001),
    (b'~' => TILDE:                 0b0000_0101_0010_0000),
}

pub fn try_ascii(c: u8) -> Option<u16> {
    ASCII.get(&c).copied()
}

pub fn ascii(c: u8) -> u16 {
    try_ascii(c).unwrap_or(QUESTION_MARK)
}

pub fn ascii_iter(i: DynIter<u8>) -> DynIter<u16> {
    DynIter::new(i.map(ascii))
}

pub struct PeriodInterpreter<I, T>
where 
    I: Iterator<Item = T>,
    T: Copy + BitOr<Output = T> + PartialEq
{
    iter: Peekable<I>,
    mask: T,
    mask_counter: usize,
}

impl<I, T> PeriodInterpreter<I, T> 
where 
    I: Iterator<Item = T>,
    T: Copy + BitOr<Output = T> + PartialEq
{
    pub fn new(iter: I, mask: T) -> Self {
        Self {
            iter: iter.peekable(),
            mask,
            mask_counter: 0,
        }
    }

    pub fn from_peekable(iter: Peekable<I>, mask: T) -> Self {
        Self {
            iter,
            mask,
            mask_counter: 0,
        }
    }
}

impl<I, T> Iterator for PeriodInterpreter<I, T>
where 
    I: Iterator<Item = T>,
    T: Copy + BitOr<Output = T> + PartialEq
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.mask_counter != 0 {
            self.mask_counter -= 2;
            return Some(self.mask);
        }

        let next = self.iter.next();
        self.mask_counter = self.iter.peeking_take_while(|b| *b == self.mask)
            .count();
            
        if self.mask_counter % 2 == 1 {
            self.mask_counter -= 1;
            next.map(|n| n | self.mask)
        } else { 
            next
        }
    }
}

pub fn period_iter(i: DynIter<u8>) -> impl Iterator<Item = u16> + '_ {
    PeriodInterpreter::new(DynIter::new(i.map(ascii).peekable()), PERIOD)
}

#[cfg(test)]
mod tests {
    use assert_hex::assert_eq_hex;
    use crate::fonts::mask::{DIGIT_ZERO, PERIOD, DIGIT_ONE, DIGIT_TWO, DIGIT_THREE};

    use super::*;

    #[test]
    fn dot_interpreter_unescaped() {
        assert_eq_hex!(
            vec![DIGIT_ZERO, DIGIT_ONE | PERIOD, DIGIT_TWO, DIGIT_THREE],
            period_iter(DynIter::new("01.23".bytes())).collect::<Vec<u16>>()
        );
    }

    #[test]
    fn dot_interpreter_escaped() {
        assert_eq_hex!(
            vec![DIGIT_ZERO, DIGIT_ONE, PERIOD, DIGIT_TWO],
            period_iter(DynIter::new("01..2".bytes())).collect::<Vec<u16>>()
        );
    }

    #[test]
    fn dot_interpreter_escaped_unescaped() {
        assert_eq_hex!(
            vec![DIGIT_ZERO, DIGIT_ONE | PERIOD, PERIOD, DIGIT_TWO],
            period_iter(DynIter::new("01...2".bytes())).collect::<Vec<u16>>()
        );
    }

    #[test]
    fn dot_interpreter_escaped_escaped() {
        assert_eq_hex!(
            vec![DIGIT_ZERO, DIGIT_ONE, PERIOD, PERIOD],
            period_iter(DynIter::new("01....".bytes())).collect::<Vec<u16>>()
        );
    }

    #[test]
    fn dot_interpreter_three_dots() {
        assert_eq_hex!(
            vec![PERIOD, PERIOD],
            period_iter(DynIter::new("...".bytes())).collect::<Vec<u16>>()
        );
    }

    #[test]
    fn dot_interpreter_four_dots() {
        assert_eq_hex!(
            vec![PERIOD, PERIOD],
            period_iter(DynIter::new("....".bytes())).collect::<Vec<u16>>()
        );
    }

    #[test]
    fn dot_interpreter_period_followed() {
        assert_eq_hex!(
            vec![PERIOD, DIGIT_ONE],
            period_iter(DynIter::new(".1".bytes())).collect::<Vec<u16>>()
        )
    }
}
