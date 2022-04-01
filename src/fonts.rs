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

use phf::phf_map;

pub const DOT_MASK: u16 =           0b0000_0000_0100_0000;
pub const QUESTIONMARK_MASK: u16 =  0b1000_0011_0001_0000;

/// TODO Insert GOOD DOCUMENTATION
pub static ASCII: phf::Map<u8, u16> = phf_map! {
    b' ' =>  0b0000_0000_0000_0000,
    b'!' =>  0b0000_0110_0000_0000,
    b'"' =>  0b0010_0000_0000_0010,
    b'#' =>  0b1100_1110_0001_0010,
    b'$' =>  0b1110_1101_0001_0010,
    b'%' =>  0b0010_0100_0000_1100,
    b'&' =>  0b0101_1101_0010_0011,
    b'\'' => 0b0000_0000_0000_0100,
    b'(' =>  0b0000_0000_0010_0100,
    b')' =>  0b0000_0000_0000_1001,
    b'*' =>  0b1100_0000_0011_1111,
    b'+' =>  0b1100_0000_0001_0010,
    b',' =>  0b0000_0000_0000_1000,
    b'-' =>  0b1100_0000_0000_0000,
    b'.' =>  DOT_MASK,
    b'/' =>  0b0000_0000_0000_1100,
    b'0' =>  0b0011_1111_0000_1100,
    b'1' =>  0b0000_0110_0000_0000,
    b'2' =>  0b1101_1011_0000_0000,
    b'3' =>  0b1000_1111_0000_0000,
    b'4' =>  0b1110_0110_0000_0000,
    b'5' =>  0b0110_1001_0010_0000,
    b'6' =>  0b1111_1101_0000_0000,
    b'7' =>  0b0000_0111_0000_0000,
    b'8' =>  0b1111_1111_0000_0000,
    b'9' =>  0b1110_1111_0000_0000,
    b':' =>  0b0000_0000_0001_0010,
    b';' =>  0b0000_0000_0000_1010,
    b'<' =>  0b0000_0000_0010_0100,
    b'=' =>  0b1100_1000_0000_0000,
    b'>' =>  0b0000_0000_0000_1001,
    b'?' =>  QUESTIONMARK_MASK,
    b'@' =>  0b1011_1011_0000_0010,
    b'A' =>  0b1111_0111_0000_0000,
    b'B' =>  0b1000_1111_0001_0010,
    b'C' =>  0b0011_1001_0000_0000,
    b'D' =>  0b0000_1111_0001_0010,
    b'E' =>  0b1111_1001_0000_0000,
    b'F' =>  0b0111_0001_0000_0000,
    b'G' =>  0b1011_1101_0000_0000,
    b'H' =>  0b1111_0110_0000_0000,
    b'I' =>  0b0000_0000_0001_0010,
    b'J' =>  0b0001_1110_0000_0000,
    b'K' =>  0b0111_0000_0010_0100,
    b'L' =>  0b0011_1000_0000_0000,
    b'M' =>  0b0011_0110_0000_0101,
    b'N' =>  0b0011_0110_0010_0001,
    b'O' =>  0b0011_1111_0000_0000,
    b'P' =>  0b1111_0011_0000_0000,
    b'Q' =>  0b0011_1111_0010_0000,
    b'R' =>  0b1111_0011_0010_0000,
    b'S' =>  0b1110_1101_0000_0000,
    b'T' =>  0b0000_0001_0001_0010,
    b'U' =>  0b0011_1110_0000_0000,
    b'V' =>  0b0011_0000_0000_1100,
    b'W' =>  0b0011_0110_0010_1000,
    b'X' =>  0b0000_0000_0010_1101,
    b'Y' =>  0b0000_0000_0001_0101,
    b'Z' =>  0b0000_1001_0000_1100,
    b'[' =>  0b0011_1001_0000_0000,
    b'\\' => 0b0000_0000_0010_0001,
    b']' =>  0b0000_1111_0000_0000,
    b'^' =>  0b0000_0011_0000_1100,
    b'_' =>  0b0000_1000_0000_0000,
    b'`' =>  0b0000_0000_0000_0001,
    b'a' =>  0b0101_1000_0001_0000,
    b'b' =>  0b0111_1000_0010_0000,
    b'c' =>  0b1101_1000_0000_0000,
    b'd' =>  0b1000_1110_0000_1000,
    b'e' =>  0b0101_1000_0000_1000,
    b'f' =>  0b0111_0001_0000_0000,
    b'g' =>  0b1000_1110_0000_0100,
    b'h' =>  0b0111_0000_0001_0000,
    b'i' =>  0b0000_0000_0001_0000,
    b'j' =>  0b0000_1110_0000_0000,
    b'k' =>  0b0000_0000_0011_0110,
    b'l' =>  0b0011_0000_0000_0000,
    b'm' =>  0b1101_0100_0001_0000,
    b'n' =>  0b0101_0000_0001_0000,
    b'o' =>  0b1101_1100_0000_0000,
    b'p' =>  0b0111_0000_0000_0001,
    b'q' =>  0b1000_0110_0000_0100,
    b'r' =>  0b0101_0000_0000_0000,
    b's' =>  0b1000_1000_0010_0000,
    b't' =>  0b0111_1000_0000_0000,
    b'u' =>  0b0001_1100_0000_0000,
    b'v' =>  0b0000_0100_0010_0000,
    b'w' =>  0b0001_0100_0010_1000,
    b'x' =>  0b1100_0000_0010_1000,
    b'y' =>  0b0000_1100_0010_0000,
    b'z' =>  0b0100_1000_0000_1000,
    b'{' =>  0b0100_1001_0000_1001,
    b'|' =>  0b0000_0000_0001_0010,
    b'}' =>  0b1000_1001_0010_0100,
    b'~' =>  0b0010_0000_0000_0101,
};

