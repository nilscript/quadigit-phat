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

/// TODO Insert GOOD DOCUMENTATION here
pub static ASCII: phf::Map<char, [u8; 2]> = phf_map! {
    ' ' => [0b0000_0000, 0b0000_0000],
    '!' => [0b0000_0000, 0b0000_0110],
    '"' => [0b0000_0010, 0b0010_0000],
    '#' => [0b0001_0010, 0b1100_1110],
    '$' => [0b0001_0010, 0b1110_1101],
    '%' => [0b0000_1100, 0b0010_0100],
    '&' => [0b0010_0011, 0b0101_1101],
    '\'' => [0b0000_0100, 0b0000_0000],
    '(' => [0b0010_0100, 0b0000_0000],
    ')' => [0b0000_1001, 0b0000_0000],
    '*' => [0b0011_1111, 0b1100_0000],
    '+' => [0b0001_0010, 0b1100_0000],
    ',' => [0b0000_1000, 0b0000_0000],
    '-' => [0b0000_0000, 0b1100_0000],
    '.' => [0b0000_0000, 0b0000_0000],
    '/' => [0b0000_1100, 0b0000_0000],
    '0' => [0b0000_1100, 0b0011_1111],
    '1' => [0b0000_0000, 0b0000_0110],
    '2' => [0b0000_0000, 0b1101_1011],
    '3' => [0b0000_0000, 0b1000_1111],
    '4' => [0b0000_0000, 0b1110_0110],
    '5' => [0b0010_0000, 0b0110_1001],
    '6' => [0b0000_0000, 0b1111_1101],
    '7' => [0b0000_0000, 0b0000_0111],
    '8' => [0b0000_0000, 0b1111_1111],
    '9' => [0b0000_0000, 0b1110_1111],
    ':' => [0b0001_0010, 0b0000_0000],
    ';' => [0b0000_1010, 0b0000_0000],
    '<' => [0b0010_0100, 0b0000_0000],
    '=' => [0b0000_0000, 0b1100_1000],
    '>' => [0b0000_1001, 0b0000_0000],
    '?' => [0b0001_0000, 0b1000_0011],
    '@' => [0b0000_0010, 0b1011_1011],
    'A' => [0b0000_0000, 0b1111_0111],
    'B' => [0b0001_0010, 0b1000_1111],
    'C' => [0b0000_0000, 0b0011_1001],
    'D' => [0b0001_0010, 0b0000_1111],
    'E' => [0b0000_0000, 0b1111_1001],
    'F' => [0b0000_0000, 0b0111_0001],
    'G' => [0b0000_0000, 0b1011_1101],
    'H' => [0b0000_0000, 0b1111_0110],
    'I' => [0b0001_0010, 0b0000_0000],
    'J' => [0b0000_0000, 0b0001_1110],
    'K' => [0b0010_0100, 0b0111_0000],
    'L' => [0b0000_0000, 0b0011_1000],
    'M' => [0b0000_0101, 0b0011_0110],
    'N' => [0b0010_0001, 0b0011_0110],
    'O' => [0b0000_0000, 0b0011_1111],
    'P' => [0b0000_0000, 0b1111_0011],
    'Q' => [0b0010_0000, 0b0011_1111],
    'R' => [0b0010_0000, 0b1111_0011],
    'S' => [0b0000_0000, 0b1110_1101],
    'T' => [0b0001_0010, 0b0000_0001],
    'U' => [0b0000_0000, 0b0011_1110],
    'V' => [0b0000_1100, 0b0011_0000],
    'W' => [0b0010_1000, 0b0011_0110],
    'X' => [0b0010_1101, 0b0000_0000],
    'Y' => [0b0001_0101, 0b0000_0000],
    'Z' => [0b0000_1100, 0b0000_1001],
    '[' => [0b0000_0000, 0b0011_1001],
    '\\' => [0b0010_0001, 0b0000_0000],
    ']' => [0b0000_0000, 0b0000_1111],
    '^' => [0b0000_1100, 0b0000_0011],
    '_' => [0b0000_0000, 0b0000_1000],
    '`' => [0b0000_0001, 0b0000_0000],
    'a' => [0b0001_0000, 0b0101_1000],
    'b' => [0b0010_0000, 0b0111_1000],
    'c' => [0b0000_0000, 0b1101_1000],
    'd' => [0b0000_1000, 0b1000_1110],
    'e' => [0b0000_1000, 0b0101_1000],
    'f' => [0b0000_0000, 0b0111_0001],
    'g' => [0b0000_0100, 0b1000_1110],
    'h' => [0b0001_0000, 0b0111_0000],
    'i' => [0b0001_0000, 0b0000_0000],
    'j' => [0b0000_0000, 0b0000_1110],
    'k' => [0b0011_0110, 0b0000_0000],
    'l' => [0b0000_0000, 0b0011_0000],
    'm' => [0b0001_0000, 0b1101_0100],
    'n' => [0b0001_0000, 0b0101_0000],
    'o' => [0b0000_0000, 0b1101_1100],
    'p' => [0b0000_0001, 0b0111_0000],
    'q' => [0b0000_0100, 0b1000_0110],
    'r' => [0b0000_0000, 0b0101_0000],
    's' => [0b0010_0000, 0b1000_1000],
    't' => [0b0000_0000, 0b0111_1000],
    'u' => [0b0000_0000, 0b0001_1100],
    'v' => [0b0010_0000, 0b0000_0100],
    'w' => [0b0010_1000, 0b0001_0100],
    'x' => [0b0010_1000, 0b1100_0000],
    'y' => [0b0010_0000, 0b0000_1100],
    'z' => [0b0000_1000, 0b0100_1000],
    '{' => [0b0000_1001, 0b0100_1001],
    '|' => [0b0001_0010, 0b0000_0000],
    '}' => [0b0010_0100, 0b1000_1001],
    '~' => [0b0000_0101, 0b0010_0000],
};
