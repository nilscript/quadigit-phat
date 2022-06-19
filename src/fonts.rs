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

use dyn_iter::DynIter;
use phf::phf_map;

pub mod mask {
    pub const DOT: u16 = 0b0100_0000_0000_0000;
    pub const QUESTIONMARK: u16 = 0b0001_0000_1000_0011;
}

/// TODO Insert GOOD DOCUMENTATION
pub static ASCII: phf::Map<u8, u16> = phf_map! {
    b' ' => 0b0000_0000_0000_0000,
    b'!' => 0b0000_0000_0000_0110,
    b'"' => 0b0000_0010_0010_0000,
    b'#' => 0b0001_0010_1100_1110,
    b'$' => 0b0001_0010_1110_1101,
    b'%' => 0b0000_1100_0010_0100,
    b'&' => 0b0010_0011_0101_1101,
    b'\''=> 0b000_0010_0000_00000,
    b'(' => 0b0010_0100_0000_0000,
    b')' => 0b0000_1001_0000_0000,
    b'*' => 0b0011_1111_1100_0000,
    b'+' => 0b0001_0010_1100_0000,
    b',' => 0b0000_1000_0000_0000,
    b'-' => 0b0000_0000_1100_0000,
    b'.' => mask::DOT,
    b'/' => 0b0000_1100_0000_0000,
    b'0' => 0b0000_1100_0011_1111,
    b'1' => 0b0000_0000_0000_0110,
    b'2' => 0b0000_0000_1101_1011,
    b'3' => 0b0000_0000_1000_1111,
    b'4' => 0b0000_0000_1110_0110,
    b'5' => 0b0010_0000_0110_1001,
    b'6' => 0b0000_0000_1111_1101,
    b'7' => 0b0000_0000_0000_0111,
    b'8' => 0b0000_0000_1111_1111,
    b'9' => 0b0000_0000_1110_1111,
    b':' => 0b0001_0010_0000_0000,
    b';' => 0b0000_1010_0000_0000,
    b'<' => 0b0010_0100_0000_0000,
    b'=' => 0b0000_0000_1100_1000,
    b'>' => 0b0000_1001_0000_0000,
    b'?' => 0b0001_0000_1000_0011,
    b'@' => 0b0000_0010_1011_1011,
    b'A' => 0b0000_0000_1111_0111,
    b'B' => 0b0001_0010_1000_1111,
    b'C' => 0b0000_0000_0011_1001,
    b'D' => 0b0001_0010_0000_1111,
    b'E' => 0b0000_0000_1111_1001,
    b'F' => 0b0000_0000_0111_0001,
    b'G' => 0b0000_0000_1011_1101,
    b'H' => 0b0000_0000_1111_0110,
    b'I' => 0b0001_0010_0000_0000,
    b'J' => 0b0000_0000_0001_1110,
    b'K' => 0b0010_0100_0111_0000,
    b'L' => 0b0000_0000_0011_1000,
    b'M' => 0b0000_0101_0011_0110,
    b'N' => 0b0010_0001_0011_0110,
    b'O' => 0b0000_0000_0011_1111,
    b'P' => 0b0000_0000_1111_0011,
    b'Q' => 0b0010_0000_0011_1111,
    b'R' => 0b0010_0000_1111_0011,
    b'S' => 0b0000_0000_1110_1101,
    b'T' => 0b0001_0010_0000_0001,
    b'U' => 0b0000_0000_0011_1110,
    b'V' => 0b0000_1100_0011_0000,
    b'W' => 0b0010_1000_0011_0110,
    b'X' => 0b0010_1101_0000_0000,
    b'Y' => 0b0001_0101_0000_0000,
    b'Z' => 0b0000_1100_0000_1001,
    b'[' => 0b0000_0000_0011_1001,
    b'\\'=> 0b001_0000_1000_00000,
    b']' => 0b0000_0000_0000_1111,
    b'^' => 0b0000_1100_0000_0011,
    b'_' => 0b0000_0000_0000_1000,
    b'`' => 0b0000_0001_0000_0000,
    b'a' => 0b0001_0000_0101_1000,
    b'b' => 0b0010_0000_0111_1000,
    b'c' => 0b0000_0000_1101_1000,
    b'd' => 0b0000_1000_1000_1110,
    b'e' => 0b0000_1000_0101_1000,
    b'f' => 0b0000_0000_0111_0001,
    b'g' => 0b0000_0100_1000_1110,
    b'h' => 0b0001_0000_0111_0000,
    b'i' => 0b0001_0000_0000_0000,
    b'j' => 0b0000_0000_0000_1110,
    b'k' => 0b0011_0110_0000_0000,
    b'l' => 0b0000_0000_0011_0000,
    b'm' => 0b0001_0000_1101_0100,
    b'n' => 0b0001_0000_0101_0000,
    b'o' => 0b0000_0000_1101_1100,
    b'p' => 0b0000_0001_0111_0000,
    b'q' => 0b0000_0100_1000_0110,
    b'r' => 0b0000_0000_0101_0000,
    b's' => 0b0010_0000_1000_1000,
    b't' => 0b0000_0000_0111_1000,
    b'u' => 0b0000_0000_0001_1100,
    b'v' => 0b0010_0000_0000_0100,
    b'w' => 0b0010_1000_0001_0100,
    b'x' => 0b0010_1000_1100_0000,
    b'y' => 0b0010_0000_0000_1100,
    b'z' => 0b0000_1000_0100_1000,
    b'{' => 0b0000_1001_0100_1001,
    b'|' => 0b0001_0010_0000_0000,
    b'}' => 0b0010_0100_1000_1001,
    b'~' => 0b0000_0101_0010_0000,
};

