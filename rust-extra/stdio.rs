use prelude::*;

use term::color;

pub fn puts(string: &str, color: color::Color,
    putc: |char, color::Color|, new_line: ||) {
    for c in string.as_bytes().iter() {
        match *c as char {
            '\n'    => new_line(),
            c       => putc(c, color)
        }
    }
}