pub fn try_ascii(c: &u8) -> Option<u16> {
    ASCII.get(c).copied()
}

pub fn ascii(c: &u8) -> u16 {
    try_ascii(c).unwrap_or(QUESTIONMARK_MASK)
}

pub struct DotInterpreter<'a> {
    buf: [u16; 3],
    buf_len: usize,
    iter: &'a mut dyn Iterator<Item = u16>,
    mask: u16,
}

impl<'a> DotInterpreter<'a> {
    pub fn new(iter: &'a mut dyn Iterator<Item = u16>, mask: u16) -> DotInterpreter {
        let mut dot = DotInterpreter {
            buf: [0; 3],
            buf_len: 0,
            iter,
            mask
        };

        dot.refill(3);
        dot
    }

    fn rotate(&mut self, n: usize) {        
        self.buf.rotate_left(n);
        self.buf_len -= n;
    } 

    fn refill(&mut self, n: usize) {
        for (i, c) in self.iter.take(n).enumerate() {
            self.buf[3 - n + i] = c;
            self.buf_len += 1; 
        }
    }

    fn id(&mut self, a: u16) -> Option<u16> {
        self.rotate(1);
        self.refill(1);
        Some(a)
    }

    fn or(&mut self, a: u16, b: u16) -> Option<u16> {
        self.rotate(2);
        self.refill(2);
        Some(a | b)
    }

    fn esc(&mut self, a: u16) -> Option<u16> {
        self.rotate(2);
        self.refill(2);
        Some(a)
    }
}

impl<'a> Iterator for DotInterpreter<'a> {
    type Item = u16;

    fn next(&mut self) -> Option<Self::Item> {
        match self.buf_len {
            3 => match self.buf[..] {
                [a, b, _] if b != self.mask => self.id(a),
                [a, b, c] if b == self.mask && c != self.mask => self.or(a, b),
                [a, b, c] if b == self.mask && c == self.mask => self.esc(a),
                _ => unreachable!()
            }
            2 => match self.buf[0..2] {
                [a, b] if b == self.mask => self.or(a, b),
                [a, _] => self.id(a),
                _ => unreachable!()
            }
            1 => self.id(self.buf[0]),
            _ => None,
        }
    }
}


/*
/// Iterates over characters and maps them to buffer.
/// Periods or dots (.) are inlined to the previous character
/// unless escaped by another dot.
/// 
/// For examples look at `PHat::set_text()` method
pub fn compile_dot(buf: &mut [u8], mapper: fn(&u8) -> u16, chars: Chars) {
    // chars and digit are not synced with iterators by design.
    let mut chars = chars.peekable();
    let mut index = CharDataAddressPointer::iter().peekable();

    while let Some((c, i)) = chars.next().zip(index.peek()) {
        // Ordering of checks matters.
        // Edge cases are covered like general cases.
        if c == '.' && chars.next_if_eq(&'.').is_none() && *i != CharDataAddressPointer::P0 {
            buf[i.first() -1] |= fonts::DOT_MASK;

        // Character is not an escaped dot or a dot.
        } else {
            let c = mapper(&c);
            buf[i.first() ] = c[0];
            buf[i.second()] = c[1];
            index.next();
        }
    }
}
*/