pub fn try_ascii(c: u8) -> Option<u16> {
    ASCII.get(&c).copied()
}

pub fn ascii(c: u8) -> u16 {
    try_ascii(c).unwrap_or(mask::QUESTIONMARK)
}

pub fn ascii_iter(i: DynIter<u8>) -> DynIter<u16> {
    DynIter::new(i.map(ascii))
}

pub struct DotInterpreter<I>
where
    I: Iterator<Item = u16>,
{
    buf: [u16; 3],
    buf_len: usize,
    iter: I,
}

impl<I> DotInterpreter<I>
where
    I: Iterator<Item = u16>,
{
    pub fn new(iter: I) -> DotInterpreter<I> {
        let mut dot = DotInterpreter {
            buf: [0; 3],
            buf_len: 0,
            iter,
        };

        dot.refill(3);
        dot
    }

    fn buf(&self) -> &[u16] {
        &self.buf[..self.buf_len]
    }

    fn rotate(&mut self, n: usize) {
        self.buf.rotate_left(n);
        self.buf_len -= n;
    }

    fn refill(&mut self, n: usize) {
        for i in 0..n {
            self.iter.next().iter().for_each(|c| {
                self.buf[3 - n + i] = *c;
                self.buf_len += 1;
            });
        }
    }

    fn consume(&mut self, n: usize) {
        self.rotate(n);
        self.refill(n);
    }

    fn identity(&mut self, a: u16) -> Option<u16> {
        self.consume(1);
        Some(a)
    }

    fn mask(&mut self, a: u16, b: u16) -> Option<u16> {
        self.consume(2);
        Some(a | b)
    }

    fn escaped(&mut self, a: u16) -> Option<u16> {
        self.consume(2);
        Some(a)
    }
}

impl<I> Iterator for DotInterpreter<I>
where
    I: Iterator<Item = u16> + Sized,
{
    type Item = u16;

    fn next(&mut self) -> Option<Self::Item> {
        match *self.buf() {
            [a, b, c] if b == mask::DOT && c != mask::DOT => self.mask(a, b),
            [a, b, c] if b == mask::DOT && c == mask::DOT => self.escaped(a),
            [a, b, _] if b != mask::DOT => self.identity(a),
            [a, b] if b != mask::DOT => self.identity(a),
            [a, b] if b == mask::DOT => self.mask(a, b),
            [a] => self.identity(a),
            [..] => None, //
        }
    }
}

pub fn dot_iter(i: DynIter<u8>) -> DynIter<u16> {
    DynIter::new(DotInterpreter::new(i.map(ascii)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dot_interpreter() {
        assert_eq!(
            vec![0x0C3F, 0x4C3F, 0x0C3F, 0x0C3F],
            dot_iter(DynIter::new("00.00".bytes())).collect::<Vec<u16>>()
        );

        assert_eq!(
            vec![0x0C3F, 0x0C3F, mask::DOT, 0x0C3F],
            dot_iter(DynIter::new("00..0".bytes())).collect::<Vec<u16>>()
        );

        assert_eq!(
            vec![0x0C3F, 0x0C3F, mask::DOT, 0x0C3F],
            dot_iter(DynIter::new("00...0".bytes())).collect::<Vec<u16>>()
        );

        assert_eq!(
            vec![0x0C3F, 0x0C3F, mask::DOT, mask::DOT],
            dot_iter(DynIter::new("00....".bytes())).collect::<Vec<u16>>()
        );
    }
}
